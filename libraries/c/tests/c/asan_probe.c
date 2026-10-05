/* The native sanitizer must detect this owned negative before coverage is claimed. */
#include <stdlib.h>
#include <thinkthen.h>
int main(void) {
    if (thinkthen_error_code(NULL) != THINKTHEN_EUSAGE) return 3;
    volatile char *bytes = malloc(32);
    if (bytes == NULL) return 2;
    bytes[0] = 7;
    free((void *)bytes);
    return bytes[0];
}
