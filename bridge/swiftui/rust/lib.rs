use nui_shared::CRoot;

extern "C" {
    pub fn run_app(root: *const CRoot);
}
