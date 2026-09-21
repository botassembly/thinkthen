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
