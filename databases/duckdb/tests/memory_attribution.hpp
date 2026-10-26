// Diagnostic observations only: never purge, trim, flush, or tune an allocator.
#include <cerrno>
#include <cstdio>
#include <cstdlib>
#include <fcntl.h>
#include <malloc.h>
#include <stdexcept>
#include <time.h>
#include <unistd.h>

extern "C" int duckdb_je_mallctl(const char *, void *, size_t *, void *, size_t);

static size_t AllocatorStat(const char *name) {
    size_t value = 0, size = sizeof(value);
    if (duckdb_je_mallctl(name, &value, &size, nullptr, 0) || size != sizeof(value))
        throw std::runtime_error(name);
    return value;
}

static void Attribution() {
    // Updating epoch aggregates statistics; it does not change allocator policy.
    uint64_t epoch = 1;
    size_t size = sizeof(epoch);
    if (duckdb_je_mallctl("epoch", &epoch, &size, &epoch, sizeof(epoch)))
        throw std::runtime_error("jemalloc epoch unavailable");
    auto allocated = AllocatorStat("stats.allocated");
    auto active = AllocatorStat("stats.active");
    auto resident = AllocatorStat("stats.resident");
    auto retained = AllocatorStat("stats.retained");
    auto libc = mallinfo2();
    timespec now{};
    if (clock_gettime(CLOCK_MONOTONIC, &now)) throw std::runtime_error("clock unavailable");
    const char *path = std::getenv("THINKTHEN_DUCKDB_ATTRIBUTION_NATIVE");
    if (!path) throw std::runtime_error("native observation path missing");
    int fd = open(path, O_WRONLY | O_CREAT | O_APPEND, 0600);
    if (fd < 0) throw std::runtime_error("native observation open failed");
    char buffer[1024];
    int length = std::snprintf(buffer, sizeof(buffer),
        "{\"pid\":%d,\"monotonic_ns\":%lld,\"jemalloc_allocated\":%zu,"
        "\"jemalloc_active\":%zu,\"jemalloc_resident\":%zu,\"jemalloc_retained\":%zu,"
        "\"glibc_uordblks\":%zu,\"glibc_hblkhd\":%zu,\"glibc_arena\":%zu,\"glibc_fordblks\":%zu}\n",
        getpid(), static_cast<long long>(now.tv_sec) * 1000000000 + now.tv_nsec,
        allocated, active, resident, retained, libc.uordblks, libc.hblkhd, libc.arena, libc.fordblks);
    bool failed = length < 0 || static_cast<size_t>(length) >= sizeof(buffer) ||
        write(fd, buffer, static_cast<size_t>(length)) != length;
    if (close(fd) || failed) throw std::runtime_error("native observation write failed");
}
