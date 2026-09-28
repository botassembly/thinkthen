"""The shared engine-setting corpus through the Python keyword boundary."""
import json
import pathlib

import pytest

from conftest import child_env, run

CASES = json.loads((pathlib.Path(__file__).resolve().parents[3] / "conformance/settings.json").read_text())
assert CASES["schema"] == "thinkthen.settings-cases/1"


@pytest.mark.parametrize("case", CASES["cases"], ids=lambda case: case["id"])
def test_shared_engine_setting(case, backend, tmp_path):
    folder = tmp_path / "recording"
    folder.mkdir()
    profile = tmp_path / "profile.json"
    if "profile" in case:
        profile.write_text(json.dumps(case["profile"]))
    for step in case["steps"]:
        settings = {name: (str(folder) if value == "$FOLDER" else
                           str(profile) if value == "$PROFILE" else value)
                    for name, value in step["settings"].items()}
        code = f"""
import json, thinkthen as tt
settings = json.loads({json.dumps(json.dumps(settings))})
try:
    engine = tt.Engine(**settings)
    if {step.get('verb') == 'relate'}:
        entities = {[(one['name'], one['kind']) for one in case.get('entities', [])]!r}
        result = {{'edges': len(engine.relate(entities, relations={{'linked': ('item', 'item')}}).value)}}
    elif {step.get('verb') == 'decide_many'}:
        rows = list(engine.decide_many(tt.question(decide={CASES['question']!r}),
                                       {step.get('records', [])!r}).value)
        result = {{'error': None, 'rows': rows}}
    elif {'model' in step}:
        details = engine.details(tt.question(decide={CASES['question']!r}), {step.get('text', '')!r}).value
        result = {{'value': details['value'], 'model': details['meta']['model']}}
    else:
        result = {{'value': engine.decide(tt.question(decide={CASES['question']!r}),
                                          {step.get('text', '')!r}).value}}
except tt.ThinkThenError as error:
    result = {{'error': error.kind}}
print(json.dumps(result))
"""
        got = json.loads(run(code, child_env(backend, tmp_path, case["arm"].removesuffix("/v1")), timeout=8))
        if "error" in step:
            assert got["error"] == step["error"]
        elif step.get("verb") == "relate":
            assert got["edges"] == step["edges"]
        else:
            assert got["value"] is step["value"]
            if "model" in step:
                assert got["model"] == step["model"]
        assert backend.count() == step["count"]
    if "entries" in case:
        assert len([path for path in folder.glob("*.json") if not path.name.startswith(".thinkthen-")]) == case["entries"]
