"""Exercise the approved typed surface through installed public entry points."""

from __future__ import annotations

import asyncio
import logging
from collections.abc import AsyncIterator, Iterator
from concurrent.futures import ThreadPoolExecutor
from dataclasses import FrozenInstanceError
from pathlib import Path
from typing import Any, cast

import pytest
from api_support import (
    Held,
    backend,
    client,
    isolated_environment,  # noqa: F401 - shared pytest fixture.
    yes,
)

import thinkthen as tt
from thinkthen import aio

pytestmark = pytest.mark.usefixtures("isolated_environment")


def test_question_overloads_preserve_lists_and_use_the_supplied_client(
    tmp_path: Path,
) -> None:
    choices = ["billing", "shipping"]
    question = tt.ChooseQuestion("Which team?", choices=choices, evidence="/body")
    choices[0] = "changed"
    toolbox = tt.Toolbox({"route": question})
    replies = [
        {
            "answer": {
                "type": "choice",
                "probabilities": {
                    "billing": 0.81234567890123,
                    "shipping": 0.18765432109877,
                },
            }
        }
    ]
    with backend(tmp_path, replies) as server, client(server) as selected:
        row = tt.choose(toolbox.choose("route"), {"body": "Refund"}, client=selected)
        assert isinstance(row, tt.ChooseRow)
        assert row.answer == "billing"
        assert row.status is tt.Status.ANSWERED
        assert row.probability == 0.81234567890123
        assert row.detail is not None
        assert row.detail.labels["shipping"] == 0.18765432109877
        with pytest.raises(TypeError):
            cast(Any, row.detail.labels)["billing"] = 0.0
        with pytest.raises(FrozenInstanceError):
            cast(Any, row).answer = "changed"
        assert row.source.backend is tt.Backend.SYSTEMONE
        assert row.source.question_identity
        assert row.position == 1 and row.id is None
        assert not callable(question) and not hasattr(question, "stream")
        assert server.count() == 1
    assert row.answer == "billing" and row.detail.labels["billing"] == row.probability


@pytest.mark.parametrize("threshold", [tt.Band(0.2, 0.8), (0.2, 0.8)])
def test_python_bands_reach_the_engine_as_numeric_bounds(
    tmp_path: Path, threshold: Any
) -> None:
    with backend(tmp_path, yes(0.55)) as server, client(server) as selected:
        row = selected.decide("Ready?", "body", threshold=threshold)
        assert row.status is tt.Status.UNSURE and row.answer is None
        assert row.probability == 0.55 and row.error is None
        row.raise_for_error()
        assert server.count() == 1


@pytest.mark.parametrize(
    "method,keyword",
    [
        ("choose", "choices"),
        ("tag", "labels"),
        ("score", "levels"),
        ("rank", "levels"),
        ("recognize", "kinds"),
        ("relate", "rules"),
    ],
)
def test_required_lists_cannot_be_omitted_or_replaced_by_strings(
    tmp_path: Path, method: str, keyword: str
) -> None:
    with backend(tmp_path, yes()) as server, client(server) as selected:
        call = getattr(selected, method)
        args = (
            (["body"],)
            if method in {"recognize", "relate"}
            else ("Which?", ["body"] if method == "rank" else "body")
        )
        with pytest.raises(TypeError):
            call(*args)
        with pytest.raises(tt.UsageError):
            call(*args, **{keyword: "not-a-list"})
        assert server.count() == 0


def test_failed_rows_finish_normally_and_explicit_raising_uses_typed_codes(
    tmp_path: Path,
) -> None:
    with (
        backend(tmp_path, [{"status": 400, "body": {}}]) as server,
        client(server) as selected,
    ):
        row = selected.decide("Ready?", "body")
        assert row.status is tt.Status.FAILED and row.answer is None
        assert row.error is not None and row.error.code is tt.ErrorCode.BACKEND_STATUS
        assert row.error.http_status == 400 and row.probability is None
        with pytest.raises(tt.RecordError) as raised:
            row.raise_for_error()
        assert raised.value.code is row.error.code
        assert str(raised.value) == row.error.message
        with selected.decide_stream("Ready?", ["one", "two"]) as stream:
            rows = list(stream)
        assert len(rows) == 2
        assert all(item.status is tt.Status.FAILED for item in rows)
        assert [item.position for item in rows] == [1, 2]
        assert list(stream) == []
        assert server.count() == 3


