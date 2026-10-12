static int failed;
static void check(int holds, const char *what) {
    if (!holds) {
        fprintf(stderr, "FAIL %s\n", what);
        failed = 1;
    }
}
