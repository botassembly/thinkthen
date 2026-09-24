/*
 * A forked child answers on the engine its parent built and used, and the
 * parent answers again after the child exits.
 */
#include <stdio.h>
#include <sys/wait.h>
#include <unistd.h>

#include <thinkthen.h>

static int asks(const thinkthen_engine *tt) {
    thinkthen_answer answer = {9, 0.0};
    return thinkthen_decide(tt, "Is this a complaint?", "x", 1, &answer) == THINKTHEN_OK &&
           answer.outcome == THINKTHEN_YES;
}

int main(void) {
    thinkthen_engine *tt = thinkthen_engine_new();
    if (tt == NULL || !asks(tt)) {
        fprintf(stderr, "FAIL the parent's first call\n");
        return 1;
    }
    pid_t child = fork();
    if (child == 0) {
        _exit(asks(tt) ? 0 : 3);
    }
    int status = 0;
    if (child < 0 || waitpid(child, &status, 0) != child || !WIFEXITED(status) || WEXITSTATUS(status) != 0) {
        fprintf(stderr, "FAIL the child did not answer: status %d\n", status);
        return 1;
    }
    if (!asks(tt)) {
        fprintf(stderr, "FAIL the parent's call after the fork\n");
        return 1;
    }
    thinkthen_engine_free(tt);
    return 0;
}
