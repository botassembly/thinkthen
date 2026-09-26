"""The public names, the question builder, and the verbs' result shapes."""

import ast
import pathlib

import pytest

import thinkthen as tt
from conftest import child_env, run

STUB = pathlib.Path(tt.__file__).with_name("__init__.pyi")


def test_the_module_the_stub_and_all_name_the_same_things():
    """Decision 12: a name added in one place and not the others fails here.
    Names that start with ``_`` are test hooks and stay out."""
    stub = ast.parse(STUB.read_text())
    stubbed = next(node.value for node in stub.body if isinstance(node, ast.Assign)
                   and node.targets[0].id == "__all__")
    public = {name for name, value in vars(tt).items()
              if not name.startswith("_") and type(value).__name__ != "module"}
    assert sorted(tt.__all__) == sorted(ast.literal_eval(stubbed)) == sorted(public)


def test_a_question_takes_one_verb():
    """R3-18: two verbs name both, and an unknown keyword is refused."""
    with pytest.raises(TypeError) as both:
        tt.question(decide="Late?", choose="Which day?", options=["Mon"])
    assert str(both.value) == "question() takes one verb, and both 'decide' and 'choose' are given"
    with pytest.raises(TypeError):
        tt.question(decide="Late?", cut=0.5)


def test_parts_and_a_file_make_the_same_question(tmp_path):
    """Decision 8: both go through the file form, so both ask the same thing."""
    (tmp_path / "q.json").write_text('{"decide": "Late?", "threshold": "0.2:0.8"}')
    assert tt.question(file=tmp_path / "q.json") == tt.question(decide="Late?", threshold=(0.2, 0.8))
    with pytest.raises(tt.LocalError):
        (tmp_path / "bad.json").write_text('{"decide": "  "}')
        tt.question(file=tmp_path / "bad.json")


def test_wrong_questions_are_usage_errors_that_name_the_verb(backend, tmp_path):
    """R6-11 and R6-13: a non-question and a wrong kind raise ``UsageError``
    with the verb named, and send nothing. Parts beside a built question
    would be dropped, so they are refused."""
    printed = run("""
        import thinkthen as tt
        score = tt.question(score="How bad?", levels=["low", "high"])
        for call in (lambda: tt.decide(42, "x"), lambda: tt.decide({"bad": 1}, "x"),
                     lambda: tt.choose(score, "x"), lambda: tt.score(score, "x", levels=["a"]),
                     lambda: tt.find("Which?", ["a", "b"], none="yes")):
            try:
                call()
            except tt.UsageError as error:
                print(error.kind, error.retryable, error)
            except TypeError as error:
                print(error)
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == [
        "usage False decide takes a question text or tt.question(), not a int",
        "usage False decide takes a question text or tt.question(), not a dict",
        "usage False choose does not take a score question",
        "score takes levels only beside a question text",
        "usage False none is True or False",
    ]
    assert backend.count() == 0


def test_the_module_functions_equal_an_explicit_engine(backend, tmp_path):
    """Change 11: one implementation serves both spellings. The module
    functions read the environment's engine, and each answer and shape
    matches ``tt.Engine`` pointed at the same backend."""
    printed = run(f"""
        import json, thinkthen as tt
        engine = tt.Engine(base_url="{backend.base()}", cache=False)
        late = tt.question(decide="Is it late?")
        level = tt.question(score="How bad?", levels=["low", "mid", "high"])
        labels = tt.question(tag="Which apply?", labels=["late", "lost"])
        texts = ["one note", "two notes", "three notes"]
        form = {{"version": 1, "questions": {{"late": {{"decide": "Late?"}}, "day": {{"choose": "Day?", "options": ["Mon", "Tue"]}}}}}}
        calls = [
            lambda on: on.decide(late, "a note"),
            lambda on: on.decide_many(late, texts),
            lambda on: on.choose("Which day?", "a note", options=["Mon", "Tue"]),
            lambda on: on.score(level, "a note"),
            lambda on: on.tag(labels, "a note"),
            lambda on: on.details(late, "a note")["value"],
            lambda on: on.filter("Is it late?", texts),
            lambda on: on.rank("Which is most urgent?", texts, top=2),
            lambda on: on.find("Which says two?", texts),
            lambda on: on.annotate(form, texts[:1]),
            lambda on: [(e.name, e.kind, e.start, e.end) for e in on.recognize("Ada is here", kinds=["person"]).entities],
            lambda on: [(e.relation, e.source.name, e.target.name) for e in on.relate([("Ada", "person"), ("Bo", "person")], relations={{"knows": ("person", "person")}})],
        ]
        for call in calls:
            print(json.dumps(call(tt)))
            assert call(tt) == call(engine)
        print(json.dumps(sorted(tt.usage())))
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == [
        "true",
        "[true, true, true]",
        '"Mon"',
        "0.15",
        '["late", "lost"]',
        "true",
        '["one note", "two notes", "three notes"]',
        '[{"index": 0, "record": "one note", "probability": 0.9}, '
        '{"index": 1, "record": "two notes", "probability": 0.9}]',
        '{"index": 0, "unit": "one note", "probability": 0.9}',
        '[{"late": true, "day": "Mon"}]',
        '[["Ada is here", "person", 0, 11]]',
        '[["knows", "Ada", "Bo"], ["knows", "Bo", "Ada"]]',
        '["cache_answers", "input_tokens", "output_tokens", "requests_sent"]',
    ]


def test_relate_reads_pairs_dicts_and_entities_alike(backend, tmp_path):
    """Decision 10: the three entity forms give one answer, and a bad item
    names its index before anything is sent."""
    printed = run("""
        import thinkthen as tt
        rules = {"knows": ("person", "person")}
        forms = [[("Ada", "person"), ("Bo", "person")],
                 [{"name": "Ada", "kind": "person"}, {"name": "Bo", "kind": "person"}],
                 [tt.Entity("Ada", "person"), tt.Entity("Bo", "person")]]
        print(len({repr(tt.relate(form, relations=rules)) for form in forms}))
        try:
            tt.relate([("Ada", "person"), 7], relations=rules)
        except tt.UsageError as error:
            print(error)
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == [
        "1",
        "entity 1 is not a (name, kind) pair, a dict with name and kind, or an Entity",
    ]


def test_a_forked_child_answers_and_the_parent_counts_nothing(backend, tmp_path):
    """0096 and Q15: the warm process engine answers in a forked child, and
    the child's calls do not move the parent's counters."""
    printed = run("""
        import os, thinkthen as tt
        late = tt.question(decide="Is it late?")
        tt.decide(late, "warm")
        before = tt.usage()
        child = os.fork()
        if child == 0:
            os._exit(0 if tt.decide(late, "in the child") is True else 1)
        _, status = os.waitpid(child, 0)
        print(os.waitstatus_to_exitcode(status), tt.usage() == before)
    """, child_env(backend, tmp_path), timeout=30)
    assert printed.split() == ["0", "True"]
    assert backend.count() == 2
