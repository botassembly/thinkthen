"""Mechanical JSON shape tests for untagged declaration alternatives."""
import json


def ungroup(expression):
    """Remove one outer group where the target syntax already supplies one."""
    return expression[1:-1] if expression.startswith('(') and expression.endswith(')') else expression


def predicate(schema, definitions, expression, target, depth=0, array_item=None):
    if schema is True or schema == {}:
        return 'true'
    if '$ref' in schema:
        return predicate(definitions[schema['$ref'].split('/')[-1]], definitions,
                         expression, target, depth, array_item)
    alternatives = (list(schema.get('primitive_schemas', {}).values()) or
                    schema.get('anyOf', schema.get('oneOf', [])))
    kinds = schema.get('type')
    if isinstance(kinds, list):
        alternatives = [{**schema, 'type': kind} for kind in kinds]
    if alternatives:
        terms = list(dict.fromkeys(predicate(item, definitions, expression, target, depth, array_item)
                                   for item in alternatives))
        if 'true' in terms:
            return 'true'
        return terms[0] if len(terms) == 1 else '(' + ' || '.join(terms) + ')'
    if 'variants' in schema or 'properties' in schema:
        kinds = 'object'
    if kinds is None and isinstance(schema.get('const'), str):
        kinds = 'string'
    if target == 'csharp':
        tokens = {'number': ['Number'], 'integer': ['Number'], 'string': ['String'],
                  'boolean': ['True', 'False'], 'array': ['Array'],
                  'object': ['Object'], 'null': ['Null']}[kinds]
        check = '(' + ' || '.join(f'{expression}.ValueKind == JsonValueKind.{token}'
                                  for token in tokens) + ')'
    elif target == 'rust':
        check = f'{expression}.kind() == {json.dumps("number" if kinds == "integer" else kinds)}'
    elif target == 'java':
        token = {'number': 'BigDecimal', 'integer': 'BigDecimal', 'string': 'String',
                 'boolean': 'Boolean', 'array': 'List<?>', 'object': 'Map<?,?>'}
        check = f'{expression} == null' if kinds == 'null' else f'{expression} instanceof {token[kinds]}'
    elif target == 'objc':
        tests = {'null': f'{expression} == NSNull.null',
                 'boolean': f'TTIsBoolean({expression})',
                 'number': f'[{expression} isKindOfClass:NSNumber.class] && !TTIsBoolean({expression})',
                 'integer': f'[{expression} isKindOfClass:NSNumber.class] && !TTIsBoolean({expression})',
                 'string': f'[{expression} isKindOfClass:NSString.class]',
                 'array': f'[{expression} isKindOfClass:NSArray.class]',
                 'object': f'[{expression} isKindOfClass:NSDictionary.class]'}
        check = '(' + tests[kinds] + ')'
    else:
        raise ValueError('unknown declaration predicate target')
    if kinds == 'array':
        item = f'union_item_{depth}'
        element = schema.get('items', {})
        inner = predicate(element, definitions, item, target, depth + 1, array_item)
        if inner == 'true':
            return check
        if target == 'csharp':
            return f'({check} && {expression}.EnumerateArray().All({item} => {inner}))'
        if target == 'rust':
            if array_item:
                return f'{expression}.array().is_ok_and(|items| items.iter().all({array_item(element)}))'
            return f'({check} && {expression}.array().is_ok_and(|items| items.iter().all(|{item}| {ungroup(inner)})))'
        if target == 'java':
            return f'({check} && ((List<?>){expression}).stream().allMatch({item} -> {inner}))'
        return f'({check} && (^BOOL {{ for (id {item} in (NSArray *){expression}) {{ if (!({inner})) return NO; }} return YES; }})())'
    return check
