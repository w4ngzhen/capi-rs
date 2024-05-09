use crate::monitor::MonitorImage;
use crate::platform::ffi::{get_monitor_screen_image_native, get_mouse_located_monitor_id};
use winit::event_loop::ActiveEventLoop;
use winit::monitor::MonitorHandle;
use winit::platform::macos::MonitorHandleExtMacOS;

pub mod monitor;
pub mod platform;

/// 获取当前鼠标所在屏幕的monitor_handle
/// 不同操作系统平台根据 ffi::get_mouse_located_monitor_id() 方法得到操作系统原生句柄ID
pub fn get_mouse_located_monitor_handle(event_loop: &ActiveEventLoop) -> Option<MonitorHandle> {
    if let Some(monitor_id) = get_mouse_located_monitor_id() {
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

pub fn get_mouse_located_monitor_screen(monitor_handle: MonitorHandle) -> Option<MonitorImage> {
    get_monitor_screen_image_native(monitor_handle.native_id())
}
