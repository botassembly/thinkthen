"""Foundation declarations and mechanical readers from the prepared Rust graph."""
import json
import re
from csharp import shape, enum_values
from host_answers import aggregate_answers, named_object_fields, usage_operations

ROOTS = ('completesessionPacket', 'completeplan')


def name(key):
    return 'TT' + ''.join(part[0].upper() + part[1:] for part in re.findall(
        r'[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+', key.removeprefix('complete')))


def member(key):
    value = name(key)[2:]
    value = value[0].lower() + value[1:]
    return value + 'Value' if value in ('class', 'description', 'id', 'new', 'copy', 'mutableCopy', 'init', 'true', 'false') else value


def literal(value):
    return '@' + json.dumps(value)


def conversion(raw, definitions, expression='value', depth=0):
    source = shape(raw)
    if '$ref' in source:
        key = source['$ref'].removeprefix('#/$defs/')
        target = shape(definitions[key])
        if any(k in target for k in ('properties', 'variants', 'primitive_variants')):
            return name(key) + ' *', f'[{name(key)} read:{expression}]'
        return conversion(target, definitions, expression, depth)
    if source == {} or source.get('type') == 'null':
        return 'id', expression
    if enum_values(source) or isinstance(source.get('const'), str):
        return 'NSString *', f'TTString({expression})'
    kind = source.get('type')
    if kind == 'array':
        typ, decode = conversion(source['items'], definitions, 'item', depth + 1)
        return f'NSArray<{typ}> *', f'TTArray({expression}, ^id(id item) {{ return {decode}; }})'
    if kind == 'object':
        if isinstance(source.get('additionalProperties'), dict):
            typ, decode = conversion(source['additionalProperties'], definitions, 'item', depth + 1)
            return f'NSDictionary<NSString *, {typ}> *', f'TTMap({expression}, ^id(id item) {{ return {decode}; }})'
        return 'NSDictionary<NSString *, id> *', f'TTObject({expression})'
    if kind in ('integer', 'number', 'boolean', 'string'):
        typ = 'NSString *' if kind == 'string' else 'NSNumber *'
        helper = {'integer': 'TTInteger', 'number': 'TTNumber', 'boolean': 'TTBoolean', 'string': 'TTString'}[kind]
        return typ, f'{helper}({expression})'
    if 'primitive_variants' in source:
        # Inline disjoint primitive union keeps its Foundation representation.
        return 'id', expression
    raise ValueError(f'Foundation conversion needs a typed shape: {source}')


def condition(tag):
    mode, field, value = tag
    if mode == 'literal':
        return f'[object[{literal(field)}] isEqual:{literal(value)}]'
    if mode == 'literals':
        return ' && '.join(f'[object[{literal(k)}] isEqual:{literal(v)}]' for k, v in value.items())
    if mode == 'member':
        return f'object[{literal(field)}] != nil'
    return ' && '.join([f'object[{literal(k)}] != nil' for k in value['required']] +
                       [f'object[{literal(k)}] == nil' for k in value['excluded']])


