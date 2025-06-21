use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSPasteboard")]
    pub Pasteboard(ns::Id),
    NS_PASTEBOARD
);

impl Pasteboard {
    /// Returns the pasteboard with the specified name
    #[objc::msg_send(pasteboardWithName:)]
    pub fn with_name(name: &ns::String) -> Option<arc::R<Self>>;

    /// Returns the number of changes made to the receiver
    #[objc::msg_send(changeCount)]
    pub fn change_count(&self) -> ns::Integer;

    /// An array of the receiver's supported data types
    #[objc::msg_send(types)]
    pub fn types(&self) -> Option<arc::R<ns::Array<ns::String>>>;
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_PASTEBOARD: &'static objc::Class<Pasteboard>;
}

// Common pasteboard names
impl Pasteboard {
    /// The general pasteboard
    pub fn general_name() -> &'static ns::String {
        unsafe { NSPasteboardNameGeneral }
    }

    /// The font pasteboard
    pub fn font_name() -> &'static ns::String {
        unsafe { NSPasteboardNameFont }
    }

    /// The ruler pasteboard  
    pub fn ruler_name() -> &'static ns::String {
        unsafe { NSPasteboardNameRuler }
    }

    /// The find pasteboard
    pub fn find_name() -> &'static ns::String {
        unsafe { NSPasteboardNameFind }
    }

    /// The drag pasteboard
    pub fn drag_name() -> &'static ns::String {
        unsafe { NSPasteboardNameDrag }
    }
}

#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {
    #[link_name = "NSPasteboardNameGeneral"]
    static NSPasteboardNameGeneral: &'static ns::String;

    #[link_name = "NSPasteboardNameFont"]
    static NSPasteboardNameFont: &'static ns::String;

    #[link_name = "NSPasteboardNameRuler"]
    static NSPasteboardNameRuler: &'static ns::String;

    #[link_name = "NSPasteboardNameFind"]
    static NSPasteboardNameFind: &'static ns::String;

    #[link_name = "NSPasteboardNameDrag"]
    static NSPasteboardNameDrag: &'static ns::String;
}
