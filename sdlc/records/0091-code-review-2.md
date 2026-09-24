ACCEPT

# 0091 code re-review: merge the branch conformance cases

I reviewed `ticket/0091-conformance-union` at `a90de0d4`. The last code commit is `906c39f2`. The first review is `/tmp/claude-1000/0091-code-review.md`. I am a fresh, read-only Claude session. All runs used a scratch clone, since deleted. `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, `THINKTHEN_URL`, `THINKTHEN_BACKEND`, `THINKTHEN_MODEL`, `THINKTHEN_KEY_ENV`, and `THINKTHEN_CACHE` were unset. No live call ran. `sdlc/scripts/live` never ran directly. The test rung's `sdlc/live-test` ran it only inside its own dummy repositories.

## Blocking findings from the first review

1. Fixed. The text-form arm now has a real control. Cases 29, 30, and 31 carry valid `evidence`, `rank` runs under `--lines`, and the runner pins each exact diagnostic (`command.rs` `SENTENCES`, asserted at line 197). Observed: with case 29's threshold set to 0.9, `command_runner_crosses_the_private_engine_for_every_case` failed at `command.rs:194` for `29-usage-json-text` (exit 4 against 2). With case 31's question set to "Is it good?", the runner failed the same way for `31-usage-rank-blank-question`. Exit 4 is `Failure::NoKey`, so each valid question got past every question rule and stopped only at the missing key. The core test also failed both cases with "breaks no rule as usage".
2. Fixed. With `deny_unknown_fields` removed from `Case`, `ported_case_mutations_are_refused` failed with "ported mutation 0 passed" (`"surprise": 1`). With it removed from `Success`, the same test failed with "ported mutation 1 passed" (`"counterz": 1`).

## Non-blocking notes the builder claimed

- Provenance number. `conformance.rs` now parses the leading number. Observed: renaming case 26 to `100-filter-empty-list` and dropping its provenance failed the core with "100-filter-empty-list names no provenance".
- Folder cleanup. A `Scratch` drop guard removes each case folder. Observed: the two runner failures above left no new `/tmp/thinkthen-conformance-*` folder. The 13 folders from the builder's 09:02 to 09:06 runs are still in `/tmp`. Anyone may delete them.
- The record's case table now has the branch 25 row and the line about the 31 recognize cases kept in the fixture. The ticket's review line names this review as pending.
- The stand-in limits stay as the first review stated them. The record says only a captured re-record can fix them.

## Checks

- Ceiling. `sdlc/ratchet.json` max is 44340. `node sdlc/scripts/ratchet.mjs` printed `ratchet: crates 44340/44340`, and lint printed the same. 44340 − 43897 = 443 = 524 added − 81 removed, as the record says.
- Production code. `git diff --name-only origin/main...a90de0d4 -- crates` lists only files under `cli/conformance_tests`, which is `#[cfg(test)]` at `cli/mod.rs:23`. No production code changed.
- Merge. Ticket 0083 landed. `origin/main` is now `ac69ec5f`. `git merge-tree` reports one conflict, in `sdlc/ratchet.json` (main 44335, branch 44340). A trial merge measured 44778 = 44335 + 443, and the six conformance tests passed on the merged tree. The owner must set the ceiling to 44778 when merging. Because main moved, the landing commit needs its own gate run.
- Ladder at `a90de0d4`. The one-minute load was 3.74 at the start. `install` exit 0. `lint` exit 0, with `ratchet: crates 44340/44340` and `pages: 1 coming, 21 green`. The first `test` run came during a load spike to 12.5, and it failed only `annotate::scheduling::one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` ("document at 32"). That is the known issue `sdlc/issues/2026-09-24-the-global-queue-concurrency-check-fails-under-load.md`, and this branch does not touch that test. I reran `test` at load 6.6: exit 0, with 730 passed, 0 failed, and 2 ignored across 13 result lines, and `live-test: all cases passed`. `spec` exit 0, with `demos: 21 green, 0 red`. `git diff --check` against the new main passed.

## Non-blocking notes

- Nothing guards the new rule that a fault with a `question_form` must carry `evidence` (`conformance.rs`, in the schema-only fault check). With that line removed, all six conformance tests stayed green. A ported mutation that drops `evidence` from case 29 would pin it.
- The `form` arm dispatches through the real command with the process environment. If someone runs the tests with a real key set, and a case's question becomes valid, the command would try a real backend call. A form case could clear the key variable before dispatch.
