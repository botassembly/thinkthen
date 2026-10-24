from __future__ import annotations

import gc
import json
import os
import signal
import subprocess
import sys
import threading
import weakref
from collections.abc import Iterator
from concurrent.futures import ThreadPoolExecutor
from dataclasses import FrozenInstanceError
from pathlib import Path
from typing import Any, cast

import pytest
from conformance import Loopback, child_environment, row_json

import thinkthen as tt


@pytest.fixture(autouse=True)
def environment(monkeypatch: pytest.MonkeyPatch) -> None:
    for name in (
        "THINKTHEN_API_KEY",
        "OPENAI_API_KEY",
        "THINKTHEN_BASE_URL",
        "THINKTHEN_BATCH",
    ):
        monkeypatch.delenv(name, raising=False)
    monkeypatch.setenv("THINKTHEN_API_KEY", "synthetic-loopback-key")


@pytest.fixture
def cases() -> Path:
    return Path(os.environ["THINKTHEN_CASES"])


def canned(tmp_path: Path, case: dict[str, Any]) -> Loopback:
    path = tmp_path / "case.json"
    path.write_text(json.dumps(case))
    return Loopback(path)


def yes_server(tmp_path: Path) -> Loopback:
    return canned(tmp_path, {"replies": [{"answer": {"type": "noul", "noul": 0.9}}]})


@pytest.mark.parametrize(
    "function,filename",
    [
        ("decide", "decide-yes.json"),
        ("choose", "choose-tie.json"),
        ("tag", "tag-batching.json"),
        ("score", "score-weighted.json"),
        ("find", "find-unicode.json"),
        ("recognize", "recognize-name.json"),
        ("relate", "relate-rounds.json"),
        ("annotate", "annotate-isolation.json"),
    ],
)
def test_single_record_argument_mapping(
    function: str,
    filename: str,
    cases: Path,
    tmp_path: Path,
) -> None:
    case = cast(dict[str, Any], json.loads((cases / filename).read_text()))
    request = case["request"]
    question = request["question"]["value"]
    text = request["records"][0]
    with Loopback(cases / filename) as server:
        arguments: list[object]
        options: dict[str, Any]
        if function == "annotate":
            toolbox = tmp_path / "questions.json"
            toolbox.write_text(json.dumps(question))
            arguments, options = [question, text], {}
        elif function == "recognize":
            arguments, options = [text], {"kinds": question}
        elif function == "relate":
            arguments, options = [text], {"rules": question}
        else:
            options = request.get("options", {})
            arguments = [question, text]
        completed = subprocess.run(
            [
                sys.executable,
                "-c",
                (
                    "import dataclasses, json, sys, thinkthen; "
                    "call = json.load(sys.stdin); "
                    "row = getattr(thinkthen, call['function'])(*call['args'], **call['kwargs']); "
                    "print(json.dumps(dataclasses.asdict(row)))"
                ),
            ],
            input=json.dumps(
                {"function": function, "args": arguments, "kwargs": options}
            ),
            text=True,
            capture_output=True,
            check=True,
            env=child_environment()
            | {
                "THINKTHEN_BASE_URL": server.url,
                "THINKTHEN_API_KEY": "synthetic-loopback-key",
            },
        )
        row = tt.Row[object](**json.loads(completed.stdout))
        assert row_json(row) == case["expected"][0]


def test_row_is_frozen_and_detail_uses_plain_python_values() -> None:
    detail: dict[str, tt.JSONValue] = {"yes": 0.9}
    row = tt.Row[bool](id=1, status="answered", answer=True, detail=detail)
    with pytest.raises(FrozenInstanceError):
        field_name = "answer"
        setattr(row, field_name, False)
    detail["yes"] = 0.8
    assert row.detail == {"yes": 0.8}


def test_json_values_stay_single_records_and_stream_order_is_preserved(
    tmp_path: Path,
) -> None:
    records: list[tt.Record] = [
        '{"id":"text"}',
        ["array"],
        7,
        False,
        None,
        {"id": ["custom"], "body": "x"},
    ]
    with yes_server(tmp_path) as server:
        client = tt.Client(url=server.url)
        for record in records:
            assert client.decide("Ready?", record).answer is True
        with client.decide_stream("Ready?", iter(records)) as rows:
            actual = list(rows)
        assert [row.id for row in actual] == [1, 2, 3, 4, 5, ["custom"]]
        assert len(actual) == len(records)
        assert all(row.answer is True for row in actual)


