"""Foundation declarations and mechanical readers from the prepared Rust graph."""
import json
import re
from csharp import shape, enum_values
from host_answers import aggregate_answers, named_object_fields

ROOTS = ('completesessionPacket', 'completeplan')


def name(key):
    native = 'TT' + ''.join(part[0].upper() + part[1:] for part in re.findall(
        r'[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+', key.removeprefix('complete')))
    return {'TTAtomicNonZeroUsize':'TTRankedRecord', 'TTAtomicDecideValue':'TTDecisionRecord',
            'TTAtomicBoolean':'TTFilteredRecord', 'TTAtomicArrayOfString':'TTTagRecord',
            'TTAtomicNullableString':'TTChoiceRecord', 'TTAtomicDouble':'TTScoreRecord'}.get(native, native)


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


def outputs(definitions, version, input_definitions=None, abi=None):
    if input_definitions is None or abi is None:
        import importlib.util
        from generate import ROOT, graph, prepare
        request = json.loads((ROOT / 'specification/request.schema.json').read_text())
        request['$defs']['Request'] = {key:value for key,value in request.items() if key != '$defs'}
        input_definitions = prepare(graph(request, ('Request', 'RequestSessionDescriptor', 'RequestReaderFailure', 'EngineSettings')))
        spec = importlib.util.spec_from_file_location('objc_abi', ROOT / 'sdlc/scripts/check-c-exports.py')
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        abi = module.header_abi(ROOT / 'libraries/c/include/thinkthen.h')
    from generate import SCHEMA, graph, immediate_schema, prepare
    definitions = {**definitions, **prepare(graph(immediate_schema(json.loads(SCHEMA.read_text())), ('completeImmediateFacts',)))}
    header = ['// Generated from the shared Rust result graph; do not edit.',
              '#import "TTResult.h"', 'NS_ASSUME_NONNULL_BEGIN',
              '' , 'FOUNDATION_EXPORT NSInteger TTNativeErrorCode(NSString *kind);']
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
        header += ['@end']
        implementation += ['@end']
    implementation += ['NSInteger TTNativeErrorCode(NSString *kind) {']
    for item in definitions['completefailureKind']['oneOf']:
        kind = item['const']
        symbol = 'THINKTHEN_E' + kind.upper()
        value = abi['constants'][symbol]
        implementation += [f'if ([kind isEqual:{literal(kind)}]) return {value};']
    implementation += ['TTInvalidResult(); return 0;', '}']
    ordered_header, ordered_implementation = ordered_maps()
    header += ordered_header
    implementation += ordered_implementation
    input_header, input_implementation = inputs(input_definitions)
    header += input_header
    implementation += input_implementation
    calls = named_calls(input_definitions)
    answer_header, answer_implementation = answer_helpers(definitions)
    implementation += answer_implementation
    declarations = lambda macro, lines: next(line.split(' ', 2)[2] for line in lines if line.startswith('#define ' + macro + ' '))
    header += ['@class TTCall, TTSession, TTTask, TTUsagePersistenceStatus;',
               'typedef void (^TTCompletion)(TTCall * _Nullable call, NSError * _Nullable error);',
               'typedef TTInputRequestSessionDescriptor * _Nullable (^TTFeed)(TTRequestObject * _Nullable * _Nonnull readerFailure);',
               '@interface TTCall : NSObject', '- (instancetype)init NS_UNAVAILABLE;', '+ (instancetype)new NS_UNAVAILABLE;',
               '@property(nonatomic, strong, readonly) TTPresence<TTFacts *> *facts;',
               '@end', '@interface TTCall (Answers)', declarations('TT_GENERATED_ANSWER_DECLARATIONS', answer_header), '@end',
               '@interface TTFoundationClient : NSObject',
               '- (nullable instancetype)initWithSettings:(nullable TTInputEngineSettings *)settings error:(NSError **)error NS_DESIGNATED_INITIALIZER;',
               '- (instancetype)init NS_UNAVAILABLE;', '+ (instancetype)new NS_UNAVAILABLE;',
               '- (nullable TTPlan *)plan:(TTInputRequest *)request error:(NSError **)error;',
               '- (nullable TTSession *)startRequest:(TTInputRequest *)request error:(NSError **)error;',
               '- (nullable TTUsagePersistenceStatus *)usagePersistence:(NSError **)error;',
               '- (nullable TTUsagePersistenceStatus *)finishUsageStatus:(NSError **)error;',
               '@end', '@interface TTFoundationClient (NamedCalls)', declarations('TT_GENERATED_NAMED_DECLARATIONS', calls), '@end']
    header += [line for line in calls + answer_header if line.startswith('#define') and 'IMPLEMENTATIONS' in line.split()[1]]
    header += ['FOUNDATION_EXPORT NSErrorDomain const TTErrorDomain;', 'typedef NS_ERROR_ENUM(TTErrorDomain, TTErrorCode) {']
    for entry in definitions['completefailureKind']['oneOf']:
        kind = entry['const']
        header += ['TTError' + name(kind)[2:] + ' = ' + str(abi['constants']['THINKTHEN_E' + kind.upper()]) + ',']
    header += ['};']
    header += ['typedef NS_ENUM(uint32_t, TTUsagePersistenceState) {']
    header += ['TTUsagePersistence' + word.removeprefix('THINKTHEN_COMPLETE_USAGE_PERSISTENCE_').removesuffix('_V1').title() + ' = ' + str(value) + ','
               for word, value in abi['constants'].items() if word.startswith('THINKTHEN_COMPLETE_USAGE_PERSISTENCE_') and word.endswith('_V1')]
    header += ['};']
    private_macros = [line for line in header if line.startswith('#define') and 'IMPLEMENTATIONS' in line.split()[1]]
    header = [line for line in header if line not in private_macros]
    implementation[3:3] = ['#import \"ThinkThenFoundation.h\"'] + private_macros
    implementation += ['@implementation TTCall (Answers)', 'TT_GENERATED_ANSWER_IMPLEMENTATIONS', '@end', '@implementation TTFoundationClient (NamedCalls)', 'TT_GENERATED_NAMED_IMPLEMENTATIONS', '@end']
    header += ['NS_ASSUME_NONNULL_END']
    return rename_inputs({'TTResults.g.h': '\n'.join(header) + '\n', 'TTResults.g.m': '\n'.join(implementation) + '\n#if __has_include(<thinkthen.h>)\n' + native_values() + '#endif\n'})


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
        '']
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
               if 'properties' in value and not key.startswith('RequestDefinition')}
    enums = {key: enum_values(value) for key, value in definitions.items() if enum_values(value)}
    def typ(source):
        if not isinstance(source, dict): return 'id'
        if '$ref' in source:
            key = source['$ref'].split('/')[-1]
            if key == 'RequestQuestion': return 'TTQuestion *'
            if key == 'RequestInput': return 'TTInput *'
            if key in objects: return 'TTInput' + name(key)[2:] + ' *'
            if key in enums: return 'TTInput' + name(key)[2:]
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
              '@end', '@interface TTQuestion : TTRequestObject', '@end', '@interface TTInput : TTRequestObject', '@end']
    implementation = ['@implementation TTQuestion', '@end', '@implementation TTInput', '@end', '@interface TTRequestObject ()', '@property(nonatomic, strong) NSMutableDictionary<NSString *, id> *requestFields;', '@end', '@implementation TTRequestObject',
                      '- (instancetype)init { if ((self = [super init])) _requestFields = [NSMutableDictionary dictionary]; return self; }', '- (instancetype)initWithFields:(NSDictionary *)fields { if ((self = [self init])) [_requestFields addEntriesFromDictionary:fields]; return self; }', '@end']
    for key, values in enums.items():
        if key == 'RequestVersion': continue
        cls = 'TTInput' + name(key)[2:]
        header += [f'typedef NSString *{cls} NS_TYPED_ENUM;']
        for value in values:
            constant = cls + name(value)[2:]
            header += [f'FOUNDATION_EXPORT {cls} const {constant};']
            implementation += [f'{cls} const {constant} = {literal(value)};']
    if objects: header += ['@class ' + ', '.join('TTInput' + name(key)[2:] for key in objects) + ';']
    for key, source in objects.items():
        cls = 'TTInput' + name(key)[2:]
        parent = 'TTQuestion' if key.startswith('RequestQuestion') else 'TTInput' if key.startswith('RequestInput') else 'TTRequestObject'
        header += [f'@interface {cls} : {parent}']
        implementation += [f'@implementation {cls}']
        constants = {'schema': enum_values(definitions['RequestVersion'])[0]} if key == 'Request' else {}
        for prop, field in source['properties'].items():
            if key == 'Request' and prop == 'schema': continue
            if isinstance(field, dict) and 'const' in field:
                constants[prop] = field['const']; continue
            accessor, quoted = member(prop), literal(prop)
            target = typ(field)
            header += [f'@property(nonatomic, strong, nullable) {target} {accessor};']
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
        signature = f'- (nullable TTTask *){function}:(TTQuestion *)question input:(TTInput *)input options:(nullable TTInputRequestOptions *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error'
        declarations += [signature + ';']
        implementations += [signature + ' { return [self start:' + literal(function) + ' question:question input:input options:options feed:feed completion:completion error:error]; }']
    return ['#define TT_GENERATED_NAMED_DECLARATIONS ' + ' '.join(declarations), '#define TT_GENERATED_NAMED_IMPLEMENTATIONS ' + ' '.join(implementations)]


