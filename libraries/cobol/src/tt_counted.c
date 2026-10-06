/* Counted copy for caller-sized COBOL fields, independent of JSON limits. */
#include <stdint.h>
#include <stddef.h>
#include <string.h>
int TT_COPY_COUNTED(const void *source, uint64_t count, void *destination,
                    uint64_t capacity, uint64_t *written) {
    if (!written || count > capacity || count > SIZE_MAX ||
        (count && (!source || !destination ||
         (uintptr_t)source > UINTPTR_MAX - count ||
         (uintptr_t)destination > UINTPTR_MAX - count))) return 1;
    /* Caller owns readable/writable storage. Empty copies accept NULL. */
    if (count) memmove(destination, source, (size_t)count);
    *written = count;
    return 0;
}
/* Return a borrowed element only after checking the complete byte range. */
int TT_ELEMENT(const void *data, uint64_t count, uint64_t index,
               uint64_t element_size, uint64_t alignment, const void **out) {
    uintptr_t base = (uintptr_t)data;
    if (!out || !data || !element_size || !alignment ||
        (alignment & (alignment - 1)) || index >= count ||
        count > SIZE_MAX / element_size || base % alignment ||
        base > UINTPTR_MAX - count * element_size) return 1;
    *out = (const unsigned char *)data + index * element_size;
    return 0;
}
