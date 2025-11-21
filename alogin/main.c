#include <wayland-client.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static struct wl_display *display = NULL;
static struct wl_compositor *compositor = NULL;

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
    printf("║      ALOGIN - ALinux Login Manager     ║\n");
    printf("╚════════════════════════════════════════╝\n");
    printf("\n");
    
    display = wl_display_connect(NULL);
    if (!display) {
        fprintf(stderr, "[alogin] Wayland display not available\n");
        printf("[alogin] Running in text mode...\n\n");
        
        printf("Username: ");
        char username[256];
        if (fgets(username, sizeof(username), stdin)) {
            username[strcspn(username, "\n")] = 0;
            printf("Welcome to ALinux, %s!\n\n", username);
        }
        return 0;
    }

    struct wl_registry *registry = wl_display_get_registry(display);
    wl_registry_add_listener(registry, &registry_listener, NULL);
    wl_display_roundtrip(display);

    printf("[alogin] Login manager initialized with Wayland\n");
    printf("[alogin] Graphical login coming soon...\n");
    
    wl_registry_destroy(registry);
    wl_display_disconnect(display);
    return 0;
}
