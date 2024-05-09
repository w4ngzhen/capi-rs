use winit::event_loop::ActiveEventLoop;
use winit::monitor::MonitorHandle;
use winit::platform::macos::MonitorHandleExtMacOS;

/// 获取当前鼠标所在屏幕的monitor_handle
/// 不同操作系统平台根据 ffi::get_mouse_located_monitor_id() 方法得到操作系统原生句柄ID
pub fn get_mouse_located_monitor_handle(event_loop: &ActiveEventLoop) -> Option<MonitorHandle> {
    if let Some(monitor_id) = ffi::get_mouse_located_monitor_id() {
        // 1. 获取当前鼠标所在屏幕的句柄ID
        // 2. 遍历已知的所有屏幕，找到与当前鼠标所在句柄ID相同的屏幕
        let mut target: Option<MonitorHandle> = None;
        for monitor_handle in event_loop.available_monitors() {
            if monitor_id == monitor_handle.native_id() {
                target = Some(monitor_handle);
                break;
            }
        }
        target
    } else {
        None
    }
}

#[cfg(target_os = "macos")]
mod ffi {
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
}
