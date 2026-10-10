#!/usr/bin/env python3
"""SQLite exposes descriptions for its installed registration inventory without sends."""

import sys

from helper import Backend, child, environment, expect, main


def test_discovery_describes_every_registered_function_without_sends():
    backend = Backend()
    for mode in ("direct", "trusted"):
        held = child("""
connection = sqlite3.connect(":memory:")
connection.enable_load_extension(True)
if os.environ['MODE'] == 'trusted':
        connection.execute("SELECT load_extension(?, 'sqlite3_thinkthen_trusted_init')", (LIB,))
else:
        connection.load_extension(LIB)
rows = connection.execute('SELECT value FROM json_each(thinkthen_functions())').fetchall()
catalog = [json.loads(row[0]) for row in rows]
scalars = {(row['name'], row['arity']) for row in catalog if row['kind'] == 'scalar'}
actual_scalars = set(connection.execute("SELECT name, narg FROM pragma_function_list WHERE name GLOB 'thinkthen_*'"))
tables = {row['name']: row for row in catalog if row['kind'] == 'table'}
actual_tables = {row[0] for row in connection.execute("SELECT name FROM pragma_module_list WHERE name GLOB 'thinkthen_*'")}
assert scalars == actual_scalars, (scalars, actual_scalars)
assert set(tables) == actual_tables, (tables, actual_tables)
assert len(catalog) == len(scalars) + len(tables), 'duplicate catalog rows'
assert all(row['description'].strip() and '\\n' not in row['description'] for row in catalog)
assert ('thinkthen_functions', 0) in scalars
for name, row in tables.items():
        hidden = connection.execute('SELECT count(*) FROM pragma_table_xinfo(?) WHERE hidden=1', (name,)).fetchone()[0]
        assert row['max_arity'] == hidden, (name, row, hidden)
        assert 0 < row['min_arity'] <= row['max_arity']
assert json.loads(connection.execute('SELECT thinkthen_functions()').fetchone()[0]) == catalog
connection.execute('CREATE VIEW discovery AS SELECT thinkthen_functions()')
assert run(connection, 'SELECT * FROM discovery') == 'unsafe use of thinkthen_functions()'
say(count=len(catalog), described=True)
""", environment(backend, MODE=mode, THINKTHEN_MAX_REQUESTS_TOTAL="0"))
        expect(held['described'], True, 'registered descriptions')
    expect(backend.close(), 0, 'loading and discovery sends')


if __name__ == '__main__':
    sys.exit(main(globals()))
