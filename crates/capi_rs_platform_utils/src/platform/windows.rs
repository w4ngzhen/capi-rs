use crate::monitor::MonitorImage;
use std::ffi::c_uint;

pub fn get_mouse_located_monitor_id() -> Option<c_uint> {
    None
}

pub fn get_monitor_screen_image_native(native_id: c_uint) -> Option<MonitorImage> {
    None
}
