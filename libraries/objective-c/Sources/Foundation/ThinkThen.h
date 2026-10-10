#import <Foundation/Foundation.h>
#import "TTResults.g.h"
NS_ASSUME_NONNULL_BEGIN
FOUNDATION_EXPORT NSErrorDomain const TTErrorDomain;
FOUNDATION_EXPORT NSString * const TTFailureDetailsKey;
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
- (nullable TTSession *)startRequest:(NSDictionary<NSString *, id> *)request error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)decide:(id)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)choose:(id)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)tag:(id)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)score:(id)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)filter:(id)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)rank:(id)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)find:(id)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)annotate:(id)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)recognize:(id)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)relate:(id)question input:(NSDictionary<NSString *, id> *)input options:(nullable NSDictionary<NSString *, id> *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
@end
NS_ASSUME_NONNULL_END
