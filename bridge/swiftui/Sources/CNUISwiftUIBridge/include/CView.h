#pragma once

struct CView {
    void *wrapped_view;
    const char *(*render_json)(const struct CView *);
};
