"""Literal named-backend expectations and owned SQL fixture folders."""
import hashlib
import json
import pathlib
import sys

ROWS = json.loads((pathlib.Path(__file__).resolve().parents[1] / "binding-backends.json").read_text())["backends"]
SLOTS = ("generic_systemone", "generic_decisions", "generic_custom", "capture_systemone", "capture_decisions", "capture_custom", "other", "non_post")
QUESTION = '{"decide":"attention?","true":{"what":"yes","examples":["refund"]}}'


def paths(slot, count=1):
    return {**dict.fromkeys(SLOTS, 0), slot: count, "overflow": False}


def configuration(env, backends):
    folder = pathlib.Path(env["HOME"]) / "Library/Application Support/thinkthen" if sys.platform == "darwin" else pathlib.Path(env["XDG_CONFIG_HOME"]) / "thinkthen"
    folder.mkdir(parents=True, exist_ok=True)
    (folder / "config.json").write_text(json.dumps({"schema":"thinkthen.config/1", "backends":backends}))


def alias(row, base):
    return {"url":base, "model":row["model"], "key_env":row["key"],
            "path":"decisions" if row["name"] == "perplexity" else "systemone",
            "both_sides":row["form"] == "both"}


def canonical_replay(folder, row):
    """Handwritten ADR 0111 answer, independent of the host's plan output."""
    folder.mkdir(parents=True, exist_ok=True)
    state = json.dumps("Each question quotes the text it asks about.", separators=(",", ":"))
    question = {"type":"noul", "instructions":'The text is "refund". attention?'}
    question["criteria"] = {"true":("yes" if row["form"] == "text" else {"what":"yes","examples":["refund"]})}
    if row["form"] == "both":
        question["criteria"]["false"] = {}
    question = json.dumps(question, separators=(",", ":"))
    model = json.dumps(row["model"])
    key = hashlib.sha256("\n".join(("systemone", row["url"], model, state, question)).encode()).hexdigest()
    sha = hashlib.sha256(state.encode()).hexdigest()
    answer = {"key":key, "url":row["url"], "model":row["model"], "state":sha,
              "question":question, "answer":'{"type":"noul","noul":0.9}',
              "answered_by":row["model"], "input_tokens":1, "output_tokens":1,
              "taken_at":0, "origin":"live"}
    (folder / "thinkthen.jsonl").write_text(json.dumps({"sha256":sha,"state":state}) + "\n" + json.dumps(answer) + "\n")


def assert_sent(backend, row, marker):
    assert backend.count() == 1
    assert backend.snapshot("paths") == paths(row["path"])
    assert backend.snapshot("bearers") == {"markers":{row["name"]:1,"unnamed":0},"absent":0,"unknown":0,"overflow":False}
    body = json.loads(backend.capture()[0])
    assert body["model"] == row["model"]
    criteria = body["questions"]["q1"]["criteria"]
    assert criteria["true"] == ("yes" if row["form"] == "text" else {"what":"yes","examples":["refund"]})
    assert (criteria.get("false") == {}) if row["form"] == "both" else ("false" not in criteria)
    assert marker not in json.dumps(body)