@pytest.mark.parametrize(
    "identity", [None, True, False, 1.5, {}, [], 2**64, -(2**63) - 1]
)
def test_invalid_record_ids_are_failed_rows_without_backend_calls(
    tmp_path: Path, identity: Any
) -> None:
    with backend(tmp_path, yes()) as server, client(server) as selected:
        row = selected.decide("Ready?", {"id": identity, "body": "text"})
        assert row.status is tt.Status.FAILED and row.error is not None
        assert row.error.code is tt.ErrorCode.RECORD_INVALID
        assert row.position == 1 and row.probability is None
        assert server.count() == 0


@pytest.mark.parametrize("identity", ["id", -(2**63), 2**63 - 1, 2**64 - 1])
def test_record_ids_keep_exact_python_scalar_values(
    tmp_path: Path, identity: str | int
) -> None:
    with backend(tmp_path, yes()) as server, client(server) as selected:
        row = selected.decide("Ready?", {"id": identity, "body": "text"})
        assert row.id == identity and type(row.id) is type(identity)
        assert row.status is tt.Status.ANSWERED and server.count() == 1


def test_missing_and_failed_filter_rows_retain_original_objects(tmp_path: Path) -> None:
    records: list[tt.Record] = [{"id": "yes", "body": "text"}, None, {"id": True}]
    with (
        backend(tmp_path, yes()) as server,
        client(server) as selected,
        selected.filter("Ready?", records) as stream,
    ):
        rows = list(stream)
        assert len(rows) == 3 and [row.position for row in rows] == [1, 2, 3]
        assert all(
            row.item is record for row, record in zip(rows, records, strict=False)
        )
        assert [row.kept for row in rows] == [True, False, False]
        assert rows[1].error is not None
        assert rows[1].error.code is tt.ErrorCode.RECORD_MISSING
        assert rows[2].error is not None
        assert rows[2].error.code is tt.ErrorCode.RECORD_INVALID
        assert server.count() == 1


def test_stream_conversion_closes_only_its_call_and_does_not_replay() -> None:
    with tt.Client(backend="stub") as selected:
        stream = selected.decide_stream("Ready?", ["first", "second", "third"])
        with stream:
            first = next(stream)
            remaining = stream.to_list()
        assert first.position == 1
        assert [row.position for row in remaining] == [2, 3]
        assert stream.to_list() == []
        stream.close()
        assert selected.decide("Ready?", "another").position == 1
    assert first.source.backend is tt.Backend.STUB
    with pytest.raises(tt.SetupError):
        selected.decide("Ready?", "closed")


def test_input_exception_survives_native_cleanup(tmp_path: Path) -> None:
    original = RuntimeError("caller source failed")

    def records() -> Iterator[str]:
        yield "first"
        raise original

    with backend(tmp_path, yes()) as server, client(server) as selected:
        with (
            selected.decide_stream("Ready?", records()) as stream,
            pytest.raises(RuntimeError) as raised,
        ):
            list(stream)
        assert raised.value is original
        assert selected.decide("Ready?", "still open").status is tt.Status.ANSWERED


