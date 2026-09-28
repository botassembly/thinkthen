# Quick Fix qf-h1-h3-h6: a dead bench citation, spec numbers with no record, and three demo checks

Status: built, awaiting landing. It carries backlog rows H1, H3, and H6 of `sdlc/planning/backlog-0-1-2026-09-26.md` as three commits. No key was used, and no request left the machine.

## H1: the removed bench commit

- `specification/audit.md`, `specification/diff.md`, and `crates/thinkthen/tests/fixtures/measure/README.md` no longer cite commit `be7cea2e`. Each says the script's history was removed and names the fixture checksums as the record. The fixture README no longer says the goldens were read with `git show`.
- Beatles Bench now holds `2cdb6445` and main `5f04abbb`. Neither holds `scripts/tools/measure.py`, so no bench path could replace the hash.
- Tickets and records that cite `be7cea2e` stay as history.
- It closes `sdlc/issues/closed/2026-09-26-audit-and-diff-cite-a-removed-bench-commit.md`. The backlog page names the file without a path, as a dated snapshot, and stays as written.

Plant: `grep -rn be7cea2 specification crates` found three lines before the fix and none after it.

## H3: spec numbers and two held flags

- Claim 5. `annotate.md` names the closed live-probe issue for 20.8, and it names `probes/annotate-0015/mixed-summary.json` for 915 and 371. `threshold.md` names section 5 of `sdlc/planning/design-study.md` for its flip rates. Demo 28 says the block above it measured 294.6 tokens a case.
- Claim 6. `specification/recognize.md` says an empty or blank text exits 2 under `--dry-run`, as a live run does. A new block in `spec/recognize.md` pins the refusal line and exit 2 for both texts.
- Page 19. The `--invert` roadmap row answers the survey. A reworded question is a different measurement, so the hold stands. `decide --details` with `jq` keeps the other side of the same question, as the third example in `filter.md` shows. ADR 0048 item 11 already settles where reference text goes.
- Page 10, partly. The `--on-error continue` row keeps the hold, dated. A step 5 in `demos/12-keep-going/` broke ADR 0016: 132 lines, 985 words, seven asserting blocks, and five steps. The step was withdrawn. The issue keeps the page owed and records the `jq` split that worked.
- The docs issue stays open with its remaining items.

The spec block answers the four questions. It protects the dry-run refusal of empty text. A plan printed for empty text fails it. No existing test runs recognize's empty text under `--dry-run`. It needs no test-only hook.

Plants:
- A stand-in `thinkthen` that printed the old zero-request plan at exit 0 turned the new block red. The diff showed the plan and `exit 0` in place of the refusal and `exit 2`.
- A malformed line added to a copy of `queue.jsonl` went to the aside output, and the three good records still reached `decide` under replay.

## H6: three demo-script checks

- Item 7. `sdlc/scripts/demos` refuses a `--replay` folder name that holds anything but letters, digits, and `. _ / -`, and names the text. Self-test case `replay-spaced` pins it.
- Item 8. Demo 27 prints `triage.sh` from `set -eu` down in a `bash` block and pins every line.
- Item 9. `sdlc/scripts/demos` refuses `set +e` or `set +o errexit` in a demo `bash` block. Self-test case `set-e-off` pins it. Demos 19 and 27 keep `set -e` and capture each code with `&& rc=0 || rc=$?`. The `spec/` pages still use `set +e`, and this check does not read them.
- Items 7, 8, and 9 of `sdlc/issues/closed/2026-09-25-test-harness-and-review-leftovers.md` are settled. That issue stays open for items 1 and 4.

The two self-test cases answer the four questions. They protect the two new refusals. Dropping either refusal fails its case. No case covered either refusal before. Each case runs the real runner on a fixture page.

Plants:
- The old `sdlc/scripts/demos` under the new self-test failed exactly the two new cases. `set-e-off` exited 0, so `false` hid under `set +e`.
- A copy of demo 27 whose `triage.sh` printed `ready now` failed two blocks.

## Choices the coordinator or Ian can overturn

- The `set +e` rule refuses the option outright. It does not try to prove that each code is pinned. The pages already recommend `&& rc=0 || rc=$?`, and a static rule for each code would be fragile.
- The `--invert` and `--on-error continue` holds stand, dated 2026-09-26.

## Line counts

Nonblank lines net per commit, before this record: H1 +1, H3 −1, H6 +20.

## Review

A fresh read-only Claude reviewer found two things and one nit. The fixture README still named `git show`. Demo 19 warned about `case $?`, a form it no longer used. The owed-page wording read as settled. All three were fixed in their own commits. The reviewer then returned ACCEPT on `478d034c`, `882d69f8`, and `fc732471`.

## Checks

With `THINKTHEN_API_KEY` unset, after merging `origin/main` at `dd9fc46d`, with the one-minute load between 4.5 and 10:

- `node sdlc/scripts/ratchet.mjs`: `ratchet: crates + conformance 69097/69097`.
- `lint`: exit 0.
- `test`: exit 0, 955 passed, 0 failed, 13 ignored across 36 result lines, `live-test: all cases passed`.
- `spec`: exit 0, `demos-self-test: 27 cases pass`, `demos: 21 green, 0 red`.
