#!/usr/bin/env python3
"""The third review's DuckDB probes, as one suite.

Every check here names its review finding and fails against the pre-fix
build. Runs offline on the null backend and the recorded conformance
cases; no network, no key.

Findings covered: 1 (the kept-connection lifetime), 6 (the identity
token), 7 (SELECT-only relate), 11 (details' NULL members), 12 (the
record cap before reading), 13 (the SIGINT disposition), 26 (the
reaper's idle cost).
"""

from __future__ import annotations

import json
import os
import resource
import signal
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from review4_lib import relate_rules  # noqa: E402 - the one either-aware rule list

ROOT = Path(__file__).resolve().parent.parent
EXTENSION = ROOT / "build" / "release" / "thinkthen.duckdb_extension"
CASES = ROOT.parent.parent / "conformance" / "conformance.json"

os.environ.setdefault("ENGINE_NULL", "1")

import duckdb  # noqa: E402


def connect(path: str | None = None) -> duckdb.DuckDBPyConnection:
    con = duckdb.connect(
        path or ":memory:", config={"allow_unsigned_extensions": "true"}
    )
    con.execute(f"LOAD '{EXTENSION}'")
    return con


def check(name: str, want: bool, detail: str = "") -> bool:
    if want:
        print(f"ok       {name}")
        return True
    print(f"FAILED   {name}{(': ' + detail) if detail else ''}")
    return False


def raises(con, sql: str, needle: str) -> tuple[bool, str]:
    try:
        con.execute(sql).fetchall()
    except Exception as error:  # noqa: BLE001 - the message is the probe
        text = str(error)
        return needle in text, text
    return False, "no error raised"


def relate_case() -> dict:
    for case in json.loads(CASES.read_text())["cases"]:
        if case["id"] == "69-relate-alerts":
            return case
    raise SystemExit("no 69-relate-alerts case")


def fill(con, case: dict) -> None:
    """The case's own records, so the recordings answer the ask."""
    con.execute("CREATE TABLE t(id INTEGER, body VARCHAR)")
    for index, body in enumerate(case["records"], 1):
        con.execute("INSERT INTO t VALUES (?, ?)", [index, body])


