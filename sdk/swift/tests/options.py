"""Compile public Swift calls that must be refused by their signatures."""
from pathlib import Path
import json
import tempfile
import subprocess
import sys

consumer = Path(sys.argv[1])
build = json.loads((consumer / ".build/debug/description.json").read_text())
compiled = next(item for item in build["swiftCommands"].values() if item["moduleName"] == "Readme")
arguments = compiled["otherArguments"]
flags = []
for flag in ("-target", "-sdk", "-resource-dir", "-swift-version"):
    if flag in arguments:
        flags += [flag, arguments[arguments.index(flag) + 1]]
flags += ["-Xcc", next(value for value in arguments if value.startswith("-fmodule-map-file="))]
temporary = tempfile.TemporaryDirectory(prefix="thinkthen-swift-signatures-")
source = Path(temporary.name) / "Readme.swift"
# Feature: Swift required domains and numeric-only thresholds are compile-time contracts.
calls = {
    "annotations reject general toolbox": ('client.annotate(Toolbox([:]), record: "a")', "Toolbox"),
    "choose requires choices": ('client.choose("Topic?", record: "a")', "choices"),
    "tag requires labels": ('client.tag("Topics?", record: "a")', "labels"),
    "score requires levels": ('client.score("Scale?", record: "a")', "levels"),
    "rank requires levels": ('client.rank("Scale?", records: ["a"])', "levels"),
    "recognize requires kinds": ('client.recognize(record: "a")', "kinds"),
    "relate requires rules": ('client.relate(record: "a")', "rules"),
    "choose refuses a band": ('client.choose("Which?", record: "a", choices: ["a", "b"], threshold: .band(low: 0.2, high: 0.8))', "band"),
    "tag refuses a band": ('client.tag("Topics?", record: "a", labels: ["a"], threshold: .band(low: 0.2, high: 0.8))', "band"),
    "filter refuses a band": ('client.filter("Ready?", records: ["a"], threshold: .band(low: 0.2, high: 0.8))', "band"),
    "find refuses a band": ('client.find("Where?", record: "a", threshold: .band(low: 0.2, high: 0.8))', "band"),
    "recognize refuses a band": ('client.recognize(record: "a", kinds: ["person"], threshold: .band(low: 0.2, high: 0.8))', "band"),
    "relate refuses a band": ('client.relate(record: "a", rules: [], threshold: .band(low: 0.2, high: 0.8))', "band"),
    "score has no threshold": ('client.score("Scale?", record: "a", levels: ["low", "high"], threshold: 0.5)', "threshold"),
    "rank has no threshold": ('client.rank("Scale?", records: ["a"], levels: ["low", "high"], threshold: 0.5)', "threshold"),
}


def compile_source(body):
    source.write_text(
        "import Foundation\nimport ThinkThen\n@main struct Readme {\n"
        "static func main() async throws {\nlet client = try Client(.init(backend: .stub))\n"
        + body + "\n}\n}\n"
    )
    return subprocess.run(
        [compiled["executable"], "-typecheck", "-parse-as-library", "-warnings-as-errors",
         "-I", compiled["importPath"], *flags, str(source)],
        capture_output=True, text=True,
    )

try:
    baseline = compile_source('_ = try await client.choose("Topic?", record: "a", choices: ["a", "b"])')
    assert baseline.returncode == 0, baseline.stdout + baseline.stderr
    for name, (call, field) in calls.items():
        result = compile_source("_ = try await " + call)
        diagnostics = result.stdout + result.stderr
        assert result.returncode != 0, (name, "invalid public call compiled", diagnostics)
        expected_type = name.split()[0].title() + "Question"
        assert (field in diagnostics or expected_type in diagnostics) and any(fragment in diagnostics for fragment in (
            "missing argument", "extra argument", "cannot convert value", "no exact matches", "has no member", "is unavailable"
        )), (name, diagnostics)
finally:
    temporary.cleanup()
print("Swift refuses missing required domains and unsupported thresholds")
