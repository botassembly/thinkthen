"""SQLite receipt projections use the canonical semantic SQL result model."""
import json
from pathlib import Path
import sys

from postgresql import Composite, ROOTS, sql_schema


def render(schema):
    schema = sql_schema(schema)
    model = Composite(schema['$defs'])
    for root in (*ROOTS, 'completesessionQuestionDetail'):
        model.shape({'$ref': '#/$defs/' + root}, root)
    types = {}

    def descriptor(value, owner, field):
        if isinstance(value, str):
            return 'Shape::Scalar(Scalar::' + {
                'json': 'Data', 'unsigned': 'Unsigned', 'string': 'Text',
                'integer': 'Integer', 'number': 'Real', 'boolean': 'Boolean',
            }[value] + ')'
        mode, child = value[:2]
        if mode == 'object':
            return 'Shape::Object(' + json.dumps(child.removeprefix('thinkthen_')) + ')'
        if mode == 'mapping':
            return 'Shape::Mapping(' + json.dumps(child[1].removeprefix('thinkthen_')) + ')'
        if mode == 'array':
            if isinstance(child, tuple) and child[0] == 'object':
                target = child[1].removeprefix('thinkthen_')
            else:
                target = owner + '_' + field
                types[target] = ([('value', descriptor(child, target, 'value'))], True)
            return 'Shape::Array(' + json.dumps(target) + ')'
        raise ValueError(value)

    for kind, (fields, union, _) in model.types.items():
        owner = kind.removeprefix('thinkthen_')
        shaped = [(field, descriptor(shape[1], owner, field))
                  for field, shape in fields.items()]
        types[owner] = (shaped, union)
    types['record_order'] = ([('ordinal', 'Shape::Scalar(Scalar::Integer)'),
                              ('result_index', 'Shape::Scalar(Scalar::Integer)'),
                              ('completed', 'Shape::Scalar(Scalar::Boolean)')], False)
    types['find_selection'] = ([('selected', 'Shape::Scalar(Scalar::Integer)')], False)
    lines = [
        '// Generated from the canonical Rust result schema; do not edit.',
        'use crate::receipt::{Field, Projection, Scalar, Shape};',
        'pub(crate) const PROJECTIONS: &[Projection] = &[',
    ]
    for name, (fields, union) in sorted(types.items()):
        lines += ['Projection { name: ' + json.dumps(name) + ', union: ' +
                  str(union).lower() + ', fields: &[']
        lines += ['Field { name: ' + json.dumps(field) + ', shape: ' + shape + ' },'
                  for field, shape in fields]
        lines.append('] },')
    lines.append('];')
    count = sum(1 + sum(3 if any(part in shape for part in ('Shape::Array', 'Shape::Mapping', 'Scalar::Data')) else 2 for _, shape in fields) for fields, _ in types.values())
    lines.append('pub(crate) const COLUMN_COUNT: usize = ' + str(count) + ';')
    return '\n'.join(lines) + '\n'


def main():
    import subprocess
    import tempfile
    root = Path(__file__).resolve().parents[4]
    sys.path.insert(0, str(root / 'conformance/children'))
    from children import CARGO, child_env
    schema = json.loads((root / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())
    with tempfile.TemporaryDirectory(prefix='thinkthen-sqlite-rustfmt-') as home:
        source = subprocess.run(['rustfmt', '--edition', '2024'], input=render(schema),
                                text=True, capture_output=True, check=True, env=child_env(
                                    keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR'), home=home)).stdout
    output = root / 'databases/sqlite/src/receipt_generated.rs'
    if '--check' in sys.argv:
        if not output.exists() or output.read_text() != source:
            print('generated SQLite receipt projections differ', file=sys.stderr)
            return 1
    else:
        output.write_text(source)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