def outputs(definitions, version, input_definitions, abi):
    header = ['// Generated from the shared Rust result graph; do not edit.',
              '#import "TTResult.h"', 'NS_ASSUME_NONNULL_BEGIN',
              'FOUNDATION_EXPORT NSString * const TTRequestVersion;', 'FOUNDATION_EXPORT NSInteger TTNativeErrorCode(NSString *kind);']
    implementation = ['// Generated from the shared Rust result graph; do not edit.',
                      '#import "TTResults.g.h"', '#import "TTResultPrivate.h"',
                      'NSString * const TTRequestVersion = ' + literal(version) + ';']
    typed = {key: shape(source) for key, source in definitions.items()
             if any(k in shape(source) for k in ('properties', 'variants', 'primitive_variants'))}
    header += ['@class ' + ', '.join(name(key) for key in typed) + ';']
    parents = {child: key for key, source in typed.items() for child, _ in source.get('variants', [])}
    order = sorted(typed, key=lambda key: key in parents)
    for key in order:
        source, typ = typed[key], name(key)
        parent = name(parents[key]) if key in parents else 'TTResultNode'
        header += [f'@interface {typ} : {parent}']
        implementation += [f'@implementation {typ}']
        if 'variants' in source:
            implementation += ['+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);']
            for child, tag in source['variants']:
                implementation += [f'if ({condition(tag)}) return (id)[{name(child)} read:value];']
            implementation += ['TTInvalidResult(); return nil;', '}']
        elif 'properties' in source:
            implementation += ['+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }']
            for field, spec in source['properties'].items():
                target, decode = conversion(spec, definitions)
                accessor, quoted = member(field), literal(field)
                header += [f'@property(nonatomic, readonly) TTPresence<{target}> *{accessor};']
                required = 'YES' if field in source.get('required', []) else 'NO'
                implementation += [f'- (TTPresence<{target}> *){accessor} {{ return [self presence:{quoted} required:{required} convert:^id(id value) {{ return {decode}; }}]; }}']
            for field in named_object_fields(source, definitions):
                accessor = member(field) + 'Name'
                header += [f'@property(nonatomic, readonly) TTPresence<NSString *> *{accessor};']
                required = 'YES' if field in source.get('required', []) else 'NO'
                implementation += [f'- (TTPresence<NSString *> *){accessor} {{ return [self presence:{literal(field)} required:{required} convert:^id(id value) {{ return TTString(TTObject(value)[@"name"]); }}]; }}']
        else:
            header += ['@property(nonatomic, readonly) id value;', '@property(nonatomic, readonly) NSString *kind;']
            implementation += ['+ (instancetype)read:(id)value {']
            for kind in source['primitive_variants']:
                test = {'null': 'value == NSNull.null', 'boolean': 'TTIsBoolean(value)',
                        'number': '[value isKindOfClass:NSNumber.class] && !TTIsBoolean(value)',
                        'integer': '[value isKindOfClass:NSNumber.class] && !TTIsBoolean(value)',
                        'string': '[value isKindOfClass:NSString.class]', 'array': '[value isKindOfClass:NSArray.class]',
                        'object': '[value isKindOfClass:NSDictionary.class]'}[kind]
                _, decode = conversion(source.get('primitive_schemas', {}).get(kind, {'type': kind}), definitions)
                implementation += [f'if ({test}) return [[self alloc] initWithFields:@{{@"value":{decode}, @"kind":{literal(kind)}}}];']
            implementation += ['TTInvalidResult(); return nil;', '}', '- (id)value { return self.rawFields[@"value"]; }', '- (NSString *)kind { return self.rawFields[@"kind"]; }']
        header += ['+ (instancetype)read:(id)value;', '@end']
        implementation += ['@end']
    implementation += ['NSInteger TTNativeErrorCode(NSString *kind) {']
    for item in definitions['completefailureKind']['oneOf']:
        kind = item['const']
        symbol = 'THINKTHEN_E' + kind.upper()
        value = abi['constants'][symbol]
        implementation += [f'if ([kind isEqual:{literal(kind)}]) return {value};']
    implementation += ['TTInvalidResult(); return 0;', '}']
    header += ['@interface TTSessionPacket (Answers)']
    implementation += ['@implementation TTSessionPacket (Answers)']
    for function, _, payload in aggregate_answers(definitions):
        target, decode = conversion(payload, definitions)
        accessor = function + 'Answer'
        header += [f'@property(nonatomic, readonly) TTPresence<{target}> *{accessor};']
        implementation += [f'- (TTPresence<{target}> *){accessor} {{ if (![self.rawFields[@"kind"] isEqual:@"aggregate"] || ![self.rawFields[@"function"] isEqual:{literal(function)}]) TTInvalidResult(); return [self presence:@"value" required:YES convert:^id(id value) {{ return {decode}; }}]; }}']
    header += ['@end']
    implementation += ['@end']
    ordered_header, ordered_implementation = ordered_maps()
    header += ordered_header
    implementation += ordered_implementation
    input_header, input_implementation = inputs(input_definitions)
    header += input_header
    implementation += input_implementation
    header += named_calls(input_definitions)
    answer_header, answer_implementation = answer_helpers(definitions)
    header += answer_header
    implementation += answer_implementation
    header += ['typedef NS_ENUM(uint32_t, TTUsagePersistenceState) {']
    header += ['TTUsagePersistence' + word.removeprefix('THINKTHEN_COMPLETE_USAGE_PERSISTENCE_').removesuffix('_V1').title() + ' = ' + str(value) + ','
               for word, value in abi['constants'].items() if word.startswith('THINKTHEN_COMPLETE_USAGE_PERSISTENCE_') and word.endswith('_V1')]
    header += ['};']
    declarations, bodies = [], []
    for _, public in usage_operations(abi['functions']):
        method = member(public)
        finish = 'YES' if public.startswith('finish_') else 'NO'
        signature = f'- (nullable TTUsagePersistenceStatus *){method}:(NSError **)error'
        declarations += [signature + ';']
        bodies += [signature + f' {{ return [self usageStatus:{finish} error:error]; }}']
    header += ['#define TT_GENERATED_USAGE_DECLARATIONS \\'] + [line + (' \\' if i + 1 < len(declarations) else '') for i, line in enumerate(declarations)]
    header += ['#define TT_GENERATED_USAGE_IMPLEMENTATIONS \\'] + [line + (' \\' if i + 1 < len(bodies) else '') for i, line in enumerate(bodies)]
    header += ['NS_ASSUME_NONNULL_END']
    return {'TTResults.g.h': '\n'.join(header) + '\n', 'TTResults.g.m': '\n'.join(implementation) + '\n'}


