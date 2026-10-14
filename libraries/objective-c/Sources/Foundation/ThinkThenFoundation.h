#import <Foundation/Foundation.h>
#import "TTResults.g.h"
NS_ASSUME_NONNULL_BEGIN
FOUNDATION_EXPORT NSErrorDomain const TTErrorDomain;
FOUNDATION_EXPORT NSString * const TTFailureDetailsKey;
/** Native refusals retain their facts before the thread diagnostic changes. */
@interface TTFailureError : NSError
@property(nonatomic, readonly) TTErrorCode kind;
@property(nonatomic, readonly) BOOL retryable;
@property(nonatomic, strong, readonly, nullable) TTCallError *failure;
@property(nonatomic, strong, readonly, nullable) TTImmediateFacts *facts;
@end
/** An immutable snapshot owned independently of the native engine. */
@interface TTUsagePersistenceStatus : NSObject
@property(nonatomic, readonly) TTUsagePersistenceState state;
@property(nonatomic, copy, readonly, nullable) NSString *advice;
- (instancetype)init NS_UNAVAILABLE;
+ (instancetype)new NS_UNAVAILABLE;
@end
/** Foundation byte data becomes the native attachment representation; Rust admits media and bytes. */
FOUNDATION_EXPORT TTByteImage *TTImageBytes(NSData *bytes, NSString *media);
/** A bounded owned session. Native work never calls a caller block. */
@interface TTSession : NSObject
- (instancetype)init NS_UNAVAILABLE;
+ (instancetype)new NS_UNAVAILABLE;
- (BOOL)tryPush:(TTDescriptor *)descriptor accepted:(BOOL *)accepted closed:(BOOL *)closed error:(NSError * _Nullable * _Nullable)error;
- (BOOL)finish:(nullable TTRequestObject *)readerFailure error:(NSError * _Nullable * _Nullable)error;
- (nullable TTSessionPacket *)tryReadEnded:(BOOL *)ended error:(NSError * _Nullable * _Nullable)error;
/** Consumes nothing and waits at most 50 ms. YES observes input room; NO waits for final output or end after cancellation. The owner remains live through concurrent close. */
- (BOOL)waitInput:(BOOL)input error:(NSError * _Nullable * _Nullable)error;
/** Both return before a held provider releases. Cancellation leaves final facts pending. */
- (void)cancel;
- (void)close;
@end
/** Waits on native queues from a private dispatch queue; buffers no feed or result queue. */
@interface TTTask : NSObject
- (instancetype)init NS_UNAVAILABLE;
+ (instancetype)new NS_UNAVAILABLE;
- (void)cancel;
@end
NS_ASSUME_NONNULL_END
