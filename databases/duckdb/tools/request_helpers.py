"""Generate SQL helper fields from canonical authored request controls."""
import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUTPUT = ROOT / 'databases/duckdb/cpp/src/request_helpers_generated.hpp'


def render():
    schema = json.loads((ROOT / 'crates/thinkthen/src/public/request/request.schema.json').read_text())
    fields = list(schema['$defs']['RequestOptions']['properties']) + ['context_schema']
    parameters = ', '.join(f'"{field}" := NULL' for field in fields)
    members = ', '.join(f"'{field}':\"{field}\"" for field in fields)
    sql = f'CREATE MACRO thinkthen_reading({parameters}) AS {{{members}}}'
    return '// Generated from canonical request controls by request_helpers.py.\n' \
           '#pragma once\nnamespace duckdb {\n' \
           'inline void RegisterReadingHelper(ExtensionLoader &loader) {\n' \
           f'    RegisterPortableMacro(loader, {json.dumps(sql)}, "Construct named native request reading controls without executing a call.");\n' \
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
