use nui_shared::CView;

extern "C" {
    pub fn bootstrap(view: *const CView);
}
