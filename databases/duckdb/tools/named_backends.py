"""Backend SQL selection reaches the complete C++/Rust session bridge."""
import json
import subprocess
import sys
import tempfile
from pathlib import Path
from harness import Backend, CHILD, EXTENSION, case, child_env, expect, main, rows
from backend_cases import ROWS, QUESTION, alias, assert_sent, canonical_replay, configuration


def execute(statements, env):
    done = subprocess.run([sys.executable,"-c",CHILD,str(EXTENSION),json.dumps(statements)],
                          env=env,capture_output=True,text=True,timeout=60,check=False)
    assert done.returncode == 0, done.stderr
    return [json.loads(line) for line in done.stdout.splitlines() if line.startswith("{")]


@case
def named_aliases_pair_provider_keys_and_paths():
    for row in ROWS:
        marker = "fake-duck-"+row["name"]
        with Backend({row["name"]:marker,"unnamed":"sk-loopback-duckdb-suite"}) as backend, tempfile.TemporaryDirectory() as folder:
            env = child_env(backend.base(),Path(folder),{row["key"]:marker})
            configuration(env,{"local-"+row["name"]:alias(row,backend.base("arm/full/capture"))})
            got = execute([f"SET thinkthen_backend = 'local-{row['name']}'", "SET thinkthen_cache = 'off'",
                           f"SELECT thinkthen_details('{QUESTION}', 'refund')"],env)
            detail = json.loads(rows(got[-1])[0][0])
            expect(detail["value"], True, row["name"])
            assert marker not in json.dumps(got)
            assert_sent(backend,{**row,"form":"both" if row["form"] == "both" else "authored"},marker)


@case
def canonical_builtin_replay_pins_url_model_path_and_form_without_sends():
    with Backend() as backend:
        for row in ROWS:
            with tempfile.TemporaryDirectory() as folder:
                root = Path(folder)
                env = child_env(backend.base(),root)
                replay = root / "replay"
                canonical_replay(replay,row)
                got = execute([f"SET thinkthen_backend = '{row['name']}'", "SET thinkthen_cache = 'off'",
                               f"SET thinkthen_replay = '{replay}'", f"SELECT thinkthen_details('{QUESTION}', 'refund')"],env)
                detail = json.loads(rows(got[-1])[0][0])
                expect((detail["value"],detail["meta"]["model"],detail["meta"]["url"]),(True,row["model"],row["url"]),row["name"])
                expect(backend.count(),0,"canonical replay sends nothing")


@case
def session_backend_isolation_reset_and_database_text_coercion():
    with Backend({"a":"fake-a","b":"fake-b"}) as backend, tempfile.TemporaryDirectory() as folder:
        env = child_env(backend.base(),Path(folder),{"THINKTHEN_BACKEND":"one","ONE_KEY":"fake-a","TWO_KEY":"fake-b"})
        base = backend.base("arm/full/capture")
        configuration(env,{"one":{"url":base,"key_env":"ONE_KEY","model":"one"},
                           "two":{"url":base,"key_env":"TWO_KEY","model":"two","path":"decisions"}})
        query = "SELECT thinkthen_details('attention?', 'refund')"
        got = execute(["SET thinkthen_cache = 'off'",query,
                       ["B","SET thinkthen_cache = 'off'"],["B","SET thinkthen_backend = 'two'"],["B",query],
                       query,["B","RESET thinkthen_backend"],["B",query],
                       "SET thinkthen_backend = ''",query,"SET thinkthen_backend = 123",query,
                       "RESET thinkthen_backend",query],env)
        expect([json.loads(rows(got[i])[0][0])["meta"]["model"] for i in (1,4,5,7,13)], ["one","two","one","one","one"], "backend isolation and reset")
        assert "backend name uses" in got[9]["error"]
        assert "unknown backend `123`" in got[11]["error"]
        expect(backend.count(),5,"invalid names send nothing")
        expect(backend.snapshot("bearers"),{"markers":{"a":4,"b":1},"absent":0,"unknown":0,"overflow":False},"each selected engine retains its key")


if __name__ == '__main__':
    main()
