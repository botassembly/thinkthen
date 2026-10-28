/* Count successful OS thread starts outside the addon under test. */
#define _GNU_SOURCE
#include <pthread.h>
#include <dlfcn.h>
#include <fcntl.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <stdint.h>
#ifdef __linux__
#include <sys/syscall.h>
#endif

static void record(const char *line, size_t length) {
    const char *path = getenv("THINKTHEN_THREAD_AUDIT");
    if (!path) return;
    int fd = open(path, O_WRONLY | O_CREAT | O_APPEND, 0600);
    if (fd < 0) _exit(90);
    if (write(fd, line, length) != (ssize_t)length) _exit(91);
    close(fd);
}

/* Opt-in identities select pthread entry images independently of OS labels. */
struct entry {
    void *(*run)(void *);
    void *argument;
    uint64_t identity;
    int selected;
    const char *image;
    struct entry *next;
};
static pthread_mutex_t identities = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t retired = PTHREAD_COND_INITIALIZER;
static struct entry *live;
static uint64_t sequence;

static void identity_record(const char *event, struct entry *entry) {
    uint64_t tid;
#ifdef __APPLE__
    if (pthread_threadid_np(NULL, &tid)) _exit(93);
#else
    tid = (uint64_t)syscall(SYS_gettid);
#endif
    char line[4096];
    int size = snprintf(line, sizeof(line), "%s\t%llu\t%llu\t%s\n", event,
                        (unsigned long long)entry->identity, (unsigned long long)tid, entry->image);
    if (size < 0 || (size_t)size >= sizeof(line)) _exit(94);
    record(line, (size_t)size);
}

static void retire(void *value) {
    struct entry *entry = value;
    pthread_mutex_lock(&identities);
    struct entry **position = &live;
    while (*position != entry) position = &(*position)->next;
    *position = entry->next;
    if (entry->selected) identity_record("retired", entry);
    pthread_cond_broadcast(&retired);
    pthread_mutex_unlock(&identities);
    free(entry);
}

static void *drive(void *value) {
    struct entry *entry = value;
    Dl_info info;
    int found = dladdr((const void *)entry->run, &info);
    entry->image = found && info.dli_fname ? info.dli_fname : "unknown";
    const char *base = strrchr(entry->image, '/');
    base = base ? base + 1 : entry->image;
    const char *selected = getenv("THINKTHEN_THREAD_ENTRY_IMAGE");
    entry->selected = selected && strcmp(base, selected) == 0;
    pthread_mutex_lock(&identities);
    entry->identity = ++sequence;
    entry->next = live;
    live = entry;
    if (entry->selected) identity_record("identified", entry);
    pthread_mutex_unlock(&identities);
    void *result;
    pthread_cleanup_push(retire, entry);
    result = entry->run(entry->argument);
    pthread_cleanup_pop(1);
    return result;
}

static uint64_t worker_locked(void) {
    uint64_t worker = 0;
    for (struct entry *entry = live; entry; entry = entry->next) {
        if (!entry->selected) continue;
        if (worker) return 0;
        worker = entry->identity;
    }
    return worker;
}

/* Test instrumentation only: no symbol is added to the installed SDK. */
uint64_t thinkthen_audit_worker(void) {
    pthread_mutex_lock(&identities);
    uint64_t worker = worker_locked();
    pthread_mutex_unlock(&identities);
    return worker;
}
uint64_t thinkthen_audit_count(void) {
    pthread_mutex_lock(&identities);
    uint64_t count = 0;
    for (struct entry *entry = live; entry; entry = entry->next) count += entry->selected != 0;
    pthread_mutex_unlock(&identities);
    return count;
}
void thinkthen_audit_join(void) {
    pthread_mutex_lock(&identities);
    for (;;) {
        int active = 0;
        for (struct entry *entry = live; entry; entry = entry->next) active |= entry->selected;
        if (!active) break;
        pthread_cond_wait(&retired, &identities);
    }
    pthread_mutex_unlock(&identities);
}

static int audit_create(pthread_t *thread, const pthread_attr_t *attr,
                        void *(*entry)(void *), void *argument) {
    struct entry *tracked = NULL;
    if (getenv("THINKTHEN_THREAD_IDENTITIES")) {
        tracked = calloc(1, sizeof(*tracked));
        if (!tracked) _exit(95);
        tracked->run = entry;
        tracked->argument = argument;
    }
#ifdef __APPLE__
    int result = pthread_create(thread, attr, tracked ? drive : entry, tracked ? tracked : argument);
#else
    int (*original)(pthread_t *, const pthread_attr_t *, void *(*)(void *), void *) =
        dlsym(RTLD_NEXT, "pthread_create");
    int result = original(thread, attr, tracked ? drive : entry, tracked ? tracked : argument);
#endif
    if (result != 0) free(tracked);
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
    if (result == 0 && selected && strcmp(name, selected) == 0) {
        record("named\n", 6);
    }
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
