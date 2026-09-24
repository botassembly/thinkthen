# Quick Fix qf-agents-budget: hold AGENTS.md to 5,000 characters

Status: landed. Quick Fix `qf-tests-earn` grew `AGENTS.md` from 4,847 to 5,892 characters. The workspace norm for an agents file is 5,000. `workspace instructions` measures only the workspace `CLAUDE.md`, so nothing caught it. A fresh read-only Opus review is in `sdlc/records/qf-agents-budget-review.md`.

## Result

- `AGENTS.md` measures 4,988 characters. `CLAUDE.md` links to it. Every rule survives. The two test lines became one line that names the four questions, the junk patterns, and the workspace decision. The OpenClaw link went, and the skill's name stays. Other lines lost repeated words.
- The `sdlc/` folder list moved out, because `sdlc/README.md` already maps it. The sentence "The package and binary are both `thinkthen`." went, because `crates/thinkthen/Cargo.toml` names both.
- `sdlc/scripts/lint` checks the size first. It reads `CLAUDE.md` as UTF-8, prints `agents file: CLAUDE.md N/5000 characters`, and exits 1 above 5,000. `sdlc/scripts/README.md` lists the check under `lint`.

## Planted-bug proof

In the worktree, with `AGENTS.md` padded to 5,001 characters, `sh sdlc/scripts/lint` printed `agents file: CLAUDE.md 5001/5000 characters` and exited 1. Cut to exactly 5,000 characters, the check printed `5000/5000` and lint went on to its next step. The file was then restored. The reviewer ran the same check on a scratch file of two-byte characters: 5,000 characters passed and 5,001 failed, so the check counts characters and not bytes.

## Checks

`sdlc/scripts/lint` exited 0 at the landing commit with `agents file: CLAUDE.md 4988/5000 characters` and `ratchet: crates + conformance 48801/48801`. `sdlc/scripts/live` did not run.
