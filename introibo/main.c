#include <wayland-client.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static struct wl_display *display = NULL;
static struct wl_compositor *compositor = NULL;
static struct wl_surface *surface = NULL;

static void registry_global(void *data, struct wl_registry *registry,
                           uint32_t name, const char *interface, uint32_t version) {
    if (strcmp(interface, wl_compositor_interface.name) == 0) {
        compositor = wl_registry_bind(registry, name, &wl_compositor_interface, 1);
    }
}

static void registry_global_remove(void *data, struct wl_registry *registry, uint32_t name) {}

static const struct wl_registry_listener registry_listener = {
    .global = registry_global,
    .global_remove = registry_global_remove,
};

int main(int argc, char *argv[]) {
    printf("\n");
    printf("╔════════════════════════════════════════╗\n");
    printf("║     INTROIBO - ALinux Installer        ║\n");
    printf("║    Powered by Azul Graphics System     ║\n");
    printf("╚════════════════════════════════════════╝\n");
    printf("\n");
    
    display = wl_display_connect(NULL);
    if (!display) {
        fprintf(stderr, "[introibo] Wayland display not available\n");
        printf("[introibo] Running in text mode...\n\n");
        
        printf("Installation Options:\n");
        printf("  1. Install ALinux to disk\n");
        printf("  2. Live session (no installation)\n");
        printf("  3. Configure system settings\n");
        printf("  4. Exit\n\n");
        
        printf("Select option (1-4): ");
        char input[10];
        if (fgets(input, sizeof(input), stdin)) {
            int choice = atoi(input);
            switch(choice) {
                case 1:
                    printf("\n[introibo] Disk installation not yet implemented\n");
                    break;
                case 2:
                    printf("\n[introibo] Starting live session...\n");
                    break;
                case 3:
                    printf("\n[introibo] System configuration not yet implemented\n");
                    break;
                default:
                    printf("\n[introibo] Exiting installer\n");
            }
        }
        return 0;
    }

    struct wl_registry *registry = wl_display_get_registry(display);
    wl_registry_add_listener(registry, &registry_listener, NULL);
    wl_display_roundtrip(display);

    if (compositor) {
        surface = wl_compositor_create_surface(compositor);
        printf("[introibo] Wayland compositor connected\n");
        printf("[introibo] Graphical installer ready\n");
        printf("[introibo] GUI features coming soon...\n");
    }

    wl_display_roundtrip(display);
    
    if (surface) wl_surface_destroy(surface);
    if (compositor) wl_compositor_destroy(compositor);
    wl_registry_destroy(registry);
    wl_display_disconnect(display);
    
    return 0;
}
