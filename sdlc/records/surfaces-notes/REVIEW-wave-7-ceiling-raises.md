# Review of the wave-7 surfaces ceiling raises

Second-agent review of the three commits on `surfaces-wave7` that raise `sdlc/surfaces-ratchet.json`. The reviewer wrote none of them. Reviewed at `14f12f5`, on 2026-09-23.

| Commit | Ceiling | Verdict |
| --- | --- | --- |
| `29cceff` | 26,848 to 34,950 | ACCEPT WITH FOLLOW-UP |
| `1c09f86` | 34,950 to 35,046 | ACCEPT WITH FOLLOW-UP |
| `cc2c036` | 35,046 to 35,159 | ACCEPT WITH FOLLOW-UP |

## What I checked

- `node sdlc/scripts/surfaces-ratchet.mjs` at `14f12f5` prints `35159/35159`.
- I recounted each growth with the ratchet's rule (non-blank `.rs` lines). `thinkthen-core/` at `29cceff` holds 8,102 lines. `1c09f86` adds 5 lines to `standin/src/lib.rs` and 91 in `standin/tests/retry_wait_bound.rs`. `cc2c036` adds 36 lines to `standin/src/replay.rs` and 77 in `standin/tests/relate_yes_no.rs`. Every number in the three commit bodies matches.
- `git diff b11a2b0:crates/thinkthen-core 29cceff:thinkthen-core` shows only the new `Cargo.toml` (own workspace and lints), a new `Cargo.lock`, and the fixture paths in `request.rs`, `response.rs`, and `find.rs`, each one level shorter. The source is otherwise the merge base's.
- `git diff 29cceff^2 29cceff -- sdlc/scripts/policy.py` is empty. The merge leaves main's policy table as main wrote it. It adds `lint-workspaces` and a `lint` hook for it.
- I listed every `thinkthen_core` import in `contract/` and `standin/`, and compared the restored crate with main's `crates/thinkthen/src/core`.
- I read both stand-in diffs line by line against the listeners in `standin/tests/` and the relate path in `replay.rs`.

## `29cceff`: merge of main, restored core

The raise is the old core restored whole. The contract and the stand-in cannot build without it. Main deleted `crates/thinkthen-core` and keeps `core` private inside `crates/thinkthen`. The root location is the right one. A copy under `crates/` would trip main's vendor-word rule and its one-member workspace rule, and the fix would weaken an accepted policy table.

A smaller restore exists but does not pay. `find.rs`, `plan_document.rs`, and `order.rs` are reached only from `lib.rs` and hold 627 lines. The stand-in uses `adapters::built_in`, which is the vendor adapter, so that code stays. Cutting 627 lines (under 8 percent) would end the "identical to `b11a2b0`" property. That property lets any reviewer verify the crate with one `git diff`. I would keep the crate verbatim.

Porting the contract and the stand-in onto main's `crates/thinkthen` (option (b) in `MERGE-NOTE.md`) touches these things:

- A public module in `crates/thinkthen/src/lib.rs` behind a feature, re-exporting about seventeen items. The contract needs `question_sha256`, `Question`, `QuestionFile`, `QuestionFileError`, `Threshold`, `Typed`, `Verb`, `resolve`, and `QuestionSet::parse`. The stand-in needs `adapters::built_in`, `Backend`, `Evidence`, `ModelName`, `Plan`, `Reply`, `Value`, `Answer`, `Outcome`, and `recording::Exchange`.
- `pub(crate)` widened to `pub` on those items and on every type in their signatures. Main's `unreachable_pub` lint and the seam rules in `policy.py` would need new accepted copies. That is a public-surface decision on main and needs its own ADR and review.
- Call-site repair. Main's core moved on by 2,829 added and 778 removed lines across 43 files. `question_sha256` is now test-only on main, and callers use `question_sha256_with_profile`. `text.rs`, `result.rs`, `reply.rs`, and the vendor adapter changed shape too.
- New weight for the contract and the stand-in. They would depend on the whole `thinkthen` package and its engine dependencies. The R tarball script would stage `crates/thinkthen` whole instead of the small core.

That port is the production API decision `MERGE-NOTE.md` assigns to the architect's plan. It does not belong in a merge. The restore is the right call for now.

The restore carries one real cost. The frozen core now answers differently from main in places. Main's digest writes `text.as_json()` and a described `levels` form, and it takes a backend profile. The stand-in and the contract still hash and parse with the `b11a2b0` rules. `NOTES-main-parity.md` does not mention this drift.

Follow-ups:

1. Record the frozen-core drift in `NOTES-main-parity.md`. Name the main core changes since `b11a2b0` that a contract or stand-in answer can observe: the digest bytes, question-file fields, and result shape. Say whether each one matters to a surface.
2. File a ticket that owns retiring `thinkthen-core/` when the production engine consumes `contract/`. Today the retirement lives only in a `Cargo.toml` comment and `MERGE-NOTE.md`.

## `1c09f86`: stand-in retry wait capped at the timeout

The fix is one call, `.min(settings.timeout)`, plus a two-line comment. It matches main's ticket 0064 exactly: the least of the header, sixty seconds, and the timeout. Nothing smaller would do.

The test file earns its lines. It pins both paths (header wait and doubled backoff) and counts the requests, so a fix that skipped the retry would fail. Its `listener` repeats `busy_listener` in `standin/tests/backoff.rs` almost line for line. The only differences are the counter and the chosen reply. The commit body says it looked there and kept them apart. The counting version covers both uses.

Follow-up: move the counting fixed-reply listener into `standin/tests/common/mod.rs` and have `backoff.rs` call it with the 503 reply. That removes about fifteen lines.

## `cc2c036`: relate replay by method H

The replay change is tight. It pulls out `edge` and `sorted` so the pick-one path and the yes/no path share them, and the new `yes_no_edges` is 20 lines. The test file holds two cases, one per direction rule, and pins exact edges and probabilities. I found nothing to cut.

The recording writes an `either` map per row, and the replay never reads it. The replay accepts an ask that declares `causes` both-ways against a one-way recording, and it accepts `contradicts` one-way against a both-ways recording. Either way it returns the recording's direction with no complaint. The test comment "with the lower record first" holds because the recording stores the pairs in that order. The code does not enforce it. The kind check just above refuses a similar mismatch.

Follow-up: have `yes_no_edges` refuse an ask whose `either` differs from the row's recorded `either`, with a usage error naming the rule, and add one test for it. The other choice drops the unused field from `build_relate_yes_no.py`. The refusal is the better one.

## A gap in the raise check

All three commits say "Second-agent review: not yet done", and `surfaces-ratchet.mjs` passes them. The check looks only for the words "second agent". This review closes the gap for these three raises. The check would still pass the next unreviewed raise.

Follow-up: file an issue asking the check to refuse a body whose review line says "not yet done" or "pending", or to require a pointer to a review record such as this file.

## Who can overturn this

Ian can overturn any verdict here. He can also overturn the root location of `thinkthen-core/`, which `MERGE-NOTE.md` already marks as his to overturn.
