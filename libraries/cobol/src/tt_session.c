/* Rust owns admission, execution, cache behavior and result lifetime. */
#define _POSIX_C_SOURCE 200809L
#include "tt_session.h"
#include <time.h>
#include <string.h>
int TT_SESSION_NEXT(thinkthen_session *session,uint32_t *status,thinkthen_session_result **out) {
    int code;
    do {
        code=thinkthen_session_try_read(session,status,out);
        if(code || *status!=THINKTHEN_SESSION_PENDING_V1) return code;
        const struct timespec pause={.tv_sec=0,.tv_nsec=1000000};
        nanosleep(&pause,NULL);
    } while(1);
}
int TT_SESSION_TEXT(const thinkthen_complete_utf8_v1 *value,char *out,uint64_t *written) {
    if(!value || !out || !written || (value->len && !value->data)) return THINKTHEN_COBOL_REPRESENTATION;
    if(value->len>THINKTHEN_COBOL_TEXT_CAPACITY) return THINKTHEN_COBOL_OVERFLOW;
    memset(out,' ',THINKTHEN_COBOL_TEXT_CAPACITY);
    if(value->len) memcpy(out,value->data,value->len);
    *written=value->len;
    return 0;
}
