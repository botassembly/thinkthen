"""Foundation declarations and mechanical readers from the prepared Rust graph."""
import json
import re
from csharp import shape, enum_values

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


def outputs(definitions, version):
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
    from pathlib import Path
    native_header = (Path(__file__).resolve().parents[4] / 'libraries/c/include/thinkthen.h').read_text()
    for item in definitions['completefailureKind']['oneOf']:
        kind = item['const']
        symbol = 'THINKTHEN_E' + kind.upper()
        value = re.search(r'^#define ' + symbol + r' (\d+)$', native_header, re.M)
        if not value:
            raise ValueError('missing native error code: ' + symbol)
        implementation += [f'if ([kind isEqual:{literal(kind)}]) return {value.group(1)};']
    implementation += ['TTInvalidResult(); return 0;', '}']
    header += ['NS_ASSUME_NONNULL_END']
    return {'TTResults.g.h': '\n'.join(header) + '\n', 'TTResults.g.m': '\n'.join(implementation) + '\n'}
