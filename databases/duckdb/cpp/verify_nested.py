"""Focused stock-host boundary check for both nested-output scalars."""

from __future__ import annotations

import argparse
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
from harness import Backend, run  # noqa: E402  the surface's loopback child


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--extension", required=True, type=Path)
    extension = parser.parse_args().extension.resolve(strict=True)
    rules = ('{"version":1,"recognize":{"kinds":{"person":null},'
             '"relations":[{"name":"near","source":"person","target":"person"}]}}')
    with Backend() as backend:
        names = run(
            ["SELECT thinkthen_recognize('Maria Chen arrived.', ['person'])",
             "SELECT thinkthen_recognize(NULL, ['person'])"],
            backend.base(), extension=extension,
        )
        entities = names[0]["rows"][0][0]
        assert entities[0] == {"text": "Maria Chen", "start": 0, "end": 10, "length": 10,
                               "kind": "person", "strength": 0.6736}
        assert names[1] == {"rows": [[None]]}
        after_names = backend.count()
        assert after_names > 0, "recognize reached the loopback engine"
        bad_kinds = run(
            ["PREPARE bad_kinds AS SELECT thinkthen_recognize('Maria Chen', ['person', 'person'])",
             "SELECT thinkthen_recognize('Maria Chen', kinds) FROM "
             "(VALUES (['person']), (['person', 'person'])) AS x(kinds)",
             "SELECT thinkthen_recognize('Maria Chen', ['person', NULL])"],
            backend.base(), extension=extension,
        )
        assert "thinkthen usage:" in bad_kinds[0]["error"]
        assert "thinkthen usage:" in bad_kinds[1]["error"]
        assert bad_kinds[2] == {"rows": [[None]]}
        assert backend.count() == after_names, "invalid or NULL kinds sent no request"
        related = run(
            [f"SELECT thinkthen_relations('Maria Chen arrived.', '{rules}')",
             f"SELECT thinkthen_relations(NULL, '{rules}')",
             f"SELECT thinkthen_relations('Maria Chen arrived.', '{rules}', NULL)"],
            backend.base(), extension=extension,
        )
        edges = related[0]["rows"][0][0]
        assert len(edges) == 2
        assert edges[0] == {"relation": "near", "source": "Maria Chen", "source_kind": "person",
                            "target": "arrived.", "target_kind": "person", "probability": 0.9,
                            "either": False}
        assert related[1] == {"rows": [[None]]}
        assert related[2] == related[0], "NULL optional settings omit settings"
        after_edges = backend.count()
        assert after_edges > after_names, "relations reached the loopback engine"
        invalid_rules = run(
            ["PREPARE bad_rules AS SELECT thinkthen_relations('Maria Chen', 'not JSON')",
             f"SELECT thinkthen_relations('Maria Chen', r) FROM "
             f"(VALUES ('{rules}'), ('not JSON')) AS x(r)"],
            backend.base(), extension=extension,
        )
        assert "thinkthen usage:" in invalid_rules[0]["error"]
        assert "thinkthen usage:" in invalid_rules[1]["error"]
        assert backend.count() == after_edges, "invalid later rules sent no partial chunk"
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder) / "rules.json"
            path.write_text(rules)
            from_file = run(
                [f"SELECT thinkthen_relations('Maria Chen arrived.', '@{path}')",
                 "SET enable_external_access = false",
                 f"SELECT thinkthen_relations('Maria Chen arrived.', '@{path}')"],
                backend.base(), extension=extension,
            )
            assert len(from_file[0]["rows"][0][0]) == 2
            assert "file settings refuse it" in from_file[2]["error"]
            assert backend.count() > after_edges
    print("C++ recognize and relations nested values, bind, NULL, file, and zero-send boundaries pass")


if __name__ == "__main__":
    main()
