/* Keep the owned profile path when a test clears its child's environment. */
#include <stdlib.h>
extern void __llvm_profile_set_filename(const char *);
#ifndef PROFILE_PATH
#error "PROFILE_PATH must name the driver's owned active collection"
#endif
__attribute__((constructor)) static void coverage_profile(void) {
    const char *path = getenv("LLVM_PROFILE_FILE");
    __llvm_profile_set_filename(path ? path : PROFILE_PATH);
}
