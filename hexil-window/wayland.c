#include <stddef.h>
#include <wayland-client-core.h>
#include "hexil-window.h"



struct window_context {
  struct wl_display* display;
  
};


struct window_context* hx_get_wayland_context() {
  struct wl_display* display = wl_display_connect(NULL);
  if (!display) {
    return NULL;
  }

  
}


