# Conformance divergences

The divergence list has a named home here, per the follow-up filed on
2026-09-21. What follows is job 3's record of the 207 stand-in against the
one conformance file, before this repository's stand-in replaced it: which
cases passed, which failed and why, and which could not run.

Run 2026-09-21, `experiments/207-thinkthen-db/engine`, `examples/conformance_check.rs`
against the stub: **12 passed, 3 failed, 5 not runnable.**

| Case group | Result | Why |
| --- | --- | --- |
| All `decide`, `filter`, `decide_many`, `details`, `usage`-and-cache, `backend-refuses`, both parse-level usage cases | passed | The decide family matched the grammar and the wire |
| `09-usage-filter-band` | failed | The band-on-filter rule lives at the case verb, and the 207 stand-in's only door resolves every question as a decide, where a band is legal. The real conformance runner must enforce the verb-level rules; no 207 stand-in door could see them. The stand-in in this repository refuses the band at `filter` |
| `18-cancel-mid-batch` | failed | The 207 stand-in's bulk path does not honor a pre-fired cancel token; its own 2026-09-20 proof drove the token mid-flight with delays. The contract's promise — nothing is sent after a fired token — is the stand-in's `cancelled` test here |
| `20-choose-backend-refuses` | failed | The fixed choose grammar cannot pass the 207 stand-in's decide-resolving `from_json`, so the refusal reads as usage rather than backend. This repository's contract parses the choose grammar natively |
| `choose`, `score`, `tag`, `annotate` (5 cases) | not runnable | The grammar fix and the score fix are exactly what the 207 stand-in predates |

The five `shaped-to-contract` exchanges (choose, score, tag, annotate) are
marked as such inside `conformance.json`: the stub answers one probability a
request and cannot distinguish options or labels, so those replies were
shaped to the wire contract rather than captured. The first live run of the
real engine against the real backend should re-capture those five and let
the digests and probabilities stand or fall then.

## The growth of 2026-09-21, and what it added

The cases-grow lane took the file from twenty to twenty-seven, and the schema
stays `thinkthen.conformance/1` with three additive keys: `none` and
`budget_ms` on a case, and `question_file` for the local kind. `case_count`
now equals the length of the cases array.

- `rank` (3 cases): the slice asserts the order, best first with ties kept in
  input order, and the probabilities in input order. No `question_sha256` is
  pinned for rank yet, because the ranked result carries `threshold: null` and
  the real engine's digest for that shape is not pinned; it arrives with the
  real engine.
- `find` (2 cases): the exchanges carry the contract-true aggregate form the
  command sends — one request, the units with `u001`-shaped ids as the state,
  one choice question over the ids — and are marked `shaped-to-contract`,
  because this repository's stand-in judges each unit alone and takes the
  best, as its crate docs name. The two agree on the answer's meaning (best
  unit, first leader on ties), so the slice asserts the winner's index and
  not the aggregate distribution, which belongs to the real engine's one
  request. The `none` case is real-engine data outright: the contract's find
  carries no none flag yet, so every slice diverges on case 25 with that
  reason.
- `local` (1 case): the case names a missing question file and expects the
  local kind with nothing sent. No stand-in slice carries a file door, so
  every slice skips it; the command and the real engine own the local kind.
- `deadline` (1 case): a spent budget of zero over a recorded exchange. The
  slice applies the budget and expects the deadline kind, which the stand-in
  returns at entry; a positive budget that expires mid-flight is the wire
  suites' own test, not this case's.
- `defect`: no case exists, honestly. A defect is an engine invariant broken,
  not a property of a recorded reply, so the wire cannot express one; the
  kind stays in the six and the engine's own tests own it.
- The unsure value across verbs: the band decide (03), choose under its
  threshold (12), and find's none (25) carry it. Score has no neutral
  marker by specification — it always answers its number from 0 to K-1 — so
  no score case shows one.

`tools/build_conformance.py` wrote the original twenty once and is history:
rerunning it would erase this growth. The file is grown by hand under
`tools/validate_conformance.py`, which recomputes every expectation offline.

