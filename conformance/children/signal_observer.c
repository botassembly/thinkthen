/* Nonshipping Darwin fixture: acknowledge only after the native handler returns. */
#define _DARWIN_C_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdatomic.h>
#include <stdlib.h>
#include <unistd.h>

typedef void (*handler)(int, siginfo_t *, void *);
_Static_assert(__atomic_always_lock_free(sizeof(handler), 0), "handler must be lock-free");
_Static_assert(ATOMIC_INT_LOCK_FREE == 2, "fixture state must be lock-free");

static _Atomic(handler) originals[2];
static atomic_int invalid;
static atomic_int registering;
static atomic_int handling;
static int acknowledgment = -1;

__attribute__((constructor)) static void initialize(void) {
    int original_errno = errno;
    const char *value = getenv("CLI_SIGNAL_OBSERVER_FD");
    char *end = NULL;
    long descriptor = value ? strtol(value, &end, 10) : -1;
    if (!value || end == value || *end || descriptor < 0 || descriptor > 0x7fffffff) {
        atomic_store(&invalid, 1);
        errno = original_errno;
        return;
    }
    acknowledgment = (int)descriptor;
    int flags = fcntl(acknowledgment, F_GETFL);
    if (flags < 0 || !(flags & O_NONBLOCK)) atomic_store(&invalid, 1);
    /* Resolve write before handler entry without producing an acknowledgement. */
    if (write(acknowledgment, "", 0) != 0) atomic_store(&invalid, 1);
    errno = original_errno;
}

static int slot(int signum) {
    return signum == SIGINT ? 0 : signum == SIGTERM ? 1 : -1;
}

static void observed(int signum, siginfo_t *info, void *context) {
    int index = slot(signum);
    handler original = atomic_load_explicit(&originals[index], memory_order_acquire);
    if (atomic_fetch_add(&handling, 1) != 0) atomic_store(&invalid, 1);
    if (original) original(signum, info, context);
    else atomic_store(&invalid, 1);
    int native_errno = errno;
    atomic_fetch_sub(&handling, 1);
    if (!atomic_load(&invalid)) {
        unsigned char byte = (unsigned char)signum;
        /* One nonblocking attempt. A missing byte fails the parent fixture. */
        (void)write(acknowledgment, &byte, 1);
    }
    errno = native_errno;
}

static int observe_sigaction(int signum, const struct sigaction *action,
                             struct sigaction *previous) {
    int index = slot(signum);
    if (index < 0) return sigaction(signum, action, previous);
    int concurrent = atomic_exchange(&registering, 1);
    if (concurrent || (action && atomic_load(&handling))) atomic_store(&invalid, 1);
    struct sigaction replacement;
    const struct sigaction *installed = action;
    if (action && action->sa_handler != SIG_DFL && action->sa_handler != SIG_IGN) {
        handler empty = NULL;
        if (concurrent || action->sa_flags != (SA_SIGINFO | SA_RESTART) ||
            !atomic_compare_exchange_strong_explicit(&originals[index], &empty,
                action->sa_sigaction, memory_order_acq_rel, memory_order_acquire)) {
            /* Preserve the real registration, but this lifecycle cannot pass. */
            atomic_store(&invalid, 1);
        } else {
            replacement = *action;
            replacement.sa_sigaction = observed;
            installed = &replacement;
        }
    }
    int result = sigaction(signum, installed, previous);
    int native_errno = errno;
    if (result != 0) atomic_store(&invalid, 1);
    if (result == 0 && previous && previous->sa_sigaction == observed) {
        previous->sa_sigaction = atomic_load_explicit(&originals[index], memory_order_acquire);
    }
    if (!concurrent) atomic_store(&registering, 0);
    errno = native_errno;
    return result;
}

/* The loader consumes this section; no private SDK header is required. */
__attribute__((used)) static const struct {
    const void *replacement;
    const void *original;
} interposition __attribute__((section("__DATA,__interpose"))) = {
    (const void *)&observe_sigaction, (const void *)&sigaction
};
