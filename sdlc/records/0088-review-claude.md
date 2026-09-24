REJECT

# 0088 independent code review (Claude, read-only)

Reviewed the uncommitted worktree diff on `cdfd0e5e` (43 paths) against ticket 0088, `sdlc/planning/relate-design.md`, and the landed 0081 relation owners. Paths below are relative to `crates/thinkthen/` unless they start with `sdlc/`, `spec/`, or `specification/`.

## What I ran

- `cargo test --locked -p thinkthen --test backend relate` with `CARGO_TARGET_DIR=/tmp/claude-1000/thinkthen-0088-review` and both backend variables unset. 12 of 12 passed.
- The test binary's `secrecy --list` and `refusals --list`. The first lists 4 tests and none is a relate test. The second matches two relate tests only because their names contain "refusals".
- The built binary against a local Python listener on 127.0.0.1 (a scratch file, no paid call), plus `--dry-run` probes. Findings 1, 2, and 3 come from those runs.
- `rustfmt` on scratch copies of the new production files with `#[rustfmt::skip]` removed, to measure finding 7.

## Findings, most severe first

### 1. High: Option A reports the wrong asker whenever the target side asks with one candidate

`src/cli/relate/result.rs:243-248` (`choice_roles`) guesses the asker from the mapping. When every option shares a source, it assumes the source asked. A reversed choice with one real candidate always shares its source, so the guess flips roles.

Failing input: `relate works_for=person:organization --details` over `[Ada/person, Acme/organization, Beta/organization]`. The listener log shows Acme and Beta asking and Ada as the only option. The output shows `"asker":{"role":"source","entity":Ada}` twice, with Acme and Beta listed as `target` candidates, and `pick` names the wrong role. The same thing happens with `--profile` over one `team` and three `organization` entities. Bare edges are correct. The public details schema is wrong. No test covers a target-side asker, although acceptance item 3 names "reverse asking".

Smallest fix: take the direction from the rule the planner already uses. `reversed = count(kind == relation.target) > count(kind == relation.source)`, and the asker is then the shared `target`. Better still, have 0081's `QuestionMap::Choice` carry the asker. Add an exact test for the target-asks entry.

### 2. High (money): `@FILE` that is not the first argument becomes a paid bogus relation

`src/cli/relate/config.rs:28-35` checks only `relations.first()` for `@`. `relate calls @q.json` resolves to two inline relations, `calls` and `@q.json`. The dry run confirmed it and exited 0. Without `--dry-run` the command sends a paid request for a relation named `@q.json`. The design says inline rules and `@FILE` never mix, and a malformed rule exits 2 with zero sends.

Smallest fix: refuse with exit 2 when any argument starts with `@` and the argument list is not exactly that one argument. Add a zero-send test.

### 3. Medium: a bare name containing `:` is accepted

`src/core/relate_file.rs:236-245`. A text without `=` becomes the name, whatever it holds. `relate works_for:person` plans a `*:*` relation named `works_for:person`. `specification/relate.md` says "A line with another `=` or `:` is malformed". A likely typo therefore spends requests on every pair. Smallest fix: in the bare branch, refuse a name containing `:`. Add the case to `relate_file/tests.rs`.

### 4. Medium: the shared secrecy and refusal matrices were not extended (acceptance item 5 and the design's proof boundary)

`tests/backend/secrecy.rs:38` still lists commands without `relate`. `tests/backend/refusals.rs` is unchanged. The design forbids "a smaller command-specific substitute", and `tests/backend/relate_security.rs` is that substitute. It lacks default and explicit cache, transport failure, backend-profile refusal, hostile or damaged requested recordings, storage failure, `Debug` inspection, and exact refusal sentences. It checks only exit codes and an absent evidence string. Because of the module name, `cargo test --test backend secrecy` runs no relate test. The ticket also required splitting the 500-line secrecy file before adding cases. That split did not happen because nothing was added. Fix: add `relate` to the shared `sweep` verb tables and the refusal sweep, splitting `secrecy.rs` first.

### 5. Medium: a second fallback owner and a second threshold comparison (acceptance item 7)

`src/cli/relate/plan.rs:60-81` copies the `ProfileLimit` → `permits_relation_fallback` → `plan_pairs` decision from `src/cli/recognize/relation.rs:54-72`. `src/cli/relate/result.rs:148` and `:179` compare `>= threshold` a second time beside `assemble_edges`. Item 7 says review "rejects a second … fallback [or] threshold". Fix: move the fallback decision into one shared function that both commands call, returning the plan and its `Fallback`. Derive `accepted` from the shared cut helper rather than a local comparison.

### 6. Medium: the unit-tested question digest is not the production digest

