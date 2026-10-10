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
static int usage_status(const thinkthen_engine *engine,thinkthen_complete_usage_persistence_v1 *state,
                        char *advice,uint64_t *written,int finish) {
    if(!state || !advice || !written) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_complete_usage_persistence_v1 observed;
    thinkthen_complete_utf8_v1 text;
    int code=finish ? thinkthen_engine_finish_usage_status_v1(engine,&observed,&text)
                    : thinkthen_engine_usage_persistence_v1(engine,&observed,&text);
    if(code) return code;
    code=TT_SESSION_TEXT(&text,advice,written);
    if(code) return code;
    *state=observed;
    return 0;
}
int TT_ENGINE_USAGE_PERSISTENCE(const thinkthen_engine *engine,thinkthen_complete_usage_persistence_v1 *state,
                              char *advice,uint64_t *written) {
    return usage_status(engine,state,advice,written,0);
}
int TT_ENGINE_FINISH_USAGE_STATUS(const thinkthen_engine *engine,thinkthen_complete_usage_persistence_v1 *state,
                                char *advice,uint64_t *written) {
    return usage_status(engine,state,advice,written,1);
}
