"""Constructor backend selection keeps provider paths, forms and captured keys."""
import json
import os
import pathlib
import sys

import pytest

from conftest import Backend, clean_env, private_windows_configuration, run

ROWS = json.loads((pathlib.Path(__file__).resolve().parents[3] / "conformance/binding-backends.json").read_text())["backends"]
SLOTS = ("generic_systemone", "generic_decisions", "generic_custom", "capture_systemone", "capture_decisions", "capture_custom", "other", "non_post")


def paths(slot, count=1):
    return {**dict.fromkeys(SLOTS, 0), slot: count, "overflow": False}


def isolated(folder, **extra):
    return clean_env(HOME=str(folder), XDG_CONFIG_HOME=str(folder / "config"),
                     XDG_CACHE_HOME=str(folder / "cache"), XDG_STATE_HOME=str(folder / "state"),
                     **({"APPDATA":str(folder / "config"), "LOCALAPPDATA":str(folder / "local")} if os.name == "nt" else {}), **extra)


def configuration(folder, value):
    directory = folder / ("Library/Application Support/thinkthen" if sys.platform == "darwin" else "config/thinkthen")
    directory.mkdir(parents=True, exist_ok=True)
    (directory / "config.json").write_text(json.dumps({"schema": "thinkthen.config/1", **value}))
    if sys.platform == "win32":
        private_windows_configuration(directory / "config.json")


@pytest.mark.parametrize("row", ROWS, ids=lambda row: row["name"])
def test_constructor_selects_provider_key_model_path_and_form(row, tmp_path):
    marker = "fake-binding-" + row["name"]
    backend = Backend({row["name"]: marker, "unnamed": "fake-unnamed"})
    try:
        base = backend.base("arm/full/capture")
        env = isolated(tmp_path, THINKTHEN_API_KEY="fake-unnamed", THINKTHEN_BASE_URL=backend.base("arm/status/500"),
                       **{row["key"]: marker})
        output = run(f'''
            import json, os, thinkthen as tt
            engine = tt.Engine(backend={row['name']!r}, base_url={base!r}, cache=False)
            os.environ[{row['key']!r}] = 'fake-later'
            question = tt.question(decide='Does it need attention?', true={{'what':'yes', 'examples':['refund']}})
            print(json.dumps({{'value': engine.decide(question, 'refund').value, 'repr': repr(engine)}}))
        ''', env)
        assert json.loads(output)["value"] is True
        assert marker not in output and "fake-unnamed" not in output
        assert backend.count() == 1
        assert backend.snapshot("paths") == paths(row["path"])
        assert backend.snapshot("bearers") == {"markers": {row["name"]: 1, "unnamed": 0}, "absent": 0, "unknown": 0, "overflow": False}
        body = json.loads(backend.snapshot("capture")["bodies"][0])
        assert body["model"] == row["model"]
        criteria = body["questions"]["q1"]["criteria"]
        assert criteria["true"] == ("yes" if row["form"] == "text" else {"what": "yes", "examples": ["refund"]})
        assert (criteria["false"] == {}) if row["form"] == "both" else ("false" not in criteria)
    finally:
        backend.close()


def test_constructor_backend_errors_send_nothing(backend, tmp_path):
    output = run('''
        import json, thinkthen as tt
        from thinkthen import _thinkthen
        failures = []
        for constructor in (tt.Engine, _thinkthen._Engine):
            for value in (1, True, [], {}, '', 'nowhere'):
                try:
                    constructor(backend=value, cache=False)
                except tt.UsageError as error:
                    failures.append(str(error))
                else:
                    raise AssertionError('backend must refuse')
        print(json.dumps(failures))
    ''', isolated(tmp_path, THINKTHEN_API_KEY="fake-unnamed", THINKTHEN_BASE_URL=backend.base()))
    failures = json.loads(output)
    assert failures[:4] == ["backend is text"] * 4
    assert failures[6:10] == ["backend is text"] * 4
    assert "the built-in backends are `liquid`, `llamacpp`, `mlx`, `ollama`, `openrouter`, `perplexity` and `typesafe`" in failures[5]
    assert backend.count() == 0


