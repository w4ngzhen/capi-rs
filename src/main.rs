fn main() {
    if let Some(monitor_img) = capi_rs_platform_utils::platform::ffi::get_mouse_located_monitor_screenshot() {
        monitor_img.write_to_file("test.png");
    }
}
