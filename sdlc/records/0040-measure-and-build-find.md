# 0040: Measure and build `find`

Ticket 0040 remains in progress. Its two paid stages prove that the one-request shape reaches the hosted backend at both accepted choice ceilings and passes the preregistered long-document comparison. The public command and local proof passed independent code review. Two exact how-to recordings remain.

## Feasibility evidence

The probe landed at `912b4c9` after design review and one code-review rejection. The remediation made summary publication atomic, derived selection from ordered probabilities under the proposed tie rule, retained every rank judgment for local stable ordering, separated logical, live, replayed, and billed counts, saved safe failure facts, and made the fake backend require the exact command shapes. Local tests pinned eight feasibility calls, 18 comparison find calls, 1,050 comparison rank judgments, six expected cache replays, both exact count ceilings, early stop, and secrecy.

The authorized command was:

```sh
sdlc/scripts/live --max-tokens 120000 probes/find-0040/run feasibility
```

All eight planned calls were accepted. The stage recorded 70,265 billed input tokens and 15,774 output tokens across 1,559 unit appearances. All eight were live answers. None replayed. The backend accepted 255 units without `none` and 254 units with `none`. The fixed 120,000-token reservation moved the live ledger from 19,500,118 to 19,620,118 charged tokens.

All eight recording files have schema `thinkthen.recording/1`, adapter `systemone`, the reviewed System One endpoint, one matching `q1` request and answer, model `jev-1.13.0`, nonnegative usage, and a filename that recomputes from the adapter, endpoint, and exact request. Each entry has mode `0600`; the empty cache-lock directory is not retained. No entry contains a credential marker or header. A local replay with no key reproduced all eight, reported eight replayed judgments and zero live requests, and left an empty failure file. The committed rows and summary contain only fixed case ids and safe derived counts, policies, selected ids, probabilities, usage, and replay facts.

## Comparison reservation

Feasibility measured 45.07055805003207 billed input tokens per unit appearance. The preregistered calculation keeps the larger old find rate, combines it with the old rank rate for the fixed 1,050 judgments, estimates 451,157.32217573223 tokens, adds 15 percent headroom, and rounds up to 519,000. That is below the accepted 525,000 stop ceiling.

The comparison command was committed before use and ran once:

```sh
sdlc/scripts/live --max-tokens 519000 probes/find-0040/run comparison
```

The 519,000-token reservation moved the live ledger from 19,620,118 to 20,139,118 charged tokens. Find completed 18 logical judgments as 12 live requests and 6 feasibility replays. Both policies hit all 6 answerable documents, including 2 of 2 at 100, 175, and 250 units. `none` found all 3 blank documents and falsely refused 0 of 6 answerable documents. Find billed 95,002 input tokens. Its minimum correct winning probability was 0.99, and no wrong result existed from which to infer a useful floor.

Rank judged all 1,050 units in the six answerable documents. It used 1,050 live requests, replayed none, billed 322,935 input tokens, and hit all 6 trusted units, including 2 of 2 at each size. Both find policies met every accepted gate.

The comparison created 1,062 recordings: 12 new find exchanges and 1,050 rank exchanges. They report 417,937 input tokens and 43,344 output tokens. Across both stages, all 1,070 recordings report 488,202 input and 59,118 output tokens. Every new entry passed the same schema, endpoint, question-answer key, model, usage, digest-link, permission, and credential-marker checks as feasibility. Every response names model `jev-1.13.0`. A keyless local replay reproduced 18 find and 1,050 rank judgments with zero live requests. Saved failures are empty; summaries contain only safe fixed ids and derived facts.

Phase two adds a pure core constructor and mapper plus a thin public command over the existing reader, adapter, transport, cache, recording, and output paths. Focused core and compiled tests cover the request, mapping, framings, preflight refusals, aggregate byte boundary, and safe input failures. The page 15 structure and its exact two-call recording script are ready. Those recordings remain pending.

## Phase-two local implementation

The first compiled red test found no public `find` command. The green implementation adds one pure core constructor and mapper plus a thin binary edge over the existing reader, adapter, transport, cache, recording, and output paths. It adds no dependency, scheduler, HTTP client, recording format, or JSON stack. Reading stops at the first count overflow and enforces one 16 MiB aggregate ceiling before constructing the request.

Core tests cover every valid count, all four count edges, hostile JSON text, both tie classes, digest scope, and reply mapping. Compiled and loopback tests cover lines, JSONL pointers, exact one-request bytes, original-unit output, dedicated details, none, dry-run, empty input, count and byte boundaries, table and jobs refusals, invalid JSON and UTF-8 secrecy, backend secrecy, and a keyless cache replay with one request. The full test rung passed 438 tests. Lint, spec, install, and diff checks passed locally. The Rust ratchet moved from 22,336 to 23,755 nonblank lines for the public command, pure core module, focused tests, and the split of an existing oversized test module; phase-two net growth is 1,419 lines.

Page 15 remains red until independent code review and its two exact hosted responses. The prepared launch is `sdlc/scripts/live --max-tokens 3000 demos/15-find-the-line/record.sh`. It makes one cached, zero-retry request for the refund line and one for the absent warranty under `--none`, pinned to the reviewed hosted URL and model. No phase-two network call has run.

## Phase-two review remediation

The first phase-two review rejected the misleading inherited help, incomplete aggregate secrecy and edge coverage, partial request and digest proof, byte-for-byte output claim, and late recorder-option validation. A red parser test first observed Clap’s unexpected-argument output where the old test expected the custom CSV sentence. The remediation gives `find` its supported parser surface while adapting into the shared request settings, runs bare and detailed `find` through all 17 shared backend and recording routes, pins the complete System One request fixture and canonical question digest, adds keyed zero-request and filesystem/output edges, and constructs Recorder before reading. CSV, TSV, and `--jobs` now receive Clap’s ordinary unexpected-argument response and never appear in help. The same reviewer accepted the remediation after the focused suites and all four repository rungs passed. No phase-two paid call had run at review time.

The two short how-to requests are estimated at about 1,106 input tokens from the prior 553-token average. Their durable reservation is reduced to 3,000 tokens, over twice that estimate.