def test_errors_use_engine_codes_and_streams_keep_failed_rows(cases: Path) -> None:
    with Loopback(cases / "backend-reply.json") as server:
        client = tt.Client(url=server.url)
        with pytest.raises(tt.BackendError) as failure:
            client.decide("Ready?", "text")
        assert failure.value.code == "backend.reply"
        with client.decide_stream("Ready?", ["text"]) as rows:
            failed = next(rows)
            assert failed.error is not None
            assert failed.error.code == failure.value.code
            assert failed.error.message == str(failure.value)
            with pytest.raises(tt.BackendError) as terminal:
                next(rows)
            assert terminal.value.code == failure.value.code
        with pytest.raises(tt.UsageError) as usage:
            client.decide("Ready?", "text", threshold="wrong")
        assert usage.value.code == "usage.threshold"
        with pytest.raises(tt.UsageError) as local:
            client.annotate({}, "text")
        assert local.value.code == "usage.question"
        with pytest.raises(tt.UsageError) as streamed:
            client.annotate_stream({}, ["text"])
        assert streamed.value.code == "usage.question"
        with pytest.raises(tt.LocalError) as explicit:
            tt.Toolbox({})
        assert explicit.value.code == "local.toolbox"
        with pytest.raises(TypeError):
            method_name = "decide"
            getattr(client, method_name)("Ready?", "text", wrong=True)


def test_filter_keeps_identity_and_finishes_before_raising_first_failure(
    tmp_path: Path,
) -> None:
    case = {
        "replies": [
            {"contains": '"bad"', "status": 400, "body": {}},
            {"contains": '"no"', "answer": {"type": "noul", "noul": 0.1}},
            {"answer": {"type": "noul", "noul": 0.9}},
        ]
    }
    records: list[tt.Record] = [
        {"id": "one", "body": "yes"},
        {"id": "bad", "body": "bad"},
        {"id": "no", "body": "no"},
        {"id": "two", "body": "yes"},
    ]
    consumed: list[int] = []

    def inputs() -> Iterator[tt.Record]:
        for index, record in enumerate(records):
            consumed.append(index)
            yield record

    with canned(tmp_path, case) as server:
        client = tt.Client(url=server.url, batch=1)
        with client.filter("Ready?", inputs(), on="/body") as kept:
            assert next(kept) is records[0]
            assert next(kept) is records[3]
            with pytest.raises(tt.BackendError) as failed:
                next(kept)
            assert failed.value.code == "backend.status"
        assert consumed == list(range(len(records)))
        assert server.count() == len(records)


