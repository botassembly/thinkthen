# 0100: Run the gate only by hand and close five fixed issues

Ticket: `sdlc/tickets/0100-manual-gate-and-closed-issues.md`. Flow: quick fix. Owner: Claude. Base: `origin/main` at `ce0e3d6f`.

## What changed

- `.github/workflows/gate.yml` runs only on `workflow_dispatch`. The `push` and `pull_request` triggers are gone. The jobs are byte-for-byte unchanged. The header comment names Ian's 2026-09-22 ruling and the issue that records it.
- Five issues in `sdlc/issues/` now say Closed and name the ticket and commit that fixed them.

## Red-green for the workflow

No rung of `sdlc/scripts/lint` reads `.github/workflows/`. The proof is a YAML check that passes only when `workflow_dispatch` is the sole trigger. It loads the file with `yaml.safe_load` and compares the keys of `on`.

- Before the change: `triggers: ['pull_request', 'push']`, exit 1.
- After the change: `triggers: ['workflow_dispatch']`, exit 0.

`gh workflow list --all -R botassembly/thinkthen` on 2026-09-24 shows `gate` and `Pages` both `disabled_manually`. A branch that still carries the old `gate.yml` would run it on push if someone re-enabled the workflow before that branch merged main.

## Each fix verified on main

| Issue | Command and result |
| --- | --- |
| `2026-09-22-command-wording-and-help-fixes-for-0-1` | `git merge-base --is-ancestor` puts `331e85f7` (0082) and `565f8b4f` (0090) on main. Ticket 0082 marks item 27 stale and item 44 moved. Ticket 0090 says landed. |
| `2026-09-20-accuracy-round-on-three-public-sets-and-a-speed-rerun` | A text-input row through `cost.jq` from `ce96895b~1` fails with `Cannot index string with string "id"`, exit 5. The same row through main's `transforms/cost/cost.jq` prints the totals, exit 0. The default-width question stays open for ticket 0077. |
| `2026-09-23-score-spec-says-the-vendor-score-agrees-and-it-differs` | `grep` on `specification/score.md` line 25 finds the differences sentence and no "so the two agree". `37746b36` (0087) is on main. |
| `2026-09-24-a-target-side-choice-asks-the-reversed-relation` | `c84051f1` (0088) is on main. `core/relation.rs` line 237 writes `___ {reads} {asking}` for the reversed side. |
| `2026-09-22-stop-github-actions-on-push` | Closed by this ticket. |

## Gates

Run in the worktree with every `THINKTHEN_` variable unset. `sdlc/scripts/live` was not run.

- `sdlc/scripts/lint`: exit 0.
- `sdlc/scripts/spec`: exit 0. Demos 21 green, 0 red.

## Review

Pending. A fresh read-only reviewer checks the branch before it merges.