def answer_helpers(definitions):
    from host_answers import scalar_answers, judgment_route
    route, guards = judgment_route(definitions)
    def at(path, root='fields'):
        return root + ''.join('[' + literal(field) + ']' for field in path)
    checks = ' && '.join('[' + at(path) + ' isEqual:' + literal(value) + ']' for path, value in guards)
    header = ['FOUNDATION_EXPORT NSErrorDomain const TTAnswerErrorDomain;',
              '' ,
              '']
    implementation = ['NSErrorDomain const TTAnswerErrorDomain = @"io.github.botassembly.thinkthen.answer";',
                      'extern NSErrorDomain const TTErrorDomain;', 'extern NSString * const TTFailureDetailsKey;',
                      'TTPresence *TTAnswerPresence(id payload, id (^convert)(id)) { TTResultNode *node = [[TTResultNode alloc] initWithFields:@{@"value":payload}]; return [node presence:@"value" required:YES convert:convert]; }',
                      'id TTAnswerPayload(NSArray<TTSessionPacket *> *packets, TTSessionPacketTerminal *terminal, NSString *function, NSString *judgment, id call, NSError **error) {',
                      'if (terminal.failure.state == TTPresenceValue) { if (error) *error = TTFailureForCall(terminal.failure.value, call); return nil; }',
                      'NSMutableArray *values = [NSMutableArray array];',
                      'if (terminal) for (TTSessionPacket *packet in packets) { NSDictionary *fields = packet.rawFields; if (![fields[@"function"] isEqual:function]) continue; id value;',
                      'if ([function isEqual:@"filter"]) { if ([fields[@"kind"] isEqual:@"row"]) [values addObject:fields[@"value"]]; continue; }',
                      'if (!judgment.length) { if (![fields[@"kind"] isEqual:@"aggregate"]) continue; value = fields[@"value"]; } else {',
                      'if (!(' + checks + ')) continue; NSDictionary *carrier = ' + at(route) + '; if (![carrier[@"kind"] isEqual:judgment]) continue; value = carrier[@"value"];',
                      '} if (value) [values addObject:value]; }',
                      'if ([function isEqual:@"filter"] && [((TTCall *)call).function isEqual:function]) return [values copy];',
                      'if (values.count != 1) { if (error) *error = [NSError errorWithDomain:TTAnswerErrorDomain code:1 userInfo:@{NSLocalizedDescriptionKey:@"Expected exactly one successful answer", @"ThinkThenFunction":function, @"ThinkThenAnswerCount":@(values.count), @"ThinkThenCall":call}]; return nil; }',
                      'return values[0];', '}']
    answers = {function: (payload, '') for function, _, payload in aggregate_answers(definitions)}
    answers.update({function: (payload, kind) for function, kind, payload in scalar_answers(definitions)})
    declarations, bodies = [], []
    for function, (payload, kind) in answers.items():
        target, decode = conversion(payload, definitions)
        if function == 'rank': target, decode = 'NSArray<NSNumber *> *', 'TTArray(value, ^id(id item) { return TTInteger(TTObject(item)[@"index"]); })'
        if function == 'filter': target, decode = 'NSArray *', 'TTArray(value, ^id(id item) { id original = TTObject(item)[@"input"]; if (!original) TTInvalidResult(); return original; })'
        method = function + 'Answer'
        collection = function in ('rank', 'filter')
        result_type = target if collection else f'TTPresence<{target}> *'
        signature = f'- (nullable {result_type}){method}:(NSError **)error'
        declarations += [signature + ';']
        returned = decode.replace('TTArray(value,', 'TTArray(payload,') if collection else 'TTAnswerPresence(payload, ^id(id value) { return ' + decode + '; })'
        bodies += [signature + ' { id payload = TTAnswerPayload(self.packets, self.terminal, ' + literal(function) + ', ' + literal(kind) + ', self, error); if (!payload) return nil; return ' + returned + '; }']
    for macro, lines in [('TT_GENERATED_ANSWER_DECLARATIONS', declarations), ('TT_GENERATED_ANSWER_IMPLEMENTATIONS', bodies)]:
        header += ['#define ' + macro + ' ' + ' '.join(lines)]
    return header, implementation

