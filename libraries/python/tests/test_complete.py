"""Private carrier contract tests, independent of native result/2 execution."""
import copy
import importlib.util
import json
from pathlib import Path
import sys
import types
import pytest

ROOT = Path(__file__).resolve().parents[1]
# Load the pure carrier modules without importing a native extension.
package = types.ModuleType("complete_carriers")
package.__path__ = list(importlib.util.find_spec("thinkthen").submodule_search_locations)
sys.modules[package.__name__] = package
from complete_carriers import _complete as c, _requests as r
FIXTURE = json.loads((ROOT / "tests/fixtures/complete.json").read_text())


def test_all_ten_results_keep_typed_answers_and_original_values():
    for row in FIXTURE["results"]:
        result = c.decode(row["type"], row["result"])
        assert isinstance(result.answer_id, c.AnswerId)
        assert isinstance(result.meta.observations[0].observation_id, c.ObservationId)
        assert result.meta.requests == ("e" * 64, "e" * 64)
        assert c.to_json(result) == row["result"]
        assert "reported" not in repr(result)
        occurrence = c.decode(row["type"], {**row["result"], "index": 0})
        assert occurrence.index == 0
        with pytest.raises(ValueError): c.decode(row["type"], {**row["result"], "index": -1})
    decide = c.decode("DecideResult", FIXTURE["results"][0]["result"])
    assert decide.value is False
    assert decide.question.true_ is None
    assert decide.question.false_["meaning"] == ("no", False)
    choose = c.decode("ChooseResult", FIXTURE["results"][1]["result"])
    assert choose.value is None and choose.answer.confidence == 0
    assert tuple(choose.answer.probabilities) == ("b", "a")
    assert choose.position.images == ("red.png", "blue.png", "red.png")
    assert choose.position.first is c.ABSENT
    candidate = c.decode("FindCandidate", {"index": 0, "input": False, "probability": .25, "source": {"file": "é.txt", "first_line": 2, "last_line": 2}})
    assert candidate.input is False and candidate.source.first_line == 2
    assert candidate.probability == .25
    recognized = c.decode("RecognizeResult", FIXTURE["results"][8]["result"])
    assert recognized.value.entities[0].end == 2
    assert recognized.answer.names[0].edges is None
    related = c.decode("RelateResult", FIXTURE["results"][9]["result"])
    assert related.value[0].source.file == related.value[0].target.file == "é.txt"
    assert isinstance(related.answer.questions[1], c.RelationFailure)
    annotate = c.decode("AnnotateResult", FIXTURE["results"][7]["result"])
    assert annotate.answers["ok"].value is None
    assert annotate.answers["bad"].failure.cause == "missing_answer"
    empty = c.decode("RelateResult", FIXTURE["empty"]["result"])
    assert empty.meta.origin is None and empty.meta.answered_by is c.ABSENT
    assert empty.value == () and empty.meta.cached is False


@pytest.mark.parametrize("meaning", [False, True, None, "private-reading", {"meaning": ["private-reading", False]}, ["private-reading", {"nested": None}]])
def test_decide_complete_preserves_authored_meanings_without_exposing_them(meaning):
    row = copy.deepcopy(FIXTURE["results"][0]["result"])
    row["value"] = meaning
    result = c.decode("DecideResult", row)
    assert c.to_json(result) == row
    assert "private-reading" not in repr(result)


def test_ids_facts_and_started_failure_have_no_synthetic_defaults():
    facts = c.decode("Facts", FIXTURE["facts"])
    assert isinstance(facts.call_id, c.CallId)
    assert facts.command_ms is c.ABSENT and facts.model is c.ABSENT
    for error in FIXTURE["errors"]:
        assert c.decode("CallError", error).facts is c.ABSENT
    failed = c.decode("CallError", FIXTURE["started_error"])
    assert failed.attempts[0].server_ms == 0
    assert isinstance(failed.attempts[0].sdk_request_id, c.SdkRequestId)
    for invalid in ("A" * 64, "a" * 63, " a" * 32, 0):
        with pytest.raises(ValueError, match="invalid identity"): c.AnswerId(invalid)


@pytest.mark.parametrize("change", [
    {"schema": "thinkthen.result/1"}, {"answer_id": "a" * 63},
    {"value": 0}, {"answer": {"kind": "yes_no", "probability": True}},
    {"position": {"file": "x", "first": 4}}, {"proxy": None},
])
def test_invalid_complete_results_refuse_without_exposing_input(change):
    row = copy.deepcopy(FIXTURE["results"][0]["result"])
    row.update(change)
    with pytest.raises(ValueError, match="invalid"): c.decode("DecideResult", row)


def test_inconsistent_provenance_is_not_repaired():
    for change in ({"origin": "proxy"}, {"cached": False}, {"answered_by": "invented"},
                   {"observations": []}, {"failed_questions": 1}):
        meta = copy.deepcopy(FIXTURE["results"][0]["result"]["meta"])
        meta.update(change)
        with pytest.raises(ValueError, match="invalid"): c.decode("Meta", meta)