## R1: recognize and relate, 2026-09-21

The forty recorded cases, the relate arms, and the one synthesized case that
joined the file. What diverges, and why, from the recordings and the design
pages:

- **The entity number's field name, settled.** This bullet is history: R1
  shipped the neutral field `number` while the name was open, and Ian
  settled it as `strength` on 2026-09-21 (the rule of
  `sdlc/issues/2026-09-21-one-rule-for-every-number-the-tool-prints.md`;
  the rename landed in `78204cb`, `14f418e`, `2356976`, and the generator
  and every expectation carry `strength` as of the fix wave). The
  recordings keep their own field name in the replay table and the stand-in
  maps it at replay.
- **The question file's relation ends are `source` and `target`.** R1's
  questions were re-keyed by the fix wave (`from`/`to` refused with the
  ruled spelling named), and the validator's rule-key checks follow. The
  harvest recordings still spell `from`/`to`; the stand-in maps them at
  replay.
- **`located_in` is recorded nowhere.** Every recording that asks the
  organization-to-place rule (the 222 demo, the 225 harvest, C13, C19,
  C36) names it `based_in`; the design page's examples and the deck's
  Python, TypeScript, and Rust recognize samples ask `located_in`. The
  replay is strict: a rule the recording does not hold is a usage error
  naming the rule and the covered set, so the deck's samples as written
  cannot run against the stand-in until the deck or a new recording aligns
  the name. The finding is the name, not the shape.
- **The per-subject relate arm is retired (2026-09-23).** It held a method
  main no longer uses, and its records now answer from the method-H
  recording in case `71-relate-staff`. Every relate case replays exactly.
- **The recordings carry no failed-question cases**, so the build team's
  failed marker (`{"failed": {"kind", "cause"}}`) is unexercised by the
  replay; nothing here fabricates one.
- **`requests` pins digests, not construction order.** The cache file names
  are the logical request digests; the order they were built in is not
  recoverable from the recordings, so the lists are sorted and the count is
  the pin.
- **Rule ends are reconstructed.** The package records rule names only;
  `works_for` and `founded` are person:organization and `based_in` is
  organization:place from the 222 demo's `relation_map.json`, and
  `located_in` is *:place from the design page. Coverage reads names, so the
  ends change no recorded answer.
- **C41 is synthesized, not recorded.** It exists for the per-host offset
  proofs (accent and emoji before the name), is marked `synthesized`, and
  carries no request digests.
- **The brief's arithmetic.** The task said the file grows to sixty-seven
  cases (27 + 40); with the four relate arms and the synthesized case it is
  seventy-two. One file, additive keys, schema unchanged.
- **The R surface's runner needs a guard.** Its conformance slice parses
  every case's question before switching on the verb, so the 45 new cases
  fail with "a question file holds one of decide, choose, tag, or score".
  The fix is one line in `libraries/r/conformance.R` — skip a verb the
  runner does not express before building the question — and it belongs to
  the R surface lane, whose recognize work will touch that file anyway. R1
  touched no surface folder.

## The two synthesized cases, 2026-09-21 (lane B, item 2)

The shapes from tickets 0053 and 0054 arrive with two cases the recordings
cannot carry, marked `synthesized` in their exchanges the way the five
`shaped-to-contract` exchanges are marked:

- **`73-details-carries-requests`** pins the ordered `requests` list (0053)
  on a one-request result. No recorded case carried `meta.requests` at
  capture time, so the pinned digest was produced by the production rule —
  the adapter name, the resolved URL, and the exact request bytes through
  `recording::Exchange::digest` — via the stand-in's
  `examples/exchange_digest.rs` under a clean environment, exactly as the
  conformance file's canonical `backend_url` (the built-in default) does.
  The stand-in's own test recomputes the same value through the engine and
  the helper, so the pin and the runtime agree by command, not by trust.