def test_native_cancellation_closes_held_transport_and_fuses_iterator() -> None:
    cancel = tt.Cancel()
    with (
        Held() as server,
        tt.Client(url=server.url, api_key="synthetic-loopback-key") as selected,
        selected.decide_stream("Ready?", ["body"], cancel=cancel) as stream,
        ThreadPoolExecutor(max_workers=1) as worker,
    ):
        receiving = worker.submit(next, stream)
        server.event("arrived")
        cancel.cancel()
        with pytest.raises(tt.Cancelled) as raised:
            receiving.result(timeout=10)
        assert raised.value.code is tt.ErrorCode.LOCAL_CANCELLED
        server.event("closed")
        assert list(stream) == []


def test_pre_cancelled_call_raises_library_error_without_requests(
    tmp_path: Path,
) -> None:
    cancel = tt.Cancel()
    cancel.cancel()
    with backend(tmp_path, yes()) as server, client(server) as selected:
        with pytest.raises(tt.Cancelled) as raised:
            selected.decide("Ready?", "body", cancel=cancel)
        assert raised.value.code is tt.ErrorCode.LOCAL_CANCELLED
        assert server.count() == 0


def test_async_input_conversion_and_explicit_client_routing() -> None:
    async def scenario() -> None:
        async def records() -> AsyncIterator[tt.Record]:
            for record in ("first", None, "third"):
                yield record

        async with tt.AsyncClient(backend="stub") as selected:
            row = await aio.decide("Ready?", "one", client=selected)
            assert isinstance(row, tt.DecideRow)
            async with selected.decide_stream("Ready?", records()) as stream:
                rows = await stream.to_list()
            assert [item.position for item in rows] == [1, 2, 3]
            assert rows[1].error is not None
            assert rows[1].error.code is tt.ErrorCode.RECORD_MISSING
            assert [item async for item in stream] == []
            assert (await selected.decide("Ready?", "again")).position == 1
            await aio.aclose_default_client()
        assert row.source.backend is tt.Backend.STUB
        with pytest.raises(tt.SetupError):
            await selected.decide("Ready?", "closed")

    asyncio.run(scenario())


def test_async_task_cancellation_closes_held_transport() -> None:
    async def scenario(server: Held) -> None:
        async with tt.AsyncClient(
            url=server.url, api_key="synthetic-loopback-key"
        ) as selected:
            task = asyncio.create_task(selected.decide("Ready?", "body"))
            await asyncio.to_thread(server.event, "arrived")
            task.cancel()
            with pytest.raises(asyncio.CancelledError):
                await task
            await asyncio.to_thread(server.event, "closed")

    with Held() as server:
        asyncio.run(scenario(server))


def test_key_redaction_covers_settings_repr_logs_and_usage_errors(
    tmp_path: Path, caplog: pytest.LogCaptureFixture
) -> None:
    secret = "synthetic-key-do-not-render"
    with (
        backend(tmp_path, yes()) as server,
        tt.Client(url=server.url, api_key=secret) as selected,
        caplog.at_level(logging.DEBUG, logger="thinkthen"),
    ):
        row = selected.decide("Ready?", "body")
        with pytest.raises(tt.UsageError) as raised:
            selected.decide("Ready?", "body", threshold=cast(Any, "bad"))
        shown = repr(selected) + repr(row) + str(raised.value) + caplog.text
        assert secret not in shown
        assert "Authorization" not in shown
        assert selected.decide("Ready?", "another").status is tt.Status.ANSWERED


