use nui_shared::CView;

extern "C" {
    pub fn run_app(view: *const CView);
}
