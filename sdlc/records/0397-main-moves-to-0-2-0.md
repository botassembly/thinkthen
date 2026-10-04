# 0397: Move main to 0.2.0 and freeze release/0.1

Status: COMPLETE. Reviewed 2026-10-04. Built from origin/main at 70618a4a1741fa2731b6818bf9b4f6ccbe2de87b in lane claude-0. Fresh independent review accepted product and proof commit 8bd84282b2d4218e64b07f471d3df75df699c203. The landing merge carries the Ticket and Review trailers.

Main's 70 version places read 0.2.0. Public install text still names 0.1.2. The Rust install page keeps `thinkthen = "0.1"`, and the smoke runner rewrites only its scratch manifests to the working crate's major and minor version. The changelogs open with unreleased 0.2.0 headings. The release process and ADR 0116 record Ian's no-patch-release ruling. Both release issues stay open for their remaining debt.

Offline proof:

- `python3 sdlc/scripts/versions`: 70 places at 0.2.0. Its self-test passed 27 of 27 cases.
- The Rust install first-call, backends and decide fragment failed before the scratch-manifest change because 0.2.0 does not meet the public 0.1 requirement. All three passed after the change.
- `node scripts/smoke-bindings.mjs`: all 308 language samples passed. `node scripts/smoke-sql.mjs`: all 42 SQL samples passed on DuckDB 1.5.5, PostgreSQL 16.15 and SQLite 3.50.0. Each runner's completed receipts were preserved. The final proof file combines their disjoint language and SQL entries.
- `node scripts/check-binding-proofs.mjs --strict`: all 350 samples match, zero pages need proof, no warning.
- `cargo build --release --locked --bin thinkthen`: passed. The binary reports `thinkthen 0.2.0`.
- `npm run build`, with `THINKTHEN_BIN` naming that release binary: passed after cached dependencies were installed through `npm ci --offline --no-audit --no-fund`. The built Rust page keeps the 0.1 requirement. The built installer keeps `--version 0.1.2`.
- `sdlc/scripts/lint`: passed, including policy, ratchet, Clippy, docs and the 71 of 71 workflow self-tests. The archive self-test needs the version bump committed because it archives HEAD; committing the candidate resolved its earlier archive-name mismatch without changing the test.
- `python3 sdlc/scripts/tickets` and `git diff --check`: passed. The retained 0.1.0 occurrences match the ticket's history and fixture list. The Go README's local build now names 0.2.0. The install text changed by 0396 is retained.

Heavy builds ran under user systemd scopes with memory and swap limits. Cargo and npm installs used their offline caches. No live call, workflow dispatch, site deployment or release/0.1 change ran. Sample code, saved outputs and recordings are unchanged. The coordinator's first full test, specification and surface checkpoint follows landing and precedes any 0.2 runtime landing.
