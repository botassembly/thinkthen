"""Generate typed error declarations from the shared engine registry."""

import ast
from pathlib import Path


def relation_type(destination: Path) -> None:
    (destination / "_relation.py").unlink(missing_ok=True)
    contract = ast.parse((destination / "_contract.py").read_text())
    codes = next(
        ast.literal_eval(node.value)
        for node in contract.body
        if isinstance(node, ast.AnnAssign)
        and isinstance(node.target, ast.Name)
        and node.target.id == "CODE_NAMES"
    )
    lines = [
        '"""Generated from the engine error registry during package assembly."""',
        "from enum import Enum",
        "",
        "class ErrorCode(str, Enum):",
    ]
    lines.extend(
        f'    {value.upper().replace(".", "_").replace("-", "_")} = "{value}"'
        for value in codes.values()
    )
    lines.extend(["", '__all__ = ["ErrorCode"]', ""])
    (destination / "_codes.py").write_text("\n".join(lines))


if __name__ == "__main__":
    relation_type(Path(__file__).parent / "thinkthen")
