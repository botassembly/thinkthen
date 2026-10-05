"""Named SQLite settings are atomic and capture the environment at first build."""
import json
import pathlib
from helper import Backend, Child, child, environment, expect, main
from backend_cases import ROWS, QUESTION, alias, assert_sent, canonical_replay, configuration


def test_named_aliases_pair_each_provider_key_with_the_declared_path():
    for row in ROWS:
        marker = "fake-sqlite-" + row["name"]
        backend = Backend({row["name"]:marker,"unnamed":"sk-sqlite-loopback"})
        env = environment(backend, **{row["key"]:marker})
        configuration(env, {"local-"+row["name"]:alias(row, backend.base("arm/full/capture"))})
        settings = json.dumps({"backend":"local-"+row["name"],"cache":False})
        got = child(f'''db = connect()
configured = run(db, "SELECT thinkthen_configure(?)", ({settings!r},))
say(answer=run(db, "SELECT thinkthen_details(?, 'refund')", ({QUESTION!r},)))
''', env)
        detail = json.loads(got["answer"][0][0])
        expect(detail["value"], True, row["name"])
        assert marker not in json.dumps(got)
        assert_sent(backend, {**row,"form":"both" if row["form"] == "both" else "authored"}, marker)
        backend.close()


def test_canonical_builtin_replay_selects_url_model_path_and_form_without_sends():
    backend = Backend()
    for row in ROWS:
        env = environment(backend)
        replay = pathlib.Path(env["SCRATCH"]) / "replay"
        canonical_replay(replay, row)
        settings = json.dumps({"backend":row["name"],"cache":False,"replay":str(replay)})
        got = child(f'''db = connect()
run(db, "SELECT thinkthen_configure(?)", ({settings!r},))
say(answer=run(db, "SELECT thinkthen_details(?, 'refund')", ({QUESTION!r},)))
''', env)
        detail = json.loads(got["answer"][0][0])
        expect((detail["value"],detail["meta"]["model"],detail["meta"]["url"]), (True,row["model"],row["url"]), row["name"])
        expect(backend.count(), 0, "canonical replay has no transport")
    backend.close()


def test_replacement_refuses_atomically_and_first_build_captures_later_key():
    backend = Backend({"first":"fake-first","later":"fake-later","unnamed":"sk-sqlite-loopback"})
    env = environment(backend, LOCAL_KEY="fake-first")
    configuration(env, {"local":{"url":backend.base("arm/full/capture"),"key_env":"LOCAL_KEY","model":"retained","path":"judgements/v2/decide"}})
    process = Child('''db = connect()
run(db, "SELECT thinkthen_configure(?)", ('{"backend":"local","cache":false}',))
refused = [run(db, "SELECT thinkthen_configure(?)", (source,)) for source in (
    '{"backend":"nowhere","model":"replaced"}', '{"backend":1}',
    '{"backend":"local","backend":"other"}', '{"base_url":"http://127.0.0.1"}')]
say(refused=refused,usage=run(db,"SELECT thinkthen_usage()"))
input()
os.environ['LOCAL_KEY'] = 'fake-later'
one = run(db, "SELECT thinkthen_details('attention?', 'refund')")
os.environ['LOCAL_KEY'] = 'fake-first'
two = run(connect(), "SELECT thinkthen_details('attention?', 'refund')")
late = run(db,"SELECT thinkthen_configure(?)", ('{"backend":"typesafe"}',))
say(one=one,two=two,late=late)
''', env)
    expect(all(message.startswith("thinkthen usage:") for message in process.read()["refused"]), True, "invalid replacements")
    expect(backend.count(), 0, "usage and refused configuration build no engine")
    process.send()
    got = process.result()
    expect([json.loads(got[name][0][0])["meta"]["model"] for name in ("one","two")], ["retained","retained"], "process configuration")
    assert "this process already built its engine" in got["late"]
    expect(backend.snapshot("bearers"), {"markers":{"first":0,"later":2,"unnamed":0},"absent":0,"unknown":0,"overflow":False}, "first build captures once across connections")
    backend.close()


def test_failed_build_allows_configuration_recovery():
    backend = Backend()
    env = environment(backend, TYPESAFE_API_KEY="fake-typesafe", LOCAL_KEY="fake-recovered")
    configuration(env, {"bad":{"url":"https://api.liquid.ai/decisions/v1","key_env":"TYPESAFE_API_KEY","model":"refused"},
                        "good":{"url":backend.base(),"key_env":"LOCAL_KEY","model":"recovered"}})
    got = child('''db = connect()
run(db,"SELECT thinkthen_configure(?)", ('{"backend":"bad","cache":false}',))
failed = run(db,"SELECT thinkthen_decide('attention?', 'refund')")
configured = run(db,"SELECT thinkthen_configure(?)", ('{"backend":"good","cache":false}',))
say(failed=failed,configured=configured,recovered=run(db,"SELECT thinkthen_decide('attention?', 'refund')"))
''', env)
    assert "never goes to the address" in got["failed"]
    expect(got["recovered"], [[1]], "failed build recovers")
    expect(backend.count(), 1, "only recovered call sends")
    backend.close()


def test_wrong_canonical_path_misses_replay_without_transport():
    backend = Backend()
    env = environment(backend)
    row = next(row for row in ROWS if row["name"] == "perplexity")
    replay = pathlib.Path(env["SCRATCH"]) / "wrong-path"
    canonical_replay(replay,{**row,"url":"https://api.perplexity.ai/v1/systemone"})
    settings = json.dumps({"backend":"perplexity","cache":False,"replay":str(replay)})
    got = child(f'''db = connect()
run(db,"SELECT thinkthen_configure(?)", ({settings!r},))
say(answer=run(db,"SELECT thinkthen_decide(?, 'refund')", ({QUESTION!r},)))
''',env)
    assert got["answer"].startswith("thinkthen local:") and "replay" in got["answer"]
    expect(backend.count(),0,"wrong canonical path never sends")
    backend.close()


def test_selected_setup_profile_survives_model_override_and_explicit_profile_wins():
    backend = Backend()
    env = environment(backend, LOCAL_KEY="fake-profile")
    configuration(env,{"local":{"url":backend.base(),"key_env":"LOCAL_KEY","model":"setup",
                                 "profile":{"schema":"thinkthen.backend-profile/1","name":"small","max_evidence_bytes":3}}})
    for explicit in (False,True):
        settings = {"backend":"local","model":"override","cache":False}
        if explicit:
            settings["profile"] = json.dumps({"schema":"thinkthen.backend-profile/1","name":"explicit","max_evidence_bytes":100})
        source = json.dumps(settings)
        got = child(f'''db = connect()
run(db,"SELECT thinkthen_configure(?)", ({source!r},))
say(answer=run(db,"SELECT thinkthen_details('attention?', 'refund')"))
''',env)
        if explicit:
            expect(json.loads(got["answer"][0][0])["meta"]["model"],"override","model overrides selected setup")
        else:
            assert "profile small" in got["answer"]
        expect(backend.count(),int(explicit),"setup profile and explicit override")
    backend.close()


if __name__ == '__main__':
    raise SystemExit(main(globals()))
