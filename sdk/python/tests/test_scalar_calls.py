"""Installed scalar boundaries reject own failures and keep partial forms readable."""

from __future__ import annotations

import asyncio
from pathlib import Path
from typing import Any

import pytest
from api_support import async_client, backend, client, isolated_environment  # noqa: F401

import thinkthen as tt

pytestmark = pytest.mark.usefixtures("isolated_environment")

FUNCTIONS = (
    "decide",
    "choose",
    "tag",
    "score",
    "find",
    "recognize",
    "relate",
    "annotate",
)


def arguments(function: str, record: Any) -> tuple[tuple[Any, ...], dict[str, Any]]:
    if function == "annotate":
        return (
            tt.AnnotationQuestions({"ready": tt.DecideQuestion("Ready?")}),
            record,
        ), {}
    if function == "recognize":
        return (record,), {"kinds": ["person", "organization"]}
    if function == "relate":
        return (record,), {
            "rules": [tt.RelationRule("person", "organization", "works_for")]
        }
    options: dict[str, Any] = {
        "choose": {"choices": ["first", "second"]},
        "tag": {"labels": ["first", "second"]},
        "score": {"levels": ["low", "high"]},
    }.get(function, {})
    return ("Ready?", record), options


@pytest.mark.parametrize("function", FUNCTIONS)
@pytest.mark.parametrize("asynchronous", [False, True])
def test_every_scalar_blank_failure_is_at_call_with_payload_and_no_request(
    tmp_path: Path, function: str, asynchronous: bool
) -> None:
    with backend(tmp_path, [{"probability": 0.9}]) as server:
        args, options = arguments(function, " ")

        async def run_async() -> tt.Failed:
            async with async_client(server) as selected:
                with pytest.raises(tt.Failed) as failure:
                    await getattr(selected, function)(*args, **options)
                assert server.count() == 0
                # A subsequent successful call proves the failed operation closed cleanly.
                assert (await selected.decide("Ready?", "body")).answered
                return failure.value

        if asynchronous:
            failure = asyncio.run(run_async())
        else:
            with client(server) as selected:
                with pytest.raises(tt.Failed) as raised:
                    getattr(selected, function)(*args, **options)
                failure = raised.value
                assert server.count() == 0
                assert selected.decide("Ready?", "body").answered
        assert failure.code is tt.ErrorCode.RECORD_EMPTY
        assert failure.http_status is None
        assert isinstance(failure, tt.NoAnswer)
        assert isinstance(failure, tt.ThinkThenError)
        assert len(failure.rows) == 1
        assert failure.rows[0].failed
        assert server.count() == 1
        assert not hasattr(tt, "RecordError")


@pytest.mark.parametrize("function", FUNCTIONS[:-1])
@pytest.mark.parametrize("asynchronous", [False, True])
def test_service_failure_is_raised_before_any_scalar_answer_read(
    tmp_path: Path, function: str, asynchronous: bool
) -> None:
    with backend(
        tmp_path, [{"status": 503, "body": {"error": "unavailable"}}]
    ) as server:
        args, options = arguments(function, "body")

        async def run_async() -> tt.Failed:
            async with async_client(server) as selected:
                with pytest.raises(tt.Failed) as failure:
                    await getattr(selected, function)(*args, **options)
                return failure.value

        if asynchronous:
            failure = asyncio.run(run_async())
        else:
            with client(server) as selected, pytest.raises(tt.Failed) as raised:
                getattr(selected, function)(*args, **options)
            failure = raised.value
        assert failure.code is tt.ErrorCode.BACKEND_STATUS
        assert failure.http_status == 503
        assert failure.rows[0].error is not None
        assert failure.rows[0].error.code is failure.code
        assert server.count() >= 1


def test_one_item_bulk_retains_failure_and_unsure_scalar_returns(
    tmp_path: Path,
) -> None:
    with (
        backend(tmp_path, [{"probability": 0.5}]) as server,
        client(server) as selected,
    ):
        answer = selected.decide("Ready?", "body", threshold=(0.2, 0.8))
        assert answer.unsure and answer.error is None
        with pytest.raises(tt.Unsure):
            _ = answer.answer
        rows = selected.decide_stream("Ready?", [""]).to_list()
        assert len(rows) == 1 and rows[0].failed
        assert rows[0].error is not None
        assert rows[0].error.code is tt.ErrorCode.RECORD_EMPTY
        assert server.count() == 1


@pytest.mark.parametrize("asynchronous", [False, True])
@pytest.mark.parametrize(
    "failed_names",
    [("refund",), ("topic",), ("urgency",), ("refund", "topic", "urgency"), ()],
)
def test_scalar_annotate_returns_failed_children_and_explicit_check_is_recursive(
    tmp_path: Path, asynchronous: bool, failed_names: tuple[str, ...]
) -> None:
    questions = tt.AnnotationQuestions(
        {
            name: tt.DecideQuestion(
                "Ready?",
                field="/absent" if name in failed_names else "/text",
                threshold=(0.2, 0.8),
            )
            for name in ("refund", "topic", "urgency")
        }
    )
    with backend(tmp_path, [{"probability": 0.5}]) as server:

        async def run_async() -> Any:
            async with async_client(server) as selected:
                return await selected.annotate(questions, {"text": "body"})

        if asynchronous:
            form = asyncio.run(run_async())
        else:
            with client(server) as selected:
                form = selected.annotate(questions, {"text": "body"})
        assert form.answered and form.error is None
        entries = form.answer
        for name, entry in entries.items():
            assert entry.failed is (name in failed_names)
            if name not in failed_names:
                assert entry.unsure
        if failed_names:
            with pytest.raises(tt.Failed) as raised:
                form.raise_if_failed()
            assert raised.value.code is tt.ErrorCode.RECORD_MISSING
        else:
            assert form.raise_if_failed() is form
            with pytest.raises(tt.Unsure):
                _ = entries["refund"].answer
        if len(failed_names) == 3:
            assert server.count() == 0


def test_async_unsure_scalar_and_one_item_failed_bulk_keep_their_states(
    tmp_path: Path,
) -> None:
    with backend(tmp_path, [{"probability": 0.5}]) as server:

        async def run() -> None:
            async with async_client(server) as selected:
                answer = await selected.decide("Ready?", "body", threshold=(0.2, 0.8))
                assert answer.unsure and answer.error is None
                with pytest.raises(tt.Unsure):
                    _ = answer.answer
                rows = await selected.decide_stream("Ready?", [""]).to_list()
                assert len(rows) == 1 and rows[0].failed
                assert rows[0].error is not None
                assert rows[0].error.code is tt.ErrorCode.RECORD_EMPTY

        asyncio.run(run())
        assert server.count() == 1
