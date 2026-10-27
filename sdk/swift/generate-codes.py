"""Derive Swift error cases and integer identities from the engine registry."""
import json
from pathlib import Path
import sys

registry, destination = sys.argv[1:]
codes = json.loads(Path(registry).read_text())["codes"]
text = "// Generated from the engine registry during packaging.\n"
text += "public enum ErrorCode: String, Sendable, Codable, CaseIterable {\n"
for code in codes:
    parts = code["name"].replace("-", ".").split(".")
    case = parts[0] + "".join(part.title() for part in parts[1:])
    text += f'  case {case} = "{code["name"]}"\n'
text += '  case unknown = "unknown"\n'
text += "  public var nativeValue: Int32? {\n    switch self {\n"
for code in codes:
    parts = code["name"].replace("-", ".").split(".")
    case = parts[0] + "".join(part.title() for part in parts[1:])
    text += f'    case .{case}: {code["value"]}\n'
text += "    case .unknown: nil\n    }\n  }\n}\n"
Path(destination).write_text(text)
