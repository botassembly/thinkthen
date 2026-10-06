"""Administrator-owned backend names, real permission checks and counted routing."""
import json
import os
import pathlib
import sys
import time
from settings_cases import query, quote
from backend_cases import ROWS, QUESTION, alias, canonical_replay, configuration, paths

socket, home, config, base, output, scratch = sys.argv[1:]
env = {"HOME":home,"XDG_CONFIG_HOME":config}


def ask(statements):
    done = query(socket,statements)
    assert done.returncode == 0, done.stderr
    assert "fake-pg" not in done.stdout + done.stderr
    return done.stdout.strip().splitlines()


def observe(command):
    with open(output) as source:
        source.seek(0,2)
        os.write(7,(command+"\n").encode())
        limit = time.monotonic()+30
        while time.monotonic() < limit:
            line = source.readline()
            if line:
                return json.loads(line)
            time.sleep(.01)
    raise AssertionError("backend did not answer " + command)


configuration(env,{"local-"+row["name"]:alias(row,base) for row in ROWS})
ask(["CREATE ROLE named_backend_reader"])
# Registration occurs when the module loads in each PostgreSQL backend.
done = query(socket,["SELECT thinkthen_usage()", "SET ROLE named_backend_reader", "SET thinkthen.backend='local-liquid'"])
assert done.returncode != 0 and 'permission denied to set parameter "thinkthen.backend"' in done.stderr, done.stderr
assert observe("count") == 0

expected = paths("capture_systemone",0)
for index,row in enumerate(ROWS,1):
    got = ask([f"SET thinkthen.backend={quote('local-'+row['name'])}", "SET thinkthen.cache='off'",
               f"SELECT thinkthen_details({quote(QUESTION)}, 'refund')"])
    detail = json.loads(got[-1])
    assert (detail["value"],detail["meta"]["model"]) == (True,row["model"])
    assert observe("count") == index
    counts = observe("paths")
    expected[row["path"]] += 1
    assert counts == expected, (counts,expected)
    markers = {one["name"]:int(at < index) for at,one in enumerate(ROWS)}
    assert observe("bearers") == {"markers":markers,"absent":0,"unknown":0,"overflow":False}

for row in ROWS:
    replay = pathlib.Path(scratch) / ("canonical-"+row["name"])
    canonical_replay(replay,row)
    got = ask([f"SET thinkthen.backend={quote(row['name'])}", "SET thinkthen.cache='off'",
               f"SET thinkthen.replay={quote(str(replay))}",
               f"SELECT thinkthen_details({quote(QUESTION)}, 'refund')"])
    detail = json.loads(got[-1])
    assert (detail["value"],detail["meta"]["model"],detail["meta"]["url"]) == (True,row["model"],row["url"]),detail
    assert observe("count") == len(ROWS)

done = query(socket,["SET thinkthen.backend=123", "SELECT thinkthen_decide('attention?', 'refund')"])
assert done.returncode != 0 and "unknown backend `123`" in done.stderr, done.stderr
done = query(socket,["SET thinkthen.backend='local-liquid'", "SET thinkthen.cache='off'",
                     "SELECT thinkthen_decide('attention?', 'refund')", "SET thinkthen.backend='nowhere'",
                     "SELECT thinkthen_decide('attention?', 'refund')"])
assert done.returncode != 0 and "unknown backend `nowhere`" in done.stderr,done.stderr
assert observe("count") == len(ROWS) + 1
# PostgreSQL's empty sentinel restores the captured unnamed environment route.
got = ask(["SET thinkthen.backend='local-liquid'", "SET thinkthen.backend=''", "SET thinkthen.cache='off'",
           "SELECT thinkthen_details('attention?', 'refund')"])
assert json.loads(got[-1])["meta"]["model"] == "jev-1.13.0"
assert observe("count") == len(ROWS) + 2

# A custom placeholder staged by an ordinary role cannot activate a privileged GUC.
done = query(socket,["SET ROLE named_backend_reader", "SET thinkthen.backend='local-liquid'",
                     "RESET ROLE", "SELECT thinkthen_plan('attention?', 'refund')"])
assert done.returncode != 0 or 'permission denied to set parameter "thinkthen.backend"' in done.stderr, (done.stdout,done.stderr)
assert observe("count") == len(ROWS) + 2
print("pass named backend keys, paths, canonical replay, permissions and sent counts")
