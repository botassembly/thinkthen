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
