#![windows_subsystem = "windows"]
#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let main_app = pick::MainApp::default();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(main_app.app_size.into()),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(Box::new(main_app), native_options);
}
