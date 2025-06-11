use crate::{arc, cf};

// typedef uint32_t CGWindowID;
pub type Id = u32;

// Window list options
pub type ListOption = u32;

pub const LIST_OPTION_ALL: ListOption = 0;
pub const LIST_OPTION_ON_SCREEN_ONLY: ListOption = 1 << 0;
pub const LIST_OPTION_ON_SCREEN_ABOVE_WINDOW: ListOption = 1 << 1;
pub const LIST_OPTION_ON_SCREEN_BELOW_WINDOW: ListOption = 1 << 2;
pub const LIST_OPTION_INCLUDING_WINDOW: ListOption = 1 << 3;
pub const LIST_OPTION_EXCLUDE_DESKTOP_ELEMENTS: ListOption = 1 << 4;

// Window info dictionary keys
#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    pub static kCGWindowNumber: &'static cf::String;
    pub static kCGWindowOwnerName: &'static cf::String;
    pub static kCGWindowName: &'static cf::String;
    pub static kCGWindowBounds: &'static cf::String;
    pub static kCGWindowLayer: &'static cf::String;
    pub static kCGWindowAlpha: &'static cf::String;
    pub static kCGWindowIsOnscreen: &'static cf::String;
    pub static kCGWindowBackingLocationVideoMemory: &'static cf::String;
    pub static kCGWindowOwnerPID: &'static cf::String;
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    #[doc(alias = "CGWindowListCopyWindowInfo")]
    #[link_name = "CGWindowListCopyWindowInfo"]
    pub fn list_copy_window_info(
        option: ListOption,
        relative_to_window: Id,
    ) -> Option<arc::R<cf::ArrayOf<cf::Dictionary>>>;
}

// Convenience wrapper function
pub fn copy_window_info(
    option: ListOption,
    relative_to_window: Id,
) -> Option<arc::R<cf::ArrayOf<cf::Dictionary>>> {
    unsafe { list_copy_window_info(option, relative_to_window) }
}
