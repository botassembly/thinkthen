# Quick Fix qf-tests-earn: red-green tests are scaffolding, and new tests pass a four-question gate

Status: landed. It carries out steps 1 and 2 of `sdlc/issues/2026-09-24-red-green-scaffold-tests-outlive-their-purpose.md`. The workspace rule is decision `2026-09-24-tests-earn-their-place.md` in the workspace's decisions folder. A fresh read-only Opus review is in `sdlc/records/qf-tests-earn-review.md`.

## Result

- The red-green line in `AGENTS.md` now calls a red-green test scaffolding. Before a ticket lands, the author turns each one into an outside-in behavior test through the command line or public API, a table of edge cases, a contract check, or a regression test that failed before its fix. The author deletes the rest. Code review checks this.
- "What reviewers keep finding" gains two lines. The first holds the four questions a new test must answer and moves a test that needs a test-only hook to the real boundary. It cites the decision. The second lists the junk patterns from the OpenClaw `test-audit` skill that review rejects.
- `CLAUDE.md` is a symlink to `AGENTS.md`, so both files carry the change.

The repository has no character budget on `AGENTS.md`. The workspace budget covers only the workspace `CLAUDE.md`. The file grows from 4,847 to 5,892 characters.

Steps 3 and 4 of the issue, the mutation audit and its report, are ticket 0119.

## Checks

The change is text only. `sdlc/scripts/lint` exited 0 with `ratchet: crates + conformance 48801/48801`. `sdlc/scripts/live` did not run.
