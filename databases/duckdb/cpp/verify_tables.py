"""Focused stock-host checks for the C++ table and aggregate functions."""

from __future__ import annotations

import argparse
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
from harness import Backend, run  # noqa: E402  the surface's loopback child


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--extension", required=True, type=Path)
    extension = parser.parse_args().extension.resolve(strict=True)
    with Backend() as backend:
        result = run(
            ["SELECT * FROM thinkthen_usage() ORDER BY metric",
             "SELECT thinkthen_decide('Is it a refund?', 'refund now')",
             "SELECT * FROM thinkthen_usage() ORDER BY metric"],
            backend.base(), extension=extension,
        )
        before = dict(result[0]["rows"])
        after = dict(result[2]["rows"])
        assert set(before) == {"requests_sent", "cache_answers", "input_tokens", "output_tokens"}
        assert result[1] == {"rows": [[True]]}
        assert after["requests_sent"] - before["requests_sent"] == 1
        assert backend.count() == 1, "the usage table did not send a request"
        warmed = run(
            ["SELECT thinkthen_warm('Is it a refund?', t) FROM "
             "(VALUES ('a warm refund'), ('a warm refund'), (NULL)) AS x(t)",
             "SELECT thinkthen_warm('Is it a refund?', NULL)",
             "SELECT thinkthen_warm(q, 'a mixed refund') FROM "
             "(VALUES ('Is it a refund?'), ('Is it a chargeback?')) AS x(q)",
             "SELECT thinkthen_warm(CASE WHEN i=2048 THEN 'Is it a refund?' END, "
             "CASE WHEN i=2048 THEN 'a boundary refund' END) FROM range(2049) AS x(i)"],
            backend.base(), extension=extension,
        )
        assert warmed[0] == {"rows": [[1]]} and warmed[1] == {"rows": [[0]]}
        assert "thinkthen usage: thinkthen_warm judges one question per group" in warmed[2]["error"]
        assert warmed[3] == {"rows": [[1]]}
        assert backend.count() == 3, "only two distinct warm groups sent across a vector boundary"
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder) / "question.json"
            path.write_text('{"decide":"Is it a refund?"}')
            files = run(
                [f"SELECT thinkthen_warm('@{path}', 'a file refund')",
                 "SELECT thinkthen_warm('@~/question.json', 'a refused refund')",
                 "SET enable_external_access = false",
                 f"SELECT thinkthen_warm('@{path}', 'a denied refund')"],
                backend.base(), extension=extension,
            )
            assert files[0] == {"rows": [[1]]}
            assert "thinkthen_warm cannot read an '@~' path" in files[1]["error"]
            assert "file settings refuse it" in files[3]["error"]
            assert backend.count() == 4, "only the allowed file sent"
        without_session_limit = run(
            ["SET thinkthen_max_requests_total = 0",
             "SELECT thinkthen_warm('Is it a refund?', 'a total-free refund')"],
            backend.base(), extension=extension,
        )
        assert without_session_limit[1] == {"rows": [[1]]}
        assert backend.count() == 5, "warm ignores the SQL process-total setting"
    print("C++ usage and warm counters, distinct rows, files, and loopback boundaries pass")


if __name__ == "__main__":
    main()
