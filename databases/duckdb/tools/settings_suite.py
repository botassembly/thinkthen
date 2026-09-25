"""The engine settings, the process request total, the cache-folder check, `@file` access, and
volatile scalars (ticket 0110 decisions 5, 10, and 16).

The access table is experiment 253's 35 cases: five file settings over
seven folders. DuckDB's own `COPY ... TO` judges each cache folder, and
DuckDB's own `read_text` judges each `@file`.
"""

from __future__ import annotations

import os
import sys
import tempfile
from pathlib import Path

from harness import Backend, case, expect, main, rows, run, said

ASK = "SELECT thinkthen_decide('Is it a refund?', 'refund now')"
PROBE_REFUSAL = "thinkthen usage: the cache folder is outside what this database's file settings allow"
SHAPE = "thinkthen usage: a cache folder set from SQL is an absolute local path with no scheme"
THROTTLE = "thinkthen usage: a throttle is a whole number from 1 through 32"
SPENT = "thinkthen usage: this process has spent its request total of 3; raise SET thinkthen_max_requests_total or RESET it"


@case
def throttle_conflict():
    with Backend() as backend:
        got = run(["SET thinkthen_throttle = 8", ASK, "SET thinkthen_throttle = 4", ASK], backend.base())
        expect(rows(got[1]), [[True]], "the first throttle answers")
        expect(said(got[3]), "thinkthen usage: throttle 8 is already active for this process; use throttle 8 or drop the throttle argument", "a second throttle")
        expect(backend.count(), 1, "counted sends")


@case
def throttle_range():
    with Backend() as backend:
        for value in (33, 0, 264, -1):
            got = run([f"SET thinkthen_throttle = {value}", ASK], backend.base())
            expect("error" in got[0], False, f"SET thinkthen_throttle = {value}")
            expect(said(got[1]), THROTTLE, f"throttle {value}")
        expect(backend.count(), 0, "counted sends")


@case
def request_limit_range():
    with Backend() as backend:
        for value in (0, -1):
            got = run([f"SET thinkthen_max_requests = {value}", ASK], backend.base())
            expect(said(got[1]), "thinkthen usage: a request limit is a whole number of 1 or more", f"max_requests {value}")
        expect(backend.count(), 0, "counted sends")


@case
def the_process_request_total_holds_across_calls():
    """Ian's ruling of 2026-09-25: a total per process, checked before each call."""
    with Backend() as backend:
        ten = "SELECT thinkthen_decide('Is it a refund?', 'refund ' || i) FROM range(10) t(i)"
        got = run(["SET thinkthen_max_requests_total = 3", ten, ASK], backend.base())
        expect(said(got[1]), SPENT, "ten rows under a total of 3")
        expect(backend.count(), 3, "counted sends")
        expect(said(got[2]), SPENT, "the next call")
        expect(backend.count(), 3, "counted sends after the refusal")


@case
def cache_cap_is_checked_on_a_map_hit():
    with Backend() as backend:
        got = run([ASK, "SET thinkthen_cache_bytes = 0", "SELECT thinkthen_decide('Is it a refund?', 'other')"], backend.base())
        expect(rows(got[0]), [[True]], "the first decide")
        expect(said(got[2]), "thinkthen usage: a cache cap is a whole number of bytes above zero", "a zero cap")
        expect(backend.count(), 1, "counted sends")


@case
def cache_folder_shape():
    with Backend() as backend:
        for folder in ("~/c", "c", "s3://b/c", "file:///tmp/c"):
            got = run([f"SET thinkthen_cache = '{folder}'", ASK], backend.base())
            expect(said(got[1]), SHAPE, f"cache folder {folder}")
        expect(backend.count(), 0, "counted sends")


@case
def settings_keep_the_environments_seeding():
    with Backend() as backend, tempfile.TemporaryDirectory() as cache:
        details = "SELECT (thinkthen_details('Is it a refund?', 'refund now')).cached"
        first = run([details], backend.base(), extra={"THINKTHEN_CACHE": cache})
        expect(rows(first[0]), [[False]], "the first run asks")
        sent = backend.count()
        second = run(["SET thinkthen_throttle = 8", details], backend.base(), extra={"THINKTHEN_CACHE": cache})
        expect(rows(second[1]), [[True]], "the second run answers from the environment's cache")
        expect(backend.count() - sent, 0, "counted sends in the second run")


