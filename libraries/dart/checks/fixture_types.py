"""Encode actual installed public typed getters for the conformance consumer."""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'sdlc/generators/results'))
from generate import graph, prepare
from native import immediate_schema
sys.path.insert(0, str(ROOT / 'sdlc/generators/results/templates'))
import dart


def write(destination, member='dart'):
    definitions = prepare(graph(immediate_schema(json.loads((ROOT / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())), dart.ROOTS))
    dart.PREFIX = ''
    lines = ['// Fixture glue reads actual installed getters; no native JSON fallback.',
             f"import 'package:thinkthen_{member}/thinkthen_{member}.dart';", 'Object? fixtureValue(Object? value) {',
             '  if (value is List) return value.map(fixtureValue).toList(growable: false);',
             '  if (value is Map) return value.map((key, child) => MapEntry(key, fixtureValue(child)));']
    for key, source in definitions.items():
        cls = dart.name(key)
        if 'properties' in source:
            lines += [f'  if (value is {cls}) return {{', '    ...value.extensions,']
            for prop in source['properties']:
                access = 'value.' + dart.member(prop)
                if prop in source.get('required', []):
                    lines += [f'    {json.dumps(prop)}: fixtureValue({access}),']
                else:
                    lines += [f'    if ({access}.isPresent) {json.dumps(prop)}: fixtureValue({access}.value),']
            lines += ['  };']
        elif 'primitive_variants' in source or dart.union(source, definitions):
            lines += [f'  if (value is {cls}) return fixtureValue(value.value);']
    lines += ['  return value;', '}']
    functions = json.loads((ROOT / 'specification/request.schema.json').read_text())['$defs']['RequestFunction']['oneOf']
    lines += ['OwnedSession startFixture(Engine engine, String function, Question question, Input input, {Options? options}) => switch (function) {']
    lines += [f'  "{item["const"]}" => engine.{item["const"]}Session(question, input, options: options),' for item in functions]
    lines += ["  _ => throw StateError('Unknown fixture function'),", '};']
    Path(destination).write_text('\n'.join(lines) + '\n')


if __name__ == '__main__':
    write(sys.argv[1])
