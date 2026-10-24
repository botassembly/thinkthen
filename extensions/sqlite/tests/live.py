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
names = ("PATH", "HOME", "TMPDIR", "TEMP", "TMP", "THINKTHEN_API_KEY", "THINKTHEN_BASE_URL")
env = {name: os.environ[name] for name in names if name in os.environ}
with subprocess.Popen([consumer, extension], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                      text=True, env=env) as host:
    def sql(statement, parameters=()):
        host.stdin.write(json.dumps(dict(sql=statement, parameters=parameters)) + "\n")
        host.stdin.flush()
        response = json.loads(host.stdout.readline())
        assert response["code"] == 101, response
        return response["rows"]

    sql("SELECT thinkthen_configure(?)", [json.dumps({"adapter": "stub"} if flags else {})])
    print(json.dumps({"sqlite_version": sql("SELECT sqlite_version()")[0][0]}))
    count = 0
    for statement in Path(sql_file).read_text().splitlines():
        if not statement.strip() or statement.lstrip().startswith("--"):
            continue
        rows = sql(statement)
        assert len(rows) == 1, rows
        name, position, status, answer, probability, error, kept = rows[0]
        assert status in ("answered", "unsure") and error is None, rows
        print(json.dumps({"function": name, "position": position, "status": status,
                          "answer": answer, "p": probability, "kept": kept}, ensure_ascii=False))
        count += 1
    assert count == 10, count
    host.stdin.close()
    assert host.wait(timeout=30) == 0