- **`74-annotate-preserves-good-answers`** pins the failed-question marker
  (0054): the reply answers `q1` and omits `q2`, `q2`'s field carries
  `{"failed":{"kind":"backend","cause":"missing_answer"}}`, the neighbour's
  good answer is preserved, and `failed_questions` counts one. No recording
  carries a failed logical question, so the stand-in answers the marker for
  exactly one record (`SYNTHETIC_PARTIAL_RECORD`, its last name-order
  question) and nothing else. The wire shape check allows the one omitted
  answer because the case's expectation names it failed; every other case
  still requires every asked question answered.

The remaining case expectations do not yet carry `requests` or
`failed_questions` on their `details` objects; main's conformance shape
(0053, "Shared conformance shape") derives `requests` for every expected
answer from its exchange at the merge, and the build team's validator owns
that derivation. What this file pins today is the shape, the ordering rule,
the retry rule, and the closed cause list.

## The review's growth, 2026-09-22: repeated texts, the NULL row, and the score and tag coverage

The review of 2026-09-22 measured the file as hollow where it mattered —
45 of 74 cases were recognize and relate lookups, score and tag had one
case each, and the checker ignored its argument. This wave added ten
cases and one table:

- **The skip table.** `conformance.json` now carries a top-level `skips`
  list: one place naming what each surface cannot run, each entry with a
  matcher, an optional surface list, and a written reason. Every runner
  reads it instead of carrying its own skip list, and the checker
  validates every entry (a case id or verb it names must exist, a reason
  is required, surfaces and dispositions are closed sets).
- **`80-decide-many-alternating-repeated-texts`** pins per-row mapping
  where two texts alternate over ten rows: five true, five false. The
  review's first finding was this shape answering the k-th distinct text
  on the k-th row.
- **`81-decide-many-null-text-passes-through`** pins the SQL NULL row:
  a JSON `null` record sends nothing, consumes no answer, and stays
  NULL. DuckDB and PostgreSQL pass it through and run the case. SQLite's
  door refuses a NULL text (`Invalid type`) rather than answering NULL,
  so the case is skipped there with that written reason and the gap is
  recorded here as a surface follow-up, not bent in the file. The library
  doors take text strings and skip it with their own reason.
- **`82-annotate-over-repeated-texts`**, **`83-annotate-score-over-repeated-texts`**,
  and **`84-annotate-multi-column-score`** pin the annotate record form:
  `records` carries the inputs, `expect.rows` one `{"input","value"}`
  answer object a record in input order, and repeated texts reuse their
  answer (one exchange a distinct record, first appearance first). The
  field value is the bare answer; a score reads as its position. The
  surfaces spell a score field differently — a bare number on Python,
  TypeScript, Rust, R, and PostgreSQL; `{"answer","nearest"}` on SQLite
  and the C JSON door; `{"nearest","position"}` on DuckDB; `[position,
  nearest]` on Ruby — and every runner reads the position out, so the
  case pins the number, not the host's spelling.
- **`75` to `79`** add two score cases (top level at 1.7, the equal
  distribution at 0.99) and three tag cases (none held, two labels in
  the question's order, a threshold that excludes the weaker label), so
  score and tag each carry three cases or more.
- **The DuckDB tag boundary, two entries.** Its SQL tag call takes the
  question as plain text and the labels in a list — a JSON question is
  refused ("tag takes its question as plain text and its labels in the
  list") — so a per-question threshold has no shape there, and its
  driver asserts only a held label, so the empty-answer case is the
  surface's own check. Both are reasoned skips in the table.
- **The numbers.** The new cases derive from the null backend's own
  rules (a `refund` text 0.97 and a score's 0.05/0.20/0.75; `maybe`
  0.55/0.20/0.55/0.25; anything else 0.03 and 0.34/0.33/0.33; tag
  labels 0.72/0.55/0.03 by keyword), so every one replays offline; each
  is marked `shaped-to-contract` the way the five older synthesized
  exchanges are. No recording is fabricated: the exchanges carry the
  request and reply the null backend answers, and the checker recomputes
  every expectation from them.
