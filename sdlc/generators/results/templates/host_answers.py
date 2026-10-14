"""Lossless typed aggregate payloads derived from the shared packet graph."""
import re


def call_results(definitions):
    """Describe each public call using its existing native value carriers.

    Scalar values come from normalized judgments; collections come from the
    aggregate packet. A caller's input shape chooses between them, even for a
    one-element list. call_shape distinguishes collection, whole aggregate and
    input cardinality independently of the shape inside an answer.
    Probability describes the yes odds or the picked odds,
    never confidence. Item probabilities stay with rank/recognize/relate items.
    Null values remain present, including unresolved scalar judgments.
    """
    scalars = {function: (kind, payload)
               for function, kind, payload in scalar_answers(definitions)}
    result = {}
    for function, carrier, payload in aggregate_answers(definitions):
        _, projection = aggregate_projection(function, payload, definitions)
        projection = projection or 'value'
        receipt_schema = payload.get('items', payload)
        receipt = definitions[receipt_schema['$ref'].split('/')[-1]]
        value = receipt['properties'][projection]
        if payload.get('type') == 'array':
            value = {'type': 'array', 'items': value}
        scalar = scalars.get(function)
        mode = ('yes' if function in ('decide', 'filter') else
                'picked' if function in ('choose', 'find') else
                'items' if function in ('rank', 'recognize', 'relate') else 'null')
        result[function] = {
            'carrier': carrier,
            'receipt_payload': payload,
            'aggregate_value': value,
            'value_projection': projection,
            'call_shape': ('collection' if projection != 'value' else
                           'aggregate' if payload.get('type') != 'array' else 'input'),
            'scalar_kind': scalar[0] if scalar else None,
            'scalar_value': scalar[1] if scalar else None,
            'probability_mode': mode,
        }
        if mode in ('yes', 'picked'):
            answer = receipt['properties']['answer']
            answer = definitions[answer['$ref'].split('/')[-1]]
            if 'variants' in answer:
                kind = 'yes_no' if mode == 'yes' else 'choice'
                answer = next(definitions[child] for child, _ in answer['variants']
                              if definitions[child]['properties']['kind']['const'] == kind)
            member = 'probability' if mode == 'yes' else 'probabilities'
            result[function]['probability_schema'] = answer['properties'][member]
            result[function]['probability_path'] = ('answer', member)
            if mode == 'picked':
                if 'pick' not in answer['required']:
                    raise ValueError(function + ' answer requires its canonical pick')
                result[function]['pick_path'] = ('answer', 'pick')
    return result


def aggregate_answers(definitions):
    packet = definitions['completesessionPacket']
    answers = []
    for child, _ in packet['variants']:
        fields = definitions[child].get('properties', {})
        if fields.get('kind', {}).get('const') != 'aggregate':
            continue
        function = fields.get('function', {}).get('const')
        if isinstance(function, str):
            answers.append((function, child, fields['value']))
    return answers


def aggregate_projection(function, payload, definitions):
    """Project convenient collection answers without changing receipt payloads.

    Rank selects the original zero-based input index, not the ranked position.
    Filter selects the original input, including an explicit JSON null.
    The returned member is read from each receipt after rows are collected.
    """
    member = {'rank': 'index', 'filter': 'input'}.get(function)
    if member is None:
        return payload, None
    if payload.get('type') != 'array':
        raise ValueError(function + ' answer requires a receipt collection')
    item = payload['items']
    if '$ref' in item:
        item = definitions[item['$ref'].split('/')[-1]]
    fields = item.get('properties', {})
    if member not in fields:
        raise ValueError(function + ' receipt has no ' + member)
    return {'type': 'array', 'items': fields[member]}, member


def named_object_fields(source, definitions):
    """Find fields whose every object alternative carries a required text name."""
    def named(value):
        if '$ref' in value: value = definitions[value['$ref'].split('/')[-1]]
        if 'variants' in value:
            return all(named(definitions[child]) for child, _ in value['variants'])
        field = value.get('properties', {}).get('name', {})
        return 'name' in value.get('required', []) and field.get('type') in ('string', ['string'])
    return [key for key, value in source.get('properties', {}).items()
            if isinstance(value, dict) and named(value)]