def ordered_maps():
    """Preserve authored object pairs while Foundation encodes scalar values."""
    header = [
        '@interface TTOrderedEntry : NSObject',
        '@property(nonatomic, copy, readonly) NSString *key;',
        '@property(nonatomic, strong, readonly) id value;',
        '- (instancetype)initWithKey:(NSString *)key value:(id)value;', '@end',
        '@interface TTOrderedMap : NSObject',
        '@property(nonatomic, copy, readonly) NSArray<TTOrderedEntry *> *entries;',
        '- (instancetype)initWithEntries:(NSArray<TTOrderedEntry *> *)entries;', '@end',
        'FOUNDATION_EXPORT NSData * _Nullable TTRequestJSONData(id value, NSError **error);']
    implementation = [r'''@implementation TTOrderedEntry
- (instancetype)initWithKey:(NSString *)key value:(id)value {
    if ((self = [super init])) { _key = [key copy]; _value = value; } return self;
}
@end
@implementation TTOrderedMap
- (instancetype)initWithEntries:(NSArray<TTOrderedEntry *> *)entries {
    if ((self = [super init])) { _entries = [entries copy]; } return self;
}
@end
NSData *TTRequestJSONData(id value, NSError **error) {
    if ([value isKindOfClass:TTRequestObject.class]) value = ((TTRequestObject *)value).requestFields;
    NSArray<TTOrderedEntry *> *members = nil;
    if ([value isKindOfClass:TTOrderedMap.class]) members = ((TTOrderedMap *)value).entries;
    else if ([value isKindOfClass:NSDictionary.class]) {
        NSMutableArray *entries = [NSMutableArray array];
        for (NSString *key in value) [entries addObject:[[TTOrderedEntry alloc] initWithKey:key value:value[key]]];
        members = entries;
    }
    BOOL array = [value isKindOfClass:NSArray.class];
    if (!members && !array) return [NSJSONSerialization dataWithJSONObject:value options:NSJSONWritingFragmentsAllowed error:error];
    NSMutableData *data = [NSMutableData data];
    [data appendData:[(array ? @"[" : @"{") dataUsingEncoding:NSUTF8StringEncoding]];
    NSArray *values = array ? value : members;
    for (NSUInteger index = 0; index < values.count; index++) {
        if (index) [data appendData:[@"," dataUsingEncoding:NSUTF8StringEncoding]];
        id item = values[index];
        if (!array) {
            TTOrderedEntry *entry = item;
            NSData *key = TTRequestJSONData(entry.key, error); if (!key) return nil;
            [data appendData:key]; [data appendData:[@":" dataUsingEncoding:NSUTF8StringEncoding]];
            item = entry.value;
        }
        NSData *encoded = TTRequestJSONData(item, error); if (!encoded) return nil;
        [data appendData:encoded];
    }
    [data appendData:[(array ? @"]" : @"}") dataUsingEncoding:NSUTF8StringEncoding]];
    return data;
}
''']
    return header, implementation


