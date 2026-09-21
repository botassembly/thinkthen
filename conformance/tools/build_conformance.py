#!/usr/bin/env python3
# One-shot builder of the original twenty cases (2026-09-21). The file is now
# grown by hand under validate_conformance.py; rerunning this script would
# erase the growth. History, not a tool: see conformance/DIVERGENCES.md.
"""Build cases2/conformance.json: the twenty shared cases in one data file.

Digest logic implements the canonical form of specification/question-file.md
and is proven by reproducing the four digests pinned in that page.

Decide-family exchanges are captured verbatim from the loopback stub.
Choose, score, tag, and annotate replies are shaped to the systemone
response contract and marked shaped-to-contract; the stub answers one
probability per request, driven by the evidence alone.
"""
import hashlib
import json
import urllib.request
import urllib.error

STUB = "http://127.0.0.1:8217/v1/systemone"

# ---------- canonical form and digest (specification/question-file.md) ----------

def num(x):
    """Shortest form that reads back as the same 64-bit float."""
    if isinstance(x, float) and x == int(x) and abs(x) < 1e15:
        s = repr(x)
    else:
        s = repr(x)
    return s

def band_str(band):
    low, high = band.split(":")
    return num(float(low)) + ":" + num(float(high))

def canonical(question):
    verb = [k for k in ("decide", "choose", "tag", "score") if k in question][0]
    text = question[verb]
    obj = {"verb": verb, "text": text}
    if verb == "decide":
        if "true" in question:
            obj["true"] = question["true"]
        if "false" in question:
            obj["false"] = question["false"]
        obj["threshold"] = band_str(question["threshold"]) if isinstance(question["threshold"], str) else float(question["threshold"])
    elif verb in ("choose", "tag"):
        key = "options" if verb == "choose" else "labels"
        items = question[key]
        if isinstance(items, list):
            items = {label: None for label in items}
        obj[key] = items  # insertion order preserved by json.dumps
        if "threshold" in question:
            obj["threshold"] = band_str(question["threshold"]) if isinstance(question["threshold"], str) else float(question["threshold"])
    elif verb == "score":
        obj["levels"] = question["levels"]
    line = json.dumps(obj, separators=(",", ":"), ensure_ascii=False)
    return line, hashlib.sha256(line.encode("utf-8")).hexdigest()

PINNED = [
    ({"decide": "Does this message ask for a refund?", "true": "The writer asks for money back.",
      "false": "The writer asks for anything else.", "threshold": "0.2:0.8"},
     "879e7c887684e9b40ff7904ebbcf9b3c545ca84f3657df876a22d7f16218810d"),
    ({"choose": "Which team owns this request?",
      "options": {"billing": "Money and invoices.", "shipping": "Parcels and dates.", "other": None}},
     "6466cfebbbc92e7d21501d45013f89fc72cf6a533a9020b82edc1784956222fe"),
    ({"tag": "Which topics?", "labels": {"billing": None, "urgent": "The item needs prompt attention."}, "threshold": 0.5},
     "00b00cf7e1d55b2bb16356f583da7d2dab8fb538f459d859f817392b59efdedf"),
    ({"score": "How much disruption does this report?", "levels": ["None.", "Some.", "Blocked."]},
     "831eb29bdbcb62c91bba7790ab0430d40ac2d8e7b866ed1134dab764646f2d34"),
]
for q, want in PINNED:
    line, got = canonical(q)
    assert got == want, f"digest mismatch: {line} -> {got}, want {want}"
print("canonical form reproduces all four pinned digests")

# ---------- wire contract (thinkthen-core adapters/systemone) ----------

