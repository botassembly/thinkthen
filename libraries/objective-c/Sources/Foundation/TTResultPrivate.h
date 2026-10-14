#import "ThinkThenFoundation.h"
NS_ASSUME_NONNULL_BEGIN
@interface TTResultNode (Transport)
@property(nonatomic, copy, readonly) NSDictionary<NSString *, id> *rawFields;
+ (instancetype)read:(id)value;
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

@interface TTRequestObject (Transport)
- (instancetype)initWithFields:(NSDictionary *)fields;
@property(nonatomic, readonly) NSMutableDictionary<NSString *, id> *requestFields;
@end
FOUNDATION_EXPORT NSData * _Nullable TTRequestJSONData(id value, NSError * _Nullable * _Nullable error);
FOUNDATION_EXPORT id _Nullable TTAnswerPayload(NSArray *packets, id terminal, NSString *function, NSString *judgment, id call, NSError * _Nullable * _Nullable error);
FOUNDATION_EXPORT TTPresence *TTAnswerPresence(id payload, id (^convert)(id));

@interface TTFoundationClient (Scheduling)
- (nullable TTTask *)start:(NSString *)verb question:(TTQuestion *)question input:(TTInput *)input options:(TTControls *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
@end

@interface TTCall (Transport)
@property(nonatomic, copy, readonly) NSArray<TTSessionPacket *> *packets;
@property(nonatomic, strong, readonly) TTSessionPacketTerminal *terminal;
@end

FOUNDATION_EXPORT NSString * const TTRequestVersion;

NS_ASSUME_NONNULL_END