def main() -> int:
    failures = 0
    case = relate_case()
    rules = relate_rules(case)

    # ---- finding 6: the identity token is not a setting anymore ----
    con = connect()
    for attack in [
        "SET thinkthen_instance_token = 'thinkthen:memory:1:1:1'",
        "SET GLOBAL thinkthen_instance_token = ''",
    ]:
        hit, text = raises(con, attack, "unrecognized")
        failures += not check(
            f"the identity token is beyond SQL: {attack.split()[0]} {attack.split()[1]}",
            hit,
            text[:90],
        )
    con.close()

    # ---- finding 7: relate reads records, never runs them ----
    con = connect()
    fill(con, case)
    for statement, label in [
        ("COPY t TO '/tmp/thinkthen-review3-copy.csv'", "COPY TO"),
        ("EXPORT DATABASE '/tmp/thinkthen-review3-export'", "EXPORT DATABASE"),
        ("ATTACH ':memory:' AS smuggled", "ATTACH"),
        ("SET GLOBAL enable_external_access = true", "SET GLOBAL"),
        ("SET VARIABLE smuggled = 1", "SET VARIABLE"),
    ]:
        smuggled = statement.replace("'", "''")
        hit, text = raises(
            con,
            f"SELECT * FROM thinkthen_relate('{smuggled}', {rules})",
            "must be a SELECT",
        )
        failures += not check(f"relate refuses {label}", hit, text[:90])
    con.close()

    # ---- finding 12: the cap refuses before reading rows ----
    con = connect()
    con.execute(
        "CREATE TABLE big AS SELECT i AS id, ? AS body FROM range(8000000) t(i)",
        [case["records"][0]],
    )
    started = time.monotonic()
    hit, text = raises(
        con,
        f"SELECT * FROM thinkthen_relate('SELECT id, body FROM big', {rules})",
        # Review 4, finding 8: the cap is enforced during the scan under
        # a LIMIT, so the refusal names the cap and the 256th row it
        # read, never the total a second execution would have counted.
        "at most 255 records and 256 came",
    )
    elapsed = time.monotonic() - started
    failures += not check(
        f"the 8-million-row cap refuses during the scan (in {elapsed:.1f}s)",
        hit and elapsed < 30,
        text[:90],
    )
    con.close()

    # ---- finding 11: details' members read NULL, never 0 ----
    con = connect()
    score = '{"score":"How strong?","levels":["low","mid","high"]}'
    row = con.execute(
        f"SELECT (thinkthen_details('{score}', 'maybe later')).probability,"
        f" (thinkthen_details('{score}', 'maybe later')).sends,"
        f" (thinkthen_details('{score}', 'maybe later')).nearest"
    ).fetchone()
    failures += not check(
        "a score's details read NULL probability and sends, and its nearest level",
        row == (None, None, "mid"),
        str(row),
    )
    con.close()

    # ---- finding 1 + the reaper: release with no caller, then a fresh
    # LOAD answers; the file lock went with the release ----
    with tempfile.TemporaryDirectory() as tmp:
        path = os.path.join(tmp, "a.db")
        con = connect(path)
        fill(con, case)
        con.execute(f"PREPARE r AS SELECT * FROM thinkthen_relate('SELECT id, body FROM t', {rules})")
        con.close()
        time.sleep(1.0)  # past the reaper's longest quiet pass
        held = os.path.exists(path)
        try:
            with open(path, "a"):
                pass
            locked = False
        except OSError:
            locked = True
        failures += not check(
            "a closed database's file is released after the reaper's pass",
            held and not locked,
        )
        con = connect(path)
        hit, text = raises(con, "EXECUTE r", "does not exist")
        # The statement died with its connection, as statements do; the
        # fresh database answers a fresh relate, so nothing stayed
        # poisoned by the guarded release.
        rows = con.execute(
            f"SELECT count(*) FROM thinkthen_relate('SELECT id, body FROM t', {rules})"
        ).fetchone()
        failures += not check(
            "a fresh LOAD after the guarded release answers relate",
            rows[0] >= 1,
            str(rows),
        )
        con.close()

    # ---- finding 26: idle databases cost a bounded slice ----
    idle = [connect(":memory:") for _ in range(50)]
    time.sleep(2.0)
    def cpu_ticks() -> int:
        with open("/proc/self/stat") as held:
            parts = held.read().split()
        return int(parts[13]) + int(parts[14])
    before = cpu_ticks()
    time.sleep(4.0)
    after = cpu_ticks()
    ticks = after - before
    hz = os.sysconf(os.sysconf_names["SC_CLK_TCK"])
    percent = ticks / hz / 4.0 * 100.0
    failures += not check(
        f"fifty idle databases burn under 5% of a core (measured {percent:.1f}%)",
        percent < 5.0,
    )
    for con in idle:
        con.close()

    # ---- finding 13: a host that ignores SIGINT is left ignoring it ----
    code = f"""
import duckdb, os, signal, sys
os.environ["ENGINE_NULL"] = "1"
signal.signal(signal.SIGINT, signal.SIG_IGN)

def sigint_ignored() -> bool:
    # The kernel's own word: SIGINT's bit in the ignored mask.
    with open("/proc/self/status") as held:
        for line in held:
            if line.startswith("SigIgn:"):
                mask = int(line.split()[1], 16)
                return bool(mask & (1 << (signal.SIGINT - 1)))
    return False

before = sigint_ignored()
con = duckdb.connect(config={{"allow_unsigned_extensions": "true"}})
con.execute("LOAD '{EXTENSION}'")
print("KEPT" if sigint_ignored() else "REPLACED", "ignored-before" if before else "not-ignored-before")
"""
    out = subprocess.run(
        [sys.executable, "-c", code], capture_output=True, text=True, timeout=60
    )
    failures += not check(
        "a host that ignored SIGINT keeps its ignore (the kernel's mask)",
        "KEPT ignored-before" in out.stdout,
        out.stdout.strip() + out.stderr.strip()[:80],
    )

    print("== review3 duckdb suite:", "green" if failures == 0 else f"{failures} failed")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
