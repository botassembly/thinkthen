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


def generated(schema):
    path = Path(__file__).parent / 'templates/csharp.py'
    spec = importlib.util.spec_from_file_location('result_csharp', path)
    target = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(target)
    return target.render(graph(schema, ('completeFacts',)))


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
