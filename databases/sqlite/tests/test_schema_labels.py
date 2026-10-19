#!/usr/bin/env python3
"""Normal installed loading labels arriving records and preserves SQLite trust."""

import sys

from helper import Backend, child, environment, expect, main

INGEST = r"""
path = os.path.join(os.environ['SCRATCH'], 'labels.db')
db = connect(path)
db.execute('PRAGMA trusted_schema=' + os.environ['SCHEMA_TRUST'])
db.execute("CREATE TABLE messages(body TEXT, label TEXT)")
db.execute('''CREATE TRIGGER label_message AFTER INSERT ON messages BEGIN
    UPDATE messages SET label=thinkthen_choose(
        '{"choose":"Which color?","options":["red","blue"]}', NEW.body)
    WHERE rowid=NEW.rowid;
END''')
inserted = run(db, "INSERT INTO messages(body) VALUES ('a red coat')")
db.close()
# Ordinary reads need neither the extension nor a model call.
saved = sqlite3.connect(path)
rows = run(saved, "SELECT body, label FROM messages WHERE label='red'")
say(inserted=inserted, rows=rows)
"""


def test_normal_load_labels_on_insert_and_plain_sql_reads_the_saved_label():
    backend = Backend()
    held = child(INGEST, environment(backend, SCHEMA_TRUST='ON'))
    expect(held, {'inserted': [], 'rows': [['a red coat', 'red']]},
           'normal loading permits the ingestion trigger and persists its label')
    expect(backend.close(), 1, 'only the trigger sends a model request')


def test_untrusted_schema_refuses_the_same_trigger_before_any_send():
    backend = Backend()
    held = child(INGEST, environment(backend, SCHEMA_TRUST='OFF'))
    expect(held, {'inserted': 'unsafe use of thinkthen_choose()', 'rows': []},
           'SQLite refuses the trigger and rolls back the insert')
    expect(backend.close(), 0, 'the refused trigger sends nothing')


if __name__ == '__main__':
    sys.exit(main(globals()))
