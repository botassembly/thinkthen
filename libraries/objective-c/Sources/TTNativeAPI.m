#include "TTNativeAPI.h"
#include <objc/runtime.h>
#include <stdlib.h>
TTErrorKind tt_native_failure_kind(const TTNativeResult *r) {
    thinkthen_summary_v1 s={0}; if(tt_native_summary(r,&s)) return TTErrorUsage;
    if(!s.error.present) return TTErrorNone;
    return s.error.value.code>=1 && s.error.value.code<=6?(TTErrorKind)s.error.value.code:TTErrorDefect;
}
static int failure_snapshot(thinkthen_engine *e,int code,TTNativeResult **failure) {
    if(!code || !failure) return code;
    thinkthen_result *r=NULL; int rc=thinkthen_error_complete(e,&r);
    if(rc) return rc;
    if(!r) return code;
    rc=tt_native_snapshot(r,failure); return rc?rc:code;
}
static void enter(TTClient *c) { pthread_mutex_lock(&c->mutex); ++c->active; pthread_mutex_unlock(&c->mutex); }
static void leave(TTClient *c) { pthread_mutex_lock(&c->mutex); if(!--c->active) pthread_cond_broadcast(&c->idle); pthread_mutex_unlock(&c->mutex); }
@implementation TTQuestion
- (void)dealloc { thinkthen_question_free(native); object_dispose(self); }
@end
@implementation TTSource
- (void)dealloc { thinkthen_source_free(native); object_dispose(self); }
@end
@implementation TTImage
- (void)dealloc { thinkthen_image_free(native); object_dispose(self); }
@end
@implementation TTBatch
- (int)next:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure {
    if(!outputValue || !pthread_equal(thread,pthread_self())) return THINKTHEN_EUSAGE;
    thinkthen_result *r=NULL; int code=thinkthen_batch_next(native,&r);
    if(code) return failure_snapshot(client->native,code,failure);
    if(!r) { *outputValue=NULL; return 0; } return tt_native_snapshot(r,outputValue);
}
- (int)facts:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure {
    if(!outputValue || !pthread_equal(thread,pthread_self())) return THINKTHEN_EUSAGE;
    thinkthen_result *r=NULL; int code=thinkthen_batch_facts(native,&r);
    if(code) return failure_snapshot(client->native,code,failure); return tt_native_snapshot(r,outputValue);
}
- (void)dealloc {
    if(!pthread_equal(thread,pthread_self())) abort();
    thinkthen_batch_free(native); leave(client); object_dispose(self);
}
@end
@implementation TTClient (Complete)
- (int)question:(const thinkthen_question_spec_v1 *)spec author:(const thinkthen_question_author_v1 *)author output:(TTQuestion **)outputValue failure:(TTNativeResult **)failure {
    if(!outputValue) return THINKTHEN_EUSAGE;
    TTQuestion *q=class_createInstance(objc_getClass("TTQuestion"),0); if(!q) return THINKTHEN_ELOCAL;
    enter(self); int code=thinkthen_question_new_authored(native,spec,author,&q->native);
    code=failure_snapshot(native,code,failure); leave(self);
    if(code) { [q dealloc]; return code; } *outputValue=q; return 0;
}
- (int)question:(const thinkthen_question_spec_v1 *)spec author:(const thinkthen_question_author_v1 *)author task:(const thinkthen_recognition_task_v1 *)task output:(TTQuestion **)outputValue failure:(TTNativeResult **)failure {
    if(!outputValue) return THINKTHEN_EUSAGE;
    TTQuestion *q=class_createInstance(objc_getClass("TTQuestion"),0); if(!q) return THINKTHEN_ELOCAL;
    enter(self); int code=thinkthen_question_new_recognition_v1(native,spec,author,task,&q->native);
    code=failure_snapshot(native,code,failure); leave(self);
    if(code) { [q dealloc]; return code; } *outputValue=q; return 0;
}
#define LOADER(selector,call,params) \
- (int)selector output:(TTQuestion **)outputValue failure:(TTNativeResult **)failure { \
    if(!outputValue) return THINKTHEN_EUSAGE; TTQuestion *q=class_createInstance(objc_getClass("TTQuestion"),0); if(!q) return THINKTHEN_ELOCAL; \
    enter(self); int code=call params; code=failure_snapshot(native,code,failure); leave(self); \
    if(code) { [q dealloc]; return code; } *outputValue=q; return 0; }