def request_for(question, evidence, extra_questions=None, model="jev-latest"):
    questions = {}
    def place(question):
        name = f"q{len(questions) + 1}"
        verb = [k for k in ("decide", "choose", "tag", "score") if k in question][0]
        text = question[verb]
        if verb == "decide":
            criteria = None
            if "true" in question or "false" in question:
                criteria = {k: question[k] for k in ("true", "false") if k in question}
            q = {"type": "noul", "instructions": text}
            if criteria:
                q["criteria"] = criteria
            questions[name] = q
        elif verb == "choose":
            opts = question["options"]
            if isinstance(opts, list):
                opts = {o: None for o in opts}
            questions[name] = {"type": "choice", "instructions": text, "criteria": opts}
        elif verb == "score":
            questions[name] = {"type": "score", "instructions": text, "criteria": question["levels"]}
        elif verb == "tag":
            for label in question["labels"]:
                questions[f"q{len(questions) + 1}"] = {
                    "type": "noul",
                    "instructions": f"{text}\n\nDetermine whether the label {json.dumps(label)} applies to this item.",
                }
    place(question)
    for q in (extra_questions or []):
        place(q)
    return {"state": evidence, "model": model, "questions": questions}

def capture(request):
    body = json.dumps(request, separators=(",", ":")).encode()
    req = urllib.request.Request(STUB, data=body, headers={"Content-Type": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=10) as r:
            return 200, json.loads(r.read())
    except urllib.error.HTTPError as e:
        return e.code, json.loads(e.read())

def noul_reply(p, model="jev-latest"):
    return {"model": model, "answers": {"q1": {"type": "noul", "noul": p}},
            "usage": {"input_tokens": 10, "output_tokens": 2}}

# ---------- the twenty cases ----------

def case(cid, source, verb, question, *, evidence=None, records=None, jobs=None, set_=None,
         calls=None, cancel_after_replies=None, exchanges=None, expect=None):
    c = {"id": cid, "source": source, "verb": verb, "question": question}
    if evidence is not None:
        c["evidence"] = evidence
    if records is not None:
        c["records"] = records
    if jobs is not None:
        c["jobs"] = jobs
    if set_ is not None:
        c["set"] = set_
    if calls is not None:
        c["calls"] = calls
    if cancel_after_replies is not None:
        c["cancel_after_replies"] = cancel_after_replies
    c["exchanges"] = exchanges or []
    c["expect"] = expect
    return c

def ex(request, reply, how):
    return {"captured": how, "request": request, "reply": reply}

S205 = "../../205-thinkthen-libs/shared/cases"
cases = []

# 205-01 decide yes at a cut
q = {"decide": "Does the writer ask for a refund?", "true": "The writer asks for money back.",
     "false": "The writer asks for anything else.", "threshold": 0.9}
ev = "I have asked three times and I still want a refund."
req = request_for(q, ev)
code, rep = capture(req)
assert code == 200 and abs(rep["answers"]["q1"]["noul"] - 0.97) < 1e-9
_, dig = canonical(q)
cases.append(case("01-decide-yes-cut", f"{S205}/01-decide-yes-cut.json", "decide", q, evidence=ev,
                  exchanges=[ex(req, rep, "stub")],
                  expect={"answer": True, "details": {"kind": "yes_no", "probability": 0.97, "model": "jev-latest", "question_sha256": dig}}))

# 205-02 decide no at a cut
q = {"decide": "Does the writer ask for a refund?", "threshold": 0.9}
ev = "Good morning, the package arrived on Tuesday."
req = request_for(q, ev); code, rep = capture(req)
assert code == 200 and abs(rep["answers"]["q1"]["noul"] - 0.03) < 1e-9
_, dig = canonical(q)
cases.append(case("02-decide-no-cut", f"{S205}/02-decide-no-cut.json", "decide", q, evidence=ev,
                  exchanges=[ex(req, rep, "stub")],
                  expect={"answer": False, "details": {"kind": "yes_no", "probability": 0.03, "model": "jev-latest", "question_sha256": dig}}))

# 205-03 band unresolved
q = {"decide": "Does the writer ask for a refund?", "threshold": "0.2:0.8"}
ev = "Maybe I will ask for my money back, maybe not."
req = request_for(q, ev); code, rep = capture(req)
assert code == 200 and abs(rep["answers"]["q1"]["noul"] - 0.55) < 1e-9
_, dig = canonical(q)
cases.append(case("03-decide-band-unresolved", f"{S205}/03-decide-band-unresolved.json", "decide", q, evidence=ev,
                  exchanges=[ex(req, rep, "stub")],
                  expect={"answer": None, "unsure": True, "details": {"kind": "yes_no", "probability": 0.55, "model": "jev-latest", "question_sha256": dig}}))

# 205-04 band resolves yes
q = {"decide": "Does the writer ask for a refund?", "threshold": "0.2:0.9"}
ev = "I demand a full refund immediately."
req = request_for(q, ev); code, rep = capture(req)
assert code == 200 and abs(rep["answers"]["q1"]["noul"] - 0.97) < 1e-9
_, dig = canonical(q)
cases.append(case("04-decide-band-resolves", f"{S205}/04-decide-band-resolves.json", "decide", q, evidence=ev,
                  exchanges=[ex(req, rep, "stub")],
                  expect={"answer": True, "details": {"kind": "yes_no", "probability": 0.97, "model": "jev-latest", "question_sha256": dig}}))

# 205-05 filter keeps some of five
q = {"decide": "Does the writer ask for a refund?", "threshold": 0.5}
records = ["i want a refund now", "good morning", "refund, please", "maybe later", "see you"]
exs = []
for r in records:
    req = request_for(q, r); code, rep = capture(req)
    assert code == 200
    exs.append(ex(req, rep, "stub"))
_, dig = canonical(q)
cases.append(case("05-filter-keeps-some-of-five", f"{S205}/05-filter-keeps-some-of-five.json", "filter", q,
                  records=records, jobs=3, exchanges=exs,
                  expect={"indexes": [0, 2, 3], "details": {"kind": "yes_no", "model": "jev-latest", "question_sha256": dig}}))

# 205-06 filter empty list
_, dig = canonical(q)
cases.append(case("06-filter-empty-list", f"{S205}/06-filter-empty-list.json", "filter", q,
                  records=[], jobs=3, exchanges=[],
                  expect={"indexes": [], "details": {"kind": "yes_no", "model": "jev-latest", "question_sha256": dig}}))

# 205-07 backend refuses
q = {"decide": "Does the writer ask for a refund?", "threshold": 0.9}
ev = "This malformed line is not a request the backend takes."
req = request_for(q, ev); code, rep = capture(req)
assert code == 422, f"expected 422, got {code}"
cases.append(case("07-backend-refuses", f"{S205}/07-backend-refuses.json", "decide", q, evidence=ev,
                  exchanges=[{"captured": "stub", "status": 422, "request": req, "reply": rep}],
                  expect={"error": {"kind": "backend"}}))

# 205-08 usage: threshold 90
q = {"decide": "Does the writer ask for a refund?", "threshold": 90}
cases.append(case("08-usage-threshold-90", f"{S205}/08-usage-threshold-90.json", "decide", q,
                  evidence="I want a refund.", exchanges=[],
                  expect={"error": {"kind": "usage"}}))

# 205-09 usage: band on filter
q = {"decide": "Does the writer ask for a refund?", "threshold": "0.2:0.8"}
cases.append(case("09-usage-filter-band", f"{S205}/09-usage-filter-band.json", "filter", q,
                  records=["I want a refund now", "Maybe later"], jobs=2, exchanges=[],
                  expect={"error": {"kind": "usage"}}))

# 205-10 usage: blank question
q = {"decide": "   "}
cases.append(case("10-usage-blank-question", f"{S205}/10-usage-blank-question.json", "decide", q,
                  evidence="I want a refund.", exchanges=[],
                  expect={"error": {"kind": "usage"}}))

# 207-01 choose picks refund (shaped distribution)
q = {"choose": "Which team owns this?", "options": ["the refund desk", "the maybe desk", "anywhere else"], "threshold": 0.5}
ev = "please route this ticket"
req = request_for(q, ev)
rep = {"model": "jev-latest",
       "answers": {"q1": {"type": "choice", "probabilities": {"the refund desk": 0.62, "the maybe desk": 0.31, "anywhere else": 0.07}}},
       "usage": {"input_tokens": 14, "output_tokens": 3}}
_, dig = canonical(q)
cases.append(case("11-choose-picks-refund", "cases2/01-choose-picks-refund.json", "choose", q, evidence=ev,
                  exchanges=[ex(req, rep, "shaped-to-contract")],
                  expect={"answer": "the refund desk",
                          "details": {"kind": "choice", "pick": "the refund desk",
                                      "probabilities": {"the refund desk": 0.62, "the maybe desk": 0.31, "anywhere else": 0.07},
                                      "model": "jev-latest", "question_sha256": dig}}))

# 207-02 choose under threshold is unsure (the empty-value teaching case)
q = {"choose": "Which team owns this?", "options": ["somewhere", "elsewhere"], "threshold": 0.9}
ev = "a plain note about nothing"
req = request_for(q, ev)
rep = {"model": "jev-latest",
       "answers": {"q1": {"type": "choice", "probabilities": {"somewhere": 0.55, "elsewhere": 0.45}}},
       "usage": {"input_tokens": 12, "output_tokens": 2}}
_, dig = canonical(q)
cases.append(case("12-choose-under-threshold-null", "cases2/02-choose-under-threshold-null.json", "choose", q, evidence=ev,
                  exchanges=[ex(req, rep, "shaped-to-contract")],
                  expect={"answer": None, "unsure": True,
                          "details": {"kind": "choice", "pick": "somewhere",
                                      "probabilities": {"somewhere": 0.55, "elsewhere": 0.45},
                                      "model": "jev-latest", "question_sha256": dig}}))

# 207-03 score, rewritten to specification/score.md
q = {"score": "How strong is the refund claim?", "levels": ["low", "mid", "high"]}
ev = "maybe later"
req = request_for(q, ev)
probs = {"0": 0.20, "1": 0.55, "2": 0.25}
rep = {"model": "jev-latest",
       "answers": {"q1": {"type": "score", "probabilities": probs}},
       "usage": {"input_tokens": 11, "output_tokens": 3}}
value = round((0 * 0.20 + 1 * 0.55 + 2 * 0.25) / (0.20 + 0.55 + 0.25), 12)
_, dig = canonical(q)
cases.append(case("13-score-levels", "cases2/03-score-levels.json", "score", q, evidence=ev,
                  exchanges=[ex(req, rep, "shaped-to-contract")],
                  expect={"answer": value,
                          "details": {"kind": "score", "nearest_level": "mid",
                                      "probabilities": {"low": 0.20, "mid": 0.55, "high": 0.25},
                                      "model": "jev-latest", "question_sha256": dig}}))

# 207-04 tag holds some (per-label noul answers, shaped)
q = {"tag": "Does this line show the word?", "labels": ["the refund word", "the maybe word", "nothing at all"]}
ev = "a plain line"
req = request_for(q, ev)
per_label = {"the refund word": 0.72, "the maybe word": 0.55, "nothing at all": 0.03}
rep = {"model": "jev-latest",
       "answers": {f"q{i+1}": {"type": "noul", "noul": p} for i, p in enumerate(per_label.values())},
       "usage": {"input_tokens": 18, "output_tokens": 4}}
_, dig = canonical(q)
cases.append(case("14-tag-holds-some", "cases2/04-tag-holds-some.json", "tag", q, evidence=ev,
                  exchanges=[ex(req, rep, "shaped-to-contract")],
                  expect={"answer": ["the refund word", "the maybe word"],
                          "details": {"kind": "tag", "probabilities": per_label,
                                      "model": "jev-latest", "question_sha256": dig}}))

# 207-05 annotate assembles two questions in one request
spam_q = {"decide": "Is this spam?", "threshold": 0.5}
band_q = {"decide": "Refund?", "threshold": "0.2:0.8"}
ev = "maybe later"
req = request_for(spam_q, ev, extra_questions=[band_q])
rep = {"model": "jev-latest",
       "answers": {"q1": {"type": "noul", "noul": 0.55}, "q2": {"type": "noul", "noul": 0.55}},
       "usage": {"input_tokens": 16, "output_tokens": 4}}
cases.append(case("15-annotate-assembles", "cases2/05-annotate-assembles.json", "annotate", spam_q,
                  set_={"spam": spam_q, "band": band_q}, evidence=ev,
                  exchanges=[ex(req, rep, "shaped-to-contract")],
                  expect={"answers": {"spam": {"answer": True, "probability": 0.55},
                                      "band": {"answer": None, "unsure": True, "probability": 0.55}}}))

# 207-06 details carries model and digest
q = {"decide": "Does the writer ask for a refund?", "threshold": 0.9}
ev = "i want a refund"
req = request_for(q, ev); code, rep = capture(req)
assert code == 200 and abs(rep["answers"]["q1"]["noul"] - 0.97) < 1e-9
_, dig = canonical(q)
cases.append(case("16-details-carries-model-and-digest", "cases2/06-details-carries-model-and-digest.json", "details", q,
                  evidence=ev, exchanges=[ex(req, rep, "stub")],
                  expect={"answer": True,
                          "details": {"kind": "yes_no", "probability": 0.97, "model": "jev-latest", "question_sha256": dig}}))

# 207-07 usage and cache: same question and text asked twice
q = {"decide": "Does the writer ask for a refund?", "threshold": 0.5}
ev = "refund now"
req = request_for(q, ev); code, rep = capture(req)
assert code == 200
cases.append(case("17-usage-and-cache", "cases2/07-usage-and-cache.json", "usage", q,
                  evidence=ev, calls=2, exchanges=[ex(req, rep, "stub")],
                  expect={"requests": 1, "cache_answers": 1,
                          "after_reset": {"requests": 0, "cache_answers": 0}}))

# 207-08 cancel mid batch
q = {"decide": "Does the writer ask for a refund?", "threshold": 0.5}
records = ["one", "two", "three", "four"]
exs = []
for r in records:
    req = request_for(q, r); code, rep2 = capture(req)
    assert code == 200
    exs.append(ex(req, rep2, "stub"))
cases.append(case("18-cancel-mid-batch", "cases2/08-cancel-mid-batch.json", "cancel", q,
                  records=records, jobs=2, cancel_after_replies=2, exchanges=exs,
                  expect={"error": {"kind": "cancelled"}}))

# 207-09 decide_many judgments
q = {"decide": "Does the writer ask for a refund?", "threshold": 0.5}
records = ["refund now", "good morning", "maybe so", "refund again", "bye"]
exs = []
for r in records:
    req = request_for(q, r); code, rep2 = capture(req)
    assert code == 200
    exs.append(ex(req, rep2, "stub"))
_, dig = canonical(q)
p_of = {"refund now": 0.97, "good morning": 0.03, "maybe so": 0.55, "refund again": 0.97, "bye": 0.03}
cases.append(case("19-decide-many-judgments", "cases2/09-decide-many-judgments.json", "decide_many", q,
                  records=records, jobs=3, exchanges=exs,
                  expect={"answers": [p >= 0.5 for p in (p_of[r] for r in records)],
                          "probabilities": [p_of[r] for r in records],
                          "details": {"kind": "yes_no", "model": "jev-latest", "question_sha256": dig}}))

# 207-10 choose backend refuses
q = {"choose": "Which team owns this?", "options": ["one desk", "other desk"], "threshold": 0.5}
ev = "this malformed line"
req = request_for(q, ev); code, rep = capture(req)
assert code == 422, f"expected 422, got {code}"
cases.append(case("20-choose-backend-refuses", "cases2/10-choose-backend-refuses.json", "choose", q, evidence=ev,
                  exchanges=[{"captured": "stub", "status": 422, "request": req, "reply": rep}],
                  expect={"error": {"kind": "backend"}}))

out = {
    "schema": "thinkthen.conformance/1",
    "case_count": len(cases),
    "error_kinds": ["usage", "backend", "local", "cancelled", "deadline", "defect"],
    "unsure_marker": None,
    "public_word_for_unsure": "unsure",
    "cases": cases,
}
with open("conformance.json", "w") as f:
    json.dump(out, f, indent=2, ensure_ascii=False)
    f.write("\n")
print(f"wrote conformance.json with {len(cases)} cases")
