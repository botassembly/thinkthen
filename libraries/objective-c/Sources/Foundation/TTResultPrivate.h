#import "TTResult.h"
NS_ASSUME_NONNULL_BEGIN
@interface TTResultNode ()
- (instancetype)initWithFields:(NSDictionary<NSString *, id> *)fields;
- (TTPresence *)presence:(NSString *)key required:(BOOL)required convert:(id (^)(id))convert;
@end
FOUNDATION_EXPORT void TTInvalidResult(void) __attribute__((noreturn));
FOUNDATION_EXPORT BOOL TTIsBoolean(id value);
FOUNDATION_EXPORT NSDictionary *TTObject(id value);
FOUNDATION_EXPORT NSString *TTString(id value);
FOUNDATION_EXPORT NSNumber *TTNumber(id value);
FOUNDATION_EXPORT NSNumber *TTInteger(id value);
FOUNDATION_EXPORT NSNumber *TTBoolean(id value);
FOUNDATION_EXPORT NSArray *TTArray(id value, id (^convert)(id));
FOUNDATION_EXPORT NSDictionary *TTMap(id value, id (^convert)(id));
NS_ASSUME_NONNULL_END
