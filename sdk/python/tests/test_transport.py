"""The language surface retains the engine's safe transport diagnostic."""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

import pytest
from api_support import backend, isolated_environment  # noqa: F401
from conformance import child_environment

import thinkthen as tt


@pytest.mark.usefixtures("isolated_environment")
@pytest.mark.parametrize(
    ("arguments", "cause"),
    [
        (["--unavailable"], "connection refused"),
        (["--held", "reset"], "connection reset"),
        (["--held", "headers"], "timeout"),
    ],
)
def test_transport_message_is_unchanged(arguments: list[str], cause: str) -> None:
    if sys.platform == "darwin" and cause == "connection refused":
        pytest.skip("Darwin keeps a bound non-listening TCP port silent")
    key = "synthetic-transport-sentinel"
    process = subprocess.Popen(
        [os.environ["THINKTHEN_LOOPBACK"], *arguments],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        env=child_environment(),
    )
    try:
        assert process.stdout is not None
        port = process.stdout.readline().strip()
        with tt.Client(
            backend="openai",
            url=f"http://127.0.0.1:{port}/v1?private-query={key}",
            api_key=key,
            retry=tt.RetryPolicy(max_retries=0, attempt_timeout=0.2),
        ) as client:
            row = client.decide("Ready?", "evidence")
        assert row.status == "failed"
        assert row.error is not None
        assert row.error.code == "backend.transport"
        assert row.error.message == (
            f"the openai backend could not be reached: {cause}"
        )
        assert key not in repr(row)
    finally:
        output, error = process.communicate("shutdown\n", timeout=10)
        assert process.returncode == 0, error
        assert key not in output + error


@pytest.mark.usefixtures("isolated_environment")
@pytest.mark.parametrize("status", [401, 402, 403, 429, 500])
def test_failed_reply_message_is_unchanged(tmp_path: Path, status: int) -> None:
    key = "loopback-test-key"
    reason = f"reason for {status} {key} sk-unknown-sentinel"
    with backend(tmp_path, [{"status": status, "body": {"error": reason}}]) as server:
        with tt.Client(
            backend="openai",
            url=server.url,
            api_key=key,
            retry=tt.RetryPolicy(max_retries=0),
        ) as client:
            if status < 500:
                with pytest.raises(tt.SetupError) as raised:
                    client.decide("Ready?", "evidence")
                assert raised.value.code == "backend.status"
                message = str(raised.value)
                shown = repr(raised.value)
            else:
                row = client.decide("Ready?", "evidence")
                assert row.status == "failed" and row.error is not None
                assert row.error.code == "backend.status"
                assert row.error.http_status == status
                message = row.error.message
                shown = repr(row)
        assert message == (
            f"the backend answered status {status}: "
            f'{{"error":"reason for {status} [redacted] [redacted]"}}'
        )
        assert key not in shown and "sk-unknown-sentinel" not in shown
        assert server.count() == 1


@pytest.mark.usefixtures("isolated_environment")
@pytest.mark.parametrize("provider", ["systemone", "openai"])
@pytest.mark.parametrize("explicit_backend", [False, True])
@pytest.mark.parametrize("explicit_url", [False, True])
def test_client_keeps_environment_addresses_with_their_provider(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    provider: tt.BackendInput,
    explicit_backend: bool,
    explicit_url: bool,
) -> None:
    system_path = tmp_path / "system"
    openai_path = tmp_path / "openai"
    system_path.mkdir()
    openai_path.mkdir()
    with (
        backend(system_path, [{"probability": 0.9}]) as system,
        backend(openai_path, [{"probability": 0.9}]) as openai,
    ):
        server, other = (
            (system, openai) if provider == "systemone" else (openai, system)
        )
        monkeypatch.setenv(
            "THINKTHEN_BACKEND", "unknown" if explicit_backend else provider
        )
        monkeypatch.setenv("THINKTHEN_API_KEY", "synthetic-system")
        monkeypatch.setenv("OPENAI_API_KEY", "synthetic-openai")
        monkeypatch.setenv("THINKTHEN_URL", other.url if explicit_url else system.url)
        monkeypatch.setenv("OPENAI_BASE_URL", other.url if explicit_url else openai.url)
        with tt.Client(
            backend=provider if explicit_backend else None,
            url=server.url if explicit_url else None,
        ) as client:
            row = client.decide("Ready?", "The build passed.")
        assert other.count() == 0
        assert server.count() == 1
        assert row.status == "answered" and row.answer is True
