"""Compile public Swift calls that must be refused by their signatures."""
from pathlib import Path
import subprocess
import sys

consumer = Path(sys.argv[1])
source = consumer / "Readme/Readme.swift"
accepted = source.read_text()
# Feature: Swift required domains and numeric-only thresholds are compile-time contracts.
calls = {
    "annotations reject rank": ('client.annotate(AnnotationQuestions(["rank": .rank(RankQuestion("Best?", levels: ["low", "high"]))]), record: .text("a"))', "rank"),
    "annotations reject general toolbox": ('client.annotate(Toolbox([:]), record: .text("a"))', "Toolbox"),
    "choose requires choices": ('client.choose("Topic?", record: .text("a"))', "choices"),
    "tag requires labels": ('client.tag("Topics?", record: .text("a"))', "labels"),
    "score requires levels": ('client.score("Scale?", record: .text("a"))', "levels"),
    "rank requires levels": ('client.rank("Scale?", records: [Record.text("a")])', "levels"),
    "recognize requires kinds": ('client.recognize(record: .text("a"))', "kinds"),
    "relate requires rules": ('client.relate(record: .text("a"))', "rules"),
    "tag refuses a band": ('client.tag("Topics?", labels: ["a"], record: .text("a"), options: .init(threshold: Threshold.band(low: 0.2, high: 0.8)))', "Threshold"),
    "find refuses a band": ('client.find("Where?", record: .text("a"), options: .init(threshold: Threshold.band(low: 0.2, high: 0.8)))', "Threshold"),
    "recognize refuses a band": ('client.recognize(kinds: ["person"], record: .text("a"), options: .init(threshold: Threshold.band(low: 0.2, high: 0.8)))', "Threshold"),
    "relate refuses a band": ('client.relate(rules: [], record: .text("a"), options: .init(threshold: Threshold.band(low: 0.2, high: 0.8)))', "Threshold"),
    "score has no threshold": ('client.score("Scale?", levels: ["low", "high"], record: .text("a"), options: .init(threshold: 0.5))', "threshold"),
    "rank has no threshold": ('client.rank("Scale?", levels: ["low", "high"], records: [Record.text("a")], options: .init(threshold: 0.5))', "threshold"),
}

def compile_source(body):
    source.write_text(
        "import Foundation\nimport ThinkThen\n@main struct Readme {\n"
        "static func main() async throws {\nlet client = try Client(.init(backend: .stub))\n"
        + body + "\n}\n}\n"
    )
    return subprocess.run(
        ["swift", "build", "--package-path", str(consumer), "--disable-sandbox",
         "--target", "Readme", "-Xswiftc", "-warnings-as-errors"],
        capture_output=True, text=True,
    )

try:
    baseline = compile_source('_ = try await client.choose("Topic?", choices: ["a", "b"], record: .text("a"))')
    assert baseline.returncode == 0, baseline.stdout + baseline.stderr
    for name, (call, field) in calls.items():
        result = compile_source("_ = try await " + call)
        diagnostics = result.stdout + result.stderr
        assert result.returncode != 0, (name, "invalid public call compiled")
        expected_type = name.split()[0].title() + "Question"
        assert (field in diagnostics or expected_type in diagnostics) and any(fragment in diagnostics for fragment in (
            "missing argument", "extra argument", "cannot convert value", "no exact matches", "has no member"
        )), (name, diagnostics)
finally:
    source.write_text(accepted)
print("Swift refuses missing required domains and unsupported thresholds")
