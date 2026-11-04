"""Derive extensible Swift code names from the engine registry."""
import json
from pathlib import Path
import sys

registry, destination = sys.argv[1:]
codes = json.loads(Path(registry).read_text())["codes"]
acronyms = {"url": "URL", "json": "JSON", "io": "IO", "abi": "ABI", "sql": "SQL", "http": "HTTP"}
text = "// Generated from the engine registry during packaging.\n"
text += "/// A stable engine error name, including names from newer engines.\n"
text += "public struct ErrorCode: RawRepresentable, Sendable, Codable, Hashable, CaseIterable {\n"
text += "  /// The stable engine diagnostic name.\n  public let rawValue: String\n  /// Preserve an engine name, including names added by a newer engine.\n  public init(rawValue: String) { self.rawValue = rawValue }\n"
names = []
for code in codes:
    parts = code["name"].replace("-", ".").split(".")
    name = parts[0] + "".join(acronyms.get(part, part.title()) for part in parts[1:])
    names.append(name)
    text += f'  /// Engine code `{code["name"]}`.\n  public static let {name} = Self(rawValue: "{code["name"]}")\n'
text += "  /// The codes known by this package; newer names remain representable.\n  public static let allCases: [Self] = [" + ", ".join("." + x for x in names) + "]\n"
text += "  /// The integer engine identity, when this package knows the code.\n  public var nativeValue: Int32? {\n    switch rawValue {\n"
for code in codes:
    text += f'    case "{code["name"]}": {code["value"]}\n'
text += "    default: nil\n    }\n  }\n"
text += "  /// Decode a stable diagnostic name.\n  public init(from decoder: any Decoder) throws { self.init(rawValue: try decoder.singleValueContainer().decode(String.self)) }\n"
text += "  /// Encode the stable diagnostic name.\n  public func encode(to encoder: any Encoder) throws { var c = encoder.singleValueContainer(); try c.encode(rawValue) }\n}\n"
Path(destination).write_text(text)
