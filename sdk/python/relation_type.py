"""Generate typed error declarations from the shared engine registry."""

import ast
from pathlib import Path


def code_source(codes: list[str]) -> str:
    lines = [
        '"""Generated from the engine error registry during package assembly."""',
        "from ._enums import StringEnum",
        "",
        "class ErrorCode(StringEnum):",
    ]
    lines.extend(
        f'    {value.upper().replace(".", "_").replace("-", "_")} = "{value}"'
        for value in codes
    )
    lines.extend(["", '__all__ = ["ErrorCode"]', ""])
    return "\n".join(lines)


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
    (destination / "_codes.py").write_text(code_source(list(codes.values())))


if __name__ == "__main__":
    relation_type(Path(__file__).parent / "thinkthen")
