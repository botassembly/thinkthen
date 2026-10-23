"""Check refused options through the same installed Swift consumer compiler."""
import json
from pathlib import Path
import subprocess
import sys

consumer, fixtures = map(Path, sys.argv[1:])
source = consumer / "Readme/Readme.swift"
accepted = source.read_text()
calls = {
    "score": 'client.score("How strong?", record: .text("alpha"), levels: ["low", "high"], callOptions: .init(threshold: 0.5))',
    "rank": 'client.rank("How strong?", records: [.text("alpha")], options: .init(threshold: 0.5))',
    "annotate": 'client.annotate(JSON(utf8: Data()), record: .text("alpha"), options: .init(threshold: 0.5))',
}
try:
    for case in json.loads(fixtures.read_text()):
        if "expected_error" not in case:
            continue
        call = calls[case["request"]["function"]]
        source.write_text(
            "import Foundation\nimport ThinkThen\n@main struct Readme {\n"
            "static func main() async throws {\nlet client = try Client()\n"
            + "_ = try await " + call + "\n}\n}\n"
        )
        result = subprocess.run(
            ["swift", "build", "--package-path", str(consumer), "--disable-sandbox",
             "--target", "Readme", "-Xswiftc", "-warnings-as-errors"],
            capture_output=True, text=True,
        )
        diagnostics = result.stdout + result.stderr
        assert result.returncode != 0, (case["name"], "forbidden threshold compiled")
        assert "extra argument 'threshold' in call" in diagnostics, (case["name"], diagnostics)
finally:
    source.write_text(accepted)
print("Swift rejects threshold arguments for score, rank and annotate")