LOADER(parseQuestion:(thinkthen_string_v1)json role:(uint32_t)role,thinkthen_question_parse,(native,role,json,&q->native))
LOADER(loadQuestion:(thinkthen_string_v1)path,thinkthen_question_load,(native,path,&q->native))
LOADER(namedQuestion:(thinkthen_string_v1)name role:(uint32_t)role,thinkthen_question_load_named,(native,role,name,&q->native))
LOADER(referenceQuestion:(thinkthen_string_v1)name role:(uint32_t)role,thinkthen_question_load_reference,(native,role,name,&q->native))
#undef LOADER
- (int)records:(const thinkthen_record_v1 *)records count:(size_t)count output:(TTSource **)outputValue failure:(TTNativeResult **)failure {
    if(!outputValue) return THINKTHEN_EUSAGE; TTSource *s=class_createInstance(objc_getClass("TTSource"),0); if(!s) return THINKTHEN_ELOCAL;
    enter(self); int code=thinkthen_source_records(native,records,count,&s->native); code=failure_snapshot(native,code,failure); leave(self);
    if(code) { [s dealloc]; return code; } *outputValue=s; return 0;
}
- (int)sourceFiles:(const thinkthen_source_spec_v1 *)spec imageReader:(int)imageReader output:(TTSource **)outputValue failure:(TTNativeResult **)failure {
    if(!outputValue || (imageReader!=0 && imageReader!=1)) return THINKTHEN_EUSAGE; TTSource *s=class_createInstance(objc_getClass("TTSource"),0); if(!s) return THINKTHEN_ELOCAL;
    enter(self); int code=imageReader?thinkthen_source_image_files(native,spec,&s->native):thinkthen_source_files(native,spec,&s->native); code=failure_snapshot(native,code,failure); leave(self);
    if(code) { [s dealloc]; return code; } *outputValue=s; return 0;
}
- (int)image:(const uint8_t *)bytes length:(size_t)length media:(uint32_t)media filename:(thinkthen_optional_string_v1)filename output:(TTImage **)outputValue failure:(TTNativeResult **)failure {
    if(!outputValue) return THINKTHEN_EUSAGE; TTImage *i=class_createInstance(objc_getClass("TTImage"),0); if(!i) return THINKTHEN_ELOCAL;
    enter(self); int code=thinkthen_image_clone(native,bytes,length,media,filename,&i->native); code=failure_snapshot(native,code,failure); leave(self);
    if(code) { [i dealloc]; return code; } *outputValue=i; return 0;
}
#define COMPLETE(name) \
- (int)name##Complete:(TTQuestion *)q source:(TTSource *)s controls:(const thinkthen_controls_v1 *)controls output:(TTNativeResult **)outputValue failure:(TTNativeResult **)failure { \
    if(!outputValue) return THINKTHEN_EUSAGE; thinkthen_controls_v1 c={0}; c.deadline_ms=-1; if(controls) c=*controls; c.surface=(thinkthen_string_v1){"objective-c",11}; \
    thinkthen_result *r=NULL; enter(self); int code=thinkthen_##name##_complete(native,q?q->native:NULL,s?s->native:NULL,&c,&r); \
    code=code?failure_snapshot(native,code,failure):tt_native_snapshot(r,outputValue); leave(self); return code; }
COMPLETE(decide) COMPLETE(choose) COMPLETE(tag) COMPLETE(score) COMPLETE(filter)
COMPLETE(rank) COMPLETE(find) COMPLETE(annotate) COMPLETE(recognize) COMPLETE(relate)
#undef COMPLETE
#define BATCH(name) \
- (int)name##Batch:(TTQuestion *)q source:(TTSource *)s controls:(const thinkthen_controls_v1 *)controls output:(TTBatch **)outputValue failure:(TTNativeResult **)failure { \
    if(!outputValue) return THINKTHEN_EUSAGE; TTBatch *b=class_createInstance(objc_getClass("TTBatch"),0); if(!b) return THINKTHEN_ELOCAL; \
    thinkthen_controls_v1 c={0}; c.deadline_ms=-1; if(controls) c=*controls; c.surface=(thinkthen_string_v1){"objective-c",11}; \
    enter(self); int code=thinkthen_##name##_batch_start(native,q?q->native:NULL,s?s->native:NULL,&c,&b->native); \
    if(code) { code=failure_snapshot(native,code,failure); leave(self); object_dispose(b); return code; } b->client=self; b->thread=pthread_self(); *outputValue=b; return 0; }
BATCH(decide) BATCH(choose) BATCH(tag) BATCH(score) BATCH(filter) BATCH(annotate)
#undef BATCH
@end
