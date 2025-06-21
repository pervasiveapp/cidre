/// PhotoKit - Photos framework
///
/// PhotoKit provides APIs to access and modify the user's Photos library.
/// This includes reading photo metadata, accessing image data, and managing photo collections.
use crate::{arc, define_obj_type, ns};

#[cfg(feature = "blocks")]
use crate::blocks;

// PHImageRequestOptionsDeliveryMode constants
pub const PH_IMAGE_REQUEST_OPTIONS_DELIVERY_MODE_OPPORTUNISTIC: i32 = 0;
pub const PH_IMAGE_REQUEST_OPTIONS_DELIVERY_MODE_HIGH_QUALITY_FORMAT: i32 = 1;
pub const PH_IMAGE_REQUEST_OPTIONS_DELIVERY_MODE_FAST_FORMAT: i32 = 2;

// PHAssetResourceType constants
pub const PH_ASSET_RESOURCE_TYPE_PHOTO: i32 = 1;
pub const PH_ASSET_RESOURCE_TYPE_VIDEO: i32 = 2;
pub const PH_ASSET_RESOURCE_TYPE_AUDIO: i32 = 3;
pub const PH_ASSET_RESOURCE_TYPE_ALTERNATE_PHOTO: i32 = 4;
pub const PH_ASSET_RESOURCE_TYPE_FULL_SIZE_PHOTO: i32 = 5;
pub const PH_ASSET_RESOURCE_TYPE_FULL_SIZE_VIDEO: i32 = 6;
pub const PH_ASSET_RESOURCE_TYPE_ADJUSTMENT_DATA: i32 = 7;
pub const PH_ASSET_RESOURCE_TYPE_ADJUSTMENT_BASE_PHOTO: i32 = 8;
pub const PH_ASSET_RESOURCE_TYPE_PAIRED_VIDEO: i32 = 9;
pub const PH_ASSET_RESOURCE_TYPE_FULL_SIZE_PAIRED_VIDEO: i32 = 10;
pub const PH_ASSET_RESOURCE_TYPE_ADJUSTMENT_BASE_PAIRED_VIDEO: i32 = 11;
pub const PH_ASSET_RESOURCE_TYPE_ADJUSTMENT_BASE_VIDEO: i32 = 12;

#[cfg(feature = "blocks")]
#[doc(alias = "PHImageDataRequestHandler")]
pub type ImageDataRequestHandler = blocks::EscBlock<
    fn(
        image_data: Option<&ns::Data>,
        data_uti: Option<&ns::String>,
        orientation: u32,
        info: Option<&ns::Id>,
    ),
>;

#[cfg(feature = "blocks")]
#[doc(alias = "PHContentEditingInputRequestCompletionHandler")]
pub type ContentEditingInputRequestCompletionHandler = blocks::EscBlock<
    fn(content_editing_input: Option<&ContentEditingInput>, info: Option<&ns::Id>),
>;

#[cfg(feature = "blocks")]
#[doc(alias = "PHAssetResourceDataRequestHandler")]
pub type AssetResourceDataRequestHandler = blocks::EscBlock<fn(data: Option<&ns::Data>)>;

define_obj_type!(
    #[doc(alias = "PHObject")]
    pub Object(ns::Id)
);

impl Object {
    pub fn local_identifier(&self) -> &ns::String {
        unsafe { ph_object_get_local_identifier(self) }
    }
}

define_obj_type!(
    #[doc(alias = "PHAsset")]
    pub Asset(Object)
);

impl Asset {
    /// Fetches assets with the given local identifiers.
    pub fn fetch_with_local_identifiers(
        identifiers: &ns::Array<ns::String>,
        options: Option<&FetchOptions>,
    ) -> arc::R<FetchResult> {
        unsafe {
            let result = ph_asset_fetch_with_local_identifiers(identifiers, options);
            std::mem::transmute(result)
        }
    }

    /// Fetches all assets with the given options.
    pub fn fetch_assets_with_options(options: Option<&FetchOptions>) -> arc::R<FetchResult> {
        unsafe {
            let result = ph_asset_fetch_assets_with_options(options);
            std::mem::transmute(result)
        }
    }

