#ifndef TT_NATIVE_API_H
#define TT_NATIVE_API_H
#include "ThinkThen.h"
#include "TTNative.h"
TTErrorKind tt_native_failure_kind(const TTNativeResult *);
@interface TTQuestion : Object { @public thinkthen_question *native; }
- (void)dealloc;
@end
@interface TTSource : Object { @public thinkthen_source *native; }
- (void)dealloc;
@end
@interface TTImage : Object { @public thinkthen_image *native; }
- (void)dealloc;
@end
/* Start/next/facts/dealloc belong to the creating thread. Keep the client
 * live through batch dealloc. next rows own independent host snapshots. */
@interface TTBatch : Object {
@public thinkthen_batch *native; TTClient *client; pthread_t thread;
}
- (int)next:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure;
- (int)facts:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure;
- (void)dealloc;
@end
@interface TTClient (Complete)
- (int)question:(const thinkthen_question_spec_v1 *)spec author:(const thinkthen_question_author_v1 *)author output:(TTQuestion **)outputValue failure:(TTNativeResult **)failure;
- (int)question:(const thinkthen_question_spec_v1 *)spec author:(const thinkthen_question_author_v1 *)author task:(const thinkthen_recognition_task_v1 *)task output:(TTQuestion **)outputValue failure:(TTNativeResult **)failure;
- (int)parseQuestion:(thinkthen_string_v1)json role:(uint32_t)role output:(TTQuestion **)outputValue failure:(TTNativeResult **)failure;
- (int)loadQuestion:(thinkthen_string_v1)path output:(TTQuestion **)outputValue failure:(TTNativeResult **)failure;
- (int)namedQuestion:(thinkthen_string_v1)name role:(uint32_t)role output:(TTQuestion **)outputValue failure:(TTNativeResult **)failure;
- (int)referenceQuestion:(thinkthen_string_v1)name role:(uint32_t)role output:(TTQuestion **)outputValue failure:(TTNativeResult **)failure;
- (int)records:(const thinkthen_record_v1 *)records count:(size_t)count output:(TTSource **)outputValue failure:(TTNativeResult **)failure;
- (int)sourceFiles:(const thinkthen_source_spec_v1 *)spec imageReader:(int)imageReader output:(TTSource **)outputValue failure:(TTNativeResult **)failure;
- (int)image:(const uint8_t *)bytes length:(size_t)length media:(uint32_t)media filename:(thinkthen_optional_string_v1)filename output:(TTImage **)outputValue failure:(TTNativeResult **)failure;
- (int)decideComplete:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure;
- (int)chooseComplete:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure;
- (int)tagComplete:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure;
- (int)scoreComplete:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure;
- (int)filterComplete:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure;
- (int)rankComplete:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure;
- (int)findComplete:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure;
- (int)annotateComplete:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure;
- (int)recognizeComplete:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure;
- (int)relateComplete:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure;
- (int)decideBatch:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTBatch **)outputValue failure:(TTNativeResult **)failure;
- (int)chooseBatch:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTBatch **)outputValue failure:(TTNativeResult **)failure;
- (int)tagBatch:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTBatch **)outputValue failure:(TTNativeResult **)failure;
- (int)scoreBatch:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTBatch **)outputValue failure:(TTNativeResult **)failure;
- (int)filterBatch:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTBatch **)outputValue failure:(TTNativeResult **)failure;
- (int)annotateBatch:(TTQuestion *)question source:(TTSource *)source controls:(const thinkthen_controls_v1 *)controls output:(TTBatch **)outputValue failure:(TTNativeResult **)failure;
@end
#endif
