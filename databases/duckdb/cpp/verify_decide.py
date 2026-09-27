"""Outside-in stock-host check for the first real C++ scalar path."""

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
        bad_bind = run(["PREPARE bad AS SELECT thinkthen_decide('', 'x')"], backend.base(), extension=extension)
        assert "thinkthen usage: a question is text, not white space" in bad_bind[0]["error"]
        assert backend.count() == 0, "bad foldable question sent a request"

        bad_chunk = run(
            ["SELECT thinkthen_decide(q, 'refund now') FROM (VALUES ('Is it a refund?'), ('')) AS t(q)"],
            backend.base(), extension=extension,
        )
        assert "thinkthen usage: a question is text, not white space" in bad_chunk[0]["error"]
        assert backend.count() == 0, "nonconstant bad question sent a partial chunk"

        file_question = run(["SELECT thinkthen_decide('@question.json', 'refund now')"],
                            backend.base(), extension=extension)
        assert "thinkthen local: question files are not yet enabled" in file_question[0]["error"]
        assert backend.count() == 0, "unfinished file handling sent a request"

        answered = run(
            ["SELECT thinkthen_decide(NULL, 'refund now')",
             "SELECT thinkthen_decide('Is it a refund?', 'refund now')",
             "SELECT typeof(thinkthen_decide('Is it a refund?', 'a separate refund'))"],
            backend.base(), extension=extension,
        )
        assert answered == [{"rows": [[None]]}, {"rows": [[True]]}, {"rows": [["BOOLEAN"]]}]
        assert backend.count() == 1, "only the evaluated non-NULL call should send"

        grouped = run(
            ["SELECT thinkthen_decide('Is it a refund?', t) "
             "FROM (VALUES ('refund now'), ('refund now'), ('a separate refund')) AS x(t)"],
            backend.base(), extension=extension,
        )
        assert grouped == [{"rows": [[True], [True], [True]]}]
        assert backend.count() == 3, "the repeated text should share its first request"
        probabilities = run(
            ["SELECT thinkthen_probability('Is it a refund?', t) "
             "FROM (VALUES ('refund now'), ('refund now')) AS x(t)",
             "SELECT typeof(thinkthen_probability('Is it a refund?', 'refund now'))"],
            backend.base(), extension=extension,
        )
        values = probabilities[0]["rows"]
        assert len(values) == 2 and values[0] == values[1]
        assert isinstance(values[0][0], float) and 0 <= values[0][0] <= 1
        assert probabilities[1] == {"rows": [["DOUBLE"]]}
        assert backend.count() == 4, "probability should deduplicate its repeated text"
        spent = run(
            ["SET thinkthen_query_budget_ms = 0",
             "SELECT thinkthen_decide('Is it a refund?', 'refund now'), "
             "thinkthen_probability('Is it a refund?', 'a separate refund')",
             "SET thinkthen_query_budget_ms = -1",
             "SELECT thinkthen_decide('Is it a refund?', 'refund now')"],
            backend.base(), extension=extension,
        )
        assert "thinkthen deadline: the query has spent its time budget" in spent[1]["error"]
        assert spent[3] == {"rows": [[True]]}
        assert backend.count() == 5, "a spent statement sent nothing and the next query recovered"
        across_chunks = run(
            ["SET thinkthen_query_budget_ms = 10000",
             "SELECT count(*) FILTER (WHERE thinkthen_decide("
             "CASE WHEN i = 2048 THEN 'Is it a refund?' END, "
             "CASE WHEN i = 2048 THEN 'refund now' END)) FROM range(2049) AS x(i)",
             "SET thinkthen_query_budget_ms = -1"],
            backend.base(), extension=extension,
        )
        assert across_chunks[1] == {"rows": [[1]]}
        assert backend.count() == 6, "one live row across the vector boundary should send once"
        bad_budget = run(
            ["SET thinkthen_query_budget_ms = -2",
             "SELECT thinkthen_decide('Is it a refund?', 'refund now')",
             "SET thinkthen_query_budget_ms = -1"],
            backend.base(), extension=extension,
        )
        assert "thinkthen usage: the query budget is outside the supported range" in bad_budget[1]["error"]
        assert backend.count() == 6, "an invalid query budget sent nothing"
        limits = run(
            ["SET thinkthen_max_requests = 0",
             "SELECT thinkthen_decide('Is it a refund?', 'refund now')",
             "RESET thinkthen_max_requests",
             "SET thinkthen_throttle = 0",
             "SELECT thinkthen_decide('Is it a refund?', 'refund now')",
             "RESET thinkthen_throttle",
             "SET thinkthen_max_requests_total = 0",
             "SELECT thinkthen_decide('Is it a refund?', 'refund now')",
             "RESET thinkthen_max_requests_total",
             "SELECT thinkthen_decide('Is it a refund?', 'refund now')"],
            backend.base(), extension=extension,
        )
        assert "thinkthen usage: a request limit" in limits[1]["error"]
        assert "thinkthen usage: a throttle" in limits[4]["error"]
        assert "thinkthen usage: this process has spent its request total" in limits[7]["error"]
        assert limits[9] == {"rows": [[True]]}
        assert backend.count() == 7, "invalid limits sent nothing and reset recovered"
    print("C++ decision, probability, query budget, numeric limits, NULL, type, deduplication, and loopback send boundary pass")


if __name__ == "__main__":
    main()
