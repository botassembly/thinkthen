"""Selected installed SQL boundaries for complete listed questions."""

import hashlib
import json
import tempfile
from pathlib import Path

from harness import Backend, expect, rows, run, said


SPECS = (
    '{"choose":"Which team?","options":{"billing":"Handles charges","shipping":null},"threshold":0.8,"model":"jev-1.13.0","profile":"measured-profile"}',
    '{"score":"How strong?","levels":{"weak":null,"strong":"Strong evidence"},"model":"jev-1.13.0"}',
    '{"tag":"Which topics?","labels":{"billing":"Charges","shipping":null},"threshold":0.5,"model":"jev-1.13.0"}',
)
CANONICAL = (
    '{"verb":"choose","text":"Which team?","options":{"billing":"Handles charges","shipping":null},"threshold":0.8,"profile":"measured-profile"}',
    '{"verb":"score","text":"How strong?","levels":{"weak":null,"strong":"Strong evidence"}}',
    '{"verb":"tag","text":"Which topics?","labels":{"billing":"Charges","shipping":null},"threshold":0.5}',
)
EXPECTED = (
    '{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"The text is \\"charged twice\\". Which team?","criteria":{"billing":"Handles charges","shipping":null}}}}',
    '{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"score","instructions":"The text is \\"strong claim\\". How strong?","criteria":[{},"Strong evidence"]}}}',
    json.dumps({"state": "Each question quotes the text it asks about.", "model": "jev-1.13.0", "questions": {
        "q1": {"type": "noul", "instructions": 'The text is "charged twice". Which topics?\n\nDetermine whether the label "billing" applies to this item.', "criteria": {"true": "Charges"}},
        "q2": {"type": "noul", "instructions": 'The text is "charged twice". Which topics?\n\nDetermine whether the label "shipping" applies to this item.'},
    }}, separators=(",", ":")),
)


def complete_question_files_keep_identity():
    """Real caller-owned files retain full metadata and independent wire identity."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        paths = [Path(folder) / f"{verb}.json" for verb in ("choose", "score", "tag")]
        for path, source in zip(paths, SPECS):
            path.write_text(source, encoding="utf-8")
        texts = ("charged twice", "strong claim", "charged twice")
        calls = [f"SELECT thinkthen_{verb}('@{path}', '{value}')" for verb, path, value in zip(("choose", "score", "tag"), paths, texts)]
        details = [f"SELECT thinkthen_details('@{path}', '{value}')" for path, value in zip(paths, texts)]
        got = run(["SET thinkthen_batch='max'", *calls, *details,
                   f"SELECT thinkthen_choose('{SPECS[0]}', '{texts[0]}')"], backend.base("arm/full/capture"))
        expect([rows(item)[0][0] for item in got[1:4]], ["billing", 0.1, ["billing", "shipping"]], "typed complete answers")
        expect(backend.capture(), list(EXPECTED), "independently fixed request bodies")
        expect(backend.count(), 3, "one send per complete question, inline cache hit")
        for index, item in enumerate(got[4:7]):
            meta = json.loads(rows(item)[0][0])["meta"]
            expect(meta["question_sha256"], hashlib.sha256(CANONICAL[index].encode()).hexdigest(), "canonical question digest")
            expect(meta["model"], "jev-1.13.0", "saved effective model")
            expect(meta["cached"], True, "same question and evidence cache identity")
        expect(rows(got[7])[0][0], "billing", "inline complete form")


def complete_question_refusals_and_nulls():
    """File content is Local and never repeats a value; typed errors are Usage; no send."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        folder = Path(folder)
        malformed = folder / "malformed.json"
        # The shared question-file contract names a refused key; a refused value stays unsaid.
        malformed.write_text('{"choose":"x","options":["a","b"],"threshold":"SYNTHETIC_PRIVATE_MARKER_0247"}')
        unknown = folder / "unknown.json"
        unknown.write_text('{"choose":"x","options":["a","b"],"line\\nbreak\\u0007":1}')
        blank = folder / "blank.json"
        blank.write_bytes(b"")
        invalid = folder / "invalid.json"
        invalid.write_bytes(b"\xff")
        large = folder / "large.json"
        large.write_bytes(b"x" * (1024 * 1024 + 1))
        good = folder / "good.json"
        good.write_text('{"choose":"x","options":["a","b"]}')
        cases = [
            (f"SELECT thinkthen_choose('@{malformed}', 'x')", "thinkthen local:"),
            (f"SELECT thinkthen_choose('@{unknown}', 'x')", "thinkthen local:"),
            (f"SELECT thinkthen_choose('@{blank}', 'x')", "thinkthen local:"),
            (f"SELECT thinkthen_choose('@{invalid}', 'x')", "thinkthen local:"),
            (f"SELECT thinkthen_choose('@{large}', 'x')", "thinkthen local:"),
            (f"SELECT thinkthen_choose('@{folder}/missing.json', 'x')", "thinkthen local:"),
            (f"SELECT thinkthen_tag('@{good}', 'x')", "thinkthen usage:"),
            ("SELECT thinkthen_choose('plain text', 'x')", "thinkthen usage:"),
            ("SELECT thinkthen_choose('{bad}', 'x')", "thinkthen usage:"),
            (f"SELECT thinkthen_choose(q, 'x') FROM (VALUES ('@{good}'), ('plain text')) t(q)", "thinkthen usage:"),
        ]
        got = run([*[sql for sql, _ in cases], "SELECT thinkthen_choose(NULL, 'x')",
                   f"SELECT thinkthen_choose('@{good}', NULL)", "SET enable_external_access=false",
                   f"SELECT thinkthen_choose('@{good}', 'x')"], backend.base())
        for item, (_, kind) in zip(got, cases):
            assert said(item).startswith(kind), said(item)
            assert "SYNTHETIC_PRIVATE_MARKER_0247" not in said(item), said(item)
        expect(said(got[1]), "thinkthen local: a question file takes no key `line\\nbreak\\u0007` (retryable: no)",
               "an unknown key named with JSON escapes on one line")
        expect([rows(item)[0][0] for item in got[len(cases):len(cases) + 2]], [None, None], "typed NULL results")
        assert said(got[-1]).startswith("thinkthen local:"), said(got[-1])
        expect(backend.count(), 0, "invalid chunk, nulls and denied caller file send nothing")


