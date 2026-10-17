/* Emit both platform line endings and a carriage return that is content. */
#include <stdio.h>
#include <thinkthen.h>
#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#endif

int main(void) {
    const char json[] = "\"cancelled\\r\\n{\\\"kind\\\":\\\"terminal\\\",\\\"facts\\\":{\\\"requests_sent\\\":1}}\\nfreed\\r\\nembedded\\rtail\\r\\n\"";
#ifdef _WIN32
    if (_setmode(_fileno(stdout), _O_BINARY) == -1) return 1;
#endif
    struct thinkthen_value *owner = NULL;
    const struct thinkthen_complete_json_v1 *value = NULL;
    if (thinkthen_value_parse(json, sizeof(json) - 1, &owner) != THINKTHEN_OK) return 1;
    int status = 1;
    if (thinkthen_value_view(owner, &value) == THINKTHEN_OK &&
        value->kind == THINKTHEN_COMPLETE_JSON_STRING_V1) {
        const struct thinkthen_complete_utf8_v1 text = value->data.string;
        status = fwrite(text.data, 1, text.len, stdout) == text.len ? 0 : 1;
    }
    thinkthen_value_free(owner);
    return status;
}
