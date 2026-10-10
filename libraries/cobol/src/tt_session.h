/* Synchronous COBOL ownership and representation conveniences. */
#ifndef TT_COBOL_SESSION_H
#define TT_COBOL_SESSION_H
#include "thinkthen.h"
#include "tt_requests_generated.h"
int TT_SESSION_NEXT(thinkthen_session *,uint32_t *,thinkthen_session_result **);
int TT_SESSION_TEXT(const thinkthen_complete_utf8_v1 *,char *,uint64_t *);
#endif
