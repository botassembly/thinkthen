#include "ThinkThen.h"
#include <objc/runtime.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <math.h>

void tt_failure_clear(TTFailure *f) { if (f) { free(f->message); free(f->facts_json); *f = (TTFailure){0}; } }
void tt_call_facts_clear(TTCallFacts *f) { if (f) { free(f->model); *f = (TTCallFacts){0}; } }
static void reject(TTFailure *f) { if (f) { tt_failure_clear(f); f->kind=TTErrorUsage; f->retryable=0; f->message=strdup("NUL or missing C-string input"); } }
static void invalid_result(TTFailure *f) { if (f) { tt_failure_clear(f); f->kind=TTErrorDefect; f->retryable=0; f->message=strdup("native JSON answer broke the type contract"); } }
static char *checked(const char *s, size_t n) {
    if (!s || n == SIZE_MAX || memchr(s,0,n)) return NULL;
    char *copy=malloc(n+1); if (!copy) abort(); memcpy(copy,s,n); copy[n]=0; return copy;
}
static int count_value(const TTJSON *n, uint64_t *out) {
    if (!n || n->type != TTJSONNumber || !n->text || !n->text_length) return 0;
    for (size_t i=0; i<n->text_length; i++) if (n->text[i]<'0' || n->text[i]>'9') return 0;
    errno=0; char *end=NULL; unsigned long long value=strtoull(n->text,&end,10);
    if (errno || !end || *end || value>UINT64_MAX) return 0;
    *out=(uint64_t)value; return 1;
}
static int facts_decode(const char *raw, size_t len, TTCallFacts *out) {
    TTJSON *tree=tt_json_parse(raw,len);
    TTCallFacts value={0}; int valid=tree && tree->type==TTJSONObject && tree->count>=4 && tree->count<=7;
    if (valid) {
        valid=count_value(tt_json_get(tree,"records"),&value.records) &&
              count_value(tt_json_get(tree,"requests_sent"),&value.requests_sent) &&
              count_value(tt_json_get(tree,"cache_answers"),&value.cache_answers);
        const TTJSON *seconds=tt_json_get(tree,"seconds");
        if (!seconds || seconds->type!=TTJSONNumber || !seconds->text) valid=0;
        else { errno=0; char *end=NULL; value.seconds=strtod(seconds->text,&end);
            if (errno || !end || *end || !isfinite(value.seconds) || value.seconds<0) valid=0; }
        const TTJSON *input=tt_json_get(tree,"input_tokens"), *output=tt_json_get(tree,"output_tokens");
        if (input) { value.has_input_tokens=1; valid=valid && count_value(input,&value.input_tokens); }
        if (output) { value.has_output_tokens=1; valid=valid && count_value(output,&value.output_tokens); }
        const TTJSON *model=tt_json_get(tree,"model");
        if (model) {
            value.has_model=1;
            if (model->type!=TTJSONString || memchr(model->text,0,model->text_length) || model->text_length==SIZE_MAX) valid=0;
            else { value.model=malloc(model->text_length+1); if (!value.model) valid=0;
                else { memcpy(value.model,model->text,model->text_length); value.model[model->text_length]=0; } }
        }
        for (size_t i=0;i<tree->count;i++) {
            const char *key=tree->keys[i];
            if (strcmp(key,"records") && strcmp(key,"requests_sent") && strcmp(key,"cache_answers") &&
                strcmp(key,"seconds") && strcmp(key,"input_tokens") && strcmp(key,"output_tokens") &&
                strcmp(key,"model")) valid=0;
        }
    }
    tt_json_free(tree);
    if (!valid) { tt_call_facts_clear(&value); return 0; }
    *out=value; return 1;
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
- (TTErrorKind)decide:(const char *)q text:(const void *)t length:(size_t)n deadline:(int64_t)d token:(TTToken *)tok answer:(TTDecision *)output facts:(TTCallFacts *)facts failure:(TTFailure *)f {
    if (!q || !output || !facts) { reject(f); return THINKTHEN_EUSAGE; }
    thinkthen_answer answer={0}; char *rawFacts=NULL; size_t factsLen=0; TTCallFacts owned={0};
    enter(self,tok);
    int rc=thinkthen_decide_with_facts_opts(native,q,t,n,d,tok ? tok->native : NULL,&answer,&rawFacts,&factsLen);
    if (rc) capture(native,rc,f);
    else if (!facts_decode(rawFacts,factsLen,&owned)) { invalid_result(f); rc=TTErrorDefect; }
    else { output->outcome=(TTOutcome)answer.outcome; output->probability=answer.probability;
        *facts=owned; capture(native,0,f); }
    thinkthen_free_string(rawFacts); leave(self,tok); return (TTErrorKind)rc;
}
- (TTErrorKind)decideBytes:(const char *)q questionLength:(size_t)ql text:(const void *)t length:(size_t)n deadline:(int64_t)d token:(TTToken *)tok answer:(TTDecision *)output facts:(TTCallFacts *)facts failure:(TTFailure *)f {
    char *copy=checked(q,ql); if (!copy) { reject(f); return THINKTHEN_EUSAGE; }
    int rc=[self decide:copy text:t length:n deadline:d token:tok answer:output facts:facts failure:f]; free(copy); return rc;
}
- (TTErrorKind)many:(const char *)q texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n deadline:(int64_t)d token:(TTToken *)tok answers:(TTDecision *)output facts:(TTCallFacts *)facts failure:(TTFailure *)f {
    if (!q || !facts || (n && !output) || n > SIZE_MAX/sizeof(thinkthen_answer)) { reject(f); return THINKTHEN_EUSAGE; }
    thinkthen_answer *answers=calloc(n?n:1,sizeof(*answers));if(!answers)abort();
    char *rawFacts=NULL; size_t factsLen=0; TTCallFacts owned={0};
    enter(self,tok); int rc=thinkthen_decide_many_with_facts_opts(native,q,ts,ns,n,d,tok ? tok->native : NULL,answers,&rawFacts,&factsLen);
    if (rc) capture(native,rc,f);
    else if (!facts_decode(rawFacts,factsLen,&owned)) { invalid_result(f); rc=TTErrorDefect; }
    else { for(size_t i=0;i<n;i++){output[i].outcome=(TTOutcome)answers[i].outcome;output[i].probability=answers[i].probability;}
        *facts=owned; capture(native,0,f); }
    thinkthen_free_string(rawFacts); free(answers); leave(self,tok); return (TTErrorKind)rc;
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
- (TTErrorKind)recognize:(const char *)spec text:(const void *)t length:(size_t)n result:(char **)output size:(size_t *)outLen facts:(TTCallFacts *)facts failure:(TTFailure *)f {
    char *raw=NULL,*rawFacts=NULL,*copy=NULL; size_t len=0,factsLen=0; TTCallFacts owned={0};
    if (!spec || !output || !outLen || !facts) { reject(f); return THINKTHEN_EUSAGE; }
    enter(self,nil);
    int rc=thinkthen_recognize_with_facts_opts(native,spec,t,n,-1,NULL,&raw,&len,&rawFacts,&factsLen);
    if (rc) capture(native,rc,f);
    else {
        TTJSON *tree=tt_json_parse(raw,len); int valid=tt_json_answer_shape(tree,"recognize"); tt_json_free(tree);
        if (valid && len<SIZE_MAX) valid=facts_decode(rawFacts,factsLen,&owned);
        if (valid) { copy=malloc(len+1); if (!copy) valid=0; }
        if (!valid) { tt_call_facts_clear(&owned); invalid_result(f); rc=TTErrorDefect; }
        else { memcpy(copy,raw,len);copy[len]=0;*output=copy;*outLen=len;*facts=owned;capture(native,0,f); }
    }
    thinkthen_free_string(raw);thinkthen_free_string(rawFacts);leave(self,nil);return (TTErrorKind)rc;
}
- (TTErrorKind)relate:(const char *)spec texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n result:(char **)output size:(size_t *)outLen facts:(TTCallFacts *)facts failure:(TTFailure *)f {
    char *raw=NULL,*rawFacts=NULL,*copy=NULL; size_t len=0,factsLen=0; TTCallFacts owned={0};
    if (!spec || !output || !outLen || !facts) { reject(f); return THINKTHEN_EUSAGE; }
    enter(self,nil);
    int rc=thinkthen_relate_with_facts_opts(native,spec,ts,ns,n,-1,NULL,&raw,&len,&rawFacts,&factsLen);
    if (rc) capture(native,rc,f);
    else {
        TTJSON *tree=tt_json_parse(raw,len); int valid=tt_json_answer_shape(tree,"relate"); tt_json_free(tree);
        if (valid && len<SIZE_MAX) valid=facts_decode(rawFacts,factsLen,&owned);
        if (valid) { copy=malloc(len+1); if (!copy) valid=0; }
        if (!valid) { tt_call_facts_clear(&owned); invalid_result(f); rc=TTErrorDefect; }
        else { memcpy(copy,raw,len);copy[len]=0;*output=copy;*outLen=len;*facts=owned;capture(native,0,f); }
    }
    thinkthen_free_string(raw);thinkthen_free_string(rawFacts);leave(self,nil);return (TTErrorKind)rc;
}
- (TTJSON *)parsedAnswer:(const char *)request length:(size_t)length kind:(const char *)kind deadline:(int64_t)d token:(TTToken *)tok failure:(TTFailure *)f {
    char *raw=[self jsonBytes:request length:length deadline:d token:tok failure:f];
    if(!raw)return NULL;
    TTJSON *tree=tt_json_parse(raw,strlen(raw)); free(raw);
    if(!tt_json_answer_shape(tree,kind)){tt_json_free(tree);invalid_result(f);return NULL;}
    return tree;
}
- (TTErrorKind)manyBytes:(const char *)q questionLength:(size_t)ql texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n deadline:(int64_t)d token:(TTToken *)tok answers:(TTDecision *)answerOut facts:(TTCallFacts *)facts failure:(TTFailure *)f {
    char *copy=checked(q,ql); if (!copy) { reject(f); return THINKTHEN_EUSAGE; }
    int rc=[self many:copy texts:ts lengths:ns count:n deadline:d token:tok answers:answerOut facts:facts failure:f]; free(copy); return rc;
}
- (TTErrorKind)recognizeBytes:(const char *)spec specLength:(size_t)sl text:(const void *)t length:(size_t)n result:(char **)answerOut size:(size_t *)len facts:(TTCallFacts *)facts failure:(TTFailure *)f {
    char *copy=checked(spec,sl); if (!copy) { reject(f); return THINKTHEN_EUSAGE; }
    int rc=[self recognize:copy text:t length:n result:answerOut size:len facts:facts failure:f]; free(copy); return rc;
}
- (TTErrorKind)relateBytes:(const char *)spec specLength:(size_t)sl texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n result:(char **)answerOut size:(size_t *)len facts:(TTCallFacts *)facts failure:(TTFailure *)f {
    char *copy=checked(spec,sl); if (!copy) { reject(f); return THINKTHEN_EUSAGE; }
    int rc=[self relate:copy texts:ts lengths:ns count:n result:answerOut size:len facts:facts failure:f]; free(copy); return rc;
}
@end
