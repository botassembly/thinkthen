#!/usr/bin/env python3
"""Generate result declarations from the existing Rust-derived result schema."""
import argparse
import importlib.util
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[3]
SCHEMA = ROOT / 'crates/thinkthen/src/public/results/complete.schema.json'
OUTPUT = Path(__file__).parent / 'csharp/CompleteFacts.g.cs'


def references(value):
    if isinstance(value, dict):
        if '$ref' in value:
            prefix = '#/$defs/'
            reference = value['$ref']
            if not reference.startswith(prefix):
                raise ValueError(f'unsupported result reference: {reference}')
            yield reference[len(prefix):]
        for child in value.values():
            yield from references(child)
    elif isinstance(value, list):
        for child in value:
            yield from references(child)


def graph(schema, roots):
    """Keep the transitive source graph; targets never keep member inventories."""
    definitions = schema['$defs']
    pending = list(roots)
    selected = {}
    while pending:
        name = pending.pop()
        if name in selected:
            continue
        selected[name] = definitions[name]
        pending.extend(references(definitions[name]))
    return dict(sorted(selected.items()))


def object_variants(schema, definitions):
    """Separate object alternatives from assertions on an existing object."""
    if 'oneOf' in schema and 'anyOf' in schema:
        raise ValueError('combined object unions need a typed alternative')
    alternatives = schema.get('oneOf', schema.get('anyOf', []))
    alternatives = [definitions[item['$ref'].removeprefix('#/$defs/')]
                    if '$ref' in item else item for item in alternatives]
    if not alternatives:
        return None
    if 'properties' in schema and all(assertion(item) for item in alternatives):
        return None
    if not all(item.get('type') == 'object' and 'properties' in item
               for item in alternatives):
        return None
    common = schema.get('properties', {})
    required = schema.get('required', [])
    result = []
    for item in alternatives:
        overlap = common.keys() & item['properties'].keys()
        if any(common[key] != item['properties'][key] for key in overlap):
            raise ValueError('conflicting object variant properties')
        result.append({**item, 'properties': {**common, **item['properties']},
                       'required': list(dict.fromkeys(required + item.get('required', [])))})
    return result


def assertion(schema):
    """Required-member assertions add no typed fields or object alternatives."""
    return (isinstance(schema, dict) and bool(schema) and
            set(schema) <= {'required', 'not'} and
            ('not' not in schema or assertion(schema['not'])))


def discriminator(variants, definitions):
    """Use unique required literals, otherwise exclusive patterned string identities."""
    common = set.intersection(*(set(item.get('required', [])) for item in variants))
    for member in sorted(common):
        fields = [item['properties'][member] for item in variants]
        fields = [definitions[field['$ref'].removeprefix('#/$defs/')]
                  if '$ref' in field else field for field in fields]
        fields = [{'const': field['enum'][0]} if len(field.get('enum', [])) == 1
                  else field for field in fields]
        if all(isinstance(field.get('const'), str) for field in fields):
            values = [field['const'] for field in fields]
            if len(set(values)) == len(values):
                return [('literal', member, value) for value in values]
    tags = []
    for index, item in enumerate(variants):
        others = set.union(*(set(other.get('required', []))
                             for j, other in enumerate(variants) if j != index))
        unique = set(item.get('required', [])) - others
        unique = {member for member in unique if
                  identity_field(item['properties'][member], definitions)}
        if len(unique) != 1:
            raise ValueError('ambiguous object union discriminator')
        tags.append(('member', unique.pop(), None))
    return tags


def identity_field(field, definitions):
    if '$ref' in field:
        field = definitions[field['$ref'].removeprefix('#/$defs/')]
    return field.get('type') == 'string' and 'pattern' in field


def prepare(definitions):
    """Derive reusable tagged object definitions without a target member inventory."""
    result = {}
    for key, source in definitions.items():
        if 'allOf' in source:
            raise ValueError('object intersections need an explicit typed representation')
        variants = object_variants(source, definitions)
        if variants is None:
            source = dict(source)
            if 'properties' in source:
                for keyword in ('oneOf', 'anyOf'):
                    alternatives = source.get(keyword, [])
                    if alternatives and all(assertion(item) for item in alternatives):
                        del source[keyword]
            result[key] = source
            continue
        tags = discriminator(variants, definitions)
        names = [key + '_' + (value if mode == 'literal' else member)
                 for mode, member, value in tags]
        if len(set(names)) != len(names) or any(name in definitions for name in names):
            raise ValueError('colliding generated object variant names')
        result[key] = {'variants': list(zip(names, tags))}
        result.update(zip(names, variants))
    return dict(sorted(result.items()))


def generated(schema):
    path = Path(__file__).parent / 'templates/csharp.py'
    spec = importlib.util.spec_from_file_location('result_csharp', path)
    target = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(target)
    roots = ('completeFacts', 'completeanswer', 'completefindAnswer',
             'completeCallError', 'completeRelationMember', 'completeObservation',
             'completeUsage', 'completeReadableQuestion', 'completesourceRelationEndpoint')
    return target.render(prepare(graph(schema, roots)))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--schema', type=Path, default=SCHEMA)
    parser.add_argument('--output', type=Path, default=OUTPUT)
    args = parser.parse_args()
    result = generated(json.loads(args.schema.read_text()))
    if args.check:
        if not args.output.exists() or args.output.read_text() != result:
            print('generated C# results differ; run sdlc/generators/results/generate.py',
                  file=sys.stderr)
            return 1
    else:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(result)
    return 0


if __name__ == '__main__':
    sys.exit(main())
