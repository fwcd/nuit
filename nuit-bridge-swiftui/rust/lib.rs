use nuit_core::CRoot;

extern "C" {
    pub fn run_app(root: *const CRoot);
}
