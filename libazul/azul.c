#include <stdio.h>
#include "azul.h"

void azul_init(void) {
    printf("[libazul] Azul Graphics System v1.0.0 initialized\n");
}

const char* azul_version(void) {
    return "1.0.0-alinux";
}

int azul_render(void) {
    return 0;
}

void azul_cleanup(void) {
    printf("[libazul] Cleanup complete\n");
}
