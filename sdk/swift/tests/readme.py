"""Compile the README's first call in a main actor."""
from pathlib import Path
import re
import sys

readme, destination = map(Path, sys.argv[1:])
blocks = re.findall(r"```swift\n(.*?)```", readme.read_text(), re.DOTALL)
first = next(block for block in blocks if "let client =" in block)
imports = "\n".join(line for line in first.splitlines() if line.startswith("import "))
body = "\n".join(line for line in first.splitlines() if not line.startswith("import "))
destination.mkdir()
(destination / "Readme.swift").write_text(
    imports + "\n@main struct Readme {\n@MainActor static func main() async throws {\n"
    + body + "\n}\n}\n"
)
