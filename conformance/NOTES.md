# conformance NOTES

## 2026-09-21 — R1: the recognize and relate cases

The one file grew from 27 cases to 72: the forty recorded recognize cases,
the synthesized offset case C41, and the four relate arms. The schema stays
`thinkthen.conformance/1`; the new keys are additive (`text`, `requests`,
`pairs`, `form`, `note`).

`tools/build_recognize_cases.py` reads the harvest package and writes both
the conformance cases and `../standin/data/recognize-replay.json` from one
source, so the file and the replay table cannot drift. It asserts every
derived answer against `cases/expected/` before writing; a mismatch stops
the run.

```
$ python3 tools/build_recognize_cases.py
wrote 72 cases (41 recognize, 4 relate); replay table 41 + 4 rows
```

`tools/validate_conformance.py` grew the two verbs' grammar checks and a
Python-side replay of every case from the replay table:

```
$ python3 tools/validate_conformance.py conformance.json
OK: 72 cases validated: schema, grammar, digests, wire contract, offline replay
```

The checks the task named, and where each lives:

- every recognize case's entities and relations replay exactly, offsets
  slicing the name out of the text in code points, the numbers by their
  ruled names (`number` on a name, `probability` on a relation);
- every relate case's edges replay exactly from its method-H row, and
  every request id a case names is one its row recorded;
- the source/target spelling holds in every expected relation and edge and
  `from`/`to` never appear;
- `requests` pins the recorded request digests (sorted; construction order
  is not recoverable from the recordings) and `pairs` pins the recorded
  question count.

The brief said the file grows to sixty-seven cases; with the four relate
arms and the synthesized offset case it is seventy-two. Recorded in
DIVERGENCES.

Case C41 is synthesized, not recorded: the text holds an accented letter and
an emoji before the name, and its recorded answer is built from the package
rules for the per-host offset proofs (code-point offsets 10 to 20; UTF-16
11 to 21; UTF-8 bytes 14 to 24).

## 2026-09-21 — lane B, item 2: main's new shapes on the branch

The core lane carried tickets 0053 (`meta.requests`) and 0054 (the
failed-question marker) plus the ruled record row onto the branch. The
branch's `crates/` snapshot predates both tickets, so the shapes are
modeled in `contract/` and emitted by `standin/`, with the serializations
main ruled, and a merge-note item records the mapping (the contract's
`Failed`/`Cause` mirror the core's `BackendFailure`/`BackendFailureCause`;
main's annotate aggregate metadata has no branch type yet, so hosts count
with `thinkthen_contract::failed_questions`).

Commands and results:

```
$ (contract) cargo test --release
test result: ok. 13 passed; 0 failed

$ (standin) cargo test --release --lib
test result: ok. 11 passed; 0 failed
  (8 before this lane; the three new: the_requests_digest_follows_the_production_rule,
   the_partial_failure_marker_is_the_ruled_shape, the_record_row_is_the_ruled_shape)

$ (conformance) python3 tools/validate_conformance.py conformance.json
OK: 74 cases validated: schema, grammar, digests, wire contract, offline replay
  (72 before; 73-details-carries-requests and 74-annotate-preserves-good-answers added;
   rows added to 05, 06, and 19; case 15 carries failed_questions: 0)

$ (standin) env -u ENGINE_BASE_URL cargo run --release --quiet --example exchange_digest -- \
    '{"decide":"Does the writer ask for a refund?","threshold":0.9}' 'I want my money back'
0df1cff5d7714cdf8c7289ac308684f7a31e86b101d27265dc869325c187a5bc
```

What the shapes are, in one place:

- `Details.requests: Vec<String>` — the ordered recording digests of the
  logical requests, in construction order; one element for one request; a
  retry adds no element; no singular form exists (0053). The stand-in
  computes it with `request_digest`, which follows `ask`'s own model and
  URL resolution, so the digest names exactly the request the engine would
  send — computed on the null backend too, because the encoder is
  deterministic and nothing is sent.
- `Annotated::Failed(Failed { kind: FailureKind::Backend, cause })` —
  serializes as the ruled `{"failed":{"kind":"backend","cause":CAUSE}}`
  with the closed cause list (0054). `failed_questions` is always present
  on `Details`, including zero; `thinkthen_contract::failed_questions`
  counts markers across annotate rows for hosts. The exit code 6 lives in
  the `Failed` doc comment so the merge has one place to read it.
- `Row<V> { input, value }` + `rows_json` — the ruled `{"input","value"}`
  record row (go-ahead item 4) for where records flow with answers; the
  conformance cases pin it on filter (05, 06) and decide_many (19).

Known fallout, for the surfaces wave (out of this lane's boundary): the new
`Annotated` variant makes every exhaustive match non-exhaustive. Seven
files carry matches that need the host's failed arm, per ruling B:
`libraries/python/src/lib.rs` (~522) and `src/arrow.rs` (~552, ~643),
`libraries/ruby/src/lib.rs` (~714), `libraries/c/src/lib.rs` (~598),
`databases/duckdb/src/lib.rs`, `databases/sqlite/src/lib.rs`,
`databases/postgresql/src/lib.rs`. The surfaces' own `check.sh` runs stay
green until their sources compile against the new contract; the branch's
full `scripts/check_surfaces.sh` is red for exactly those crates until the
surfaces lane lands the adoption.
