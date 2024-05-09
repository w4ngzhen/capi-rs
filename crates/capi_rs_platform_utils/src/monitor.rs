use crate::platform::ffi::{get_monitor_screen_image_native, get_mouse_located_monitor_id};
use winit::event_loop::ActiveEventLoop;
use winit::monitor::MonitorHandle;
use winit::platform::macos::MonitorHandleExtMacOS;

#[derive(Default)]
pub struct MonitorImage {
    width: usize,
    height: usize,
    img: Option<Vec<u8>>,
}

impl MonitorImage {
    pub fn new(width: usize, height: usize, img: Option<Vec<u8>>) -> Self {
        Self { width, height, img }
    }
}
