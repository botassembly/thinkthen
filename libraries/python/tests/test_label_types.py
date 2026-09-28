"""Python label forms preserve the existing native request identity."""

import json

from conftest import child_env, run
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
        engine = tt.Engine(cache=False)
        digests = []
        def hold(call):
            digests.append([list(call.details[0]["request_digests"]), call.facts.requests_sent])
            return call.value
        plain = ["person", "company"]
        described = {"person": {"what": "a person"}, "company": ["a company"]}
        assert hold(engine.choose("Who?", "Alice", options=plain)) == "person"
        assert hold(tt.choose("Who?", "Alice", options=Kind)) == "person"
        assert hold(engine.choose("Who?", "Alice", options=Literal["person", "company"])) == "person"
        assert hold(engine.choose("Who?", "Alice", options=described)) == "person"
        assert hold(engine.choose("Who?", "Alice", options=Described)) == "person"
        assert hold(engine.choose_many("Who?", ["Alice"], options=Kind, batch=1)) == ["person"]
        assert hold(engine.choose("Who?", "Alice", options=Kind,
                                  descriptions={"company": ["a company"]})) == "person"
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
        assert len(digests) == 16
        groups, offset = [], 0
        for _, count in digests:
            groups.append(bodies[offset:offset + count])
            offset += count
        assert offset == len(bodies)
        for left, right in ((0, 1), (0, 2), (3, 4), (1, 5), (6, 7),
                            (8, 9), (10, 11), (12, 13), (14, 15)):
            assert groups[left] == groups[right], (left, right)
            assert digests[left] == digests[right]
        requests = [json.loads(group[0]) for group in groups]
        assert list(requests[0]["questions"]["q1"]["criteria"]) == ["person", "company"]
        assert requests[3]["questions"]["q1"]["criteria"] == {
            "person": {"what": "a person"}, "company": ["a company"]}
        assert b'"what":"a person"' in b"".join(groups[12])
        assert requests[0]["state"] == "Alice"
        assert backend.count() == 0
