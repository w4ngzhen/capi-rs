use capi_rs_platform_utils::platform;

fn main() {
    if let Some(monitor_img) = platform::ffi::get_mouse_located_monitor_screenshot() {
        monitor_img.write_to_file("test.png");
    }
}
