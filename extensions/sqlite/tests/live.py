"""Run the all-ten SQL file manually through a compiled SQLite host.

Each non-comment line is one complete SQL statement. Use --stub offline.
"""

import json
import os
from pathlib import Path
import subprocess
import sys

consumer, extension, sql_file, *flags = sys.argv[1:]
assert flags in ([], ["--stub"])
names = ("PATH", "HOME", "TMPDIR", "TEMP", "TMP", "THINKTHEN_API_KEY", "OPENAI_API_KEY")
env = {name: os.environ[name] for name in names if name in os.environ}
with subprocess.Popen([consumer, extension], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                      text=True, env=env) as host:
    def sql(statement, parameters=()):
        host.stdin.write(json.dumps(dict(sql=statement, parameters=parameters)) + "\n")
        host.stdin.flush()
        response = json.loads(host.stdout.readline())
        assert response["code"] == 101, response
        return [dict(zip(response['columns'],row)) for row in response['rows']]

    if flags:
        sql("SELECT thinkthen_set(?,?)", ["backend", "stub"])
    print(json.dumps({"sqlite_version": sql("SELECT sqlite_version() AS version")[0]["version"]}))
    count = 0
    for statement in Path(sql_file).read_text().splitlines():
        if not statement.strip() or statement.lstrip().startswith("--"):
            continue
        rows = sql(statement)
        assert len(rows) == 1, rows
        row=rows[0]
        assert row['status'] in ('answered','unsure') and row['error_code'] is None,rows
        print(json.dumps(row,ensure_ascii=False))
        count += 1
    assert count == 10, count
    host.stdin.close()
    assert host.wait(timeout=30) == 0
