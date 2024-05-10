use std::ffi::c_uint;

use core_foundation::base::{CFAllocatorRef, CFRange, TCFType};
use core_foundation::data::{CFDataRef, CFMutableDataRef};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::{CFString, CFStringRef};
use core_graphics::display::CGDirectDisplayID;
use objc2::msg_send;
use objc2::rc::Id;
use objc2_app_kit::{NSEvent, NSScreen};
use objc2_foundation::{MainThreadMarker, NSPointInRect, NSString};

use crate::monitor::MonitorImageWrapper;

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

pub fn get_monitor_screen_image_native(native_id: c_uint) -> Option<MonitorImageWrapper> {
    unsafe {
        let img_ref = CGDisplayCreateImage(native_id);
        if img_ref.is_null() {
            return None;
        }
        let (width, height) = (CGImageGetWidth(img_ref), CGImageGetHeight(img_ref));
        let formated_img_data = CFDataCreateMutable(std::ptr::null(), 0);
        if formated_img_data.is_null() {
            return None;
        }
        // kUTTypeJPEG -> "public. jpeg"
        let ut_type = CFString::from("public.jpeg").as_concrete_TypeRef();
        let data_dest =
            CGImageDestinationCreateWithData(formated_img_data, ut_type, 1, std::ptr::null());
        if data_dest.is_null() {
            return None;
        }
        CGImageDestinationAddImage(data_dest, img_ref, std::ptr::null());
        CGImageDestinationFinalize(data_dest);
        let bytes_len = CFDataGetLength(formated_img_data);
        #[cfg(debug_assertions)]
        println!("image len: {}", bytes_len);
        // get formated image bytes
        let mut bytes = vec![0u8; bytes_len as usize];
        CFDataGetBytes(
            formated_img_data,
            CFRange::init(0, bytes_len),
            bytes.as_mut_ptr(),
        );
        let monitor_img_wrapper = MonitorImageWrapper::new(width, height, Some(bytes));
        return Some(monitor_img_wrapper);
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
    fn CFDataGetBytes(data_ref: CFDataRef, range: CFRange, buffer: *mut u8);
    ///
    /// **ffi CGImageDestinationCreateWithData**
    ///
    /// The parameter `ut_type` specifies the type identifier of the resulting image file.
    /// Constants for `ut_type` are found in the LaunchServices framework header UTCoreTypes.h.
    /// ```obj-c
    /// extern const CFStringRef kUTTypeJPEG
    /// // kUTTypeJPEG -> "public.jpeg"
    /// extern const CFStringRef kUTTypePNG
    /// // kUTTypePNG -> "public.png"
    /// // ... etc.
    /// ```
    ///
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
