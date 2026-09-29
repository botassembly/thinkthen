#include <objc/Object.h>
#include <pthread.h>
#include "thinkthen.h"
#include "TTJSON.h"

typedef enum { TTOutcomeNo=0, TTOutcomeYes=1, TTOutcomeNotSure=2 } TTOutcome;
typedef struct { TTOutcome outcome; double probability; } TTDecision;
typedef enum { TTErrorNone=0, TTErrorUsage=1, TTErrorBackend=2, TTErrorDeadline=3, TTErrorLocal=4, TTErrorCancelled=5, TTErrorDefect=6 } TTErrorKind;
typedef struct { TTErrorKind kind; int retryable; char *message; char *facts_json; } TTFailure;
/* Distinguish an unresolved JSON null from a failure object. */
typedef enum { TTFieldUnresolved, TTFieldValue, TTFieldFailed } TTFieldState;
typedef struct { TTFieldState state; TTErrorKind kind; char *cause; } TTField;
/* The facade owns a token until every thread carrying it has joined. Do not
 * begin a new call or access a handle while another thread deallocates it. */
void tt_failure_clear(TTFailure *failure);

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
+ (instancetype)create;
+ (instancetype)createWithSettings:(const char *)settings length:(size_t)length failure:(TTFailure *)failure;
- (void)dealloc;
- (TTErrorKind)decide:(const char *)question text:(const void *)text length:(size_t)length deadline:(int64_t)deadline token:(TTToken *)token answer:(TTDecision *)output failure:(TTFailure *)failure;
- (TTErrorKind)decideBytes:(const char *)question questionLength:(size_t)questionLength text:(const void *)text length:(size_t)length deadline:(int64_t)deadline token:(TTToken *)token answer:(TTDecision *)output failure:(TTFailure *)failure;
- (TTErrorKind)manyBytes:(const char *)question questionLength:(size_t)questionLength texts:(const char *const *)texts lengths:(const size_t *)lengths count:(size_t)count deadline:(int64_t)deadline token:(TTToken *)token answers:(TTDecision *)output failure:(TTFailure *)failure;
- (TTErrorKind)many:(const char *)question texts:(const char *const *)texts lengths:(const size_t *)lengths count:(size_t)count deadline:(int64_t)deadline token:(TTToken *)token answers:(TTDecision *)output failure:(TTFailure *)failure;
- (char *)json:(const char *)request deadline:(int64_t)deadline token:(TTToken *)token failure:(TTFailure *)failure;
- (char *)jsonBytes:(const char *)request length:(size_t)length deadline:(int64_t)deadline token:(TTToken *)token failure:(TTFailure *)failure;
- (TTErrorKind)recognizeBytes:(const char *)spec specLength:(size_t)specLength text:(const void *)text length:(size_t)length result:(char **)output size:(size_t *)outLen failure:(TTFailure *)failure;
- (TTErrorKind)relateBytes:(const char *)spec specLength:(size_t)specLength texts:(const char *const *)texts lengths:(const size_t *)lengths count:(size_t)count result:(char **)output size:(size_t *)outLen failure:(TTFailure *)failure;
- (TTErrorKind)recognize:(const char *)spec text:(const void *)text length:(size_t)length result:(char **)output size:(size_t *)outLen failure:(TTFailure *)failure;
/* Returned tree belongs to the caller. Annotate failures are TTJSONObject
 * {"failed":{"kind":...,"cause":...}}, never TTJSONNull. Entity offsets
 * count zero-based Unicode code points, end exclusive. */
- (TTJSON *)parsedAnswer:(const char *)request length:(size_t)length kind:(const char *)kind deadline:(int64_t)deadline token:(TTToken *)token failure:(TTFailure *)failure;
- (TTErrorKind)relate:(const char *)spec texts:(const char *const *)texts lengths:(const size_t *)lengths count:(size_t)count result:(char **)output size:(size_t *)outLen failure:(TTFailure *)failure;
@end
