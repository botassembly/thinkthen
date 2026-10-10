#import <Foundation/Foundation.h>
#import "TTResults.g.h"
#import <thinkthen.h>
NS_ASSUME_NONNULL_BEGIN
FOUNDATION_EXPORT NSErrorDomain const TTErrorDomain;
FOUNDATION_EXPORT NSString * const TTFailureDetailsKey;
typedef NS_ENUM(uint32_t, TTUsagePersistenceState) {
    TTUsagePersistenceDisabled = THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1,
    TTUsagePersistencePending = THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1,
    TTUsagePersistenceWritten = THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1,
    TTUsagePersistenceFailed = THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1,
};
/** An immutable snapshot owned independently of the native engine. */
@interface TTUsagePersistenceStatus : NSObject
@property(nonatomic, readonly) TTUsagePersistenceState state;
@property(nonatomic, copy, readonly, nullable) NSString *advice;
- (instancetype)init NS_UNAVAILABLE;
+ (instancetype)new NS_UNAVAILABLE;
@end
/** Foundation byte data becomes the native attachment representation; Rust admits media and bytes. */
FOUNDATION_EXPORT NSDictionary<NSString *, id> *TTImageBytes(NSData *bytes, NSString *media);
/** Retains every delivered packet and the native terminal. */
@interface TTCall : NSObject
@property(nonatomic, copy, readonly) NSArray<TTSessionPacket *> *packets;
@property(nonatomic, strong, readonly) TTSessionPacketTerminal *terminal;
@end
/** A bounded owned session. Native work never calls a caller block. */
@interface TTSession : NSObject
- (BOOL)tryPush:(NSDictionary<NSString *, id> *)descriptor accepted:(BOOL *)accepted closed:(BOOL *)closed error:(NSError * _Nullable * _Nullable)error;
- (BOOL)finish:(nullable NSDictionary<NSString *, id> *)readerFailure error:(NSError * _Nullable * _Nullable)error;
- (nullable TTSessionPacket *)tryReadEnded:(BOOL *)ended error:(NSError * _Nullable * _Nullable)error;
/** Both return before a held provider releases. Cancellation leaves final facts pending. */
- (void)cancel;
- (void)close;
@end
/** Polls the native queues using one timer; buffers no feed or result queue. */
@interface TTTask : NSObject
- (void)cancel;
@end
typedef void (^TTCompletion)(TTCall * _Nullable call, NSError * _Nullable error);
/** Return one descriptor, nil for EOF, or nil plus a native reader-failure object.
    The block runs on a private serial callback queue and must not block. */
typedef NSDictionary<NSString *, id> * _Nullable (^TTFeed)(NSDictionary<NSString *, id> * _Nullable * _Nonnull readerFailure);
@interface TTFoundationClient : NSObject
- (nullable instancetype)initWithSettings:(nullable NSDictionary<NSString *, id> *)settings error:(NSError * _Nullable * _Nullable)error NS_DESIGNATED_INITIALIZER;
- (instancetype)init NS_UNAVAILABLE;
+ (instancetype)new NS_UNAVAILABLE;
/** Observe without waiting for the writer. Failed is a returned state. */
- (nullable TTUsagePersistenceStatus *)usagePersistence:(NSError * _Nullable * _Nullable)error;
/** Finish current deltas; only usage-lock acquisition has a deadline. */
- (nullable TTUsagePersistenceStatus *)finishUsageStatus:(NSError * _Nullable * _Nullable)error;
- (nullable TTSession *)startRequest:(NSDictionary<NSString *, id> *)request error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)decide:(NSDictionary<NSString *, id> *)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)choose:(NSDictionary<NSString *, id> *)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)tag:(NSDictionary<NSString *, id> *)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)score:(NSDictionary<NSString *, id> *)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)filter:(NSDictionary<NSString *, id> *)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)rank:(NSDictionary<NSString *, id> *)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)find:(NSDictionary<NSString *, id> *)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)annotate:(NSDictionary<NSString *, id> *)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)recognize:(NSDictionary<NSString *, id> *)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)relate:(NSDictionary<NSString *, id> *)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
@end
NS_ASSUME_NONNULL_END
