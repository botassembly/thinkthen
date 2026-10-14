"""RBS types matching Ruby's native result conversion and request grammar."""
import json
from pathlib import Path
import re

HEADER = '# Generated from the shared Rust result and request graphs; do not edit.'


def union(types):
    types = list(dict.fromkeys(types))
    return 'untyped' if 'untyped' in types else ' | '.join(types)


def alias(key):
    return 'request_' + re.sub(r'(?<!^)(?=[A-Z][a-z])', '_', key).lower()


def schema_type(source, reference, objects=False):
    if source is True or source == {}:
        return 'untyped'
    if source is False:
        raise ValueError('uninhabited Ruby schema')
    if '$ref' in source:
        return reference(source['$ref'].removeprefix('#/$defs/'))
    alternatives = source.get('anyOf', source.get('oneOf'))
    if alternatives:
        return union(schema_type(child, reference, objects) for child in alternatives)
    kinds = source.get('type')
    if kinds is None:
        return 'untyped'
    if isinstance(kinds, list):
        return union(schema_type({**source, 'type': kind}, reference, objects) for kind in kinds)
    if kinds == 'array':
        return 'Array[' + schema_type(source.get('items', {}), reference, objects) + ']'
    if kinds == 'object':
        if objects and source.get('properties'):
            required = source.get('required', [])
            fields = [f'{"" if key in required else "?"}{key}: ' + schema_type(field, reference, objects)
                      for key, field in source['properties'].items()]
            return '{ ' + ', '.join(fields) + ' }'
        keys = 'Symbol | String' if objects else 'String'
        return 'Hash[' + keys + ', ' + schema_type(source.get('additionalProperties', {}), reference, objects) + ']'
    return {'string': 'String', 'boolean': 'bool', 'integer': 'Integer',
            'number': 'Integer | Float', 'null': 'nil'}[kinds]


def render(definitions, name):
    request = json.loads((Path(__file__).resolve().parents[4] / 'specification/request.schema.json').read_text())['$defs']
    selected = {}

    def request_type(key):
        if key not in selected:
            selected[key] = None
            selected[key] = schema_type(request[key], request_type, objects=True)
        return alias(key)

    def result_type(key):
        source = definitions[key]
        if 'properties' in source:
            return name(key)
        if 'variants' in source:
            return union(result_type(child) for child, _ in source['variants'])
        if 'primitive_schemas' in source:
            return union(schema_type(child, result_type) for child in source['primitive_schemas'].values())
        if key == 'completeDecideValue':
            # core/result/complete/wire.rs: Boolean(Option<bool>) or Meaning. The
            # request grammar owns Meaning's admitted shapes, including null.
            return union(['bool', request_type('Authored_criterion')])
        return schema_type(source, result_type)

    def member_type(member, source):
        # Rust's QuestionText/Meaning/Description serialize through Json and
        # erase their admitted shapes in the result schema. Reuse the authored
        # grammar instead of keeping a second union of those shapes here.
        authored = {'text': 'Authored_questionText', 'instructions': 'Authored_questionText',
                    'entity_definition': 'Authored_questionText', 'true': 'Authored_criterion',
                    'false': 'Authored_criterion', 'description': 'Authored_description'}
        if source == {} and member in authored:
            return request_type(authored[member])
        return schema_type(source, result_type)

    lines = [HEADER, 'module ThinkThen', '  module Results']
    for key, source in definitions.items():
        if 'properties' not in source:
            continue
        lines.append(f'    class {name(key)} < Native::Result')
        if key == 'completeplan':
            lines.append('      FIELDS: Array[String]')
        lines.extend(f'      def {member}: () -> ({member_type(member, field)})'
                     for member, field in source['properties'].items())
        lines.append('    end')
    lines += ['  end', '  class Client']

    def keywords(key, omit=()):
        return ', '.join(f'?{member}: {schema_type(field, request_type, objects=True)}'
                         for member, field in request[key]['properties'].items() if member not in omit)

    recognition = next(source for source in request['RequestDefinition']['anyOf'] if source.get('title') == 'RecognizeDefinition')
    recognition_fields = recognition['properties']['recognize']['properties']
    recognition_keywords = ', '.join(f'?{member}: {schema_type(field, request_type, objects=True)}' for member, field in recognition_fields.items() if member != 'kinds')
    options = keywords('RequestOptions')
    settings = keywords('EngineSettings')
    reading = keywords('RequestReader')
    fields = keywords('RequestItem', ('original',))
    question = 'String | Hash[Symbol | String, untyped] | Client::Question question'
    args = f'{question}, untyped input'
    # Ruby consumes cancel separately; the native RequestOptions grammar sees
    # every other keyword unchanged and rejects invalid controls.
    for verb in request['RequestFunction']['oneOf']:
        value = {'decide': result_type('completeDecideValue'), 'choose': 'String | nil', 'tag': 'Array[String] | nil', 'score': 'Integer | Float | nil', 'find': 'Results::answer', 'rank': 'Array[Integer]' }.get(verb['const'], 'Array[Native::Result]' if verb['const'] in ('rank', 'relate') else 'Native::Result | Array[Native::Result]' if verb['const'] in ('annotate', 'recognize') else 'Array[untyped]')
        answer = value if verb['const'] in ('filter', 'rank', 'relate') else f'{value} | Array[{value}]'
        lines.append(f'    def {verb["const"]}: ({args}, ?cancel: Cancel?, {options}) -> Results::Completed[{answer}]')
    lines += [f'    def initialize: ({settings}) -> void',
              f'    def self.open: ({settings}) {{ (Client client) -> untyped }} -> untyped',
              f'                 | ({settings}) -> Client',
              f'    def plan: (String verb, {args}, {options}) -> Results::NativePlan',
              f'    def start: (String verb, {args}, ?cancel: Cancel?, {options}) {{ (Client::Operation operation) -> untyped }} -> untyped',
              f'             | (String verb, {args}, ?cancel: Cancel?, {options}) -> Client::Operation',
              f'    def self.files: (String | Array[String] paths, ?framing: {request_type("RequestFraming")} | nil, ?media: {request_type("ReaderMedia")}, {reading}) -> Client::Source',
              f'    def self.item: (untyped value, {fields}) -> Client::Item', f'    def self.choose_question: ({request_type("Authored_questionText")} text, options: {request_type("Authored_options")}) -> {request_type("Authored_choose")}',
              f'    def self.relation: (String name, {keywords("Authored_relation", ("name",))}) -> {request_type("Authored_relation")}',
              f'    def self.relate_question: (relations: Array[{request_type("Authored_relation")}], ?fields: {{ name: String, kind: String }}?) -> {request_type("Authored_relate")}',
              f'    def self.recognize_question: (kinds: {schema_type(recognition_fields["kinds"], request_type, objects=True)}, {recognition_keywords}) -> ({schema_type(recognition, request_type, objects=True)})', '  end']
    # Reachable aliases include authored text, cache documents and nested
    # recognition fields; no handwritten request member inventory remains.
    lines.extend(f'  type {alias(key)} = {value}' for key, value in sorted(selected.items()))
    return '\n'.join(lines + ['end']) + '\n'
