"""Installed optional namespaces preserve column identity and engine outcomes."""

from __future__ import annotations

import importlib
import json
import os
import subprocess
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import Any, cast

import pytest
from conformance import Loopback, child_environment

import thinkthen as tt


@pytest.fixture(params=["pandas", "polars"])
def library(request: pytest.FixtureRequest) -> Any:
    module = pytest.importorskip(request.param)
    importlib.import_module("thinkthen." + request.param)
    return module


def column(library: Any, values: list[Any]) -> Any:
    if library.__name__ == "pandas":
        return library.Series(values, index=["same"] * len(values), dtype=object)
    # Object keeps arbitrary JSON records intact instead of coercing dictionaries.
    return library.Series("body", values, dtype=library.Object)


def rows(library: Any, frame: Any) -> list[dict[str, Any]]:
    if library.__name__ == "pandas":
        return cast(
            list[dict[str, Any]],
            frame.astype(object).where(frame.notna(), None).to_dict("records"),
        )
    # Polars 1.0's Python row conversion loses outer Struct nulls.
    result = cast(list[dict[str, Any]], frame.to_dicts())
    for key in frame.columns:
        for item, missing in zip(result, frame[key].is_null()):
            if missing:
                item[key] = None
    return result


def invoke(series: Any, request: dict[str, Any], client: tt.Client) -> Any:
    function = request["function"]
    question = request["question"]["value"]
    options = request.get("options", {})
    if function == "recognize":
        return series.thinkthen.recognize(kinds=question, client=client, **options)
    if function == "relate":
        return series.thinkthen.relate(rules=question, client=client, **options)
    return getattr(series.thinkthen, function)(question, client=client, **options)


def test_all_ten_columns_match_shared_stub_rows(library: Any) -> None:
    cases = json.loads(
        (
            Path(os.environ["THINKTHEN_CASES"]).parent / "construction/stub.json"
        ).read_text()
    )
    for case in cases:
        request = case["request"]
        values = request["records"]
        source = column(library, values)
        result = invoke(source, request, tt.Client(adapter="stub"))
        actual = rows(library, result)
        assert len(actual) == len(values)
        if library.__name__ == "pandas":
            assert result.index.equals(source.index)
        by_id = {json.dumps(row["id"], sort_keys=True): row for row in case["expected"]}
        for position, item in enumerate(actual):
            identity = (
                values[position].get("id", position + 1)
                if isinstance(values[position], dict)
                else position + 1
            )
            expected = by_id.get(json.dumps(identity, sort_keys=True))
            assert item["position"] == position
            if expected is None:
                assert item["answer"] is None and item["status"] is None
                continue
            for key, value in expected.items():
                if (
                    library.__name__ == "polars"
                    and request["function"] == "relate"
                    and key == "answer"
                    and value is not None
                ):
                    value = [{"either": None, **relation} for relation in value]
                assert item[key] == value, (request["function"], key, item, expected)
            if request["function"] == "filter":
                assert item["kept"] == (
                    expected["status"] == "answered" and expected["answer"] is True
                )


def test_missing_unsure_and_empty_columns_keep_native_types(library: Any) -> None:
    source = column(library, [None, "alpha"])
    result = source.thinkthen.decide(
        "Ready?", threshold="0.01:1", client=tt.Client(adapter="stub")
    )
    actual = rows(library, result)
    assert actual[0] == {
        "position": 0,
        "id": None,
        "status": None,
        "answer": None,
        "p": None,
        "error": None,
        "detail": None,
    }
    assert actual[1]["status"] == "unsure" and actual[1]["answer"] is None
    assert (
        result["answer"].isna().all()
        if library.__name__ == "pandas"
        else result["answer"].null_count() == 2
    )
    found = column(library, [None]).thinkthen.find(
        "Which line?", client=tt.Client(adapter="stub")
    )
    assert all(item["answer"] is None for item in rows(library, found))
    for values in ([], [None, None]):
        frame = column(library, values).thinkthen.recognize(
            client=tt.Client(adapter="stub")
        )
        assert len(frame) == len(values)
        assert str(frame["answer"].dtype) == (
            "object"
            if library.__name__ == "pandas"
            else "List(Struct({'text': String, 'start': Int64, 'end': Int64, 'kind': String, 'p': Float64}))"
        )


