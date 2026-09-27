"""Outside-in stock-host check for the first real C++ scalar path."""

from __future__ import annotations

import argparse
import json
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
        assert "thinkthen local: the question file question.json was not read: it does not exist or could not be opened" in file_question[0]["error"]
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
        detail_rows = run(
            ["SELECT thinkthen_details('Is it a refund?', t) "
             "FROM (VALUES ('refund now'), ('refund now')) AS x(t)",
             "SELECT typeof(thinkthen_details('Is it a refund?', 'refund now'))"],
            backend.base(), extension=extension,
        )
        rows = detail_rows[0]["rows"]
        assert len(rows) == 2 and rows[0] == rows[1]
        assert isinstance(json.loads(rows[0][0]), dict)
        assert detail_rows[1] == {"rows": [["VARCHAR"]]}
        assert backend.count() == 8, "details should deduplicate its repeated text"
        prepared_try = run(
            ["PREPARE try_bad AS SELECT thinkthen_try_details('', 'x'), "
             "thinkthen_try_details('Is it a refund?', 'refund now')",
             "EXECUTE try_bad"],
            backend.base(), extension=extension,
        )
        assert backend.count() == 9, "try-details bind should send nothing"
        bad, good = [json.loads(value) for value in prepared_try[1]["rows"][0]]
        assert bad["status"] == "failed" and bad["error"]["kind"] == "usage"
        assert good["status"] == "answered" and isinstance(good["details"], dict)
        row_try = run(
            ["SELECT thinkthen_try_details(q, t) FROM "
             "(VALUES ('', 'x'), ('Is it a refund?', 'refund now')) AS x(q,t)"],
            backend.base(), extension=extension,
        )
        assert [json.loads(row[0])["status"] for row in row_try[0]["rows"]] == ["failed", "answered"]
        assert backend.count() == 10, "a failed row should not send or stop a later good row"
        file_try = run(
            ["SELECT thinkthen_try_details('@missing-question.json', 'x'), "
             "thinkthen_try_details('Is it a refund?', 'refund now')"],
            backend.base(), extension=extension,
        )
        failed_file, good_after_file = [json.loads(value) for value in file_try[0]["rows"][0]]
        assert failed_file["status"] == "failed" and failed_file["error"]["kind"] == "local"
        assert "missing-question.json" not in json.dumps(failed_file)
        assert good_after_file["status"] == "answered"
        assert backend.count() == 11, "a local failure should not stop a later good expression"
        assert run(["SELECT thinkthen_try_details(NULL, 'x')"], backend.base(), extension=extension) == [
            {"rows": [[None]]}
        ]
        assert backend.count() == 11, "a NULL try-details row should not send"
        with tempfile.TemporaryDirectory() as folder:
            question_file = Path(folder) / "question.json"
            question_file.write_text('{"decide":"Is it a refund?"}')
            malformed = Path(folder) / "malformed.json"
            malformed.write_text('{"decide":')
            file_rows = run(
                [f"SELECT thinkthen_decide('@{question_file}', 'refund now')",
                 f"SELECT thinkthen_decide('@{malformed}', 'refund now')",
                 f"SELECT thinkthen_try_details('@{malformed}', 'refund now')",
                 "SET enable_external_access = false",
                 f"SELECT thinkthen_decide('@{question_file}', 'refund now')"],
                backend.base(), extension=extension,
            )
            assert file_rows[0] == {"rows": [[True]]}
            assert "thinkthen local: " in file_rows[1]["error"]
            assert json.loads(file_rows[2]["rows"][0][0])["error"]["kind"] == "local"
            assert "file settings refuse it" in file_rows[4]["error"]
            assert backend.count() == 12, "malformed and forbidden files sent no requests"
            mixed_files = run(
                ["SELECT thinkthen_decide(q, 'refund now') FROM "
                 f"(VALUES ('@{question_file}'), ('@{malformed}')) AS x(q)"],
                backend.base(), extension=extension,
            )
            assert "thinkthen local: " in mixed_files[0]["error"]
            assert backend.count() == 12, "a later bad file must stop the whole chunk before sends"
            home_file = run(
                [f"SET home_directory = '{folder}'",
                 "SELECT thinkthen_decide('@~/question.json', 'refund now')"],
                backend.base(), extension=extension,
            )
            assert home_file[1] == {"rows": [[True]]}
            assert backend.count() == 13, "the caller's home_directory should resolve a file once"
            endless = run(
                ["SELECT thinkthen_decide('@/dev/zero', 'refund now')"],
                backend.base(), extension=extension, timeout=15,
            )
            assert "it holds more than 1 MiB" in endless[0]["error"]
            assert backend.count() == 13, "an endless file must stop before a send"
        with tempfile.TemporaryDirectory() as folder:
            cache = run(
                [f"SET thinkthen_cache = '{folder}'",
                 "SELECT thinkthen_decide('Is it a refund?', 'refund now')",
                 "SET thinkthen_cache = 'relative-cache'",
                 "SELECT thinkthen_decide('Is it a refund?', 'refund now')"],
                backend.base(), extension=extension,
            )
            assert cache[1] == {"rows": [[True]]}
            assert "thinkthen usage: a cache folder set from SQL is an absolute local path" in cache[3]["error"]
            assert backend.count() == 14, "an invalid cache path sent no request"
            forbidden = run(
                [f"SET thinkthen_cache = '{folder}'",
                 "SET enable_external_access = false",
                 "SELECT thinkthen_decide('Is it a refund?', 'refund now')"],
                backend.base(), extension=extension,
            )
            assert "thinkthen usage: the cache folder is outside what this database's file settings allow" in forbidden[2]["error"]
            assert backend.count() == 14, "a caller-refused cache folder sent no request"
    print("C++ scalar, budget, limits, cache permissions, NULL, type, deduplication, and loopback boundaries pass")


if __name__ == "__main__":
    main()
