"""Compile and run the guide's runnable snippets in their documented context."""
from pathlib import Path
import re
import sys

source, destination = map(Path, sys.argv[1:])
text = source.read_text()
blocks = re.findall(r"```swift\n(.*?)\n```", text, re.S)
client = next(line for block in blocks for line in block.splitlines() if line.startswith("let client ="))
messages = next(block for block in blocks if block.startswith("let messages ="))
destination.mkdir()
for block in re.findall(r"```json\n(.*?)\n```", text, re.S):
    if '"entries"' in block:
        (destination / "toolbox.json").write_text(block + "\n")
body = [client, messages]
examples = 0
for block in blocks:
    if block == messages or block.startswith("platforms:") or "records.jsonl" in block:
        continue
    code = "\n".join(line for line in block.splitlines() if not line.startswith("import "))
    # The examples that introduce values need no print call. Mark those values
    # used so warnings still fail compilation everywhere in the actual sources.
    bindings = re.findall(r"^(?:let|var) (\w+)\b", code, re.M)
    body += ["do {", code, *("_ = " + name for name in bindings), "}"]
    examples += 1
body += [f'print("Swift guide snippets passed: {examples}")']
(destination / "Examples.swift").write_text(
    "import Foundation\nimport ThinkThen\n@main struct Examples {\n"
    "@MainActor static func main() async throws {\n" + "\n".join(body) + "\n}\n}\n"
)
print(f"Swift guide: staged {examples} runnable snippets")
