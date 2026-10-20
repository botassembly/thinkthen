#import <Foundation/Foundation.h>
#import "TTResults.g.h"
NS_ASSUME_NONNULL_BEGIN
FOUNDATION_EXPORT NSErrorDomain const TTErrorDomain;
FOUNDATION_EXPORT NSString * const TTFailureDetailsKey;
/** Native failures retain their code, facts and optional record position. */
@interface TTFailureError : NSError
@property(nonatomic, readonly) TTErrorCode kind;
@property(nonatomic, readonly) BOOL retryable;
@property(nonatomic, strong, readonly, nullable) NSNumber *position;
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
/** Runs a blocking native call on one dispatch worker. Cancellation requests native completion. */
@interface TTTask : NSObject
- (void)cancel;
@end
NS_ASSUME_NONNULL_END
