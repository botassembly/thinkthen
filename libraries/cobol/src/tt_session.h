/* Synchronous COBOL ownership and representation conveniences. */
#ifndef TT_COBOL_SESSION_H
#define TT_COBOL_SESSION_H
#include "thinkthen.h"
#include "tt_requests_generated.h"
int TT_SESSION_NEXT(thinkthen_session *,uint32_t *,thinkthen_session_result **);
int TT_SESSION_TEXT(const thinkthen_complete_utf8_v1 *,char *,uint64_t *);
/* Advice storage has THINKTHEN_COBOL_TEXT_CAPACITY bytes. Nonzero returns
 * preserve outputs; native operation errors use thinkthen_session_error_message. */
int TT_ENGINE_USAGE_PERSISTENCE(const thinkthen_engine *,thinkthen_complete_usage_persistence_v1 *,char *,uint64_t *);
int TT_ENGINE_FINISH_USAGE_STATUS(const thinkthen_engine *,thinkthen_complete_usage_persistence_v1 *,char *,uint64_t *);
#endif