def test_columns_batch_once_and_report_record_errors(
    library: Any, tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    case = tmp_path / "batch.json"
    case.write_text('{"replies":[{"answer":{"type":"noul","noul":0.9}}]}')
    monkeypatch.setenv("THINKTHEN_API_KEY", "synthetic-loopback-key")
    with Loopback(case) as server:
        source = column(
            library,
            [{"id": "same", "body": "a"}, {"id": "same"}, {"id": "same", "body": "c"}],
        )
        client = tt.Client(url=server.url, batch=3)
        with pytest.raises(tt.UsageError) as failure:
            source.thinkthen.decide("Ready?", on="/body", client=client)
        assert failure.value.code == "usage.missing"
        assert server.count() == 1
        with pytest.raises(tt.ThinkThenError):
            source.thinkthen.filter("Ready?", on="/body", client=client)
        assert server.count() == 2


def test_rank_top_and_duplicate_labels_use_source_ordinals(library: Any) -> None:
    source = column(
        library,
        [{"id": "same", "body": "alpha"}, None, {"id": "same", "body": "evidence 5"}],
    )
    result = source.thinkthen.rank(
        "Ready?", top=1, on="/body", client=tt.Client(adapter="stub")
    )
    actual = rows(library, result)
    selected = [item for item in actual if item["answer"] is not None]
    assert len(selected) == 1 and selected[0]["answer"] == 1
    assert selected[0]["position"] == 2
    assert actual[0]["answer"] is None and actual[1]["answer"] is None


def test_json_values_and_cancel_remain_native(library: Any) -> None:
    source = column(library, [{"id": {"nul\x00key": 2**64 - 1}, "body": None}])
    result = source.thinkthen.decide(
        "Ready?", on="/body", client=tt.Client(adapter="stub")
    )
    actual = rows(library, result)
    assert actual[0]["id"] == {"nul\x00key": 2**64 - 1}
    assert actual[0]["status"] == "answered"
    question = tt.Toolbox({"decide": "Ready?"}).question()
    named = source.thinkthen.decide(
        question, on="/body", client=tt.Client(adapter="stub")
    )
    assert rows(library, named) == actual
    if library.__name__ == "polars":
        with pytest.raises(ValueError):
            column(library, [float("nan")]).thinkthen.decide(
                "Ready?", client=tt.Client(adapter="stub")
            )
    cancel = tt.Cancel()
    cancel.cancel()
    with pytest.raises(KeyboardInterrupt):
        source.thinkthen.decide(
            "Ready?", client=tt.Client(adapter="stub"), cancel=cancel
        )


def test_pandas_multiindex_and_scalar_missing_values() -> None:
    pd = pytest.importorskip("pandas")
    importlib.import_module("thinkthen.pandas")
    index = pd.MultiIndex.from_tuples(
        [("b", 2), ("a", 1), ("b", 2), ("c", 0)], names=["group", "mail"]
    )
    source = pd.Series(
        [pd.NA, float("nan"), pd.NaT, "alpha"], index=index, dtype=object
    )
    result = source.thinkthen.decide("Ready?", client=tt.Client(adapter="stub"))
    assert result.index.equals(index)
    assert result["status"].isna().tolist() == [True, True, True, False]


def test_running_column_cancel_closes_held_request(
    library: Any, monkeypatch: pytest.MonkeyPatch
) -> None:
    monkeypatch.setenv("THINKTHEN_API_KEY", "synthetic-loopback-key")
    process = subprocess.Popen(
        [os.environ["THINKTHEN_LOOPBACK"], "--held", "body"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        env=child_environment(),
    )
    assert process.stdin is not None and process.stdout is not None
    url = "http://127.0.0.1:" + process.stdout.readline().strip() + "/v1"
    source = column(library, ["alpha"])
    cancel = tt.Cancel()
    client = tt.Client(url=url, batch=1)
    try:
        with ThreadPoolExecutor(max_workers=1) as executor:
            future = executor.submit(
                source.thinkthen.decide, "Ready?", client=client, cancel=cancel
            )
            process.stdin.write("arrived\n")
            process.stdin.flush()
            assert process.stdout.readline().strip() == "arrived"
            cancel.cancel()
            with pytest.raises(KeyboardInterrupt):
                future.result()
            process.stdin.write("closed\n")
            process.stdin.flush()
            assert process.stdout.readline().strip() == "closed"
    finally:
        cancel.cancel()
        _, error = process.communicate("shutdown\n")
        assert process.returncode == 0, error


@pytest.mark.parametrize(
    "function,domain", [("choose", "options"), ("tag", "labels"), ("score", "levels")]
)
def test_saved_column_questions_keep_their_domains(
    library: Any, function: str, domain: str
) -> None:
    client = tt.Client(adapter="stub")
    source = column(library, ["alpha", None, "beta"])
    question = tt.Toolbox({function: "Which?", domain: ["a", "b"]}).question()
    actual = getattr(source.thinkthen, function)(question, client=client)
    explicit = getattr(source.thinkthen, function)(
        "Which?", client=client, **{domain: ["a", "b"]}
    )
    assert rows(library, actual) == rows(library, explicit)
    assert len(actual) == 3


def test_unordered_relation_columns_keep_the_shared_answer(
    library: Any, monkeypatch: pytest.MonkeyPatch
) -> None:
    path = Path(os.environ["THINKTHEN_CASES"]) / "relate-undirected.json"
    case = json.loads(path.read_text())
    text = case["request"]["records"][0]
    rules = case["request"]["question"]["value"]
    monkeypatch.setenv("THINKTHEN_API_KEY", "synthetic-loopback-key")
    with Loopback(path) as server:
        client = tt.Client(url=server.url)
        expected = client.relate(text, rules=rules)
        assert expected.answer is not None and len(expected.answer) == 1
        assert expected.answer[0]["either"] is True
        result = column(library, [None, text]).thinkthen.relate(
            rules=rules, client=client
        )
        actual = rows(library, result)
        assert len(actual) == 2
        assert actual[0]["answer"] is None
        assert actual[1]["answer"] == expected.answer


@pytest.mark.parametrize("kind", ["List", "Array"])
def test_polars_native_array_cells_keep_evidence_and_source_positions(
    kind: str,
) -> None:
    pl = pytest.importorskip("polars")
    importlib.import_module("thinkthen.polars")
    dtype = pl.List(pl.Int64) if kind == "List" else pl.Array(pl.Int64, 2)
    source = pl.Series("body", [[1, 2], None, [3, 4]], dtype=dtype)
    before = source.clone()
    client = tt.Client(adapter="stub")
    result = source.thinkthen.decide("Ready?", on="/0", client=client)
    actual = rows(pl, result)
    expected = list(client.decide_stream("Ready?", [[1, 2], [3, 4]], on="/0"))
    assert len(actual) == 3
    assert [row["position"] for row in actual] == [0, 1, 2]
    assert actual[1]["status"] is None
    for item, row in zip((actual[0], actual[2]), expected):
        assert item["status"] == row.status and item["answer"] == row.answer
        assert item["detail"] == row.detail
    assert source.equals(before)
