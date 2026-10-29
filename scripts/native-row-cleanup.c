/* Feature: forwarding row fixtures preserve nullable and short ABI cleanup. */
#define _DEFAULT_SOURCE
#include "thinkthen.h"
#include <dlfcn.h>
#include <stdint.h>
#include <sys/mman.h>
#include <unistd.h>

int main(int argc, char **argv) {
    if (argc != 2) return 2;
    void *library = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
    if (!library) return 2;
    void (*release)(thinkthen_row *) = (void (*)(thinkthen_row *))dlsym(library, "thinkthen_row_free");
    if (!release) return 2;
    release(NULL);
    size_t page = (size_t)sysconf(_SC_PAGESIZE);
    char *memory = mmap(NULL, page * 2, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANON, -1, 0);
    if (memory == MAP_FAILED || mprotect(memory + page, page, PROT_NONE)) return 2;
    uint32_t *prefix = (uint32_t *)(memory + page - sizeof(uint32_t));
    *prefix = sizeof(uint32_t);
    release((thinkthen_row *)prefix);
    int changed = *prefix != sizeof(uint32_t);
    munmap(memory, page * 2);
    dlclose(library);
    return changed;
}