def test_client_reuses_settings_and_toolboxes_hold_loaded_questions(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    with yes_server(tmp_path) as server:
        monkeypatch.setenv("THINKTHEN_BASE_URL", server.url)
        client = tt.Client(url=server.url)
        assert tt.decide("Ready?", "first").answer is True
        toolbox_path = tmp_path / "questions.json"
        toolbox_path.write_text('{"decide":"Ready?"}')
        toolbox = tt.Toolbox({"decide": "Ready?"})
        question = toolbox.question()
        toolbox_path.write_text("invalid")
        monkeypatch.delenv("THINKTHEN_API_KEY")
        monkeypatch.setenv("THINKTHEN_BASE_URL", "http://127.0.0.1:1/v1")
        assert tt.decide("Ready?", "second").answer is True
        assert client.decide("Ready?", "third").answer is True
        assert question("fourth").answer is True
        with question.stream(["fifth"]) as rows:
            assert next(rows).answer is True
        annotated = client.annotate(toolbox, "sixth")
        assert annotated.status == "answered"
        assert server.count() == 6


def test_cancelled_handles_raise_keyboard_interrupt_without_requests(
    tmp_path: Path,
) -> None:
    cancel = tt.Cancel()
    cancel.cancel()
    with yes_server(tmp_path) as server:
        client = tt.Client(url=server.url)
        with pytest.raises(KeyboardInterrupt):
            client.decide("Ready?", "text", cancel=cancel)
        with pytest.raises(KeyboardInterrupt):
            client.decide_stream("Ready?", ["text"], cancel=cancel)
        assert server.count() == 0


def test_iteration_releases_python_lock_and_propagates_original_exception(
    tmp_path: Path,
) -> None:
    entered = threading.Event()
    release = threading.Event()
    original = RuntimeError("source stopped")

    def inputs() -> Iterator[tt.Record]:
        entered.set()
        release.wait()
        yield "first"
        raise original

    with yes_server(tmp_path) as server:
        client = tt.Client(url=server.url, batch=1)
        with (
            client.decide_stream("Ready?", inputs()) as rows,
            ThreadPoolExecutor(max_workers=1) as executor,
        ):
            result = executor.submit(list, rows)
            entered.wait()
            release.set()
            with pytest.raises(RuntimeError) as failure:
                result.result()
            assert failure.value is original


def test_running_cancel_stops_yields_and_raises_keyboard_interrupt(
    tmp_path: Path,
) -> None:
    cancel = tt.Cancel()
    with yes_server(tmp_path) as server:
        client = tt.Client(url=server.url, batch=1)
        with client.decide_stream(
            "Ready?", ("text" for _ in range(10_000)), cancel=cancel
        ) as rows:
            assert next(rows).answer is True
            cancel.cancel()
            with pytest.raises(KeyboardInterrupt):
                next(rows)
            assert list(rows) == []


def test_rank_delivers_rows_before_raising_its_operation_failure(
    tmp_path: Path,
) -> None:
    case = {
        "replies": [
            {"contains": '"bad"', "status": 400, "body": {}},
            {"answer": {"type": "noul", "noul": 0.9}},
        ]
    }
    with canned(tmp_path, case) as server:
        client = tt.Client(url=server.url, batch=1)
        with client.rank("Ready?", ["yes", "bad"], top=1) as rows:
            first = next(rows)
            assert first.id == 1
            assert first.answer == 1
            with pytest.raises(tt.BackendError) as failure:
                next(rows)
            assert failure.value.code == "backend.status"
            assert list(rows) == []
        assert server.count() == 2


def test_stream_close_bounds_input_and_releases_python_objects(tmp_path: Path) -> None:
    class RecordDict(dict[str, tt.JSONValue]):
        pass

    consumed: list[weakref.ReferenceType[RecordDict]] = []

    def inputs() -> Iterator[tt.Record]:
        for index in range(10_000):
            record = RecordDict(id=index, body="yes")
            consumed.append(weakref.ref(record))
            yield record

    with yes_server(tmp_path) as server:
        client = tt.Client(url=server.url, batch=1)
        with client.decide_stream("Ready?", inputs(), on="/body") as rows:
            assert next(rows).answer is True
            assert len(consumed) < 10_000
        gc.collect()
        assert all(reference() is None for reference in consumed)
        assert list(rows) == []


def test_ctrl_c_raises_keyboard_interrupt_and_cleans_up(tmp_path: Path) -> None:
    entered = threading.Event()
    release = threading.Event()

    def inputs() -> Iterator[tt.Record]:
        entered.set()
        release.wait()
        yield "text"

    def interrupt() -> None:
        entered.wait()
        os.kill(os.getpid(), signal.SIGINT)
        release.set()

    with yes_server(tmp_path) as server:
        client = tt.Client(url=server.url)
        worker = threading.Thread(target=interrupt)
        worker.start()
        try:
            with (
                pytest.raises(KeyboardInterrupt),
                client.decide_stream("Ready?", inputs()) as rows,
            ):
                list(rows)
        finally:
            release.set()
            worker.join()


def test_selected_question_uses_explicit_client_without_reading_files() -> None:
    client = tt.Client(adapter="stub")
    question = tt.Toolbox(
        {"questions": {"ready": {"decide": "literal.json"}}}
    ).question("ready")
    plain = client.decide("literal.json", "evidence")
    named = client.decide(question, "evidence")
    assert named == plain


def test_close_from_another_thread_releases_the_python_lock(tmp_path: Path) -> None:
    entered = threading.Event()
    closing = threading.Event()
    release = threading.Event()

    def inputs() -> Iterator[tt.Record]:
        entered.set()
        release.wait()
        yield "text"

    with yes_server(tmp_path) as server:
        rows = tt.Client(url=server.url, batch=1).decide_stream("Ready?", inputs())

        def close() -> None:
            closing.set()
            rows.close()

        with ThreadPoolExecutor(max_workers=2) as executor:
            receiving = executor.submit(next, rows)
            entered.wait()
            closed = executor.submit(close)
            closing.wait()
            release.set()
            assert receiving.result().answer is True
            closed.result()
        assert list(rows) == []


def test_row_detail_decodes_only_when_read(monkeypatch: pytest.MonkeyPatch) -> None:
    original = json.loads
    decoded = 0

    def loads(text: str, **options: Any) -> Any:
        nonlocal decoded
        value = original(text, **options)
        if isinstance(value, dict) and "source" in value:
            decoded += 1
        return value

    monkeypatch.setattr(json, "loads", loads)
    row = tt.Client(adapter="stub").decide("Ready?", "text")
    assert decoded == 0
    detail = row.detail
    assert isinstance(detail, dict) and "source" in detail
    assert decoded == 1
    assert row.detail is detail
    assert decoded == 1


def test_rank_top_releases_discarded_host_records_while_reading() -> None:
    class RecordDict(dict[str, tt.JSONValue]):
        pass

    consumed: list[weakref.ReferenceType[RecordDict]] = []

    def inputs() -> Iterator[tt.Record]:
        for index in range(20):
            assert all(reference() is None for reference in consumed[:-2])
            record = RecordDict(id=index, body=f"record {index}")
            consumed.append(weakref.ref(record))
            yield record

    with tt.Client(adapter="stub").rank("Ready?", inputs(), top=1) as stream:
        rows = list(stream)
    assert len(rows) == 1 and rows[0].status == "answered"
    assert len(consumed) == 20
    assert all(reference() is None for reference in consumed)


@pytest.mark.parametrize(
    "function,domain", [("choose", "options"), ("tag", "labels"), ("score", "levels")]
)
def test_saved_questions_keep_domains_in_clients_and_streams(
    function: str, domain: str
) -> None:
    client = tt.Client(adapter="stub")
    question = tt.Toolbox({function: "Which?", domain: ["a", "b"]}).question()
    expected = getattr(client, function)("Which?", "alpha", **{domain: ["a", "b"]})
    assert getattr(client, function)(question, "alpha") == expected
    with getattr(client, function + "_stream")(question, ["alpha"]) as stream:
        assert list(stream) == [expected]
    with pytest.raises(tt.UsageError) as missing:
        getattr(client, function)("Which?", "alpha")
    assert missing.value.code == "usage.question"


def test_saved_score_callable_omits_absent_threshold(tmp_path: Path) -> None:
    # A subprocess gives the question's default client its own loopback settings.
    with canned(
        tmp_path,
        {"replies": [{"answer": {"type": "score", "probabilities": {"0": 1, "1": 0}}}]},
    ) as server:
        completed = subprocess.run(
            [
                sys.executable,
                "-c",
                (
                    "import thinkthen as tt; "
                    "q = tt.Toolbox({'score': 'How much?', 'levels': ['low', 'high']}).question(); "
                    "assert q('alpha').answer == 0; "
                    "assert tt.score(q, 'alpha').answer == 0; "
                    "assert [row.answer for row in tt.score_stream(q, ['alpha'])] == [0]; "
                    "assert [row.answer for row in q.stream(['alpha'])] == [0]"
                ),
            ],
            check=False,
            capture_output=True,
            text=True,
            env=child_environment()
            | {
                "THINKTHEN_BASE_URL": server.url,
                "THINKTHEN_API_KEY": "synthetic-loopback-key",
            },
        )
        assert completed.returncode == 0, completed.stderr
        assert server.count() == 4


def test_unknown_error_family_uses_base_error() -> None:
    from thinkthen import _native

    def records() -> Iterator[tt.Record]:
        raise _native.Error("future", "future.failure", "future message")
        yield "unreachable"

    with (
        tt.Client(adapter="stub").decide_stream("Ready?", records()) as rows,
        pytest.raises(tt.ThinkThenError) as failure,
    ):
        next(rows)
    assert type(failure.value) is tt.ThinkThenError
    assert failure.value.code == "future.failure"
    assert str(failure.value) == "future message"


def test_stream_close_preserves_shared_cancel() -> None:
    client = tt.Client(adapter="stub", batch=1)
    cancel = tt.Cancel()
    with client.decide_stream("Ready?", ["first", "second"], cancel=cancel) as rows:
        next(rows)
    assert client.decide("Ready?", "third", cancel=cancel).status == "answered"
