use nuit_core::CRoot;

#[cfg(target_vendor = "apple")]
extern "C" {
    pub fn run_app(root: *const CRoot);
}
