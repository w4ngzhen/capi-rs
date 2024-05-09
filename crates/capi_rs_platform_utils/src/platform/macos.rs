use crate::monitor::MonitorImage;
use core_foundation::base::{CFAllocatorRef, TCFType};
use core_foundation::data::{CFDataRef, CFMutableDataRef};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::{CFString, CFStringRef};
use core_graphics::display::CGDirectDisplayID;
use objc2::msg_send;
use objc2::rc::Id;
use objc2_app_kit::{NSEvent, NSScreen};
use objc2_foundation::{MainThreadMarker, NSPointInRect, NSString};
use std::ffi::c_uint;

pub fn get_mouse_located_monitor_id() -> Option<c_uint> {
    unsafe {
        let mouse_pos = NSEvent::mouseLocation();
        let mtm = MainThreadMarker::new().unwrap();
        let ns_screen_arr = NSScreen::screens(mtm);
        let mut target_screen: Option<Id<NSScreen>> = None;
        for screen in ns_screen_arr {
            if NSPointInRect(mouse_pos, screen.frame()).into() {
                target_screen = Some(screen);
                break;
            }
        }
        if let Some(target_screen) = target_screen {
            #[cfg(debug_assertions)]
            println!("target_screen {:?}", target_screen);
            let desc = target_screen.deviceDescription();
            #[cfg(debug_assertions)]
            println!("desc {:?}", desc);
            let prop_key = NSString::from_str("NSScreenNumber");
            let screen_handle_num = desc.objectForKey(&prop_key);
            if let Some(screen_handle_num) = screen_handle_num {
                let handle: c_uint = msg_send![&screen_handle_num, unsignedIntValue];
                return Some(handle);
            }
        }
    }
    return None;
}

pub fn get_monitor_screen_image_native(native_id: c_uint) -> Option<MonitorImage> {
    unsafe {
        let img_ref = CGDisplayCreateImage(native_id);
        if img_ref.is_null() {
            return None;
        }
        let (width, height) = (CGImageGetWidth(img_ref), CGImageGetHeight(img_ref));
        let pixel_data = CFDataCreateMutable(std::ptr::null(), 0);
        if pixel_data.is_null() {
            return None;
        }
        let ut_type = CFString::from("kUTTypeJPEG").as_concrete_TypeRef();
        if ut_type.is_null() {
            return None;
        }
        let data_dest = CGImageDestinationCreateWithData(pixel_data, ut_type, 1, std::ptr::null());
        if data_dest.is_null() {
            return None;
        }
        CGImageDestinationAddImage(data_dest, img_ref, std::ptr::null());
        CGImageDestinationFinalize(data_dest);
        let bytes_len = CFDataGetLength(pixel_data);
        #[cfg(debug_assertions)]
        println!("image len: {}", bytes_len);
        return Some(MonitorImage::new(width, height, None));
    }
    None
}

enum CGImageDestination {}
type CGImageDestinationRef = *mut CGImageDestination;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGDisplayCreateImage(display_id: CGDirectDisplayID) -> core_graphics::sys::CGImageRef;
    fn CGImageGetWidth(image_ref: core_graphics::sys::CGImageRef) -> usize;
    fn CGImageGetHeight(image_ref: core_graphics::sys::CGImageRef) -> usize;
    fn CFDataCreateMutable(allocator: CFAllocatorRef, capacity: isize) -> CFMutableDataRef;
    fn CFDataGetLength(data_ref: CFDataRef) -> isize;

    fn CGImageDestinationCreateWithData(
        mut_data_ref: CFMutableDataRef,
        ut_type: CFStringRef,
        count: usize,
        options: CFDictionaryRef,
    ) -> CGImageDestinationRef;

    fn CGImageDestinationAddImage(
        dest: CGImageDestinationRef,
        image_ref: core_graphics::sys::CGImageRef,
        properties: CFDictionaryRef,
    );

    fn CGImageDestinationFinalize(dest: CGImageDestinationRef);
}