    /// Requests content editing input for the asset.
    /// This triggers iCloud download if needed and provides file URLs.
    #[cfg(feature = "blocks")]
    pub fn request_content_editing_input_with_options(
        &self,
        options: Option<&ContentEditingInputRequestOptions>,
        completion_handler: &mut ContentEditingInputRequestCompletionHandler,
    ) -> i32 {
        unsafe {
            ph_asset_request_content_editing_input_with_options(self, options, completion_handler)
        }
    }
}

define_obj_type!(
    #[doc(alias = "PHFetchResult")]
    pub FetchResult(ns::Id)
);

define_obj_type!(
    #[doc(alias = "PHImageManager")]
    pub ImageManager(ns::Id)
);

define_obj_type!(
    #[doc(alias = "PHImageRequestOptions")]
    pub ImageRequestOptions(ns::Id)
);

impl ImageManager {
    /// Returns the default shared image manager.
    pub fn default() -> arc::R<Self> {
        unsafe {
            let result = ph_image_manager_default();
            std::mem::transmute(result)
        }
    }

    /// Requests image data and orientation for the specified asset.
    /// Returns the request ID that can be used to cancel the request.
    #[cfg(feature = "blocks")]
    pub fn request_image_data_and_orientation_for_asset(
        &self,
        asset: &Asset,
        options: Option<&ImageRequestOptions>,
        result_handler: &mut ImageDataRequestHandler,
    ) -> i32 {
        unsafe {
            ph_image_manager_request_image_data_and_orientation_for_asset(
                self,
                asset,
                options,
                result_handler,
            )
        }
    }

    /// Async version that requests image data and orientation for the specified asset.
    #[cfg(all(feature = "async", feature = "blocks"))]
    pub async fn request_image_data_and_orientation_for_asset_async(
        &self,
        asset: &Asset,
        options: Option<&ImageRequestOptions>,
    ) -> ns::Result<(arc::R<ns::Data>, Option<arc::R<ns::String>>, u32), arc::R<ns::Error>> {
        let shared = blocks::Shared::new();
        let comp = blocks::Completion::new(shared.clone());
        let mut block = blocks::EscBlock::new4(
            move |image_data: Option<&ns::Data>,
                  data_uti: Option<&ns::String>,
                  orientation: u32,
                  _info: Option<&ns::Id>| {
                if let Some(data) = image_data {
                    let data_uti_retained = data_uti.map(|s| s.retained());
                    shared
                        .lock()
                        .ready(Ok((data.retained(), data_uti_retained, orientation)));
                } else {
                    // Create a generic error for failed requests
                    let domain_str = crate::nsstr!(c"PHPhotosErrorDomain");
                    let domain =
                        unsafe { std::mem::transmute::<&ns::String, &ns::ErrorDomain>(domain_str) };
                    let error = ns::Error::with_domain(domain, -1, None);
                    shared.lock().ready(Err(error));
                }
            },
        );
        self.request_image_data_and_orientation_for_asset(asset, options, &mut block);
        comp.await
    }
}

impl ImageRequestOptions {
    /// Creates a new image request options object.
    pub fn new() -> arc::R<Self> {
        unsafe {
            let result = ph_image_request_options_new();
            std::mem::transmute(result)
        }
    }

    /// Sets the delivery mode for the request.
    pub fn set_delivery_mode(&mut self, mode: i32) {
        unsafe {
            ph_image_request_options_set_delivery_mode(self, mode);
        }
    }

    /// Sets whether the request should be synchronous.
    pub fn set_synchronous(&mut self, synchronous: bool) {
        unsafe {
            ph_image_request_options_set_synchronous(self, synchronous);
        }
    }

    /// Sets whether the request should allow network access for iCloud downloads.
    pub fn set_network_access_allowed(&mut self, allowed: bool) {
        unsafe {
            ph_image_request_options_set_network_access_allowed(self, allowed);
        }
    }
}

