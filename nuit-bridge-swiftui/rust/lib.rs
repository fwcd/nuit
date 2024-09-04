use nuit_core::CRoot;

#[cfg(target_vendor = "apple")]
extern "C" {
    #[link_name = "nuit_bridge_swiftui_run_app"]
    pub fn run_app(root: *const CRoot);
}
