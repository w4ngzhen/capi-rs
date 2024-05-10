use crate::monitor::MonitorImageWrapper;
use crate::platform::ffi::{get_monitor_screen_image_native, get_mouse_located_monitor_id};
use winit::event_loop::ActiveEventLoop;
use winit::monitor::MonitorHandle;
use winit::platform::macos::MonitorHandleExtMacOS;

pub mod monitor;
mod platform;
mod utils;

/// Get the monitor handle where the mouse is located
pub fn get_mouse_located_monitor_handle(event_loop: &ActiveEventLoop) -> Option<MonitorHandle> {
    if let Some(monitor_id) = get_mouse_located_monitor_id() {
        // Iterate through all screens
        // to find the one with the same handle ID as the current mouse
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

/// Take a screenshot of the screen where the mouse is located
///
/// > note: the screenshot image data is the JPEG formatted data.
pub fn get_mouse_located_monitor_screen(monitor_handle: MonitorHandle) -> Option<MonitorImageWrapper> {
    get_monitor_screen_image_native(monitor_handle.native_id())
}
