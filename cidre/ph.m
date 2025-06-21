#import <Foundation/Foundation.h>
#import <Photos/Photos.h>

// [PHFetchOptions new]
PHFetchOptions* ph_fetch_options_new() {
    return [[PHFetchOptions alloc] init];
}

// [PHAsset fetchAssetsWithLocalIdentifiers:options:]
PHFetchResult* ph_asset_fetch_with_local_identifiers(NSArray<NSString*>* identifiers, PHFetchOptions* options) {
    return [PHAsset fetchAssetsWithLocalIdentifiers:identifiers options:options];
}

// [PHFetchResult count]
NSUInteger ph_fetch_result_get_count(PHFetchResult* result) {
    return result.count;
}

// [PHFetchResult objectAtIndex:]
PHAsset* ph_fetch_result_object_at_index(PHFetchResult* result, NSUInteger index) {
    if (index >= result.count) {
        return nil;
    }
    return [result objectAtIndex:index];
}

// [PHObject localIdentifier]
NSString* ph_object_get_local_identifier(PHObject* object) {
    return object.localIdentifier;
}

// Set includeAllBurstAssets property
void ph_fetch_options_set_include_all_burst_assets(PHFetchOptions* options, BOOL include) {
    options.includeAllBurstAssets = include;
}

// Set includeHiddenAssets property
void ph_fetch_options_set_include_hidden_assets(PHFetchOptions* options, BOOL include) {
    options.includeHiddenAssets = include;
}

// [PHAsset fetchAssetsWithOptions:]
PHFetchResult* ph_asset_fetch_assets_with_options(PHFetchOptions* options) {
    return [PHAsset fetchAssetsWithOptions:options];
}

// [PHImageManager defaultManager]
PHImageManager* ph_image_manager_default() {
    return [PHImageManager defaultManager];
}

// [PHImageRequestOptions new]
PHImageRequestOptions* ph_image_request_options_new() {
    return [[PHImageRequestOptions alloc] init];
}

// Set delivery mode
void ph_image_request_options_set_delivery_mode(PHImageRequestOptions* options, NSInteger mode) {
    options.deliveryMode = mode;
}

// Set synchronous
void ph_image_request_options_set_synchronous(PHImageRequestOptions* options, BOOL synchronous) {
    options.synchronous = synchronous;
}

// Set network access allowed
void ph_image_request_options_set_network_access_allowed(PHImageRequestOptions* options, BOOL allowed) {
    options.networkAccessAllowed = allowed;
}

// Request image data and orientation for asset (modern API)
PHImageRequestID ph_image_manager_request_image_data_and_orientation_for_asset(
    PHImageManager* manager,
    PHAsset* asset,
    PHImageRequestOptions* options,
    void (^resultHandler)(NSData* imageData, NSString* dataUTI, CGImagePropertyOrientation orientation, NSDictionary* info)
) {
    return [manager requestImageDataAndOrientationForAsset:asset
                                                   options:options
                                             resultHandler:resultHandler];
}

// Content Editing Input Request Options
PHContentEditingInputRequestOptions* ph_content_editing_input_request_options_new() {
    return [[PHContentEditingInputRequestOptions alloc] init];
}

void ph_content_editing_input_request_options_set_network_access_allowed(PHContentEditingInputRequestOptions* options, BOOL allowed) {
    options.networkAccessAllowed = allowed;
}

void ph_content_editing_input_request_options_set_can_handle_adjustment_data(PHContentEditingInputRequestOptions* options, BOOL canHandle) {
    // Create a block that returns the requested value
    options.canHandleAdjustmentData = ^BOOL(PHAdjustmentData *adjustmentData) {
        (void)adjustmentData; // Mark as used
        return canHandle;
    };
}

// Asset content editing input request
PHImageRequestID ph_asset_request_content_editing_input_with_options(PHAsset* asset, PHContentEditingInputRequestOptions* options, void (^completionHandler)(PHContentEditingInput* _Nullable, NSDictionary* _Nullable)) {
    return [asset requestContentEditingInputWithOptions:options completionHandler:completionHandler];
}

// Content Editing Input accessors
NSURL* ph_content_editing_input_get_full_size_image_url(PHContentEditingInput* input) {
    return input.fullSizeImageURL;
}

NSString* ph_content_editing_input_get_full_size_image_uti(PHContentEditingInput* input) {
    return input.uniformTypeIdentifier;
}

// Asset Resource functions
NSArray<PHAssetResource*>* ph_asset_resource_resources_for_asset(PHAsset* asset) {
    return [PHAssetResource assetResourcesForAsset:asset];
}

NSInteger ph_asset_resource_get_type(PHAssetResource* resource) {
    return resource.type;
}

// Asset Resource Manager functions
PHAssetResourceManager* ph_asset_resource_manager_default() {
    return [PHAssetResourceManager defaultManager];
}

PHAssetResourceDataRequestID ph_asset_resource_manager_request_data_for_resource(
    PHAssetResourceManager* manager,
    PHAssetResource* resource,
    PHAssetResourceRequestOptions* options,
    void (^dataReceivedHandler)(NSData* data),
    void (^completionHandler)(NSError* _Nullable error)
) {
    return [manager requestDataForAssetResource:resource
                                        options:options
                            dataReceivedHandler:dataReceivedHandler
                              completionHandler:completionHandler];
}

// Asset Resource Request Options functions
PHAssetResourceRequestOptions* ph_asset_resource_request_options_new() {
    return [[PHAssetResourceRequestOptions alloc] init];
}

void ph_asset_resource_request_options_set_network_access_allowed(PHAssetResourceRequestOptions* options, BOOL allowed) {
    options.networkAccessAllowed = allowed;
}