def native_values():
    """Copy native typed graphs; Foundation never reparses result JSON."""
    import c
    from generate import SCHEMA, graph, immediate_schema, prepare, references
    definitions = prepare(graph(immediate_schema(json.loads(SCHEMA.read_text())),
                                ('completesessionPacket', 'completeImmediateFacts')))
    available, definitions, pending = definitions, {}, ['completesessionPacket', 'completeImmediateFacts']
    while pending:
        key = pending.pop()
        if key in definitions: continue
        schema = definitions[key] = available[key]
        pending.extend(references(schema))
        pending.extend(child for child, _ in schema.get('variants', []))
    class Layout(c.Target):
        def __init__(self):
            super().__init__(definitions)
            self.members, self.arms = {}, {}
        def structure(self, key, members):
            self.members[key] = members
            super().structure(key, members)
        def union(self, key, arms):
            self.arms[c.ident(key)] = arms
            super().union(key, arms)
    target = Layout()
    for key, schema in definitions.items(): target.definition(key, schema)
    def decode(typ, expression, boolean=False):
        if boolean: return '@((BOOL)' + expression + ')'
        if typ in ('u32', 'u64', 'usize', 'f64', 'i64'): return '@(' + expression + ')'
        if typ == 'thinkthen_complete_utf8_v1': return 'TTNativeString(' + expression + ')'
        pointer = typ.startswith('*const ')
        typ = typ.removeprefix('*const ')
        return 'TTNative_' + typ + '(' + (expression if pointer else '&(' + expression + ')') + ')'
    lines = ['// Generated from the shared Rust result graph; do not edit.',
             '#import "TTResultPrivate.h"', '#import <thinkthen.h>', '#include <math.h>']
    lines += ['id TTNative_' + key + '(const struct ' + key + ' *value);' for key in target.members]
    lines += [r'''static NSString *TTNativeString(struct thinkthen_complete_utf8_v1 value) {
    NSString *text = [[NSString alloc] initWithBytes:value.data ?: "" length:value.len encoding:NSUTF8StringEncoding];
    if (!text) TTInvalidResult(); return text;
}
id TTNative_thinkthen_complete_json_v1(const struct thinkthen_complete_json_v1 *value) {
    if (!value) TTInvalidResult();
    switch (value->kind) {
    case THINKTHEN_COMPLETE_JSON_NULL_V1: return NSNull.null;
    case THINKTHEN_COMPLETE_JSON_BOOLEAN_V1: return @((BOOL)value->data.boolean);
    case THINKTHEN_COMPLETE_JSON_STRING_V1: return TTNativeString(value->data.string);
    case THINKTHEN_COMPLETE_JSON_NUMBER_V1: {
        NSString *text = TTNativeString(value->data.number);
        NSDecimalNumber *number = [NSDecimalNumber decimalNumberWithString:text locale:@{NSLocaleDecimalSeparator:@"."}];
        double floating = text.doubleValue;
        if (![number isEqual:NSDecimalNumber.notANumber] &&
            !([number isEqual:NSDecimalNumber.zero] && floating != 0)) return number;
        if (!isfinite(floating)) TTInvalidResult(); return @(floating);
    }
    case THINKTHEN_COMPLETE_JSON_ARRAY_V1: {
        NSMutableArray *items = [NSMutableArray array];
        for (size_t i = 0; i < value->data.array.len; i++) [items addObject:TTNative_thinkthen_complete_json_v1(value->data.array.data[i])];
        return [items copy];
    }
    case THINKTHEN_COMPLETE_JSON_OBJECT_V1: {
        NSMutableDictionary *items = [NSMutableDictionary dictionary];
        for (size_t i = 0; i < value->data.object.len; i++) {
            const struct thinkthen_complete_json_entry_v1 *item = &value->data.object.data[i];
            items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
        }
        return [items copy];
    }
    default: TTInvalidResult();
    }
}''']
    for key, members in target.members.items():
        schema = target.schemas.get(key, {})
        body = ['if (!value) TTInvalidResult();']
        if key in target.arms:
            body += ['switch(value->kind) {']
            for arm, _, arm_type, _ in target.arms[key]:
                symbol = key.removesuffix('_v1').upper() + '_' + arm.upper() + '_V1'
                field = arm + '_' if arm in ('true', 'false') else arm
                expression = 'NSNull.null' if arm == 'null' else decode(arm_type, 'value->data.' + field, arm == 'boolean')
                body += [f'case {symbol}: return {expression};']
            body += ['default: TTInvalidResult(); }']
        elif 'enum' in schema:
            body += ['switch(value->kind) {']
            for value in schema['enum']:
                symbol = key.removesuffix('_v1').upper() + '_' + re.sub(r'[^A-Za-z0-9]+', '_', value).strip('_').upper() + '_V1'
                body += [f'case {symbol}: return {literal(value)};']
            body += ['default: TTInvalidResult(); }']
        elif 'properties' in schema:
            body += ['NSMutableDictionary *items = [NSMutableDictionary dictionary];']
            for field, field_type in members:
                if field == 'extensions': continue
                expression = 'value->' + (field + '_' if field in ('true', 'false') else field)
                boolean = c.normalized(schema['properties'][field]).get('type') == 'boolean'
                if field_type.endswith('_presence_v1'):
                    contained = dict(target.members[field_type])['value']
                    decoded = decode(contained, expression + '.value', boolean)
                    body += [f'if ({expression}.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[{literal(field)}] = NSNull.null;',
                             f'else if ({expression}.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[{literal(field)}] = {decoded};']
                else: body += [f'items[{literal(field)}] = {decode(field_type, expression, boolean)};']
            body += ['for (size_t i = 0; i < value->extensions.len; i++) {',
                     'const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];',
                     'items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);', '}', 'return [items copy];']
        elif members[0][0] == 'presence':
            body += ['return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? ' + decode(dict(members)['value'], 'value->value') + ' : NSNull.null;']
        elif members[0][0] == 'data':
            mapping = key.endswith('_map_v1')
            body += ['NSMutableDictionary *items = [NSMutableDictionary dictionary];' if mapping else 'NSMutableArray *items = [NSMutableArray array];', 'for (size_t i = 0; i < value->len; i++) {']
            element = dict(members)['data'].removeprefix('*const ')
            if mapping:
                body += ['items[TTNativeString(value->data[i].name)] = ' + decode(dict(target.members[element])['value'], 'value->data[i].value') + ';']
            else: body += ['[items addObject:' + decode(element, 'value->data[i]') + '];']
            body += ['}', 'return [items copy];']
        else:
            body += ['return ' + decode(dict(members)['value'], 'value->value', schema.get('type') == 'boolean') + ';']
        lines += ['id TTNative_' + key + '(const struct ' + key + ' *value) {'] + body + ['}']
    return '\n'.join(lines) + '\n'


