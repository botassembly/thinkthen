"""Run shared cases using only an installed public package and the test backend."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from collections.abc import Iterable
from pathlib import Path
from typing import TYPE_CHECKING, Any, cast

import thinkthen as tt

if TYPE_CHECKING:
    from typing_extensions import Self


def child_environment() -> dict[str, str]:
    names = ("PATH", "HOME", "SYSTEMROOT", "TMPDIR", "TEMP", "TMP")
    return {name: os.environ[name] for name in names if name in os.environ}


class Loopback:
    def __init__(self, case_path: Path) -> None:
        self.process = subprocess.Popen(
            [os.environ["THINKTHEN_LOOPBACK"], str(case_path)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            env=child_environment(),
        )
        assert self.process.stdout is not None
        self.url = f"http://127.0.0.1:{self.process.stdout.readline().strip()}/v1"

    def count(self) -> int:
        assert self.process.stdin is not None
        assert self.process.stdout is not None
        self.process.stdin.write("count\n")
        self.process.stdin.flush()
        return int(self.process.stdout.readline())

    def close(self) -> None:
        assert self.process.stdin is not None
        self.process.stdin.write("shutdown\n")
        self.process.stdin.flush()
        _, stderr = self.process.communicate()
        assert self.process.returncode == 0, stderr

    def __enter__(self) -> Self:
        return self

    def __exit__(self, *exception: object) -> None:
        self.close()


def row_json(row: tt.Row[object]) -> dict[str, Any]:
    result: dict[str, Any] = {"id": row.id, "status": row.status}
    if row.error is not None:
        result["error"] = {"code": row.error.code}
    else:
        result["answer"] = row.answer
    if row.p is not None:
        result["p"] = row.p
    if row.detail is not None:
        result["detail"] = row.detail
    if isinstance(row.answer, dict) and row.status == "answered":
        for entry in row.answer.values():
            if isinstance(entry, dict) and isinstance(entry.get("error"), dict):
                entry["error"].pop("message", None)
    return result


def run_case(
    case: dict[str, Any], url: str, toolbox_path: Path
) -> list[dict[str, Any]] | dict[str, str]:
    request = case["request"]
    settings = case.get("client", {})
    options = request.get("options", {})
    question = request["question"]["value"]
    records = request["records"]
    cancel = tt.Cancel()
    rows: list[dict[str, Any]] = []
    if case.get("cancelled"):
        cancel.cancel()
    try:
        client = tt.Client(
            url=url,
            model=settings.get("model"),
            batch=int(settings["batch"]) if "batch" in settings else None,
        )
        on = options.get("on")
        cut = options.get("threshold")
        function = request["function"]
        stream: tt.Stream[tt.Row[object]]
        if function == "rank":
            stream = client.rank(
                question,
                records,
                levels=options.get("levels"),
                top=options.get("top"),
                on=on,
                cancel=cancel,
            )
        elif function == "annotate":
            stream = client.annotate_stream(question, records, on=on, cancel=cancel)
        elif function == "filter":
            stream = (
                tt.Toolbox({"filter": question} | options)
                .question()
                .stream(records, on=on, cancel=cancel)
            )
        elif function == "decide":
            stream = client.decide_stream(
                question, records, threshold=cut, on=on, cancel=cancel
            )
        elif function == "choose":
            stream = client.choose_stream(
                question,
                records,
                options=options["options"],
                threshold=cut,
                on=on,
                cancel=cancel,
            )
        elif function == "tag":
            stream = client.tag_stream(
                question,
                records,
                labels=options["labels"],
                threshold=cut,
                on=on,
                cancel=cancel,
            )
        elif function == "score":
            stream = client.score_stream(
                question,
                records,
                levels=options["levels"],
                on=on,
                cancel=cancel,
            )
        elif function == "find":
            stream = client.find_stream(
                question, records, threshold=cut, on=on, cancel=cancel
            )
        elif function == "recognize":
            stream = client.recognize_stream(
                records,
                kinds=question,
                threshold=cut,
                on=on,
                cancel=cancel,
            )
        else:
            stream = client.relate_stream(
                records, rules=question, threshold=cut, on=on, cancel=cancel
            )
        with stream:
            for row in stream:
                rows.append(row_json(row))
        return rows
    except KeyboardInterrupt:
        return {"error": "local.cancelled"}
    except tt.ThinkThenError as error:
        return {"error": error.code}
    finally:
        if "expected_rows" in case:
            assert rows == case["expected_rows"], (rows, case["expected_rows"])


def run_all(directory: Path, scratch: Path) -> None:
    paths = sorted(directory.glob("*.json"))
    assert paths
    for path in paths:
        case = cast(dict[str, Any], json.loads(path.read_text()))
        toolbox = scratch / "questions.json"
        request = case["request"]
        question = request["question"]["value"]
        if request["function"] != "annotate":
            question = {request["function"]: question} | request.get("options", {})
        toolbox.write_text(json.dumps(question))
        with Loopback(path) as server:
            os.environ["THINKTHEN_BASE_URL"] = server.url
            actual = run_case(case, server.url, toolbox)
            assert actual == case["expected"], (path.name, actual, case["expected"])
            assert server.count() == case["requests"], path.name
    print(f"Installed wheel: {len(paths)} shared conformance cases passed")


def main(arguments: Iterable[str]) -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cases", required=True, type=Path)
    parser.add_argument("--scratch", required=True, type=Path)
    options = parser.parse_args(list(arguments))
    assert Path(tt.__file__).is_relative_to(Path(sys.prefix)), tt.__file__
    os.environ["THINKTHEN_API_KEY"] = "synthetic-loopback-key"
    run_all(options.cases, options.scratch)


if __name__ == "__main__":
    main(sys.argv[1:])
