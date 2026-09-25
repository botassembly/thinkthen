# Red-green scaffold tests outlive their purpose

Status: Open. Steps 1 and 2 landed, and ticket 0119 holds steps 3 and 4. Filed from the workspace on Ian's request. The workspace rule is decision `2026-09-24-tests-earn-their-place.md` in the dotfiles repo.

`AGENTS.md` asks for red-green development and says nothing about the red-green tests afterward. Each one pins a step while the code is written. After green, many guard nothing that a stronger test misses, and they break on behavior-preserving refactors. The OpenClaw project deleted about 400,000 lines of such tests with little change in coverage, using its `test-audit` skill (https://github.com/openclaw/openclaw/tree/main/.agents/skills/test-audit).

Measured on main at `ffb8fb79`:

- `crates/` and `conformance/` hold 52,676 lines of Rust. Files with `test` in their path hold 28,164 of them. Another 56 files carry inline `#[cfg(test)]` modules.
- `crates/` has 787 `#[test]` and `#[tokio::test]` functions.
- `sdlc/ratchet.json` counts test code against the source ceiling. Every scaffold test left behind spends room that product code could use.

## What would fix it

1. Amend the red-green line in `AGENTS.md` and `CLAUDE.md`. After green, the author turns each scaffold test into one of four kinds or deletes it before landing. The four kinds are an outside-in behavior test through the command line or public API, a table of edge cases, a contract check, and a regression test that failed before its fix. Code review checks this.
2. Add a four-question gate for new tests. What behavior does it protect? What credible regression makes it fail? Why does an existing test not already catch it? Does it need a hook that only tests use?
3. Run one audit over the engine's test surface. Use `cargo-mutants` for evidence. A test that kills no mutant the rest of the suite misses is a deletion candidate. For each candidate, record what it can catch and which stronger test covers it. Keep conformance cases, secrecy tests, `spec/` pages, and green demos unless the evidence shows a duplicate.
4. Report the deleted test lines, the mutation score before and after, the suite time before and after, and any pattern that keeps recurring. The workspace uses this report to decide whether other repos get the same audit.

## Timing

The owner decided the timing on 2026-09-24. Ian can overturn it.

- Steps 1 and 2 landed as Quick Fix `qf-tests-earn` at merge `dda5ca79`. The record is `sdlc/records/qf-tests-earn.md`, and its review is `sdlc/records/qf-tests-earn-review.md`. Every build in flight already follows the workspace rule.
- Steps 3 and 4 are ticket 0119, `sdlc/tickets/0119-mutation-audit-of-the-engine-tests.md` on branch `ticket/0119-mutation-audit-of-the-engine-tests`. It builds after 0098 lands and before 0.1 ships. Most test code lives in the engine. Tickets 0085, 0097, 0096, 0086, and 0098 edit the engine until then, so an earlier audit would collide with them. Surface tickets add their own folders, so the audit may build beside them.
- The 0119 report decides whether other repos get the audit.