def inputs(definitions):
    """Prepared question builders keep Foundation values and Rust admission."""
    objects = {key: value for key, value in definitions.items()
               if 'properties' in value}
    def typ(source):
        if not isinstance(source, dict): return 'id'
        if '$ref' in source:
            key = source['$ref'].split('/')[-1]
            if key in objects: return 'TTInput' + name(key)[2:] + ' *'
            return typ(definitions[key])
        if source.get('x-preserve-order'): return 'TTOrderedMap *'
        kind = source.get('type')
        if isinstance(kind, list): return 'id'
        if kind == 'string': return 'NSString *'
        if kind in ('boolean', 'integer', 'number'): return 'NSNumber *'
        if kind == 'array': return 'NSArray *'
        if kind == 'object': return 'NSDictionary *'
        return 'id'
    header = ['@interface TTRequestObject : NSObject',
              '@property(nonatomic, readonly) NSMutableDictionary<NSString *, id> *requestFields;', '@end']
    implementation = ['@implementation TTRequestObject',
                      '- (instancetype)init { if ((self = [super init])) _requestFields = [NSMutableDictionary dictionary]; return self; }', '@end']
    if objects: header += ['@class ' + ', '.join('TTInput' + name(key)[2:] for key in objects) + ';']
    for key, source in objects.items():
        cls = 'TTInput' + name(key)[2:]
        header += [f'@interface {cls} : TTRequestObject']
        implementation += [f'@implementation {cls}']
        constants = {}
        for prop, field in source['properties'].items():
            if isinstance(field, dict) and 'const' in field:
                constants[prop] = field['const']; continue
            accessor, quoted = member(prop), literal(prop)
            target = typ(field)
            header += [f'@property(nonatomic, strong, nullable) {target}{" " if target == "id" else ""}{accessor};']
            setter = accessor[:1].upper() + accessor[1:]
            implementation += [f'- ({target}){accessor} {{ return self.requestFields[{quoted}]; }}',
                               f'- (void)set{setter}:({target})value {{ if (value) self.requestFields[{quoted}] = value; else [self.requestFields removeObjectForKey:{quoted}]; }}']
        if constants:
            writes = ' '.join('self.requestFields[' + literal(prop) + '] = ' +
                              (literal(value) if isinstance(value, str) else '@(' + json.dumps(value) + ')') + ';'
                              for prop, value in constants.items())
            implementation += [f'- (instancetype)init {{ if ((self = [super init])) {{ {writes} }} return self; }}']
        header += ['@end']; implementation += ['@end']
    return header, implementation


def named_calls(definitions):
    """Expand inside the existing class and scheduler implementation."""
    declarations, implementations = [], []
    for item in definitions['RequestFunction']['oneOf']:
        function = item['const']
        signature = f'- (nullable TTTask *){function}:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error'
        declarations += [signature + ';']
        implementations += [signature + ' { return [self start:' + literal(function) + ' question:question input:input options:options feed:feed completion:completion error:error]; }']
    return ['#define TT_GENERATED_NAMED_DECLARATIONS \\'] + [line + (' \\' if i + 1 < len(declarations) else '') for i, line in enumerate(declarations)] + [
        '#define TT_GENERATED_NAMED_IMPLEMENTATIONS \\'] + [line + (' \\' if i + 1 < len(implementations) else '') for i, line in enumerate(implementations)]