def scalar_input_selectors(definitions):
    """Return the canonical single-input alternatives, independent of names in feeds."""
    selectors = {'text': 'text', 'json': 'value'}
    found = {}
    for child, _ in definitions['RequestInput'].get('variants', []):
        source = definitions[child]
        fields = source.get('properties', {})
        kind = fields.get('kind', {}).get('const')
        if kind not in selectors:
            continue
        member = selectors[kind]
        if member not in source.get('required', []) or member not in fields:
            raise ValueError('single input selector needs its canonical payload')
        if kind == 'text' and fields[member].get('type') != 'string':
            raise ValueError('literal text selector needs a text payload')
        if kind == 'json' and any(key in fields[member] for key in ('type', '$ref', 'oneOf', 'anyOf')):
            raise ValueError('authored JSON selector needs an unrestricted payload')
        if kind in found:
            raise ValueError('canonical single input selector is duplicated')
        found[kind] = child
    if found.keys() != selectors.keys():
        raise ValueError('canonical single input selectors are missing')
    return list(found.values())


def usage_operations(functions):
    return [(symbol, symbol.removeprefix('thinkthen_engine_').removesuffix('_v1'))
            for symbol, prototype in functions.items()
            if symbol.startswith('thinkthen_engine_') and any(
                'thinkthen_complete_usage_persistence_v1' in argument
                for argument in prototype['arguments'])]


def scalar_answers(definitions):
    result = []
    for child, _ in definitions['completesessionJudgment']['variants']:
        source = definitions[child]
        function = source.get('x-request-function')
        if not isinstance(function, str):
            raise ValueError('normalized judgment needs a canonical request function')
        fields = source['properties']
        result.append((function, fields['kind']['const'], fields['value']))
    return result


def judgment_route(definitions):
    """Find the nested judgment carrier and discriminants inside observations."""
    def visit(key, path, guards, seen):
        if key == 'completesessionJudgment': return path, guards
        if key in seen: return None
        source = definitions[key]
        seen = seen | {key}
        for child, _ in source.get('variants', []):
            result = visit(child, path, guards, seen)
            if result: return result
        fields = source.get('properties', {})
        guards = guards + [(path + [field], value['const']) for field, value in fields.items()
                           if isinstance(value, dict) and isinstance(value.get('const'), str)]
        for field, value in fields.items():
            if isinstance(value, dict) and '$ref' in value:
                result = visit(value['$ref'].split('/')[-1], path + [field], guards, seen)
                if result: return result
        return None
    for child, _ in definitions['completesessionPacket']['variants']:
        if definitions[child].get('properties', {}).get('kind', {}).get('const') == 'observation':
            route = visit(child, [], [], set())
            if route: return route
    raise ValueError('packet graph has no normalized judgment route')


def alternative_name(source):
    """Name a union arm from its identity, never its position in the schema."""
    if not isinstance(source, dict) or not source:
        return 'JsonValue'
    identity = source.get('title') or source.get('$ref', '').removeprefix('#/$defs/')
    if not identity and 'const' in source:
        value = source['const']
        identity = ('True' if value else 'False') if isinstance(value, bool) else str(value)
    if not identity:
        literals = [field['const'] for field in source.get('properties', {}).values()
                    if isinstance(field, dict) and isinstance(field.get('const'), str)]
        identity = '_'.join(literals) if literals else source.get('type')
    if not isinstance(identity, str):
        raise ValueError('union alternative needs a semantic schema title')
    names = {'string': 'String', 'integer': 'Integer', 'number': 'Number',
             'boolean': 'Boolean', 'array': 'List', 'object': 'Object', 'null': 'Null'}
    identity = names.get(identity, identity)
    token = re.sub(r'[^A-Za-z0-9]+', '_', identity).strip('_')
    if not token:
        raise ValueError('union alternative needs a nonempty semantic schema title')
    return 'Value_' + token if token[:1].isdigit() else token
