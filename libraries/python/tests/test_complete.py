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
package.__path__ = [str(ROOT / "thinkthen")]
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
    decide = c.decode("DecideResult", FIXTURE["results"][0]["result"])
    assert decide.value is False
    assert decide.question.true_ is None
    assert decide.question.false_["meaning"] == ("no", False)
    choose = c.decode("ChooseResult", FIXTURE["results"][1]["result"])
    assert choose.value is None and choose.answer.confidence == 0
    assert tuple(choose.answer.probabilities) == ("b", "a")
    assert choose.position.images == ("red.png", "blue.png", "red.png")
    assert choose.position.first is c.ABSENT
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
