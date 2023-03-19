pub fn bootstrap() {
    unsafe {
        #[cfg(target_os = "macos")]
        nui_swiftui_bridge::bootstrap();
    }
}
