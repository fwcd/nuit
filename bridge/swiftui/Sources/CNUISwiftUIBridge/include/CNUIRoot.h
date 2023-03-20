#pragma once

struct CNUIRoot {
    void *wrapped;
    const char *(*render_json)(const struct CNUIRoot *);
    void (*fire_event_json)(const struct CNUIRoot *, const char *, const char *);
};
