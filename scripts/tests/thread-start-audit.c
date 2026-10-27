/* Count successful OS thread starts outside the addon under test. */
#define _GNU_SOURCE
#include <pthread.h>
#include <dlfcn.h>
#include <fcntl.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

static void record(const char *line, size_t length) {
    const char *path = getenv("THINKTHEN_THREAD_AUDIT");
    if (!path) return;
    int fd = open(path, O_WRONLY | O_CREAT | O_APPEND, 0600);
    if (fd < 0) _exit(90);
    if (write(fd, line, length) != (ssize_t)length) _exit(91);
    close(fd);
}

static int audit_create(pthread_t *thread, const pthread_attr_t *attr,
                        void *(*entry)(void *), void *argument) {
#ifdef __APPLE__
    int result = pthread_create(thread, attr, entry, argument);
#else
    int (*original)(pthread_t *, const pthread_attr_t *, void *(*)(void *), void *) =
        dlsym(RTLD_NEXT, "pthread_create");
    int result = original(thread, attr, entry, argument);
#endif
    if (result == 0) {
        Dl_info info;
        int found = dladdr((const void *)entry, &info);
        const char *image = found && info.dli_fname ? info.dli_fname : "unknown";
        const char *symbol = found && info.dli_sname ? info.dli_sname : "unknown";
        char line[4096];
        int length = snprintf(line, sizeof(line), "start\t%s\t%s\n", image, symbol);
        if (length < 0 || (size_t)length >= sizeof(line)) _exit(92);
        record(line, (size_t)length);
    }
    return result;
}

#ifdef __APPLE__
static int audit_name(const char *name) {
    int result = pthread_setname_np(name);
#else
static int audit_name(pthread_t thread, const char *name) {
    int (*original)(pthread_t, const char *) = dlsym(RTLD_NEXT, "pthread_setname_np");
    int result = original(thread, name);
#endif
    const char *selected = getenv("THINKTHEN_THREAD_NAME");
    if (result == 0 && selected && strcmp(name, selected) == 0) record("named\n", 6);
    return result;
}

#ifdef __APPLE__
#define INTERPOSE(replacement, original) \
    __attribute__((used)) static struct { const void *replace; const void *original; } \
    pair_##original __attribute__((section("__DATA,__interpose"))) = \
        { (const void *)&replacement, (const void *)&original };
INTERPOSE(audit_create, pthread_create)
INTERPOSE(audit_name, pthread_setname_np)
#else
int pthread_create(pthread_t *thread, const pthread_attr_t *attr,
                   void *(*entry)(void *), void *argument) {
    return audit_create(thread, attr, entry, argument);
}
int pthread_setname_np(pthread_t thread, const char *name) { return audit_name(thread, name); }
#endif
