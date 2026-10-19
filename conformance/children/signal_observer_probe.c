/* Exercise the observer's native boundary, not the product's cancellation flag. */
#define _DARWIN_C_SOURCE
#include <errno.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static volatile sig_atomic_t completed;
static volatile sig_atomic_t arguments_valid;
static int descriptor;
static int reentrant;

static void native_handler(int signum, siginfo_t *info, void *context) {
    sigset_t mask;
    arguments_valid = info && info->si_signo == signum && context &&
        sigprocmask(SIG_BLOCK, NULL, &mask) == 0 &&
        sigismember(&mask, signum) && sigismember(&mask, SIGUSR1);
    if (reentrant) {
        struct sigaction action = {0};
        action.sa_handler = SIG_IGN;
        (void)sigaction(signum, &action, NULL);
    }
    unsigned char marker = 'H';
    (void)write(descriptor, &marker, 1);
    completed = 1;
    errno = EINTR;
}

static int line(const char *text) {
    return write(STDOUT_FILENO, text, strlen(text)) == (ssize_t)strlen(text) ? 0 : 1;
}

int main(int argc, char **argv) {
    if (argc != 3) return 1;
    int signum = atoi(argv[1]);
    descriptor = atoi(getenv("CLI_SIGNAL_OBSERVER_FD"));
    const char *mode = argv[2];
    reentrant = strcmp(mode, "reentrant") == 0;
    sigset_t blocked;
    sigemptyset(&blocked);
    sigaddset(&blocked, signum);
    if (sigprocmask(SIG_BLOCK, &blocked, NULL) != 0) return 2;
    struct sigaction action = {0}, old = {0}, query = {0};
    if (strcmp(mode, "ignored") == 0) {
        action.sa_handler = SIG_IGN;
        if (sigaction(signum, &action, NULL) != 0) return 3;
    }
    action.sa_sigaction = native_handler;
    action.sa_flags = SA_SIGINFO | SA_RESTART;
    sigemptyset(&action.sa_mask);
    sigaddset(&action.sa_mask, SIGUSR1);
    if (strcmp(mode, "reset") == 0) action.sa_flags |= SA_RESETHAND;
    if (sigaction(signum, &action, &old) != 0) return 3;
    if (old.sa_handler != (strcmp(mode, "ignored") == 0 ? SIG_IGN : SIG_DFL)) return 4;
    if (sigaction(signum, NULL, &query) != 0 ||
        query.sa_sigaction != native_handler || query.sa_flags != action.sa_flags ||
        !sigismember(&query.sa_mask, SIGUSR1)) {
        fprintf(stderr, "query flags: %x; registered flags: %x\n", query.sa_flags, action.sa_flags);
        return 5;
    }
    if (strcmp(mode, "replacement") == 0 && sigaction(signum, &action, &old) != 0) return 6;
    if (strcmp(mode, "failure") == 0) {
        errno = 0;
        if (sigaction(SIGKILL, &action, NULL) != -1 || errno != EINVAL) return 7;
    }
    if (line("ready\n")) return 8;
    char command;
    if (read(STDIN_FILENO, &command, 1) != 1 || command != 'p') return 9;
    sigset_t pending;
    if (sigpending(&pending) != 0 || !sigismember(&pending, signum)) return 10;
    if (line("pending\n")) return 11;
    if (read(STDIN_FILENO, &command, 1) != 1 || command != 'u') return 12;
    if (sigprocmask(SIG_UNBLOCK, &blocked, NULL) != 0) return 13;
    if (!completed || !arguments_valid || errno != EINTR) return 14;
    if (strcmp(mode, "stable") == 0 || strcmp(mode, "ignored") == 0 || strcmp(mode, "replacement") == 0 ||
        strcmp(mode, "failure") == 0) {
        action.sa_handler = SIG_DFL;
        action.sa_flags = 0;
        if (sigaction(signum, &action, &old) != 0 || old.sa_sigaction != native_handler) return 15;
        if (sigaction(signum, NULL, &query) != 0 || query.sa_handler != SIG_DFL) return 16;
    }
    return line("completed\n");
}
