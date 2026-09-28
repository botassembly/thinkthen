"""The shared engine-settings corpus through DuckDB SQL."""

from __future__ import annotations

import json
import tempfile
from pathlib import Path

from harness import Backend, expect, rows, run, said

def shared_settings_corpus():
    """Run every shared engine-setting row through the DuckDB SQL surface."""
    corpus = json.loads((Path(__file__).resolve().parents[3] / "conformance" / "settings.json").read_text())
    expect(corpus["schema"], "thinkthen.settings-cases/1", "settings corpus version")
    spellings = {"timeout": "thinkthen_timeout", "max_retries": "thinkthen_max_retries",
                 "profile": "thinkthen_profile", "model": "thinkthen_model", "record": "thinkthen_record",
                 "replay": "thinkthen_replay", "max_requests": "thinkthen_max_requests",
                 "max_request_bytes": "thinkthen_max_request_bytes", "cache": "thinkthen_cache"}
    for shared in corpus["cases"]:
        with Backend() as backend, tempfile.TemporaryDirectory() as folder:
            base = backend.base(shared["arm"].removesuffix("/v1"))
            for step in shared["steps"]:
                settings = []
                for name, value in step["settings"].items():
                    if value == "$PROFILE":
                        value = json.dumps(shared["profile"], separators=(",", ":"))
                    elif value == "$FOLDER":
                        value = folder
                    elif value is False:
                        value = "off"
                    literal = str(value) if isinstance(value, int) else "'" + str(value).replace("'", "''") + "'"
                    settings.append(f"SET {spellings[name]} = {literal}")
                if step.get("verb") == "relate":
                    entities = ", ".join(f"({at}, '{one['name']}', '{one['kind']}')"
                                         for at, one in enumerate(shared["entities"]))
                    source = f"SELECT * FROM (VALUES {entities}) v(id, name, kind)"
                    query = f"SELECT count(*) FROM thinkthen_relate('{source.replace(chr(39), chr(39) * 2)}', ['{shared['relation']}'])"
                elif step.get("verb") == "decide_many":
                    values = ", ".join("('" + value.replace("'", "''") + "')" for value in step["records"])
                    query = f"SELECT thinkthen_decide('Is this a refund?', body) FROM (VALUES {values}) t(body)"
                elif "model" in step:
                    query = f"SELECT thinkthen_details('Is this a refund?', '{step['text']}') ->> '$.meta.model'"
                else:
                    query = f"SELECT thinkthen_decide('Is this a refund?', '{step['text']}')"
                got = run([*settings, query], base)[-1]
                label = shared["id"]
                if "error" in step:
                    expect(said(got).startswith(f"thinkthen {step['error']}:"), True, label)
                elif "model" in step:
                    expect(rows(got), [[step["model"]]], label)
                elif step.get("verb") == "relate":
                    expect(rows(got), [[step["edges"]]], label)
                else:
                    expect(rows(got), [[step["value"]]], label)
                if label == "max-requests-refuses-past-the-limit":
                    # DuckDB may stop its concurrent row batch after the first
                    # transport refusal. Two allowed records must still answer.
                    expect(0 < backend.count() <= step["count"], True, f"{label} send ceiling")
                    allowed = ", ".join("('" + value.replace("'", "''") + "')"
                                        for value in step["records"][:2])
                    with Backend() as allowance:
                        got = run(["SET thinkthen_max_requests = 2",
                                   f"SELECT thinkthen_decide('Is this a refund?', body) "
                                   f"FROM (VALUES {allowed}) t(body)"], allowance.base())[-1]
                        expect(rows(got), [[True], [True]], f"{label} two allowed records")
                        expect(allowance.count(), 1, f"{label} one batched transport")
                else:
                    expect(backend.count(), step["count"], f"{label} listener count")
            if "entries" in shared:
                expect(sum(path.is_file() and path.suffix == ".json" and path.name != ".thinkthen-backend.json"
                           for path in Path(folder).rglob("*")),
                       shared["entries"], f"{shared['id']} saved entries")