impl FetchResult {
    pub fn count(&self) -> usize {
        unsafe { ph_fetch_result_get_count(self) }
    }

    pub fn object_at(&self, index: usize) -> Option<&Asset> {
        unsafe { ph_fetch_result_object_at_index(self, index) }
    }
}

define_obj_type!(
    #[doc(alias = "PHFetchOptions")]
    pub FetchOptions(ns::Id)
);

define_obj_type!(
    #[doc(alias = "PHContentEditingInputRequestOptions")]
    pub ContentEditingInputRequestOptions(ns::Id)
);

define_obj_type!(
    #[doc(alias = "PHContentEditingInput")]
    pub ContentEditingInput(ns::Id)
);

define_obj_type!(
    #[doc(alias = "PHAssetResource")]
    pub AssetResource(ns::Id)
);

define_obj_type!(
    #[doc(alias = "PHAssetResourceManager")]
    pub AssetResourceManager(ns::Id)
);

define_obj_type!(
    #[doc(alias = "PHAssetResourceRequestOptions")]
    pub AssetResourceRequestOptions(ns::Id)
);

impl FetchOptions {
    pub fn new() -> arc::R<Self> {
        unsafe {
            let options = ph_fetch_options_new();
            std::mem::transmute(options)
        }
    }

    pub fn set_include_all_burst_assets(&mut self, include: bool) {
        unsafe { ph_fetch_options_set_include_all_burst_assets(self, include) }
    }

    pub fn set_include_hidden_assets(&mut self, include: bool) {
        unsafe { ph_fetch_options_set_include_hidden_assets(self, include) }
    }
}

impl ContentEditingInputRequestOptions {
    pub fn new() -> arc::R<Self> {
        unsafe {
            let options = ph_content_editing_input_request_options_new();
            std::mem::transmute(options)
        }
    }

    pub fn set_network_access_allowed(&mut self, allowed: bool) {
        unsafe {
            ph_content_editing_input_request_options_set_network_access_allowed(self, allowed);
        }
    }

    pub fn set_can_handle_adjustment_data(&mut self, can_handle: bool) {
        unsafe {
            ph_content_editing_input_request_options_set_can_handle_adjustment_data(
                self, can_handle,
            );
        }
    }
}

impl ContentEditingInput {
    pub fn full_size_image_url(&self) -> Option<&ns::Url> {
        unsafe { ph_content_editing_input_get_full_size_image_url(self) }
    }

    pub fn full_size_image_uti(&self) -> Option<&ns::String> {
        unsafe { ph_content_editing_input_get_full_size_image_uti(self) }
    }
}

impl AssetResource {
    /// Returns resources for the given asset
    pub fn resources_for_asset(asset: &Asset) -> arc::R<ns::Array<AssetResource>> {
        unsafe {
            let result = ph_asset_resource_resources_for_asset(asset);
            std::mem::transmute(result)
        }
    }

    /// Returns the type of this resource
    pub fn resource_type(&self) -> i32 {
        unsafe { ph_asset_resource_get_type(self) }
    }
}

impl AssetResourceManager {
    /// Returns the default asset resource manager
    pub fn default() -> arc::R<Self> {
        unsafe {
            let result = ph_asset_resource_manager_default();
            std::mem::transmute(result)
        }
    }

    /// Requests data for the given asset resource
    #[cfg(feature = "blocks")]
    pub fn request_data_for_resource(
        &self,
        resource: &AssetResource,
        options: Option<&AssetResourceRequestOptions>,
        data_received_handler: &mut AssetResourceDataRequestHandler,
        completion_handler: &mut blocks::EscBlock<fn(Option<&ns::Error>)>,
    ) -> i32 {
        unsafe {
            ph_asset_resource_manager_request_data_for_resource(
                self,
                resource,
                options,
                data_received_handler,
                completion_handler,
            )
        }
    }
}

impl AssetResourceRequestOptions {
    pub fn new() -> arc::R<Self> {
        unsafe {
            let result = ph_asset_resource_request_options_new();
            std::mem::transmute(result)
        }
    }

