#[cfg(target_os = "macos")]
#[path = "macos.rs"]
pub mod ffi;
#[cfg(target_os = "windows")]
#[path = "windows.rs"]
pub mod ffi;