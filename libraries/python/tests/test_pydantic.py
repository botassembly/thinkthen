"""The optional model adapter shares the native question and result contract."""

import json

import pytest

from conftest import child_env, run
from test_call import capturing_filter_listener
from test_label_types import _answer


def test_optional_authoring_and_strict_row_validation(backend, tmp_path):
    with capturing_filter_listener(_answer) as (url, bodies):
        env = child_env(backend, tmp_path)
        env["THINKTHEN_BASE_URL"] = url.removesuffix("/systemone")
        printed = run('''
        import json
        from typing import Annotated, Literal
        from pydantic import BaseModel, Field, ValidationError
        import thinkthen as tt
        from thinkthen.pydantic import row_model
        class Labels(BaseModel):
            first: int = Field(description="first meaning")
            second: int = Field(description="second meaning")
        class Questions(BaseModel):
            open: bool | None = Field(description="Open?")
            kind: Literal["first", "second"] | None = Field(description="Which kind?")
            score: float = Field(description="Severity?", json_schema_extra={
                "levels": ["low", "high"]})
            tags: list[Literal["first", "second"]] = Field(description="What tags?")
        class WrongLabels(BaseModel):
            kind: Literal["first", "second"] | None = Field(
                description="Which kind?", json_schema_extra={"options": ["third", "fourth"]})
        class OptionalScore(BaseModel):
            score: float | None = Field(description="Severity?", json_schema_extra={
                "levels": ["low", "high"]})
        class OptionalTags(BaseModel):
            tags: list[Literal["first", "second"]] | None = Field(description="What tags?")
        class BareDecide(BaseModel):
            open: bool = Field(description="Open?")
        class BareChoose(BaseModel):
            kind: Literal["first", "second"] = Field(description="Which kind?")
        engine = tt.Engine(cache=False)
        taken = []
        def hold(call):
            taken.append([list(call.details[0]["requests"]), call.facts["requests_sent"]])
            return call.value
        assert hold(engine.choose("Which?", "Alice", options=Labels)) == "first"
        assert hold(engine.choose("Which?", "Alice", options={
            "first": "first meaning", "second": "second meaning"})) == "first"
        one = Annotated[Literal["first"], Field(description="first meaning")]
        two = Annotated[Literal["second"], Field(description="second meaning")]
        assert hold(engine.choose("Which?", "Alice", options=(one, two))) == "first"
        assert hold(engine.choose("Which?", "Alice", options={
            "first": "first meaning", "second": "second meaning"})) == "first"
        shared = Annotated[Literal["first", "second"], Field(description="same meaning")]
        assert hold(engine.choose("Which?", "Alice", options=shared)) == "first"
        assert hold(engine.choose("Which?", "Alice", options={
            "first": "same meaning", "second": "same meaning"})) == "first"
        batch = engine.annotate(Questions, ["Alice"], batch=1)
        taken.append([list(batch.details[0]["requests"]), batch.facts["requests_sent"]])
        row = row_model(Questions)
        assert row.model_validate(batch.value[0]).kind == "first"
        assert row.model_validate({"open": None, "kind": None, "score": 0.5,
                                   "tags": []}).kind is None
        failed = {"failed": {"kind": "backend", "cause": "no reply"}}
        assert row.model_validate({"open": failed, "kind": failed,
                                   "score": failed, "tags": failed}).score.failed.cause == "no reply"
        for bad in ({"open": "yes", "kind": "first", "score": 0.5, "tags": []},
                    {"open": True, "kind": "third", "score": 0.5, "tags": []},
                    {"open": True, "kind": "first", "score": "0.5", "tags": []},
                    {"open": True, "kind": "first", "score": 0.5,
                     "tags": [], "extra": 1}):
            try:
                row.model_validate(bad)
            except ValidationError:
                pass
            else:
                raise AssertionError("a wrong row was coerced")
        for invalid in (lambda: engine.choose("Which?", "Alice", options=Labels(first=1, second=2)),
                        lambda: engine.annotate(Annotated[Questions,
                             Field(description="root")], ["Alice"]),
                        lambda: engine.annotate(WrongLabels, ["Alice"]),
                        lambda: engine.annotate(OptionalScore, ["Alice"]),
                        lambda: row_model(OptionalScore),
                        lambda: engine.annotate(OptionalTags, ["Alice"]),
                        lambda: row_model(OptionalTags),
                        lambda: engine.annotate(BareDecide, ["Alice"]),
                        lambda: row_model(BareDecide),
                        lambda: engine.annotate(BareChoose, ["Alice"]),
                        lambda: row_model(BareChoose)):
            try:
                invalid()
            except tt.UsageError:
                pass
            else:
                raise AssertionError("an instance or root Field was accepted")
        print(json.dumps(taken))
        ''', env)
        taken = json.loads(printed)
        groups, offset = [], 0
        for _, count in taken:
            groups.append(bodies[offset:offset + count])
            offset += count
        assert offset == len(bodies)
        assert groups[0] == groups[1] == groups[2] == groups[3]
        assert taken[0] == taken[1] == taken[2] == taken[3]
        assert groups[4] == groups[5] and taken[4] == taken[5]
        assert len(groups[6]) == 1
        assert list(json.loads(groups[6][0])["questions"]) == ["q1", "q2", "q3", "q4", "q5"]
        assert backend.count() == 0


def test_plain_import_does_not_import_pydantic(backend, tmp_path):
    printed = run('''
    import importlib.abc, sys
    class Block(importlib.abc.MetaPathFinder):
        def find_spec(self, name, path=None, target=None):
            if name == "pydantic" or name.startswith("pydantic."):
                raise ImportError("blocked optional extra")
    sys.meta_path.insert(0, Block())
    import thinkthen as tt
    assert "pydantic" not in sys.modules
    assert tt.question(choose="Which?", options=["a", "b"]).kind == "choose"
    try:
        import thinkthen.pydantic
    except ImportError as error:
        assert "thinkthen[pydantic]" in str(error)
    else:
        raise AssertionError("the missing optional extra imported")
    print("plain import")
    ''', child_env(backend, tmp_path))
    assert printed.strip() == "plain import"
    assert backend.count() == 0


def test_row_model_file_uses_native_validation_and_local_errors(tmp_path):
    import thinkthen as tt
    from thinkthen.pydantic import row_model

    valid = tmp_path / "questions.json"
    valid.write_text('{"version":1,"questions":{"open":{"decide":"Open?"}}}')
    assert row_model(valid).model_validate({"open": None}).open is None
    valid.write_text('{"version":1,"questions":{"open":{"decide":" "}}}')
    with pytest.raises(tt.LocalError):
        row_model(valid)
    with pytest.raises(tt.UsageError):
        row_model({"version": 1, "questions": {"open": {"decide": " "}}})
