"""Lossless typed aggregate payloads derived from the shared packet graph."""


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
