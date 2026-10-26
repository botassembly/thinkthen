"""Local transport fixtures for public Python boundary tests."""

from __future__ import annotations

import json
import os
import subprocess
from collections.abc import Iterator
from contextlib import contextmanager
from pathlib import Path
from typing import Any

import pytest
from conformance import Loopback, child_environment
from typing_extensions import Self

import thinkthen as tt


@pytest.fixture
def isolated_environment(
    monkeypatch: pytest.MonkeyPatch, tmp_path: Path
) -> Iterator[None]:
    """Keep caller configuration and credentials outside each case."""
    for name in tuple(os.environ):
        if name.startswith(("THINKTHEN_", "OPENAI_")) and name not in {
            "THINKTHEN_CASES",
            "THINKTHEN_LOOPBACK",
            "THINKTHEN_STRESS_FULL",
        }:
            monkeypatch.delenv(name)
    for name in ("HOME", "XDG_CONFIG_HOME", "XDG_CACHE_HOME"):
        destination = tmp_path / name.lower()
        destination.mkdir()
        monkeypatch.setenv(name, str(destination))
    tt.close_default_client()
    try:
        yield
    finally:
        tt.close_default_client()


@contextmanager
def backend(tmp_path: Path, replies: list[dict[str, Any]]) -> Iterator[Loopback]:
    path = tmp_path / "backend.json"
    path.write_text(json.dumps({"replies": replies}), encoding="utf-8")
    with Loopback(path) as server:
        yield server


def yes(probability: float = 0.912345678901234) -> list[dict[str, Any]]:
    return [{"answer": {"type": "noul", "noul": probability}}]


def client(server: Loopback, **settings: Any) -> tt.Client:
    return tt.Client(
        backend="systemone",
        url=server.url,
        api_key="synthetic-loopback-key",
        batch=1,
        retry=tt.RetryPolicy(max_retries=0),
        **settings,
    )


class Held:
    """Observe arrival and socket closure through the existing fixture protocol."""

    def __enter__(self) -> Self:
        self.process = subprocess.Popen(
            [os.environ["THINKTHEN_LOOPBACK"], "--held", "body"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            env=child_environment(),
        )
        assert self.process.stdout is not None
        self.url = "http://127.0.0.1:" + self.process.stdout.readline().strip() + "/v1"
        return self

    def event(self, name: str) -> None:
        assert self.process.stdin is not None and self.process.stdout is not None
        self.process.stdin.write(name + "\n")
        self.process.stdin.flush()
        assert self.process.stdout.readline().strip() == name

    def __exit__(self, *exception: object) -> None:
        _, error = self.process.communicate("shutdown\n", timeout=10)
        assert self.process.returncode == 0, error
