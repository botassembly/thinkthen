#import <Foundation/Foundation.h>
NS_ASSUME_NONNULL_BEGIN
/** Missing, present null, and present value are independent states. */
typedef NS_ENUM(NSUInteger, TTPresenceState) { TTPresenceMissing, TTPresenceNull, TTPresenceValue };
@interface TTPresence<__covariant ValueType> : NSObject
@property(nonatomic, readonly) TTPresenceState state;
@property(nonatomic, readonly, nullable) ValueType value;
@end
/** A host-owned result. Unknown members remain available in fields. */
@interface TTResultNode : NSObject
@property(nonatomic, copy, readonly) NSDictionary<NSString *, id> *rawFields;
@end
NS_ASSUME_NONNULL_END