    pub fn set_network_access_allowed(&mut self, allowed: bool) {
        unsafe {
            ph_asset_resource_request_options_set_network_access_allowed(self, allowed);
        }
    }
}

#[link(name = "Photos", kind = "framework")]
unsafe extern "C" {
    // From ph.m
    fn ph_fetch_options_new() -> &'static mut FetchOptions;
    fn ph_fetch_options_set_include_all_burst_assets(options: &FetchOptions, include: bool);
    fn ph_fetch_options_set_include_hidden_assets(options: &FetchOptions, include: bool);

    fn ph_asset_fetch_with_local_identifiers(
        identifiers: &ns::Array<ns::String>,
        options: Option<&FetchOptions>,
    ) -> &'static mut FetchResult;
    fn ph_asset_fetch_assets_with_options(
        options: Option<&FetchOptions>,
    ) -> &'static mut FetchResult;

    fn ph_fetch_result_get_count(result: &FetchResult) -> usize;
    fn ph_fetch_result_object_at_index(result: &FetchResult, index: usize) -> Option<&Asset>;
    fn ph_object_get_local_identifier(object: &Object) -> &ns::String;

    // Image Manager functions
    fn ph_image_manager_default() -> &'static mut ImageManager;

    #[cfg(feature = "blocks")]
    fn ph_image_manager_request_image_data_and_orientation_for_asset(
        manager: &ImageManager,
        asset: &Asset,
        options: Option<&ImageRequestOptions>,
        result_handler: &mut ImageDataRequestHandler,
    ) -> i32;

    // Image Request Options functions
    fn ph_image_request_options_new() -> &'static mut ImageRequestOptions;
    fn ph_image_request_options_set_delivery_mode(options: &ImageRequestOptions, mode: i32);
    fn ph_image_request_options_set_synchronous(options: &ImageRequestOptions, synchronous: bool);
    fn ph_image_request_options_set_network_access_allowed(
        options: &ImageRequestOptions,
        allowed: bool,
    );

    // Content Editing Input functions
    #[cfg(feature = "blocks")]
    fn ph_asset_request_content_editing_input_with_options(
        asset: &Asset,
        options: Option<&ContentEditingInputRequestOptions>,
        completion_handler: &mut ContentEditingInputRequestCompletionHandler,
    ) -> i32;

    fn ph_content_editing_input_request_options_new()
    -> &'static mut ContentEditingInputRequestOptions;
    fn ph_content_editing_input_request_options_set_network_access_allowed(
        options: &ContentEditingInputRequestOptions,
        allowed: bool,
    );
    fn ph_content_editing_input_request_options_set_can_handle_adjustment_data(
        options: &ContentEditingInputRequestOptions,
        can_handle: bool,
    );

    fn ph_content_editing_input_get_full_size_image_url(
        input: &ContentEditingInput,
    ) -> Option<&ns::Url>;
    fn ph_content_editing_input_get_full_size_image_uti(
        input: &ContentEditingInput,
    ) -> Option<&ns::String>;

    // Asset Resource functions
    fn ph_asset_resource_resources_for_asset(
        asset: &Asset,
    ) -> &'static mut ns::Array<AssetResource>;
    fn ph_asset_resource_get_type(resource: &AssetResource) -> i32;

    // Asset Resource Manager functions
    fn ph_asset_resource_manager_default() -> &'static mut AssetResourceManager;

    #[cfg(feature = "blocks")]
    fn ph_asset_resource_manager_request_data_for_resource(
        manager: &AssetResourceManager,
        resource: &AssetResource,
        options: Option<&AssetResourceRequestOptions>,
        data_received_handler: &mut AssetResourceDataRequestHandler,
        completion_handler: &mut blocks::EscBlock<fn(Option<&ns::Error>)>,
    ) -> i32;

    // Asset Resource Request Options functions
    fn ph_asset_resource_request_options_new() -> &'static mut AssetResourceRequestOptions;
    fn ph_asset_resource_request_options_set_network_access_allowed(
        options: &AssetResourceRequestOptions,
        allowed: bool,
    );
}
