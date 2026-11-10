"""Usage snapshots copy native values and survive explicit client close."""

import asyncio
from dataclasses import FrozenInstanceError
from pathlib import Path

import pytest
from api_support import backend, isolated_environment  # noqa: F401 - shared fixture.

import thinkthen as tt

pytestmark = pytest.mark.usefixtures("isolated_environment")


def test_stub_usage_is_immutable_and_survives_close() -> None:
    client = tt.Client(backend="stub")
    before = client.usage
    client.decide("Ready?", "alpha")
    client.close()
    after = client.usage
    client.close()
    assert before.attempts == after.attempts == 0
    assert after.responses == after.active_attempts == after.active_calls == 0
    assert after.cache_hits == after.replay_hits == 0
    assert after.input_tokens == tt.TokenUsage(0, 0)
    assert after.output_tokens == tt.TokenUsage(0, 0)
    with pytest.raises(FrozenInstanceError):
        after.attempts = 9  # type: ignore[misc]


def test_async_usage_survives_close() -> None:
    async def scenario() -> None:
        client = tt.AsyncClient(backend="stub")
        await client.decide("Ready?", "alpha")
        await client.aclose()
        assert client.usage.attempts == client.usage.active_calls == 0
        assert client.usage.input_tokens.reported == 0

    asyncio.run(scenario())


def test_usage_copies_remote_counters(tmp_path: Path) -> None:
    with backend(tmp_path, [{"probability": 0.9}]) as server:
        client = tt.Client(backend="openai", url=server.url, api_key="synthetic")
        before = client.usage
        assert client.decide("Ready?", "alpha").answer is True
        client.close()
        after = client.usage
        assert before.attempts == 0
        assert after.attempts == after.responses == server.count() == 1
        assert after.request_body_bytes > 0
        assert after.input_tokens.unknown_attempts == 1
        assert after.output_tokens.unknown_attempts == 1
        assert after.active_attempts == after.active_calls == 0


def test_explicit_request_allowance_survives_settings_projection() -> None:
    with tt.Client(backend="stub", max_requests=0) as client:
        assert client.settings.max_requests == 0
        client.decide("Ready?", "alpha")
    with tt.Client(backend="stub") as client:
        assert client.settings.max_requests is None
    with pytest.raises(tt.UsageError):
        tt.Client(backend="stub", max_requests=-1)


def test_response_store_projects_settings_and_replay_hits(tmp_path: Path) -> None:
    folder = str(tmp_path / "responses")
    with backend(tmp_path, [{"probability": 0.9}]) as server:
        store = tt.ResponseStore(folder, "record", "sdk-test", "fixed-generation")
        with tt.Client(
            backend="openai", url=server.url, api_key="synthetic", response_store=store
        ) as client:
            admitted = client.settings.response_store
            assert admitted is not None and admitted.namespace == "sdk-test"
            assert admitted.max_entries == 4096
            assert client.decide("Ready?", "alpha").answer is True
        replay = tt.ResponseStore(folder, "replay", "sdk-test", "fixed-generation")
        with tt.Client(
            backend="openai", url=server.url, response_store=replay
        ) as client:
            assert client.decide("Ready?", "alpha").answer is True
        assert client.usage.attempts == client.usage.cache_hits == 0
        assert client.usage.replay_hits == 1
        assert server.count() == 1


def test_response_store_zero_limit_is_not_defaulted(tmp_path: Path) -> None:
    store = tt.ResponseStore(
        str(tmp_path / "responses"), "cache", "sdk-test", "generation", max_entries=0
    )
    with pytest.raises(tt.UsageError):
        tt.Client(backend="openai", response_store=store)
