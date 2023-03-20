#pragma once

struct CNUIRoot {
    void *wrapped;
    const char *(*render_json)(const struct CNUIRoot *);
};