`src/core/relate_file.rs:37-49` (`impl Serialize for RelateSpec`) is used only by `relate_file/tests.rs`. The shipped `meta.question_sha256` comes from `QuestionView` plus inline SHA-256 in `src/cli/relate/result.rs:72-86`. The two disagree under `--lines`: core writes `fields`, and the CLI writes `null`. So acceptance item 2's "canonical bytes and digest" test pins code that production never runs. No test asserts `question_sha256`. Fix: keep one canonical serializer in core that knows the framing's null fields, hash it there, and pin the production digest in a backend test.

### 7. Medium: the budget was met by suppressing the formatter, not by smaller code

Main at `cdfd0e5e` has 2 `#[rustfmt::skip]` in `src/`. This diff adds about 40. Examples:
- `src/cli/failure/relate.rs:8` holds an entire `match` on one line of roughly 330 characters.
- `src/cli/relate/dry_run.rs:34-35` and `src/cli/relate/result.rs:296` are similar.

Formatting the new files normally adds about 88 nonblank lines: `failure/relate.rs` 6 → 26, `result.rs` 278 → 313, `dry_run.rs` 75 → 103, `plan.rs` 107 → 119. That puts production near 1,287 against a 1,200 cap. `src/cli/failure.rs` reaches exactly 500 by deleting two existing doc comments, where the ticket said to split that file before adding. `src/cli/relate.rs:98-101` keeps `#[expect(clippy::too_many_arguments)]` on an 8-argument function. The ratchet rise from 41,045 to 43,020 matches the gross count. That count is only honest if skip-compressed lines are accepted. Fix: drop the skips, fix finding 5 to delete duplicated lines, split `failure.rs`, and re-measure. If it still exceeds 1,200, re-score the ticket as its own stop rule requires.

### 8. Low: numbers in one object are written two ways

`result.rs:146,177,184` format probabilities with Rust `Display`, which gives `1` and `0.0000001`. `value` edges use serde, which gives `1.0` and `1e-7`. The local run showed `"probability":1.0` in `value` and `"probability":1` in `candidates` for the same answer. The record says it changed the test to match rather than making the serializer consistent. Fix: build the entries with the serde serializer rather than `format!`.

### 9. Low: dead or redundant code

- `result.rs:111-114` string-replaces `"failed_questions":0`. `with_failed_questions` already set the value. Delete it.
- `plan.rs:70-72` is an unreachable `EvidenceBytes | Questions` arm, because `permits_relation_fallback` already excludes those kinds.
- `relate_file.rs:127-133` sends any file with another verb key to exit 2, even when that file is invalid. The design says invalid means exit 5.

### 10. High, but outside this diff: the landed 0081 choice wording reverses the relation when the target asks

`src/core/relation.rs:215` writes "`{asker} {reads} ___`" even when the asker is the target. The live request for finding 1 was "Which listed person fills the blank: Item 2 (organization "Acme") works for ___?". The edge then records Ada works_for Acme. The model is being asked the opposite question. 0088 may not change the planner, so this finding needs its own `sdlc/issues/` file and a fix before relate output can be trusted for one-way cross-kind rules.

## Acceptance items not proven by a failing-without-change test

- Item 2:
  - The 255/256 boundary, line restrictions, duplicates, absent kinds, and relation order are tested only at the backend, not in `--lib relate_file`.
  - The digest test targets dead code (finding 6).
- Item 3:
  - The dry-run test checks selected keys, not the exact JSON.
  - No Rust test gives an exact H entry. `spec/relate.md` does give one.
  - No test covers a target-side asker, wildcard expansion, a probability exactly at the cut, `--cache` or the default cache, bare-mode exit 6, `backend_profile` with `--profile FILE`, or dry-run digests equal to sent digests.
  - `relation_recording_replays_…_and_cache_reuses_the_same_identity` runs both calls with `--no-cache`, so it proves nothing about the cache.
- Item 4: the claim that saved calibration identity is distinct from `--profile FILE` is not tested. The help test checks fragments with `contains`.
- Item 5: see finding 4.

## Claims in `sdlc/records/0088-build-relate-cli.md` that the diff does not support

- "These tests cover … cache": every relate test passes `--no-cache`.
- "passed the shared secrecy and refusal filters": true, but those filters contain no relate secrecy case.
- "no second owner was added": contradicted by finding 5.
- The claim that 1,199 lines are within budget rests on finding 7.

Checked and sound: per-concrete fallback and `backend_profile` naming in the dry run (verified by hand), `--either` wildcard de-duplication, zero sends before any refusal in the cases tested, the default cache reusing identity (second run sent 0 requests), no key or authorization header in the cache files, record and replay, cancellation flush, and the exit 4 versus exit 6 split.
