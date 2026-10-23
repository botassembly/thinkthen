"""Derive the Python relation fields from the engine row schema."""

import json
from pathlib import Path


def relation_type(destination: Path) -> None:
    schema = Path(__file__).resolve().parents[2] / "schemas/row.schema.json"
    target = destination / "_relation.py"
    if not schema.exists():
        # Source archives already carry the type generated from their row schema.
        target.read_text()
        return
    relation = json.loads(schema.read_text())["$defs"]["Relation"]
    types = {
        "boolean": "bool",
        "string": "str",
        "number": "float",
        "#/$defs/Probability": "float",
        "#/$defs/RelationName": "Entity",
    }
    fields = {
        name: types[value.get("$ref", value.get("type"))]
        for name, value in relation["properties"].items()
    }
    required = set(relation["required"])
    lines = [
        '"""Generated from the shared row schema during package assembly."""',
        "from typing import TypedDict",
        "from ._types import Entity",
        "",
        "class _OptionalRelation(TypedDict, total=False):",
        *[
            f"    {name}: {value}"
            for name, value in fields.items()
            if name not in required
        ],
        "",
        "class Relation(_OptionalRelation):",
        *[f"    {name}: {value}" for name, value in fields.items() if name in required],
        "",
    ]
    target.write_text("\n".join(lines))


if __name__ == "__main__":
    relation_type(Path(__file__).parent / "thinkthen")
