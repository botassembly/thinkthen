/* Fixture operations only. Product ABI and compiler warnings are unchanged. */
#ifndef THINKTHEN_FIXTURE_PLATFORM_H
#define THINKTHEN_FIXTURE_PLATFORM_H
#include <stdio.h>
#include <stdlib.h>
#ifdef _WIN32
#include <windows.h>
#include <io.h>
#include <fcntl.h>
#define THREAD_RESULT DWORD WINAPI
#define THREAD_DONE 0
#define THREAD_TYPE HANDLE
static int fixture_start(HANDLE *thread, LPTHREAD_START_ROUTINE entry, void *argument) {
    *thread = CreateThread(NULL, 0, entry, argument, 0, NULL);
    return *thread == NULL;
}
static int fixture_join(HANDLE thread) {
    DWORD waited = WaitForSingleObject(thread, 60000);
    int closed = CloseHandle(thread) != 0;
    return waited != WAIT_OBJECT_0 || !closed;
}
#define FIXTURE_UNUSED
static void binary_streams(void) {
    if (_setmode(_fileno(stdin), _O_BINARY) == -1 ||
        _setmode(_fileno(stdout), _O_BINARY) == -1 ||
        _setmode(_fileno(stderr), _O_BINARY) == -1) exit(1);
}
#else
#include <pthread.h>
#define THREAD_RESULT void *
#define THREAD_DONE NULL
#define THREAD_TYPE pthread_t
#define fixture_start(thread, entry, argument) pthread_create(thread, NULL, entry, argument)
#define fixture_join(thread) pthread_join(thread, NULL)
#define FIXTURE_UNUSED __attribute__((unused))
static void binary_streams(void) FIXTURE_UNUSED;
static void binary_streams(void) {}
#endif
#endif
