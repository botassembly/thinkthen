"""Focused stock-host checks for the C++ table and aggregate functions."""

from __future__ import annotations

import argparse
import sys
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
    print("C++ usage counters and one real loopback send pass")


if __name__ == "__main__":
    main()
