"""Shared loopback stress through the installed public Python API."""

from __future__ import annotations

import json
import os
import subprocess
import sys
from collections.abc import Iterator
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import Any

import pytest
from api_support import isolated_environment  # noqa: F401

import thinkthen as tt

_RESOURCE_CHECK = """import json,runpy,sys
from pathlib import Path
root=Path(sys.argv[1])
sys.path.insert(0,str(root/'scripts'))
runner=runpy.run_path(str(root/'scripts/stress-adapters.py'))
table=json.loads((root/'conformance/stress.json').read_text())
healthy=next(s for s in table['scenarios'] if s['name']=='healthy')
result={}
try:
    runner['run']([sys.executable,sys.argv[2]],table,'systemone',healthy,
        table['records']['smoke'],rounds=table['records']['soak_rounds'],
        resources=True,surface='python',result=result,short_gate=True)
finally:
    print(json.dumps(result),flush=True)
"""


def test_resource_failure_prints_metrics(tmp_path: Path) -> None:
    (tmp_path / "conformance").mkdir()
    (tmp_path / "conformance/stress.json").write_text(
        json.dumps(
            {
                "scenarios": [{"name": "healthy"}],
                "records": {"smoke": 1, "soak_rounds": 20},
            }
        )
    )
    (tmp_path / "scripts").mkdir()
    (tmp_path / "scripts/stress-adapters.py").write_text(
        "def run(*args, **kwargs):\n"
        "    assert kwargs['short_gate'] is True\n"
        "    kwargs['result']['resources'] = {'growth': {'passed': False}}\n"
        "    raise RuntimeError('retained resources fail growth criterion')\n"
    )
    completed = subprocess.run(
        [sys.executable, "-c", _RESOURCE_CHECK, str(tmp_path), "unused-adapter"],
        check=False,
        capture_output=True,
        text=True,
    )
    assert completed.returncode != 0
    assert "retained resources fail growth criterion" in completed.stderr
    assert json.loads(completed.stdout) == {"resources": {"growth": {"passed": False}}}


def settings(table: dict[str, Any], wire: str, url: str) -> dict[str, Any]:
    client = table["client"]
    retry = client["retry"]
    return {
        "backend": wire,
        "url": url,
        "api_key": client["api_key"],
        "model": client["model"],
        "batch": client["batch"],
        "retry": tt.RetryPolicy(
            retry["max_retries"],
            retry["initial_backoff_ms"] / 1000,
            retry["attempt_timeout_ms"] / 1000,
        ),
    }


def consume(
    client: tt.Client,
    scenario: dict[str, Any],
    count: int,
    probability: float,
    call: int,
    basis: int,
    widths: set[int],
) -> int:
    call += basis

    def records() -> Iterator[tt.Record]:
        for i in range(count):
            yield {"id": f"call-{call}-record-{basis + i}", "body": f"record {i}"}

    received = failed = 0
    with client.decide_stream("Ready?", records()) as stream:
        for row in stream:
            assert isinstance(row.id, str)
            widths.add(len(row.id))
            assert row.position == received + 1
            assert row.id == f"call-{call}-record-{basis + received}"
            received += 1
            if row.status == "failed":
                failed += 1
                assert row.error is not None and row.error.code == scenario["code"]
            else:
                assert row.status == "answered" and row.answer is True
                assert row.probability == probability
            if scenario["status"] == "answered" or scenario["name"] == "refused":
                assert row.status == scenario["status"]
    assert received == count
    assert bool(failed) == (scenario["status"] == "failed")
    return failed


def adapter(case: dict[str, Any]) -> None:
    calls = rows = failed = 0
    widths: set[int] = set()
    with tt.Client(
        **settings(case, case["client"]["backend"], case["client"]["url"])
    ) as client:
        if case["cancel"]:
            token = tt.Cancel()
            with (
                client.decide_stream(
                    "Ready?", ("body" for _ in range(case["records"])), cancel=token
                ) as stream,
                ThreadPoolExecutor(max_workers=1) as pool,
            ):
                pending = pool.submit(next, stream)
                assert input() == "cancel"
                token.cancel()
                with pytest.raises(tt.Cancelled) as error:
                    pending.result()
                assert error.value.code == "local.cancelled"
                assert list(stream) == []
            calls = 1
        else:
            with ThreadPoolExecutor(max_workers=case["concurrency"]) as pool:
                for round_index in range(case["rounds"]):
                    futures = [
                        pool.submit(
                            consume,
                            client,
                            case["scenario"],
                            case["records"],
                            case["probability"],
                            round_index * case["concurrency"] + index,
                            case["identifier_base"],
                            widths,
                        )
                        for index in range(case["concurrency"])
                    ]
                    for future in futures:
                        failed += future.result()
                        calls += 1
                        rows += case["records"]
                    if hook := os.environ.get("THINKTHEN_STRESS_CHECKPOINT_COMMAND"):
                        subprocess.run(hook, shell=True, check=True)
    print(
        json.dumps(
            {
                "identifier_width_mask": sum(1 << width for width in widths),
                "calls": calls,
                "rows": rows,
                "failed": failed,
                "cancelled": case["cancel"],
            }
        )
    )


@pytest.mark.usefixtures("isolated_environment")
def test_shared_stress_preserves_positions_and_same_client_threads() -> None:
    root = Path(__file__).resolve().parents[3]
    subprocess.run(
        [
            sys.executable,
            str(root / "scripts/stress-adapters.py"),
            "--surface",
            "python",
            "--",
            sys.executable,
            str(Path(__file__).resolve()),
        ],
        check=True,
    )
    if sys.platform == "linux":
        # Feature: the installed adapter checkpoints live completed soak rounds.
        subprocess.run(
            [
                sys.executable,
                "-c",
                _RESOURCE_CHECK,
                str(root),
                str(Path(__file__).resolve()),
            ],
            check=True,
        )


if __name__ == "__main__":
    adapter(json.loads(Path(os.environ["THINKTHEN_STRESS_CASE"]).read_text()))