def test_selected_setup_survives_overrides_and_pickle_recaptures_keys(tmp_path):
    backend = Backend({"first": "fake-first", "second": "fake-second"})
    try:
        base = backend.base("arm/full/capture")
        configuration(tmp_path, {"usd_per_million_input":"9", "usd_per_million_output":"9", "backends": {
            "local": {"url": backend.base(), "model":"setup", "key_env":"LOCAL_KEY", "path":"judgements/v2/decide",
                      "usd_per_million_input":"1", "usd_per_million_output":"2",
                      "profile":{"schema":"thinkthen.backend-profile/1", "name":"small", "max_evidence_bytes":3}}}})
        profile = tmp_path / "profile.json"
        profile.write_text(json.dumps({"schema":"thinkthen.backend-profile/1", "name":"explicit", "max_evidence_bytes":100}))
        output = run(f'''
            import json, os, pickle, thinkthen as tt
            first = tt.Engine(backend='local', base_url={base!r}, model='override', cache=False)
            try:
                first.decide('attention?', 'refund')
            except tt.UsageError as error:
                assert 'profile small' in str(error)
            else:
                raise AssertionError('setup profile must refuse')
            engine = tt.Engine(backend='local', base_url={base!r}, model='override', cache=False, profile={str(profile)!r})
            judge = engine.decide('attention?')
            frozen = pickle.dumps(judge)
            assert b'fake-first' not in frozen and b'fake-second' not in frozen
            assert 'fake-first' not in str(engine) + repr(engine) + engine._settings_json
            os.environ['LOCAL_KEY'] = 'fake-second'
            one = judge('refund')
            two = pickle.loads(frozen)('refund')
            print(json.dumps({{'values':[one.value,two.value], 'cost':one.facts['estimated_cost_usd']}}))
        ''', isolated(tmp_path, LOCAL_KEY="fake-first", THINKTHEN_API_KEY="fake-unnamed"))
        assert json.loads(output) == {"values": [True, True], "cost": "0.000003"}
        assert backend.count() == 2
        assert backend.snapshot("paths") == paths("capture_custom", 2)
        assert backend.snapshot("bearers") == {"markers":{"first":1,"second":1},"absent":0,"unknown":0,"overflow":False}
        assert all(json.loads(body)["model"] == "override" for body in backend.snapshot("capture")["bodies"])
    finally:
        backend.close()


def test_successful_wrong_provider_path_fails_the_count_expectation():
    import http.client
    backend = Backend({"provider": "fake-provider"})
    try:
        body = '{"state":"refund","model":"pplx-decider-v1-27b","questions":{"q1":{"type":"noul","instructions":"attention?"}}}'
        connection = http.client.HTTPConnection("127.0.0.1", backend.port)
        connection.request("POST", "/arm/full/capture/v1/systemone", body,
                           {"Authorization": "Bearer fake-provider"})
        response = connection.getresponse()
        assert response.status == 200
        response.read()
        connection.close()
        assert backend.count() == 1
        assert backend.snapshot("capture") == {"bodies": [body]}
        assert backend.snapshot("bearers") == {"markers":{"provider":1},"absent":0,"unknown":0,"overflow":False}
        with pytest.raises(AssertionError, match="posting_path_mismatch"):
            assert backend.snapshot("paths") == paths("capture_decisions"), "posting_path_mismatch"
    finally:
        backend.close()


def test_a_named_loopback_backend_never_uses_the_unnamed_key(tmp_path):
    backend = Backend({"unnamed": "fake-unnamed"})
    try:
        output = run(f'''
            import thinkthen as tt
            engine = tt.Engine(backend="ollama", base_url={backend.base()!r}, cache=False)
            print(engine.decide('attention?', 'refund').value)
        ''', isolated(tmp_path, THINKTHEN_API_KEY="fake-unnamed"))
        assert output.strip() == "True"
        assert backend.count() == 1
        assert backend.snapshot("bearers") == {"markers":{"unnamed":0},"absent":1,"unknown":0,"overflow":False}
    finally:
        backend.close()
