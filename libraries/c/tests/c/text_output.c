/* Emit both platform line endings and a carriage return that is content. */
#include <stdio.h>
#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#endif

int main(void) {
    const char output[] = "cancelled\r\nfreed\nembedded\rtail\r\n";
#ifdef _WIN32
    if (_setmode(_fileno(stdout), _O_BINARY) == -1) return 1;
#endif
    return fwrite(output, 1, sizeof(output) - 1, stdout) == sizeof(output) - 1 ? 0 : 1;
}
