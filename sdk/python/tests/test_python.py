"""Check the existing facade against the canonical request and result contract."""

from __future__ import annotations

import gc
import json
import os
import signal
import threading
import weakref
from collections.abc import Iterator
from concurrent.futures import ThreadPoolExecutor
from contextlib import suppress
from pathlib import Path
from typing import Any

import pytest
from conformance import Loopback, row_json

import thinkthen as tt


@pytest.fixture
def cases() -> Path:
    return Path(os.environ["THINKTHEN_CASES"]) / "typed"


@pytest.mark.parametrize("backend", ["systemone", "openai"])
def test_single_and_stream_calls_return_failed_rows_without_terminal_error(
    backend: str, cases: Path
) -> None:
    with Loopback(cases / "backend-isolation.json") as server:
        client = tt.Client(
            backend=tt.Backend(backend), url=server.url, api_key="synthetic", batch=1
        )
        single = client.decide("Ready?", "bad")
        assert single.status == "failed" and single.answer is None
        assert single.error is not None
        assert single.error.code == "backend.status" and single.error.http_status == 400
        with client.decide_stream("Ready?", ["good", "bad", "good again"]) as stream:
            rows = list(stream)
        assert [row.status for row in rows] == ["answered", "failed", "answered"]
        assert [row.position for row in rows] == [1, 2, 3]
        assert list(stream) == [] and server.count() == 4


def test_explicit_client_and_toolbox_questions_use_the_same_admitted_contract(
    cases: Path,
) -> None:
    question = tt.Toolbox(
        {"route": tt.ChooseQuestion("Which?", choices=["z", "a"])}
    ).choose("route")
    with Loopback(cases / "choose-tie.json") as server:
        client = tt.Client(url=server.url, api_key="synthetic")
        selected = client.choose(question, "evidence")
        inline = client.choose("Which?", "evidence", choices=["z", "a"])
        assert row_json(selected) == row_json(inline)
        assert selected.answer == "z" and selected.probability == 0.5
        assert selected.detail is not None and dict(selected.detail.labels) == {
            "z": 0.5,
            "a": 0.5,
        }
        assert server.count() == 2


def test_missing_records_and_invalid_ids_preserve_position_without_calls(
    cases: Path,
) -> None:
    with Loopback(cases / "record-admission.json") as server:
        client = tt.Client(url=server.url, api_key="synthetic")
        with client.decide_stream(
            "Ready?", [None, {"id": True}, {"id": 2**64}]
        ) as stream:
            rows = list(stream)
        assert [row.position for row in rows] == [1, 2, 3]
        assert [row.error.code if row.error else None for row in rows] == [
            "record.missing",
            "record.invalid",
            "record.invalid",
        ]
        assert all(row.source is not None for row in rows)
        assert server.count() == 0


def test_filter_yields_false_and_failed_rows_as_well_as_selected_rows(
    cases: Path,
) -> None:
    with Loopback(cases / "filter-rows.json") as server:
        client = tt.Client(url=server.url, api_key="synthetic")
        with client.filter("Ready?", ["refund", "ordinary", None]) as stream:
            rows = list(stream)
        assert len(rows) == 3 and [row.position for row in rows] == [1, 2, 3]
        assert [row.answer for row in rows] == [True, False, None]
        assert rows[2].error is not None and rows[2].error.code == "record.missing"


def test_numeric_band_strings_are_rejected_by_the_shared_admission(cases: Path) -> None:
    with Loopback(cases / "decide-unsure.json") as server:
        client = tt.Client(url=server.url, api_key="synthetic")
        row = client.decide("Ready?", "evidence", threshold=(0.2, 0.8))
        assert row.status == "unsure"
        invalid: Any = "0.2:0.8"
        with pytest.raises(tt.UsageError) as failure:
            client.decide("Ready?", "evidence", threshold=invalid)
        assert failure.value.code == "usage.threshold"
        assert server.count() == 1


def test_stream_close_keeps_rows_alive_and_preserves_input_exception() -> None:
    original = RuntimeError("source stopped")

    def records() -> Any:
        yield "first"
        raise original

    client = tt.Client(backend="stub")
    with client.decide_stream("Ready?", ["first", "second"]) as stream:
        row = next(stream)
    assert list(stream) == [] and row.position == 1 and row.source is not None
    with (
        client.decide_stream("Ready?", records()) as stream,
        pytest.raises(RuntimeError) as failure,
    ):
        list(stream)
    assert failure.value is original


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


def canned(tmp_path: Path, case: dict[str, Any]) -> Loopback:
    path = tmp_path / "case.json"
    path.write_text(json.dumps(case))
    return Loopback(path)


def yes_server(tmp_path: Path) -> Loopback:
    return canned(tmp_path, {"replies": [{"answer": {"type": "noul", "noul": 0.9}}]})


@pytest.mark.parametrize("finite", [True, False])
def test_cancelled_handles_raise_keyboard_interrupt_without_requests(
    tmp_path: Path,
    finite: bool,
) -> None:
    cancel = tt.Cancel()
    cancel.cancel()
    with yes_server(tmp_path) as server:
        client = tt.Client(url=server.url)
        with pytest.raises(tt.Cancelled):
            client.decide("Ready?", "text", cancel=cancel)
        stream = None
        records = ["text"] if finite else iter(["text"])
        with pytest.raises(tt.Cancelled):
            stream = client.decide_stream("Ready?", records, cancel=cancel)
            with stream:
                next(stream)
        if stream is not None:
            assert list(stream) == []
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


def test_running_cancel_reports_terminal_error_after_published_rows(
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
            with pytest.raises(tt.Cancelled):
                list(rows)
            assert list(rows) == []


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
        with client.decide_stream("Ready?", inputs(), evidence="/body") as rows:
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
                pytest.raises(tt.Cancelled),
                client.decide_stream("Ready?", inputs()) as rows,
            ):
                list(rows)
        finally:
            release.set()
            worker.join()


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
            with suppress(tt.Cancelled):
                assert receiving.result().answer is True
            closed.result()
        assert list(rows) == []


def test_stream_close_preserves_shared_cancel() -> None:
    client = tt.Client(backend="stub", batch=1)
    cancel = tt.Cancel()
    with client.decide_stream("Ready?", ["first", "second"], cancel=cancel) as rows:
        next(rows)
    assert client.decide("Ready?", "third", cancel=cancel).status == "answered"
