use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::macos::MonitorHandleExtMacOS;

use crate::app::App;

mod app;

fn main() -> Result<(), EventLoopError> {
    // 创建事件循环
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app)
}