@pytest.mark.parametrize(
    "function,question,fields,row_type",
    [
        ("decide", "Ready?", {}, "DecideRow"),
        ("filter", "Ready?", {}, "FilterRow"),
        ("choose", "Which?", {"choices": ["a", "b"]}, "ChooseRow"),
        ("tag", "Which?", {"labels": ["a", "b"]}, "TagRow"),
        ("score", "How much?", {"levels": ["low", "high"]}, "ScoreRow"),
        ("rank", "How much?", {"levels": ["low", "high"]}, "RankRow"),
        ("find", "Which line?", {}, "FindRow"),
        ("recognize", None, {"kinds": ["person"]}, "RecognizeRow"),
        ("relate", None, None, "RelateRow"),
        ("annotate", None, None, "AnnotateRow"),
    ],
)
def test_all_ten_functions_return_their_specific_typed_rows(
    function: str,
    question: str | None,
    fields: dict[str, Any] | None,
    row_type: str,
) -> None:
    with tt.Client(backend="stub") as selected:
        options = {} if fields is None else fields
        if function == "relate":
            options = {
                "rules": [
                    tt.RelationRule(source="person", target="person", relation="knows")
                ]
            }
        if function == "annotate":
            args: list[Any] = [
                tt.AnnotationQuestions({"ready": tt.DecideQuestion("Ready?")})
            ]
        else:
            args = [] if question is None else [question]
        if function in {"filter", "rank"}:
            with getattr(selected, function)(
                *args, ["Ada knows Bob."], **options
            ) as stream:
                rows = list(stream)
            assert len(rows) == 1
            row = rows[0]
        else:
            row = getattr(selected, function)(*args, "Ada knows Bob.", **options)
        assert isinstance(row, getattr(tt, row_type))
        assert row.function.value == function and row.position == 1
        assert isinstance(row.status, tt.Status)
        assert row.source.backend is tt.Backend.STUB
        if function in {"tag", "score", "rank", "recognize", "relate", "annotate"}:
            assert row.probability is None
        if function == "annotate":
            assert row.answer is not None and "ready" in row.answer
            child = row.answer["ready"]
            assert child.function is tt.Function.DECIDE
            assert isinstance(child.status, tt.Status)
            assert not hasattr(child, "position")


def test_level_detail_is_typed_and_survives_client_close(tmp_path: Path) -> None:
    replies = [{"answer": {"type": "score", "probabilities": {"0": 0.25, "1": 0.75}}}]
    with backend(tmp_path, replies) as server, client(server) as selected:
        row = selected.score("How urgent?", "body", levels=["low", "high"])
        assert isinstance(row, tt.ScoreRow) and row.answer == 0.75
        assert row.probability is None and isinstance(row.detail, tt.LevelDetail)
        assert row.detail.levels == (0.25, 0.75)
        assert row.source.backend is tt.Backend.SYSTEMONE
        assert server.count() == 1
        assert row.detail.levels == (0.25, 0.75)
        with pytest.raises(FrozenInstanceError):
            cast(Any, row.detail).levels = (1.0, 0.0)


def test_named_execution_does_not_come_from_toolbox_selection() -> None:
    with tt.Client(backend="stub") as selected:
        question = tt.Toolbox({"selected": tt.DecideQuestion("Ready?")}).decide(
            "selected"
        )
        unnamed = selected.decide(question, "body")
        named = selected.decide(question, "body", name="execution")
        assert unnamed.source.name is None
        assert named.source.name == "execution"
        direct = selected.decide("Ready?", "body")
        assert unnamed.source.question_identity == direct.source.question_identity
        assert unnamed.source.question_identity != named.source.question_identity


def test_resolved_settings_have_seconds_and_no_credentials(tmp_path: Path) -> None:
    secret = "synthetic-never-display-key"
    with (
        backend(tmp_path, yes()) as server,
        tt.Client.from_env(
            url=server.url,
            api_key=secret,
            model="selected-model",
            batch=2,
            retry=tt.RetryPolicy(
                max_retries=0, initial_backoff=0.125, attempt_timeout=2.5
            ),
        ) as selected,
    ):
        settings = selected.settings
        assert settings.backend is tt.Backend.SYSTEMONE
        assert settings.model == "selected-model" and settings.url == server.url
        assert settings.batch == 2
        assert settings.retry == tt.RetryPolicy(0, 0.125, 2.5)
        assert not hasattr(settings, "api_key") and secret not in repr(settings)
        assert selected.decide("Ready?", "body").status is tt.Status.ANSWERED


