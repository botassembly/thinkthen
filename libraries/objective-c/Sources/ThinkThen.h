#include <objc/Object.h>
#include <pthread.h>
#include <stdint.h>
#include <thinkthen.h>
#include "TTJSON.h"

typedef enum { TTOutcomeNo=0, TTOutcomeYes=1, TTOutcomeNotSure=2 } TTOutcome;
typedef struct { TTOutcome outcome; double probability; } TTDecision;
typedef enum { TTErrorNone=0, TTErrorUsage=1, TTErrorBackend=2, TTErrorDeadline=3, TTErrorLocal=4, TTErrorCancelled=5, TTErrorDefect=6 } TTErrorKind;
typedef struct { TTErrorKind kind; int retryable; char *message; char *facts_json; } TTFailure;
/* One annotate answer member: JSON null is unresolved, {"failed":{...}} is a
 * failure, and any other value is answered. `cause` borrows from the member. */
typedef enum { TTFieldUnresolved, TTFieldValue, TTFieldFailed } TTFieldState;
typedef struct { TTFieldState state; TTErrorKind kind; const char *cause; } TTField;
/* The facade owns a token until every thread carrying it has joined. Do not
 * begin a new call or access a handle while another thread deallocates it. */
void tt_failure_clear(TTFailure *failure);
/* Read one member of an annotate row's answers. Returns 0, leaving `out`
 * alone, for an object that is not a failure or a failure of unknown kind. */
int tt_field_read(const TTJSON *member, TTField *out);

@interface TTToken : Object {
@public
    thinkthen_cancel_token *native;
    pthread_mutex_t mutex;
    pthread_cond_t idle;
    unsigned active;
}
+ (instancetype)create;
- (void)fire;
- (void)dealloc;
@end

@interface TTClient : Object {
@public
    thinkthen_engine *native;
    pthread_mutex_t mutex;
    pthread_cond_t idle;
    unsigned active;
}
/* Every typed selector sets `*facts` on success to host-owned JSON text; free
 * it with `free` and parse it with tt_json_parse. A failure leaves it alone. */
+ (instancetype)create;
+ (instancetype)createWithSettings:(const char *)settings length:(size_t)length failure:(TTFailure *)failure;
- (void)dealloc;
- (TTErrorKind)decide:(const char *)question text:(const void *)text length:(size_t)length deadline:(int64_t)deadline token:(TTToken *)token answer:(TTDecision *)output facts:(char **)facts failure:(TTFailure *)failure;
- (TTErrorKind)decideBytes:(const char *)question questionLength:(size_t)questionLength text:(const void *)text length:(size_t)length deadline:(int64_t)deadline token:(TTToken *)token answer:(TTDecision *)output facts:(char **)facts failure:(TTFailure *)failure;
- (TTErrorKind)manyBytes:(const char *)question questionLength:(size_t)questionLength texts:(const char *const *)texts lengths:(const size_t *)lengths count:(size_t)count deadline:(int64_t)deadline token:(TTToken *)token answers:(TTDecision *)output facts:(char **)facts failure:(TTFailure *)failure;
- (TTErrorKind)many:(const char *)question texts:(const char *const *)texts lengths:(const size_t *)lengths count:(size_t)count deadline:(int64_t)deadline token:(TTToken *)token answers:(TTDecision *)output facts:(char **)facts failure:(TTFailure *)failure;
/* Explicit native reader for every verb. source is JSON paths/unit/window.
 * Caller frees a successful JSON reply with free, as for json:. */
- (char *)files:(const char *)question source:(const char *)source deadline:(int64_t)deadline token:(TTToken *)token failure:(TTFailure *)failure;
- (char *)json:(const char *)request deadline:(int64_t)deadline token:(TTToken *)token failure:(TTFailure *)failure;
- (char *)jsonBytes:(const char *)request length:(size_t)length deadline:(int64_t)deadline token:(TTToken *)token failure:(TTFailure *)failure;
- (TTErrorKind)recognizeBytes:(const char *)spec specLength:(size_t)specLength text:(const void *)text length:(size_t)length result:(char **)output size:(size_t *)outLen deadline:(int64_t)deadline token:(TTToken *)token facts:(char **)facts failure:(TTFailure *)failure;
- (TTErrorKind)relateBytes:(const char *)spec specLength:(size_t)specLength texts:(const char *const *)texts lengths:(const size_t *)lengths count:(size_t)count result:(char **)output size:(size_t *)outLen deadline:(int64_t)deadline token:(TTToken *)token facts:(char **)facts failure:(TTFailure *)failure;
- (TTErrorKind)recognize:(const char *)spec text:(const void *)text length:(size_t)length result:(char **)output size:(size_t *)outLen deadline:(int64_t)deadline token:(TTToken *)token facts:(char **)facts failure:(TTFailure *)failure;
/* Preview a decide, choose, score or tag call without sending it. The
 * question is bare text, or one question object when it starts with "{".
 * settings is NULL or a thinkthen.settings/1 object. Returns the result
 * schema's plan object as host-owned JSON text; needs no key, sends nothing. */
- (char *)plan:(const char *)verb question:(const char *)question texts:(const char *const *)texts lengths:(const size_t *)lengths count:(size_t)count settings:(const char *)settings failure:(TTFailure *)failure;
- (TTErrorKind)relate:(const char *)spec texts:(const char *const *)texts lengths:(const size_t *)lengths count:(size_t)count result:(char **)output size:(size_t *)outLen deadline:(int64_t)deadline token:(TTToken *)token facts:(char **)facts failure:(TTFailure *)failure;
@end
