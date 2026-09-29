#include "ThinkThen.h"
#include <objc/runtime.h>
#include <stdlib.h>
#include <string.h>

void tt_failure_clear(TTFailure *f) { if (f) { free(f->message); free(f->facts_json); *f = (TTFailure){0}; } }
static void reject(TTFailure *f) { if (f) { tt_failure_clear(f); f->kind=TTErrorUsage; f->retryable=0; f->message=strdup("NUL or missing C-string input"); } }
static void invalid_result(TTFailure *f) { if (f) { tt_failure_clear(f); f->kind=TTErrorDefect; f->retryable=0; f->message=strdup("native JSON answer broke the type contract"); } }
static char *checked(const char *s, size_t n) {
    if (!s || n == SIZE_MAX || memchr(s,0,n)) return NULL;
    char *copy=malloc(n+1); if (!copy) abort(); memcpy(copy,s,n); copy[n]=0; return copy;
}
static void capture(const thinkthen_engine *e, int code, TTFailure *f) {
    if (!f) return;
    /* Snapshot on the calling native thread, before another native call. */
    tt_failure_clear(f);
    f->kind = code;
    f->retryable = code ? thinkthen_error_retryable(e) : 0;
    const char *s = code ? thinkthen_error_message(e) : "";
    const char *facts = code ? thinkthen_error_facts_json(e) : NULL;
    f->message = strdup(s ? s : "");
    f->facts_json = facts ? strdup(facts) : NULL;
}
static void enter(TTClient *e, TTToken *t) {
    pthread_mutex_lock(&e->mutex); e->active++; pthread_mutex_unlock(&e->mutex);
    if (t) { pthread_mutex_lock(&t->mutex); t->active++; pthread_mutex_unlock(&t->mutex); }
}
static void leave(TTClient *e, TTToken *t) {
    if (t) { pthread_mutex_lock(&t->mutex); if (!--t->active) pthread_cond_broadcast(&t->idle); pthread_mutex_unlock(&t->mutex); }
    pthread_mutex_lock(&e->mutex); if (!--e->active) pthread_cond_broadcast(&e->idle); pthread_mutex_unlock(&e->mutex);
}
@implementation TTToken
+ (instancetype)create { TTToken *o = class_createInstance(self, 0); if (o) { o->native = thinkthen_cancel_token_new(); if (!o->native) { object_dispose(o); return nil; } pthread_mutex_init(&o->mutex,NULL); pthread_cond_init(&o->idle,NULL); o->active=0; } return o; }
- (void)fire { thinkthen_cancel(native); }
- (void)dealloc { pthread_mutex_lock(&mutex); while(active) pthread_cond_wait(&idle,&mutex); pthread_mutex_unlock(&mutex); thinkthen_cancel_token_free(native); native = NULL; pthread_cond_destroy(&idle); pthread_mutex_destroy(&mutex); object_dispose(self); }
@end
@implementation TTClient
+ (instancetype)create { TTClient *o = class_createInstance(self, 0); if (o) { o->native = thinkthen_engine_new(); if (!o->native) { object_dispose(o); return nil; } pthread_mutex_init(&o->mutex,NULL); pthread_cond_init(&o->idle,NULL); o->active=0; } return o; }
+ (instancetype)createWithSettings:(const char *)settings length:(size_t)length failure:(TTFailure *)failure {
    char *copy=checked(settings,length); if (!copy) { reject(failure); return nil; }
    TTClient *o=class_createInstance(self,0);
    if (!o) { free(copy); if (failure) { tt_failure_clear(failure); failure->kind=TTErrorLocal; failure->message=strdup("client allocation failed"); } return nil; }
    o->native=thinkthen_engine_new_with(copy); free(copy);
    if (!o->native) { capture(NULL,thinkthen_error_code(NULL),failure); object_dispose(o); return nil; }
    pthread_mutex_init(&o->mutex,NULL); pthread_cond_init(&o->idle,NULL); o->active=0;
    capture(o->native,0,failure); return o;
}
- (void)dealloc { pthread_mutex_lock(&mutex); while(active) pthread_cond_wait(&idle,&mutex); pthread_mutex_unlock(&mutex); thinkthen_engine_free(native); native = NULL; pthread_cond_destroy(&idle); pthread_mutex_destroy(&mutex); object_dispose(self); }
- (TTErrorKind)decide:(const char *)q text:(const void *)t length:(size_t)n deadline:(int64_t)d token:(TTToken *)tok answer:(TTDecision *)output failure:(TTFailure *)f {
    if (!q || !output) { reject(f); return THINKTHEN_EUSAGE; }
    thinkthen_answer answer={0};enter(self,tok);
    int rc = thinkthen_decide_opts(native,q,t,n,d,tok ? tok->native : NULL,&answer);
    capture(native,rc,f);if(!rc){output->outcome=(TTOutcome)answer.outcome;output->probability=answer.probability;}
    leave(self,tok); return (TTErrorKind)rc;
}
- (TTErrorKind)decideBytes:(const char *)q questionLength:(size_t)ql text:(const void *)t length:(size_t)n deadline:(int64_t)d token:(TTToken *)tok answer:(TTDecision *)output failure:(TTFailure *)f {
    char *copy=checked(q,ql); if (!copy) { reject(f); return THINKTHEN_EUSAGE; }
    int rc=[self decide:copy text:t length:n deadline:d token:tok answer:output failure:f]; free(copy); return rc;
}
- (TTErrorKind)many:(const char *)q texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n deadline:(int64_t)d token:(TTToken *)tok answers:(TTDecision *)output failure:(TTFailure *)f {
    if (!q || n > SIZE_MAX/sizeof(thinkthen_answer)) { reject(f); return THINKTHEN_EUSAGE; }
    thinkthen_answer *answers=calloc(n?n:1,sizeof(*answers));if(!answers)abort();
    enter(self,tok); int rc = thinkthen_decide_many_opts(native,q,ts,ns,n,d,tok ? tok->native : NULL,answers);
    capture(native,rc,f);if(!rc && output)for(size_t i=0;i<n;i++){output[i].outcome=(TTOutcome)answers[i].outcome;output[i].probability=answers[i].probability;}
    free(answers);leave(self,tok);return (TTErrorKind)rc;
}
- (char *)json:(const char *)request deadline:(int64_t)d token:(TTToken *)tok failure:(TTFailure *)f {
    if (!request) { reject(f); return NULL; }
    enter(self,tok); char *raw = thinkthen_call_opts(native,request,d,tok ? tok->native : NULL);
    /* Copy before freeing the native allocation; this is host-owned. */
    char *copy = raw ? strdup(raw) : NULL;
    if (raw) thinkthen_free_string(raw);
    capture(native,raw ? 0 : thinkthen_error_code(native),f); leave(self,tok);
    return copy;
}
- (char *)jsonBytes:(const char *)request length:(size_t)n deadline:(int64_t)d token:(TTToken *)tok failure:(TTFailure *)f {
    char *copy=checked(request,n); if (!copy) { reject(f); return NULL; }
    char *answer=[self json:copy deadline:d token:tok failure:f]; free(copy); return answer;
}
- (TTErrorKind)recognize:(const char *)spec text:(const void *)t length:(size_t)n result:(char **)output size:(size_t *)outLen failure:(TTFailure *)f {
    char *raw = NULL; size_t len = 0;
    if (!spec || !output || !outLen) { reject(f); return THINKTHEN_EUSAGE; }
    enter(self,nil); int rc = thinkthen_recognize(native,spec,t,n,&raw,&len);
    if (rc == 0) { TTJSON *tree=tt_json_parse(raw,len); int valid=tt_json_answer_shape(tree,"recognize"); tt_json_free(tree);
        if (!valid) { thinkthen_free_string(raw); invalid_result(f); leave(self,nil); return TTErrorDefect; }
        if (len == SIZE_MAX) { thinkthen_free_string(raw); invalid_result(f); leave(self,nil); return TTErrorDefect; }
        *output = malloc(len+1); if (!*output) abort(); memcpy(*output,raw,len); (*output)[len]=0; *outLen=len; thinkthen_free_string(raw); }
    capture(native,rc,f); leave(self,nil); return rc;
}
- (TTErrorKind)relate:(const char *)spec texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n result:(char **)output size:(size_t *)outLen failure:(TTFailure *)f {
    char *raw = NULL; size_t len = 0;
    if (!spec || !output || !outLen) { reject(f); return THINKTHEN_EUSAGE; }
    enter(self,nil); int rc = thinkthen_relate(native,spec,ts,ns,n,&raw,&len);
    if (rc == 0) { TTJSON *tree=tt_json_parse(raw,len); int valid=tt_json_answer_shape(tree,"relate"); tt_json_free(tree);
        if (!valid) { thinkthen_free_string(raw); invalid_result(f); leave(self,nil); return TTErrorDefect; }
        if (len == SIZE_MAX) { thinkthen_free_string(raw); invalid_result(f); leave(self,nil); return TTErrorDefect; }
        *output = malloc(len+1); if (!*output) abort(); memcpy(*output,raw,len); (*output)[len]=0; *outLen=len; thinkthen_free_string(raw); }
    capture(native,rc,f); leave(self,nil); return rc;
}
- (TTJSON *)parsedAnswer:(const char *)request length:(size_t)length kind:(const char *)kind deadline:(int64_t)d token:(TTToken *)tok failure:(TTFailure *)f {
    char *raw=[self jsonBytes:request length:length deadline:d token:tok failure:f];
    if(!raw)return NULL;
    TTJSON *tree=tt_json_parse(raw,strlen(raw)); free(raw);
    if(!tt_json_answer_shape(tree,kind)){tt_json_free(tree);invalid_result(f);return NULL;}
    return tree;
}
- (TTErrorKind)manyBytes:(const char *)q questionLength:(size_t)ql texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n deadline:(int64_t)d token:(TTToken *)tok answers:(TTDecision *)answerOut failure:(TTFailure *)f {
    char *copy=checked(q,ql); if (!copy) { reject(f); return THINKTHEN_EUSAGE; }
    int rc=[self many:copy texts:ts lengths:ns count:n deadline:d token:tok answers:answerOut failure:f]; free(copy); return rc;
}
- (TTErrorKind)recognizeBytes:(const char *)spec specLength:(size_t)sl text:(const void *)t length:(size_t)n result:(char **)answerOut size:(size_t *)len failure:(TTFailure *)f {
    char *copy=checked(spec,sl); if (!copy) { reject(f); return THINKTHEN_EUSAGE; }
    int rc=[self recognize:copy text:t length:n result:answerOut size:len failure:f]; free(copy); return rc;
}
- (TTErrorKind)relateBytes:(const char *)spec specLength:(size_t)sl texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n result:(char **)answerOut size:(size_t *)len failure:(TTFailure *)f {
    char *copy=checked(spec,sl); if (!copy) { reject(f); return THINKTHEN_EUSAGE; }
    int rc=[self relate:copy texts:ts lengths:ns count:n result:answerOut size:len failure:f]; free(copy); return rc;
}
@end
