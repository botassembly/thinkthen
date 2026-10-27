"""Check Python enums and generated declarations against their owning sources."""

import ast
import json
import re
from pathlib import Path

from relation_type import code_source


def check(root: Path) -> None:
    package = root / "sdk/python/thinkthen"
    contract = json.loads((root / "schemas/contract.json").read_text())
    classes = {
        node.name: [
            ast.literal_eval(member.value)
            for member in node.body
            if isinstance(member, ast.Assign)
        ]
        for node in ast.parse((package / "_types.py").read_text()).body
        if isinstance(node, ast.ClassDef)
        and node.name in {"Function", "Status", "Backend", "BoundaryTag"}
    }
    for name, key in (
        ("Function", "functions"),
        ("Status", "statuses"),
        ("Backend", "backends"),
    ):
        assert set(classes[name]) == {entry["name"] for entry in contract[key]}, name
    source = (root / "crates/thinkthen/src/core/function/recognize.rs").read_text()
    tags = source.split("const TAGS:", 1)[1].split("\n];", 1)[0]
    assert classes["BoundaryTag"] == re.findall(r'\(\s*"([^"]+)"', tags), "BoundaryTag"
    expected = code_source([entry["name"] for entry in contract["codes"]])
    assert (package / "_codes.py").read_text() == expected, "_codes.py is stale"


if __name__ == "__main__":
    check(Path(__file__).resolve().parents[2])
