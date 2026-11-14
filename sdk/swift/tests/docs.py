"""Stage retained quickstart and collection targets from literal public code.

LiteralGuide owns complete guide scenarios, output and refusal checks.
"""
from pathlib import Path
import re
import sys

source, destination, guide = map(Path, sys.argv[1:])
text = source.read_text()
first = re.findall(r"```swift\n(.*?)\n```", text, re.S)[0]
collection = re.findall(r"```swift\n(.*?)\n```", text.split("## Many records", 1)[1], re.S)[0]
client = next(line for line in first.splitlines() if line.startswith("let client ="))
for target, body, name in ((destination, first, "Examples"), (guide, client + "\n" + collection, "Guide")):
    target.mkdir()
    code = "\n".join(line for line in body.splitlines() if not line.startswith("import "))
    (target / (name + ".swift")).write_text(
        "import Foundation\nimport ThinkThen\n@main struct " + name + " {\n"
        "@MainActor static func main() async throws {\n" + code + "\n}\n}\n")
print("Staged public Swift quickstart and collection targets")
