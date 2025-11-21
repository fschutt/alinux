#include <wayland-server.h>
#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include <stdbool.h>

static struct wl_display *display = NULL;
static bool running = true;

static void handle_signal(int sig) {
    running = false;
}

int main(int argc, char *argv[]) {
    printf("\n");
    printf("╔════════════════════════════════════════╗\n");
    printf("║   ACOMP - Azul Wayland Compositor      ║\n");
    printf("╚════════════════════════════════════════╝\n");
    printf("\n");
    
    signal(SIGINT, handle_signal);
    signal(SIGTERM, handle_signal);
    
    display = wl_display_create();
    if (!display) {
        fprintf(stderr, "[acomp] Failed to create Wayland display\n");
        return 1;
    }

    const char *socket = wl_display_add_socket_auto(display);
    if (!socket) {
        fprintf(stderr, "[acomp] Failed to add socket\n");
        wl_display_destroy(display);
        return 1;
    }

    printf("[acomp] Compositor running on: %s\n", socket);
    printf("[acomp] Setting WAYLAND_DISPLAY environment variable\n");
    setenv("WAYLAND_DISPLAY", socket, 1);
    
    printf("[acomp] Ready to accept clients\n");
    printf("[acomp] Press Ctrl+C to stop compositor\n\n");
    
    while (running) {
        wl_display_flush_clients(display);
        wl_event_loop_dispatch(wl_display_get_event_loop(display), 100);
    }
    
    printf("\n[acomp] Shutting down compositor...\n");
    wl_display_destroy(display);
    
    return 0;
}
