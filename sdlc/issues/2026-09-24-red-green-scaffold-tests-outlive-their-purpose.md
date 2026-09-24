# Red-green scaffold tests outlive their purpose

Status: Open. Filed from the workspace on Ian's request. The workspace rule is decision `2026-09-24-tests-earn-their-place.md` in the dotfiles repo.

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

The owner decides whether this lands before or after 0.1.