@pytest.mark.parametrize(
    "duration", [True, float("nan"), float("inf"), -1, 0.0001, 10**400, 2**53 + 1]
)
def test_retry_conversion_rejects_unrepresentable_seconds(duration: Any) -> None:
    with pytest.raises(tt.UsageError) as raised:
        tt.Client(backend="stub", retry=tt.RetryPolicy(attempt_timeout=duration))
    assert raised.value.code is tt.ErrorCode.USAGE_SETTINGS


def test_error_code_enums_match_native_values_and_integers() -> None:
    from thinkthen import _native
    from thinkthen._contract import CODE_NAMES

    assert dict(_native.ERROR_CODES) == CODE_NAMES
    assert {item.value for item in tt.ErrorCode} == set(CODE_NAMES.values())
    assert {item.name for item in tt.ErrorCode} == {
        value.upper().replace(".", "_").replace("-", "_")
        for value in CODE_NAMES.values()
    }


def test_async_close_preserves_shared_token_and_stops_pending_input() -> None:
    async def scenario() -> None:
        token = tt.Cancel()
        entered = asyncio.Event()
        waiting = asyncio.Event()

        async def inputs() -> AsyncIterator[tt.Record]:
            entered.set()
            await waiting.wait()
            yield "never supplied"

        async with tt.AsyncClient(backend="stub") as selected:
            async with selected.decide_stream(
                "Ready?", ["first", "second"], cancel=token
            ) as rows:
                await anext(rows)
            assert (
                await selected.decide("Ready?", "again", cancel=token)
            ).position == 1
            stream = selected.decide_stream("Ready?", inputs(), cancel=token)
            task = asyncio.create_task(anext(stream))
            await entered.wait()
            task.cancel()
            with pytest.raises(asyncio.CancelledError):
                await task
            await stream.aclose()
            assert (
                await selected.decide("Ready?", "still open", cancel=token)
            ).position == 1

    asyncio.run(scenario())


def test_client_close_cancels_its_active_call_and_is_idempotent() -> None:
    with Held() as server, ThreadPoolExecutor(max_workers=1) as worker:
        selected = tt.Client(url=server.url, api_key="synthetic-loopback-key")
        rows = selected.decide_stream("Ready?", ["body"])
        receiving = worker.submit(next, rows)
        server.event("arrived")
        selected.close()
        with pytest.raises(tt.Cancelled):
            receiving.result(timeout=10)
        server.event("closed")
        assert list(rows) == []
        selected.close()
        with pytest.raises(tt.SetupError) as raised:
            selected.decide("Ready?", "closed")
        assert raised.value.code is tt.ErrorCode.USAGE_ABI


@pytest.mark.parametrize(
    "operation",
    ["decide", "choose", "find", "filter", "rank", "recognize", "relate", "annotate"],
)
def test_omitted_records_are_call_errors(operation: str) -> None:
    with tt.Client(backend="stub") as selected, pytest.raises(TypeError):
        getattr(selected, operation)()


@pytest.mark.parametrize("asynchronous", [False, True])
def test_kept_items_consumes_remaining_filter_rows_without_replay(
    tmp_path: Path, asynchronous: bool
) -> None:
    records: list[tt.Record] = [{"body": "consumed"}, None, {"body": "kept"}]
    with backend(tmp_path, yes()) as server:
        if asynchronous:

            async def scenario() -> None:
                async with (
                    tt.AsyncClient(
                        url=server.url, api_key="synthetic-loopback-key", batch=1
                    ) as selected,
                    selected.filter("Ready?", records) as rows,
                ):
                    await anext(rows)
                    kept = [item async for item in rows.kept_items()]
                    assert len(kept) == 1 and kept[0] is records[2]
                    assert await rows.to_list() == []

            asyncio.run(scenario())
        else:
            with (
                client(server) as selected,
                selected.filter("Ready?", records) as rows,
            ):
                next(rows)
                kept = list(rows.kept_items())
                assert len(kept) == 1 and kept[0] is records[2]
                assert rows.to_list() == []
        assert server.count() == 2