def rename_inputs(outputs):
    replacements = {'TTInputImageMedia':'TTImageMedia', 'TTInputReaderMedia':'TTReaderMedia',
                    'TTInputRecognitionMode':'TTRecognitionMode', 'TTInputSourceUnit':'TTSourceUnit',
                    'TTInputRequestFraming':'TTFraming', 'TTInputRequestFunction':'TTFunction',
                    'TTInputRequestSessionDescriptor':'TTDescriptor', 'TTInputRequestReader':'TTReading',
                    'TTInputRequestReaderFailureInvalidInput':'TTInvalidInputFailure',
                    'TTInputRequestReaderFailureIo':'TTReaderIOFailure',
                    'TTInputRequestReaderFailureProducer':'TTProducerFailure',
                    'TTInputRequestReaderFailureUtf8':'TTReaderUTF8Failure',
                    'TTInputRequestOriginalJson':'TTOriginalJSON', 'TTInputRequestOriginalText':'TTOriginalText',
                    'TTInputRequestImageBytes':'TTByteImage', 'TTInputRequestImageFile':'TTFileImage',
                    'TTInputEngineSettings':'TTSettings', 'TTInputRequestOptions':'TTControls',
                    'TTInputRequestQuestionText':'TTTextQuestion', 'TTInputRequestQuestionFile':'TTQuestionFile',
                    'TTInputRequestQuestionName':'TTQuestionName', 'TTInputRequestQuestionReference':'TTQuestionReference',
                    'TTInputRequestQuestionDefinition':'TTQuestionDefinition',
                    'TTInputRequestInputText':'TTTextInput', 'TTInputRequestInputJson':'TTJSONInput',
                    'TTInputRequestInputRecords':'TTRecordsInput', 'TTInputRequestInputFeed':'TTFeedInput',
                    'TTInputRequestInputSource':'TTSourceInput', 'TTInputRequestInputEntities':'TTEntitiesInput',
                    'TTInputRequestInputUnits':'TTUnitsInput', 'TTInputRequestItem':'TTItem',
                    'TTInputRequestSource':'TTFiles', 'TTInputRequestCall':'TTCallArguments', 'TTInputRequest':'TTRequest'}
    def rename(text):
        for old in sorted(replacements, key=len, reverse=True): text = text.replace(old, replacements[old])
        return re.sub(r'\bTTInput([A-Z]\w*)', r'TT\1Builder', text)
    return {key:rename(value).replace('NSError **)', 'NSError * _Nullable * _Nullable)') for key,value in outputs.items()}
