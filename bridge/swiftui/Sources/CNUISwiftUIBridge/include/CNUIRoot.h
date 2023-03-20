#pragma once

struct CNUIRoot {
    void *wrapped;
    const char *(*render_json)(const struct CNUIRoot *);
    void (*fire_click_action)(const struct CNUIRoot *, const char *);
};
