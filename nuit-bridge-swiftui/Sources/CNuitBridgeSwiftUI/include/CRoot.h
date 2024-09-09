#pragma once

struct CRoot {
    void *wrapped;
    const char *(*render_json)(const struct CRoot *);
    const char *(*fire_event_json)(const struct CRoot *, const char *, const char *);
    void (*set_update_callback)(const struct CRoot *, void (*)(const char *));
};
