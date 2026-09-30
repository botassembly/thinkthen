#include "ThinkThen.h"
#include <objc/runtime.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void tt_failure_clear(TTFailure *f) { if (f) { free(f->message); free(f->facts_json); *f = (TTFailure){0}; } }
static void reject(TTFailure *f) { if (f) { tt_failure_clear(f); f->kind=TTErrorUsage; f->retryable=0; f->message=strdup("NUL or missing C-string input"); } }
static void invalid_result(TTFailure *f) { if (f) { tt_failure_clear(f); f->kind=TTErrorDefect; f->retryable=0; f->message=strdup("native JSON answer broke the type contract"); } }
/* Copy counted bytes into a host-owned C string; NUL inside is refused. */
static char *checked(const char *s, size_t n) {
    if (!s || n == SIZE_MAX || memchr(s,0,n)) return NULL;
    char *copy=malloc(n+1); if (!copy) abort(); memcpy(copy,s,n); copy[n]=0; return copy;
}
int tt_field_read(const TTJSON *member, TTField *out) {
    if (!member || !out) return 0;
    if (member->type == TTJSONNull) { *out=(TTField){TTFieldUnresolved,TTErrorNone,NULL}; return 1; }
    if (member->type != TTJSONObject) { *out=(TTField){TTFieldValue,TTErrorNone,NULL}; return 1; }
    const TTJSON *failed=tt_json_get(member,"failed"), *kind=tt_json_get(failed,"kind"), *cause=tt_json_get(failed,"cause");
    if (!kind || kind->type != TTJSONString || !cause || cause->type != TTJSONString) return 0;
    static const char *kinds[]={"usage","backend","deadline","local","cancelled","defect"};
    for (int i=0; i<6; i++)
        if (kind->text_length == strlen(kinds[i]) && !memcmp(kind->text,kinds[i],kind->text_length)) {
            *out=(TTField){TTFieldFailed,(TTErrorKind)(i+1),cause->text}; return 1;
        }
    return 0;
}
/* Append `text` as one JSON string: quote, backslash and control bytes escape. */
static void quoted(char **buf, size_t *len, const char *text, size_t n) {
    *buf=realloc(*buf,*len+n*6+3); if (!*buf) abort();
    char *out=*buf+*len; *out++='"';
    for (size_t i=0;i<n;i++) {
        unsigned char c=(unsigned char)text[i];
        if (c=='"' || c=='\\') { *out++='\\'; *out++=(char)c; }
        else if (c<0x20) out+=sprintf(out,"\\u%04x",c);
        else *out++=(char)c;
    }
    *out++='"'; *len=(size_t)(out-*buf);
}
static void raw_text(char **buf, size_t *len, const char *text) {
    size_t n=strlen(text); *buf=realloc(*buf,*len+n+1); if (!*buf) abort(); memcpy(*buf+*len,text,n); *len+=n;
}
static int json_object(const char *text) {
    TTJSON *tree=tt_json_parse(text,strlen(text)); int object=tree && tree->type==TTJSONObject; tt_json_free(tree); return object;
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
- (TTErrorKind)decide:(const char *)q text:(const void *)t length:(size_t)n deadline:(int64_t)d token:(TTToken *)tok answer:(TTDecision *)output facts:(char **)facts failure:(TTFailure *)f {
    if (!q || !output || !facts) { reject(f); return THINKTHEN_EUSAGE; }
    thinkthen_answer answer={0}; char *rawFacts=NULL, *owned=NULL; size_t factsLen=0;
    enter(self,tok);
    int rc=thinkthen_decide_with_facts_opts(native,q,t,n,d,tok ? tok->native : NULL,&answer,&rawFacts,&factsLen);
    if (rc) capture(native,rc,f);
    else if (!(owned=checked(rawFacts,factsLen))) { invalid_result(f); rc=TTErrorDefect; }
    else { output->outcome=(TTOutcome)answer.outcome; output->probability=answer.probability;
        *facts=owned; capture(native,0,f); }
    thinkthen_free_string(rawFacts); leave(self,tok); return (TTErrorKind)rc;
}
- (TTErrorKind)decideBytes:(const char *)q questionLength:(size_t)ql text:(const void *)t length:(size_t)n deadline:(int64_t)d token:(TTToken *)tok answer:(TTDecision *)output facts:(char **)facts failure:(TTFailure *)f {
    char *copy=checked(q,ql); if (!copy) { reject(f); return THINKTHEN_EUSAGE; }
    int rc=[self decide:copy text:t length:n deadline:d token:tok answer:output facts:facts failure:f]; free(copy); return rc;
}
- (TTErrorKind)many:(const char *)q texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n deadline:(int64_t)d token:(TTToken *)tok answers:(TTDecision *)output facts:(char **)facts failure:(TTFailure *)f {
    if (!q || !facts || (n && !output) || n > SIZE_MAX/sizeof(thinkthen_answer)) { reject(f); return THINKTHEN_EUSAGE; }
    thinkthen_answer *answers=calloc(n?n:1,sizeof(*answers));if(!answers)abort();
    char *rawFacts=NULL, *owned=NULL; size_t factsLen=0;
    enter(self,tok); int rc=thinkthen_decide_many_with_facts_opts(native,q,ts,ns,n,d,tok ? tok->native : NULL,answers,&rawFacts,&factsLen);
    if (rc) capture(native,rc,f);
    else if (!(owned=checked(rawFacts,factsLen))) { invalid_result(f); rc=TTErrorDefect; }
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
- (TTErrorKind)recognize:(const char *)spec text:(const void *)t length:(size_t)n result:(char **)output size:(size_t *)outLen deadline:(int64_t)d token:(TTToken *)tok facts:(char **)facts failure:(TTFailure *)f {
    char *raw=NULL,*rawFacts=NULL,*copy=NULL,*owned=NULL; size_t len=0,factsLen=0;
    if (!spec || !output || !outLen || !facts) { reject(f); return THINKTHEN_EUSAGE; }
    enter(self,tok);
    int rc=thinkthen_recognize_with_facts_opts(native,spec,t,n,d,tok ? tok->native : NULL,&raw,&len,&rawFacts,&factsLen);
    if (rc) capture(native,rc,f);
    else if (!(copy=checked(raw,len)) || !(owned=checked(rawFacts,factsLen))) { free(copy); invalid_result(f); rc=TTErrorDefect; }
    else { *output=copy; *outLen=len; *facts=owned; capture(native,0,f); }
    thinkthen_free_string(raw);thinkthen_free_string(rawFacts);leave(self,tok);return (TTErrorKind)rc;
}
- (TTErrorKind)relate:(const char *)spec texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n result:(char **)output size:(size_t *)outLen deadline:(int64_t)d token:(TTToken *)tok facts:(char **)facts failure:(TTFailure *)f {
    char *raw=NULL,*rawFacts=NULL,*copy=NULL,*owned=NULL; size_t len=0,factsLen=0;
    if (!spec || !output || !outLen || !facts) { reject(f); return THINKTHEN_EUSAGE; }
    enter(self,tok);
    int rc=thinkthen_relate_with_facts_opts(native,spec,ts,ns,n,d,tok ? tok->native : NULL,&raw,&len,&rawFacts,&factsLen);
    if (rc) capture(native,rc,f);
    else if (!(copy=checked(raw,len)) || !(owned=checked(rawFacts,factsLen))) { free(copy); invalid_result(f); rc=TTErrorDefect; }
    else { *output=copy; *outLen=len; *facts=owned; capture(native,0,f); }
    thinkthen_free_string(raw);thinkthen_free_string(rawFacts);leave(self,tok);return (TTErrorKind)rc;
}
- (char *)plan:(const char *)verb question:(const char *)q texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n settings:(const char *)settings failure:(TTFailure *)f {
    if (!verb || !q || (n && (!ts || !ns))) { reject(f); return NULL; }
    const char *start=q; while (*start==' ' || *start=='\t' || *start=='\n' || *start=='\r') start++;
    int object=*start=='{';
    if ((object && !json_object(q)) || (settings && !json_object(settings))) {
        if (f) { tt_failure_clear(f); f->kind=TTErrorUsage; f->message=strdup("plan question object or settings is not a JSON object"); }
        return NULL;
    }
    char *request=NULL; size_t len=0;
    raw_text(&request,&len,"{\"verb\":"); quoted(&request,&len,verb,strlen(verb));
    raw_text(&request,&len,",\"question\":");
    if (object) raw_text(&request,&len,q); else quoted(&request,&len,q,strlen(q));
    raw_text(&request,&len,",\"input\":[");
    for (size_t i=0;i<n;i++) { if (i) raw_text(&request,&len,","); quoted(&request,&len,ts[i],ns[i]); }
    raw_text(&request,&len,"]");
    if (settings) { raw_text(&request,&len,",\"settings\":"); raw_text(&request,&len,settings); }
    raw_text(&request,&len,"}"); request[len]=0;
    char *out=NULL, *copy=NULL; size_t outLen=0;
    enter(self,nil);
    int rc=thinkthen_plan_json(native,request,&out,&outLen);
    if (rc) capture(native,rc,f);
    else if (!(copy=checked(out,outLen))) invalid_result(f);
    else capture(native,0,f);
    thinkthen_free_string(out); free(request); leave(self,nil); return copy;
}
- (TTErrorKind)manyBytes:(const char *)q questionLength:(size_t)ql texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n deadline:(int64_t)d token:(TTToken *)tok answers:(TTDecision *)answerOut facts:(char **)facts failure:(TTFailure *)f {
    char *copy=checked(q,ql); if (!copy) { reject(f); return THINKTHEN_EUSAGE; }
    int rc=[self many:copy texts:ts lengths:ns count:n deadline:d token:tok answers:answerOut facts:facts failure:f]; free(copy); return rc;
}
- (TTErrorKind)recognizeBytes:(const char *)spec specLength:(size_t)sl text:(const void *)t length:(size_t)n result:(char **)answerOut size:(size_t *)len deadline:(int64_t)d token:(TTToken *)tok facts:(char **)facts failure:(TTFailure *)f {
    char *copy=checked(spec,sl); if (!copy) { reject(f); return THINKTHEN_EUSAGE; }
    int rc=[self recognize:copy text:t length:n result:answerOut size:len deadline:d token:tok facts:facts failure:f]; free(copy); return rc;
}
- (TTErrorKind)relateBytes:(const char *)spec specLength:(size_t)sl texts:(const char *const *)ts lengths:(const size_t *)ns count:(size_t)n result:(char **)answerOut size:(size_t *)len deadline:(int64_t)d token:(TTToken *)tok facts:(char **)facts failure:(TTFailure *)f {
    char *copy=checked(spec,sl); if (!copy) { reject(f); return THINKTHEN_EUSAGE; }
    int rc=[self relate:copy texts:ts lengths:ns count:n result:answerOut size:len deadline:d token:tok facts:facts failure:f]; free(copy); return rc;
}
@end
