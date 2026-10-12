/* Keep the owned profile path when a test clears its child's environment. */
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/resource.h>
#include <sys/stat.h>
#include <unistd.h>
extern void __llvm_profile_set_filename(const char *);
#ifndef PROFILE_PATH
#error "PROFILE_PATH must name the driver's owned active collection"
#endif
#ifndef PROFILE_DIR
#error "PROFILE_DIR must name the driver's owned active directory"
#endif
__attribute__((constructor)) static void coverage_profile(void) {
    struct rlimit limit;
    if (getrlimit(RLIMIT_FSIZE, &limit) == 0 && limit.rlim_cur != RLIM_INFINITY) {
        /* A directory entry records the gap without a denied regular-file write. */
        char observation[sizeof(PROFILE_DIR) + 64];
        snprintf(observation, sizeof(observation), "%s/limited-fsize-%ld", PROFILE_DIR, (long)getpid());
        if (mkdir(observation, 0700) == 0 || errno == EEXIST) {
            __llvm_profile_set_filename("/dev/null");
            return;
        }
        /* Failure to record the limitation must leave the real failure visible. */
    }
    const char *path = getenv("LLVM_PROFILE_FILE");
    __llvm_profile_set_filename(path ? path : PROFILE_PATH);
}
