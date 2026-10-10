#import "TTResultPrivate.h"
#import <CoreFoundation/CoreFoundation.h>
static id TTFreeze(id value) {
    if ([value isKindOfClass:NSDictionary.class]) return TTMap(value, ^id(id item) { return TTFreeze(item); });
    if ([value isKindOfClass:NSArray.class]) return TTArray(value, ^id(id item) { return TTFreeze(item); });
    if ([value isKindOfClass:NSString.class] || [value isKindOfClass:NSNumber.class]) return [value copy];
    if (value == NSNull.null || [value isKindOfClass:TTResultNode.class]) return value;
    TTInvalidResult();
}
@interface TTPresence ()
@property(nonatomic) TTPresenceState state;
@property(nonatomic, strong, nullable) id value;
@end
@implementation TTPresence
@end
@implementation TTResultNode
- (instancetype)initWithFields:(NSDictionary *)fields {
    if ((self = [super init])) _rawFields = TTFreeze(fields);
    return self;
}
- (TTPresence *)presence:(NSString *)key required:(BOOL)required convert:(id (^)(id))convert {
    id value = self.rawFields[key];
    if (!value && required) TTInvalidResult();
    TTPresence *presence = [TTPresence new];
    presence.state = !value ? TTPresenceMissing : value == NSNull.null ? TTPresenceNull : TTPresenceValue;
    if (presence.state == TTPresenceValue) presence.value = convert(value);
    return presence;
}
@end
void TTInvalidResult(void) { @throw [NSException exceptionWithName:NSInternalInconsistencyException reason:@"Native result has an incompatible representation" userInfo:nil]; }
BOOL TTIsBoolean(id value) { return [value isKindOfClass:NSNumber.class] && CFGetTypeID((__bridge CFTypeRef)value) == CFBooleanGetTypeID(); }
NSDictionary *TTObject(id value) { if (![value isKindOfClass:NSDictionary.class]) TTInvalidResult(); return value; }
NSString *TTString(id value) { if (![value isKindOfClass:NSString.class]) TTInvalidResult(); return value; }
NSNumber *TTNumber(id value) { if (![value isKindOfClass:NSNumber.class] || TTIsBoolean(value)) TTInvalidResult(); return value; }
NSNumber *TTInteger(id value) { NSNumber *number = TTNumber(value); if ([number.stringValue rangeOfString:@"."].location != NSNotFound) TTInvalidResult(); return number; }
NSNumber *TTBoolean(id value) { if (!TTIsBoolean(value)) TTInvalidResult(); return value; }
NSArray *TTArray(id value, id (^convert)(id)) {
    if (![value isKindOfClass:NSArray.class]) TTInvalidResult();
    NSMutableArray *result = [NSMutableArray array];
    for (id item in value) [result addObject:convert(item)];
    return [result copy];
}
NSDictionary *TTMap(id value, id (^convert)(id)) {
    NSDictionary *object = TTObject(value);
    NSMutableDictionary *result = [NSMutableDictionary dictionary];
    for (NSString *key in object) result[key] = convert(object[key]);
    return [result copy];
}