def complete_question_rechecks_prepared_authority():
    """A prepared call opens under the executing statement's authority."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        path = Path(folder) / "question.json"
        path.write_text('{"choose":"Which?","options":["a","b"]}')
        got = run([f"PREPARE named AS SELECT thinkthen_choose('@{path}', 'hello')",
                   "EXECUTE named", "SET enable_external_access=false", "EXECUTE named"], backend.base())
        expect(rows(got[1]), [["a"]], "prepared first execution")
        assert said(got[3]).startswith("thinkthen local:"), said(got[3])
        expect(backend.count(), 1, "later denied execution sends nothing")
    with Backend() as backend:
        got = run(["SET thinkthen_query_budget_ms=0",
                   "SELECT thinkthen_choose('{\"choose\":\"x\",\"options\":[\"a\",\"b\"]}', 'x')"], backend.base())
        assert said(got[1]).startswith("thinkthen deadline:"), said(got[1])
        expect(backend.count(), 0, "spent statement budget sends nothing")


def native_complete_file_authorization_precedes_content_parsing():
    """The additive complete door authorizes every prepared execution before parsing."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        path = Path(folder) / "question.json"
        path.write_text('{"decide":"Refund?"}')
        payload = '{"records":[]}'
        got = run([f"PREPARE complete AS SELECT thinkthen_decide_complete('@{path}', '{payload}')",
                   "EXECUTE complete", "SET enable_external_access=false", "EXECUTE complete"], backend.base())
        first = json.loads(rows(got[1])[0][0])
        assert 'error' not in first['native'], first
        refused = json.loads(rows(got[3])[0][0])
        assert refused['native']['error']['kind'] == 'local' and 'facts' not in refused['native'], refused
        assert refused['observations'] == []
        path.write_text('{"decide":"PRIVATE_DENIED_COMPLETE_CONTENT"')
        denied = json.loads(rows(run(["SET enable_external_access=false",
                               f"SELECT thinkthen_decide_complete('@{path}', '{payload}')"], backend.base())[-1])[0][0])
        assert denied['native']['error']['kind'] == 'local'
        assert 'file' in denied['native']['error']['message'] and 'parse' not in denied['native']['error']['message'], denied
        assert 'PRIVATE_DENIED_COMPLETE_CONTENT' not in json.dumps(denied), denied
        expect(backend.count(), 0, "denied complete question content sends nothing")
