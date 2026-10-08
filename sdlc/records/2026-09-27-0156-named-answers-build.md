# 0156 named answers build

Ticket: [0156](../tickets/0156-named-answers-in-repo-text.md). Branch: `ticket/0156-named-answers-in-repo-text`. Source preflight: merged `origin/main` `65aede94` before editing. Status: done. Fresh independent Sol Medium review accepted `e2f9fa58` against `65aede94` in session `01a0e4ed-5b30-70f1-a4e7-7e43de6a74f6`.

## What changed

The lint rung now calls a Node check that imports the marketing-owned `site/scripts/named-answers.mjs`. It scans Markdown fences in the five queue-owned folders and self-tests the folder walk, fence handling, tag handling and module connection. It writes no page. Fourteen current samples now name their answers, including the two CLI help samples and the PostgreSQL statement that `check.sh` executes. Backlog question 4 records the coordinator's reach ruling. No `site/` file changed.

## Evidence

- Red: the first scanner run on merged main reported 14 rule failures in the claimed pages. Green: `node sdlc/scripts/named-answers.mjs --self-test` passed five folder plants and seven fence rows; the real scan reported 210 accepted-language blocks in 82 pages, 57 skipped, zero problems.
- `node site/scripts/named-answers.test.mjs` passed all 57 module cases and 7 site article rows, including R0/R1/R2. The module was read and imported; no site file changed.
- `python3 sdlc/scripts/policy.py` passed 189 packages; `node sdlc/scripts/ratchet.mjs` reported 76,686/76,686 aggregate source lines. `cargo fmt --all -- --check`, `git diff --check`, `python3 sdlc/scripts/tickets --self-test`, `python3 sdlc/scripts/tickets`, `python3 sdlc/scripts/pages-self-test`, and `python3 sdlc/scripts/pages` passed. The script has 120 nonblank lines; the pages and `check.sh` grew by 25 net lines; the help source stayed at zero net.
- A warm command from codex-4 at `bbf9d6ce` passed the two executable documentation pages before this lane built. It did not prove changed help. This lane then ran `flock -o "$THINKTHEN_HEAVY_LOCK" cargo build --locked --offline -p thinkthen`; the built command's `decide --help` showed `refund_code`, and `choose --help` showed `team_code` and `team`. With that command, `mustmatch test spec/decide.md` passed 24 blocks and demo 01 passed 3 blocks with 1 skipped.
- `flock -o "$THINKTHEN_HEAVY_LOCK" cargo clippy --locked --offline -p thinkthen --all-targets --all-features -- -D warnings` passed. `flock -o "$THINKTHEN_HEAVY_LOCK" env STEPS=slide_sample bash databases/postgresql/check.sh` passed its focused SQL step: 1 passed, 0 failed. That check also completed its prerequisite formatting, clippy and library tests.

The current focused-batch ruling in [the work plan](https://github.com/botassembly/thinkthen/blob/eafcd3f26/sdlc/planning/work-plan-2026-09-27.md) puts the full integration ladder at the coordinator's next coherent batch checkpoint. The independent code review accepted the scanner, samples, scope, focused proof and lessons without a required correction.

## Lessons and routing

The old survey correctly found the affected files, but R0/R1/R2 and later pages changed its counts. The current-source scan prevented edits to three valid pipeline samples. The two named-answer issues were already closed for their own scopes by a separate Quick Fix, so this ticket does not move them. The coordinator routes the later gaps listed under the ticket's Deferred gaps if Ian broadens the rule.


## Integration and preparation follow-up

After merging the reviewed candidate with landed 0154, the coordinator reran the scanner self-test and real scan, ticket evidence, exact 77,421-line aggregate ratchet and diff check. They passed. An incremental command build and the two affected executable pages passed: 24 `spec/decide` blocks and three demo 01 blocks, with its existing skipped block retained. Generated `decide` and `choose` help contained the named exit-code captures. The outer build/check command took 12.37 seconds including any lock wait; it is not a compile-only measurement. Revision and logs live under the coordinator's `target/codex-builds/0156/`. The unchanged PostgreSQL extension retains the successful focused branch proof; it was not rebuilt again at integration.

The PostgreSQL script's `STEPS` selects runtime cases but still builds/packages its normal and panic-probe variants unless `THINKTHEN_ARTIFACT` names a release archive. Future page-only checks should inspect that setup and use a proven unchanged artifact when its source inputs and host toolchain match. An arbitrary older artifact cannot validate changed extension code. The retained builder next prepares accepted 0163; it keeps its context and investigates only the next related ticket.