@case
def volatile_scalars_are_not_folded_at_plan_time():
    with Backend() as backend:
        got = run(["EXPLAIN SELECT thinkthen_decide('q', 'a constant text')"], backend.base(), timeout=30)
        rows(got[0])
        expect(backend.count(), 0, "counted sends while planning")


def folders(root: Path) -> dict[str, str]:
    ok, out = root / "ok", root / "out"
    ok.mkdir()
    out.mkdir()
    (ok / "file").write_text("x")
    os.symlink(out, ok / "link")
    return {
        "ok": str(ok),
        "ok/new": f"{ok}/new",
        "out": str(out),
        "ok/../out": f"{ok}/../out",
        "ok/../out/new": f"{ok}/../out/new",
        "ok/link": f"{ok}/link",
        "ok/link/new": f"{ok}/link/new",
    }


def settings(root: Path) -> dict[str, list[str]]:
    ok = root / "ok"
    return {
        "default": [],
        "external_off": ["SET enable_external_access = false"],
        "dirs_ok": [f"SET allowed_directories = ['{ok}']", "SET enable_external_access = false"],
        "paths_ok": [f"SET allowed_paths = ['{ok}']", "SET enable_external_access = false"],
        "local_disabled": ["SET disabled_filesystems = 'LocalFileSystem'"],
    }


def judged(result: dict) -> str:
    """DuckDB's own answer. `read_text` on a missing file counts 0 rows."""
    if "error" not in result:
        return "missing" if result["rows"] == [[0]] else "allowed"
    return "refused" if "Permission Error" in result["error"] else "missing"


@case
def access_cases_match_duckdb():
    """The 35 cache cases against COPY, then the 35 `@file` cases against
    read_text. A refused cache case sends nothing and creates nothing."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        root = Path(folder)
        paths = folders(root)
        for place in ("ok", "out"):
            (root / place / "q.json").write_text('{"decide": "Is it a refund?"}')
        wrong = []
        for setting, setup in settings(root).items():
            for name, path in paths.items():
                before = sorted(os.listdir(root / "out")), sorted(os.listdir(root / "ok"))
                sent = backend.count()
                got = run(
                    [f"SET thinkthen_cache = '{path}'", *setup, f"COPY (SELECT 1) TO '{path}/oracle.csv'", ASK],
                    backend.base(),
                )
                oracle = judged(got[-2]) != "refused"
                ours = got[-1].get("error") is None or PROBE_REFUSAL not in said(got[-1])
                for leftover in (Path(path) / "oracle.csv",):
                    leftover.unlink(missing_ok=True)
                if oracle != ours:
                    wrong.append(f"cache {setting} {name}: COPY {'allows' if oracle else 'refuses'}, we {'allow' if ours else 'refuse'}")
                if not ours and (backend.count() != sent or before != (sorted(os.listdir(root / "out")), sorted(os.listdir(root / "ok")))):
                    wrong.append(f"cache {setting} {name}: a refused folder sent or created something")
                files = run([*setup, f"SELECT count(*) FROM read_text('{path}/q.json')", f"SELECT thinkthen_decide('@{path}/q.json', 'refund now')"], backend.base())
                oracle = judged(files[-2])
                ours = "allowed" if "error" not in files[-1] else ("refused" if "file settings refuse it" in said(files[-1]) else "missing")
                if oracle != ours:
                    wrong.append(f"@file {setting} {name}: read_text {oracle}, we {ours}")
        if wrong:
            raise AssertionError("; ".join(wrong))


@case
def two_databases_each_judge_their_own_access():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        question = Path(folder) / "q.json"
        question.write_text('{"decide": "Is it a refund?"}')
        cache = Path(folder) / "cache"
        got = run(
            [
                ["B", "SET enable_external_access = false"],
                f"SET thinkthen_cache = '{cache}'",
                ["B", f"SET thinkthen_cache = '{cache}'"],
                ASK,
                ["B", ASK],
            ],
            backend.base(),
        )
        expect(rows(got[3]), [[True]], "A's decide")
        expect(said(got[4]), PROBE_REFUSAL, "B's decide on A's folder")
        expect(backend.count(), 1, "counted sends")
        files = run(
            [
                ["B", "SET enable_external_access = false"],
                f"SELECT thinkthen_decide('@{question}', 'refund now')",
                ["B", f"SELECT thinkthen_decide('@{question}', 'refund now')"],
            ],
            backend.base(),
        )
        expect(rows(files[1]), [[True]], "A reads the file")
        expect(said(files[2]), f"thinkthen local: the question file {question} was not read: this database's file settings refuse it", "B's read")


if __name__ == "__main__":
    sys.exit(main())
