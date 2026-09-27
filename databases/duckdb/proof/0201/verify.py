"""Small stock-host proof for the 0201 C++/Rust bridge."""

import argparse
import subprocess
import sys
from pathlib import Path

import duckdb


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cli", type=Path, required=True)
    parser.add_argument("--extension", type=Path, required=True)
    parser.add_argument("--panic-child", action="store_true")
    args = parser.parse_args()
    extension = str(args.extension.resolve())
    quoted = extension.replace("'", "''")
    if args.panic_child:
        session = duckdb.connect(config={"allow_unsigned_extensions": "true"})
        session.execute(f"LOAD '{quoted}'")
        marker = "SYNTHETIC_KEY_AND_EVIDENCE_MARKER_0201"
        try:
            session.execute("SELECT thinkthen_probe_nested(?)", [f"panic:{marker}"]).fetchone()
        except duckdb.InvalidInputException as error:
            assert "tagged Rust error 4" in str(error), "wrong panic status"
            assert marker not in str(error), "panic payload reached SQL error"
        else:
            raise AssertionError("Rust panic crossed the C bridge")
        next_answer = session.execute("SELECT thinkthen_probe_nested('after')").fetchone()[0]
        assert next_answer["text"] == "after", "the next query failed"
        session.execute("SELECT thinkthen_probe_nested('other-thread')").fetchone()
        return
    sql = (
        f"LOAD '{quoted}'; "
        "SELECT thinkthen_probe_nested('alpha'); "
        "SET thinkthen_probe_limit=7; "
        "SELECT thinkthen_probe_setting(i::INTEGER) FROM range(3) t(i); "
        "PREPARE p AS SELECT thinkthen_probe_setting(i::INTEGER) FROM range(3) t(i); "
        "SET thinkthen_probe_limit=13; EXECUTE p;"
    )
    cli = subprocess.run(
        [str(args.cli.resolve()), "-unsigned", "-c", sql],
        capture_output=True,
        text=True,
        check=True,
    )
    assert "owned-by-rust, nested" in cli.stdout, cli.stdout
    assert "│                                           7 │" in cli.stdout, cli.stdout
    assert "│                                          13 │" in cli.stdout, cli.stdout

    sessions = [duckdb.connect(config={"allow_unsigned_extensions": "true"}) for _ in range(2)]
    for session in sessions:
        session.execute(f"LOAD '{quoted}'")
    sessions[0].execute("SET thinkthen_probe_limit=11")
    sessions[1].execute("SET thinkthen_probe_limit=29")
    query = "SELECT thinkthen_probe_setting(i::INTEGER) FROM range(3) t(i)"
    actual = [sessions[index].execute(query).fetchone()[0] for index in (0, 1, 0)]
    assert actual == [11, 29, 11], actual
    nested = sessions[0].execute("SELECT thinkthen_probe_nested('alpha')").fetchone()[0]
    assert nested == {"text": "alpha", "tags": ["owned-by-rust", "nested"]}, nested
    child = subprocess.run(
        [sys.executable, str(Path(__file__).resolve()), "--cli", str(args.cli),
         "--extension", extension, "--panic-child"],
        capture_output=True, text=True, check=True,
    )
    assert "SYNTHETIC_KEY_AND_EVIDENCE_MARKER_0201" not in child.stdout + child.stderr, "panic payload leaked"
    assert "unrelated host panic marker" in child.stderr, "the prior host hook did not run on another thread"
    print("stock CLI: nested value and prepared 7→13 setting passed")
    print("stock Python: two caller sessions 11/29/11, redacted panic, later query, and host hook passed")


if __name__ == "__main__":
    main()
