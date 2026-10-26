"""Run shared cases using only an installed public package and the test backend."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from collections.abc import Iterable, Mapping
from dataclasses import fields, is_dataclass
from enum import Enum
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


def plain(value: Any) -> Any:
    if isinstance(value, Enum):
        return value.value
    if is_dataclass(value) and hasattr(value, "status"):
        return outcome_json(value)
    if is_dataclass(value) and not isinstance(value, type):
        return {
            field.name: plain(getattr(value, field.name))
            for field in fields(value)
            if (field.name != "name" or getattr(value, field.name) is not None)
            and (field.name != "either" or getattr(value, field.name))
        }
    if isinstance(value, Mapping):
        return {key: plain(item) for key, item in value.items()}
    if isinstance(value, (list, tuple)):
        return [plain(item) for item in value]
    return value


def outcome_json(row: Any) -> dict[str, Any]:
    result: dict[str, Any] = {"function": row.function, "status": row.status}
    if row.status == "answered":
        result["answer"] = plain(row.answer)
    if row.error is not None:
        result["error"] = {"code": row.error.code, "message": row.error.message}
        if row.error.http_status is not None:
            result["error"]["http_status"] = row.error.http_status
    if row.probability is not None:
        result["probability"] = row.probability
    if row.detail is not None:
        result["detail"] = plain(row.detail)
    return result


def row_json(row: Any) -> dict[str, Any]:
    result = outcome_json(row)
    result.update(position=row.position, source=plain(row.source))
    if row.id is not None:
        result["id"] = row.id
    return result


def admitted(value: dict[str, Any]) -> Any:
    fields = dict(value)
    function = fields.pop("function")
    if function == "relate":
        fields["rules"] = [tt.RelationRule(**rule) for rule in fields["rules"]]
    if isinstance(fields.get("threshold"), dict):
        band = fields["threshold"]
        fields["threshold"] = tt.Band(band["low"], band["high"])
    question = fields.pop("question", None)
    return (
        getattr(tt, function.capitalize() + "Question")(**fields)
        if question is None
        else getattr(tt, function.capitalize() + "Question")(question, **fields)
    )


def request_stream(client: tt.Client, request: dict[str, Any]) -> tt.RowStream[Any]:
    definition = request["question"]
    function = definition["function"]
    records = request.get("records", [])
    if function == "annotate":
        return client.annotate_stream(
            tt.AnnotationQuestions(
                {
                    name: admitted(value)
                    for name, value in definition["questions"].items()
                }
            ),
            records,
            evidence=definition.get("evidence"),
        )
    question = admitted(definition)
    if function in {"filter", "rank"}:
        return cast(tt.RowStream[Any], getattr(client, function)(question, records))
    if function in {"recognize", "relate"}:
        return cast(
            tt.RowStream[Any],
            getattr(client, function + "_stream")(question, records),
        )
    return cast(
        tt.RowStream[Any], getattr(client, function + "_stream")(question, records)
    )


def assert_expected(actual: Any, expected: Any) -> None:
    """Shared cases omit messages and source, but every declared value is exact."""
    if isinstance(expected, dict):
        assert isinstance(actual, dict)
        assert set(actual) - set(expected) <= {"source", "message"}
        for name, value in expected.items():
            assert name in actual
            assert_expected(actual[name], value)
    elif isinstance(expected, list):
        assert isinstance(actual, list) and len(actual) == len(expected)
        for item, wanted in zip(actual, expected, strict=False):
            assert_expected(item, wanted)
    else:
        assert actual == expected


def admission_cases(directory: Path) -> None:
    cases = json.loads(
        (directory / "public-contract" / "usage-admission.json").read_text()
    )
    for case in cases:
        if case["kind"] != "admission":
            continue
        value = case["input"]
        message = case["message"]
        if case["boundary"] == "settings":
            print(
                f"Admission unsupported: {case['name']}: retry fields convert before admission"
            )
            continue
        projection = case["boundary"] == "question" and "tags" in value
        if projection:
            # TagQuestion rejects unknown keywords before entering the library.
            value = {"entries": {"fixture": {"question": value}}}
            message = "toolbox.entries.[0]." + message
        route = "public projection (saved entry)" if projection else "direct"
        try:
            if case["boundary"] == "toolbox" or projection:
                tt.Toolbox.from_json(json.dumps(value))
            else:
                admitted(value)
        except tt.ThinkThenError as error:
            assert error.code.value == case["code"], case["name"]
            assert str(error) == message, (case["name"], str(error), message)
            assert "secret-placeholder" not in str(error)
        else:
            raise AssertionError(f"admitted malformed fixture: {case['name']}")
        print(f"Admission {route}: {case['name']}")


def run_all(directory: Path, scratch: Path) -> None:
    admission_cases(directory.parent if directory.name == "typed" else directory)
    if not (directory / "manifest.json").exists():
        directory = directory / "typed"
    manifest = json.loads((directory / "manifest.json").read_text())
    count = 0
    for backend in manifest["backends"]:
        for filename in manifest["cases"]:
            path = directory / filename
            case = json.loads(path.read_text())
            # Defaults and credential policy remain engine-owned. Keys are synthetic.
            settings = (
                manifest["client"]
                | case.get("client", {})
                | {"backend": backend, "api_key": "synthetic-loopback-key"}
            )
            with Loopback(path) as server:
                client = tt.Client(
                    backend=settings["backend"],
                    api_key=settings["api_key"],
                    url=server.url,
                    model=settings["model"],
                    batch=settings["batch"],
                )
                actual: Any = []
                try:
                    with request_stream(client, case["request"]) as stream:
                        for row in stream:
                            actual.append(row_json(row))
                except tt.ThinkThenError as error:
                    assert not actual, (filename, actual)
                    actual = {"error": error.code.value}
                assert_expected(actual, case["expected"])
                for row in actual if isinstance(actual, list) else []:
                    assert set(row["source"]) == {
                        "backend",
                        "model",
                        "question_identity",
                    }
                    assert row["source"]["backend"] == backend
                    assert row["source"]["model"] == settings["model"]
                assert server.count() == case["requests"], (backend, filename)
                count += 1
    print(f"Installed wheel: {count} canonical backend cases passed")


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
