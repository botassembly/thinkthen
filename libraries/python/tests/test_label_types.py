"""Python label forms preserve the existing native request identity."""

import json

from conftest import child_env, question_keys, run
from test_call import capturing_filter_listener


def test_checked_enum_example(backend, tmp_path):
    from pathlib import Path

    example = json.loads((Path(__file__).resolve().parents[1] / "examples.json").read_text())
    chosen = example["examples"]["choose_enum"]
    code = "import thinkthen as tt\n" + chosen["imports"] + "\nprint(repr(" + chosen["python"] + "))"
    assert run(code, child_env(backend, tmp_path)).strip() == chosen["expected"]


def _answer(question):
    if question["type"] == "noul":
        return {"type": "noul", "noul": 0.9}
    if question["type"] == "choice":
        names = list(question["criteria"])
        return {"type": "choice", "probabilities": {
            name: float(index == 0) for index, name in enumerate(names)}}
    names = [str(index) for index in range(len(question["criteria"]))]
    return {"type": "score", "probabilities": {
        name: float(index == 0) for index, name in enumerate(names)}}


def test_label_forms_and_recognize_keywords_keep_captured_identity(backend, tmp_path):
    with capturing_filter_listener(_answer) as (url, bodies):
        env = child_env(backend, tmp_path)
        env["THINKTHEN_BASE_URL"] = url.removesuffix("/systemone")
        printed = run('''
        import json
        from enum import Enum
        from typing import Literal
        import thinkthen as tt
        class Kind(Enum):
            person = "person"
            company = "company"
        class Described(Enum):
            person = "person"
            company = "company"
            @property
            def description(self):
                return {"what": "a person"} if self.name == "person" else ["a company"]
        class Alias(Enum):
            first = "same"
            second = "same"
        class MixedScore(Enum):
            low = "low"
            high = "high"
            @property
            def description(self):
                return None if self.name == "low" else "very high"
        engine = tt.Engine(cache=False)
        digests = []
        def hold(call):
            digests.append([list(call.details[0]["requests"]), call.facts["requests_sent"]])
            return call.value
        plain = ["person", "company"]
        described = {"person": {"what": "a person"}, "company": ["a company"]}
        assert hold(engine.choose("Who?", "Alice", options=plain)) == "person"
        assert hold(tt.choose("Who?", "Alice", options=Kind)) == "person"
        assert hold(engine.choose("Who?", "Alice", options=Literal["person", "company"])) == "person"
        assert hold(engine.choose("Who?", "Alice", options=described)) == "person"
        assert hold(engine.choose("Who?", "Alice", options=Described)) == "person"
        assert hold(engine.choose("Who?", ["Alice"], options=Kind, batch=1)) == ["person"]
        assert hold(engine.choose("Who?", "Alice", options={"person": None,
                                                              "company": ["a company"]})) == "person"
        hold(engine.recognize("Alice", kinds=plain)).entities
        hold(engine.recognize("Alice", ask={"version": 1,
                     "recognize": {"kinds": {"person": None, "company": None}}})).entities
        rules = {"works_for": ("person", "company")}
        hold(tt.recognize("Alice", kinds=Kind, relations=rules,
                    either=["works_for"], threshold=0.6, relation_threshold=0.7)).entities
        hold(engine.recognize("Alice", ask={"version": 1,
                    "recognize": {"kinds": {"person": None, "company": None},
                                  "relations": [{"name": "works_for", "source": "person",
                                                 "target": "company", "either": True}]},
                    "threshold": 0.6, "relation_threshold": 0.7})).entities
        hold(engine.recognize("Alice", kinds=Described)).entities
        hold(engine.recognize("Alice", ask={"version": 1,
                    "recognize": {"kinds": described}})).entities
        hold(engine.recognize("Alice")).entities
        hold(engine.recognize("Alice", ask={"version": 1,
                    "recognize": {"kinds": {}}})).entities
        assert hold(engine.score("Urgency?", "Alice", levels=MixedScore)) == 0.0
        assert hold(engine.score("Urgency?", "Alice", levels={
            "low": "low", "high": "very high"})) == 0.0
        assert hold(engine.score("Urgency?", "Alice", levels={
            "low": None, "high": "very high"})) == 0.0
        for invalid in (lambda: engine.choose("Who?", "Alice", options=Kind,
                                             descriptions={"unknown": "x"}),
                        lambda: engine.choose("Who?", "Alice", options=Alias),
                        lambda: engine.choose("Who?", "Alice", options=Literal["a", "a"]),
                        lambda: engine.recognize("Alice", kinds=Kind,
                                                 descriptions={"unknown": "x"})):
            try:
                invalid()
            except tt.UsageError:
                pass
            else:
                raise AssertionError("invalid labels sent or passed")
        print(json.dumps(digests))
        ''', env)
        digests = json.loads(printed)
        assert len(digests) == 18
        groups, offset = [], 0
        for _, count in digests:
            groups.append(bodies[offset:offset + count])
            offset += count
        assert offset == len(bodies)
        for left, right in ((0, 1), (0, 2), (3, 4), (1, 5),
                            (7, 8), (9, 10), (11, 12), (13, 14), (15, 16)):
            assert groups[left] == groups[right], (left, right)
            assert digests[left] == digests[right]
        requests = [json.loads(group[0]) for group in groups]
        assert list(requests[0]["questions"]["q1"]["criteria"]) == ["person", "company"]
        assert requests[3]["questions"]["q1"]["criteria"] == {
            "person": {"what": "a person"}, "company": ["a company"]}
        assert b'"what":"a person"' in b"".join(groups[11])
        assert requests[0]["state"] == "Each question quotes the text it asks about."
        expected = (b'{"state":"Each question quotes the text it asks about.",'
                    b'"model":"jev-1.13.0","questions":'
                    b'{"q1":{"type":"score","instructions":"The text is \\"Alice\\". Urgency?",'
                    b'"criteria":["low","very high"]}}}')
        assert groups[15] == groups[16] == [expected]
        digest = question_keys(url, expected)[0]
        assert digests[15] == digests[16] == [[digest], 1]
        assert json.loads(groups[17][0])["questions"]["q1"]["criteria"] == [{}, "very high"]
        assert digests[17][0] != [digest]
        assert backend.count() == 0