def test_named_requests_preserve_descriptions_payloads_files_and_duplicate_images():
    q = c.decode("DecideSpec", {"decide": ["Q", {"active": False}], "false": None, "on": ["/body"]})
    records = c.decode("RecordInput", {"records": [False, None, {"id": 1}, {"id": 1}], "context": {"context": []}})
    choices = c.decode("ChooseSpec", {"choose": "Q", "options": {"b": None, "a": {"nested": [False]}}})
    levels = c.decode("ScoreSpec", {"score": "Q", "levels": ["low", "high"]})
    tags = c.decode("TagSpec", {"tag": "Q", "labels": ["a"]})
    find = c.decode("FindSpec", {"find": "Q", "none": True})
    members = c.decode("QuestionSet", {"version": 1, "questions": {"a": c.to_json(q)}})
    recognition = c.decode("RecognitionSpec", {"version": 1, "recognize": {"kinds": {"person": {"nested": [False]}}}})
    relation = c.decode("RelationSpec", {"version": 1, "relate": {"relations": [{"name": "knows", "source": "*", "target": "*"}]}})
    for builder, spec in ((r.decide, q), (r.choose, choices), (r.tag, tags), (r.score, levels),
                          (r.filter, q), (r.rank, levels), (r.find, find), (r.annotate, members),
                          (r.recognize, recognition), (r.relate, relation)):
        request = builder(spec, records)
        assert c.to_json(request.input)["records"] == [False, None, {"id": 1}, {"id": 1}]
        assert request.question_json() == c.to_json(spec)
    image_bytes = bytearray([1, 2, 3])
    images = c.decode("ImageInput", {"images": [{"data": image_bytes, "name": "red.png"}, {"data": b"b"}, {"data": image_bytes, "name": "red.png"}], "text": None})
    request = r.decide(q, images)
    image_bytes[0] = 99
    assert request.input.images[0].data == request.input.images[2].data == b"\x01\x02\x03"
    assert request.input.text is None
    files = c.decode("Files", {"paths": ["x", "x"], "unit": "file", "media": "image"})
    assert r.choose(choices, files).input.paths == ("x", "x")
    for builder, spec in ((r.tag, tags), (r.filter, q), (r.rank, q), (r.find, find),
                          (r.annotate, members), (r.recognize, recognition), (r.relate, relation)):
        with pytest.raises(ValueError, match="text-only"): builder(spec, images)
    named = r.decide(c.decode("QuestionFile", {"path": "question.json"}), records)
    with pytest.raises(ValueError, match="native loader"): named.question_json()


def test_input_carriers_keep_native_grammar_boundaries():
    for kind, body in (
        ("DecideSpec", {"decide": False}),
        ("ChooseSpec", {"choose": "Q", "options": {"a": True}}),
        ("ChooseSpec", {"choose": "Q", "options": ["a", "b"], "threshold": 0}),
        ("QuestionSet", {"version": 1, "questions": {"ready": {"decide": "Q", "profile": "other"}}}),
    ):
        with pytest.raises(ValueError, match="invalid"): c.decode(kind, body)
    assert c.to_json(c.decode("DecideSpec", {"decide": {}, "on": "/body", "false": None})) == {"decide": {}, "on": "/body", "false": None}
    corpus = json.loads((ROOT.parents[1] / "specification/fixtures/question-file/corpus.json").read_text())
    for case in corpus["cases"]:
        if case["valid"]:
            kind = "RelationSpec" if case["verb"] == "relate" else case["verb"].title() + "Spec"
            assert c.to_json(c.decode(kind, case["file"])) == case["file"]


def test_rank_members_preserve_order_positions_partial_usage_and_closed_children():
    parent = copy.deepcopy(FIXTURE['results'][5]['result'])
    judgment = copy.deepcopy(FIXTURE['results'][0]['result'])
    child = {k: judgment[k] for k in ('schema', 'answer_id', 'question', 'answer', 'meta')}
    child.update(value=3, threshold=None, source={'file':'é.txt','first_line':2,'last_line':2})
    child['meta']['usage'] = {'input_tokens':2}
    parent.update(question=child['question'],answer=child['answer'],question_name='saved',members=[{'name':'saved','result':child}])
    ranked = c.decode('RankResult',parent)
    assert ranked.members[0].result.value == 3
    assert ranked.members[0].result.meta.usage.output_tokens is c.ABSENT
    assert ranked.members[0].result.source.first_line == 2
    assert c.to_json(ranked) == parent
    for changes in ({'value':0}, {'value':-1}, {'input':False}, {'members':[]}, {'question':{'verb':'score','text':'Q','levels':['x']}}):
        invalid = copy.deepcopy(parent)
        invalid['members'][0]['result'].update(changes)
        with pytest.raises(ValueError,match='invalid'): c.decode('RankResult',invalid)
    for members in ([],None,[{'name':'saved'}]):
        with pytest.raises(ValueError,match='invalid'): c.decode('RankResult',{**parent,'members':members})
    with pytest.raises(ValueError,match='invalid'): c.decode('Usage',{})


def test_current_facts_preserve_request_measurements_and_persistence():
    for raw in FIXTURE['observed_facts']:
        facts = c.decode('Facts', raw)
        assert facts.largest_request_bytes == raw['largest_request_bytes']
        assert facts.largest_request_estimated_input_tokens == raw['largest_request_estimated_input_tokens']
        assert facts.token_estimate_method == raw['token_estimate_method']
        assert facts.usage_persistence.state == raw['usage_persistence']['state']
        assert facts.usage_persistence.observed_at == 'facts_snapshot'
        assert facts.usage_persistence.advice == raw['usage_persistence'].get('advice', c.ABSENT)
        assert c.to_json(facts) == raw
