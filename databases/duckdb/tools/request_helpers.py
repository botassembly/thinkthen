"""Generate SQL helper fields from canonical authored request controls."""
import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUTPUT = ROOT / 'databases/duckdb/cpp/src/request_helpers_generated.hpp'


def helper(name, fields, purpose):
    parameters = ', '.join(f'"{field}"' + (f' := {default}' if default is not None else '')
                           for field, default in fields)
    members = ', '.join(f"'{field}':\"{field}\"" for field, _ in fields)
    sql = f'CREATE MACRO thinkthen_{name}({parameters}) AS {{{members}}}'
    return f'    RegisterPortableMacro(loader, {json.dumps(sql)}, {json.dumps(purpose)});\n'


def render():
    schema = json.loads((ROOT / 'crates/thinkthen/src/public/request/request.schema.json').read_text())
    fields = list(schema['$defs']['RequestOptions']['properties']) + ['context_schema']
    body = helper('reading', [(field, 'NULL') for field in fields],
                  'Construct named native request reading controls without executing a call.')
    body += helper('source', [('paths', None), ('unit', "'line'"), ('window', 'NULL'),
                             ('format', 'NULL'), ('media', "'text'")],
                   'Select explicit local source paths, physical reading units, logical framing and media for an authorized call.')
    body += helper('location', [('file', None), ('first_line', 'NULL'), ('last_line', 'NULL')],
                   'Retain physical source identity and optional line coordinates without adding model evidence.')
    body += helper('record', [(field, 'NULL') for field in
                             ['text', 'json', 'document', 'context', 'options', 'examples',
                              'seed_spans', 'images', 'source']],
                   'Construct one native original with its context, shortlist, recognition controls, images and source.')
    return '// Generated native helpers; reading fields follow the canonical request controls.\n' \
           '#pragma once\nnamespace duckdb {\n' \
           'inline void RegisterRequestHelpers(ExtensionLoader &loader) {\n' + body + \
           '}\n} // namespace duckdb\n'


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    wanted = render()
    if args.check:
        if OUTPUT.read_text() != wanted:
            raise SystemExit('native request helpers differ from canonical controls')
    else:
        OUTPUT.write_text(wanted)