def answer_helpers(definitions):
    from host_answers import scalar_answers, judgment_route
    route, guards = judgment_route(definitions)
    def at(path, root='fields'):
        return root + ''.join('[' + literal(field) + ']' for field in path)
    checks = ' && '.join('[' + at(path) + ' isEqual:' + literal(value) + ']' for path, value in guards)
    header = ['FOUNDATION_EXPORT NSErrorDomain const TTAnswerErrorDomain;',
              'FOUNDATION_EXPORT id _Nullable TTAnswerPayload(NSArray<TTSessionPacket *> *packets, TTSessionPacketTerminal *terminal, NSString *function, NSString *judgment, id call, NSError **error);',
              'FOUNDATION_EXPORT TTPresence *TTAnswerPresence(id payload, id (^convert)(id));']
    implementation = ['NSErrorDomain const TTAnswerErrorDomain = @"io.github.botassembly.thinkthen.answer";',
                      'extern NSErrorDomain const TTErrorDomain;', 'extern NSString * const TTFailureDetailsKey;',
                      'TTPresence *TTAnswerPresence(id payload, id (^convert)(id)) { TTResultNode *node = [[TTResultNode alloc] initWithFields:@{@"value":payload}]; return [node presence:@"value" required:YES convert:convert]; }',
                      'id TTAnswerPayload(NSArray<TTSessionPacket *> *packets, TTSessionPacketTerminal *terminal, NSString *function, NSString *judgment, id call, NSError **error) {',
                      'if (terminal.failure.state == TTPresenceValue) { TTCallError *failure = terminal.failure.value; if (error) *error = [NSError errorWithDomain:TTErrorDomain code:TTNativeErrorCode(failure.error.value.kind.value) userInfo:@{NSLocalizedDescriptionKey:failure.error.value.message.value, TTFailureDetailsKey:failure, @"ThinkThenCall":call}]; return nil; }',
                      'NSMutableArray *values = [NSMutableArray array];',
                      'if (terminal) for (TTSessionPacket *packet in packets) { NSDictionary *fields = packet.rawFields; if (![fields[@"function"] isEqual:function]) continue; id value;',
                      'if (!judgment.length) { if (![fields[@"kind"] isEqual:@"aggregate"]) continue; value = fields[@"value"]; } else {',
                      'if (!(' + checks + ')) continue; NSDictionary *carrier = ' + at(route) + '; if (![carrier[@"kind"] isEqual:judgment]) continue; value = carrier[@"value"];',
                      '} if (value) [values addObject:value]; }',
                      'if (values.count != 1) { if (error) *error = [NSError errorWithDomain:TTAnswerErrorDomain code:1 userInfo:@{NSLocalizedDescriptionKey:@"Expected exactly one successful answer", @"ThinkThenFunction":function, @"ThinkThenAnswerCount":@(values.count), @"ThinkThenCall":call}]; return nil; }',
                      'return values[0];', '}']
    answers = {function: (payload, '') for function, _, payload in aggregate_answers(definitions)}
    answers.update({function: (payload, kind) for function, kind, payload in scalar_answers(definitions)})
    declarations, bodies, client_declarations, client_bodies = [], [], [], []
    for function, (payload, kind) in answers.items():
        target, decode = conversion(payload, definitions)
        method = function + 'Answer'
        signature = f'- (nullable TTPresence<{target}> *){method}:(NSError **)error'
        declarations += [signature + ';']
        bodies += [signature + ' { id payload = TTAnswerPayload(self.packets, self.terminal, ' + literal(function) + ', ' + literal(kind) + ', self, error); if (!payload) return nil; return TTAnswerPresence(payload, ^id(id value) { return ' + decode + '; }); }']
        signature = f'- (nullable TTPresence<{target}> *){method}:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error'
        client_declarations += [signature + ';']
        client_bodies += [signature + ' { __block TTCall *result; __block NSError *failure = nil; dispatch_semaphore_t settled = dispatch_semaphore_create(0); TTTask *task = [self ' + function + ':question input:input options:options feed:nil completion:^(TTCall *call, NSError *problem) { result = call; failure = problem; dispatch_semaphore_signal(settled); } error:&failure]; if (!task) { if (error) *error = failure; return nil; } dispatch_semaphore_wait(settled, DISPATCH_TIME_FOREVER); if (failure) { if (error) *error = failure; return nil; } return [result ' + method + ':error]; }']
    for macro, lines in [('TT_GENERATED_ANSWER_DECLARATIONS', declarations), ('TT_GENERATED_ANSWER_IMPLEMENTATIONS', bodies), ('TT_GENERATED_BLOCKING_ANSWER_DECLARATIONS', client_declarations), ('TT_GENERATED_BLOCKING_ANSWER_IMPLEMENTATIONS', client_bodies)]:
        header += ['#define ' + macro + ' \\'] + [line + (' \\' if i + 1 < len(lines) else '') for i, line in enumerate(lines)]
    return header, implementation
