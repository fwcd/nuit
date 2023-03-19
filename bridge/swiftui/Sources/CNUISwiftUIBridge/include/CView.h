struct CView {
    void *wrapped_view;
    const char *(*render_json)(const struct CView *);
};

extern void *nui_c_view_drop(const struct CView *view);
