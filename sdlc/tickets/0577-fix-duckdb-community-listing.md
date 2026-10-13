# 0577: Pass DuckDB community CI and answer its review

Status: OPEN.

Milestone: 0.2

## Outcome

The ThinkThen DuckDB extension builds in DuckDB's community CI on Linux x86-64 and ARM64. The listing pull request and the DuckDB README page read plainly, in a few human sentences, and say what the extension costs. Ian approves every public word before anyone posts it. The listing ships through 0.2: its descriptor pins the 0.2 release commit, and it builds against DuckDB 1.5.6 and DuckDB 2.0.

## Evidence

- Starts from: the marketing lead's 2026-10-13 report on duckdb/community-extensions#2936, which 0415 opened on 2026-10-11 and marked complete.
  - The PR has failed community CI since 2026-10-06. The linux_arm64 job stops at `Failed to load manifest from directory /duckdb_build_dir ... vcpkg.json: No such file or directory`. Community CI runs with `VCPKG_BUILD=1` and expects `vcpkg.json` at the repository root, which this repo lacks. The linux_amd64 job was cancelled when ARM failed, so neither Linux target has passed.
  - 0415's offline rehearsal did not use the vcpkg toolchain or the CI's Rust version.
  - A DuckDB maintainer's review is unanswered. It says the PR text and the linked README read as machine-written and hard to follow, pricing is unclear, and the ARM build fails.
  - Ian did not know the PR was open. He has since replied on the PR himself, saying he will work through the maintainer's four points.
  - The CI's Rust is fine, because `rust-toolchain.toml` installs 1.95.0. Every step after CMake configure has never run in community CI: compile, SQL tests, macOS and doc_test. More defects may follow.
  - DuckDB's newest release is v1.5.6 (2026-09-28). DuckDB 2.0 has been in alpha since 2026-09-02, feature-frozen on branch v2.0-cyanoptera, with the final release projected for the second half of October 2026. The alpha installs with `DUCKDB_VERSION=alpha` on install.duckdb.org or `pip install duckdb --pre`.
  - Community CI builds the descriptor's `ref` against 1.5.6. Its "Community Extension with latest DuckDB" job builds an optional `repo: ref_next` commit against v2.0-cyanoptera; `extensions/psql/description.yml` shows the form. Our descriptor has no `ref_next`, so that job was skipped. Once 2.0.0 is final, CI builds the plain `ref` against 2.0, so the listing breaks within weeks unless `ref` compiles against 2.0. A separate job builds the 1.4 LTS line (v1.4.5).
- Keeps: the extension's SQL surface and 0415's adapter.
- Changes:
  - Add what community CI needs at the repository root, at least `vcpkg.json`, checked against DuckDB's extension template.
  - Reproduce the community build exactly as its CI runs it, with the vcpkg toolchain and the CI's Rust version, on Linux x86-64 and ARM64, and on macOS where possible. Run its doc_test hello_world example. Record each result.
  - Build the extension and pass its SQL tests against DuckDB 1.5.6 as `ref` and against v2.0-cyanoptera, the 2.0 alpha, as `ref_next`. DuckDB 2.0 blocks this ticket. Set `ref_next` until 2.0.0 ships.
  - Say whether the 1.4 LTS line is worth supporting, with its cost.
  - Compare moving the extension to DuckDB's stable C extension API, which may let one build load across 1.x and 2.0, with keeping the C++ extension and one build per DuckDB version. Recommend one with its cost, and record the recommendation for Ian.
  - Draft a replacement PR description of at most five plain sentences.
  - Draft a DuckDB README page that says what the extension does, shows one example, and states that the extension is free and MIT licensed, hosted models charge per token, and local models are free.
  - The PM sends both drafts to the marketing lead for Ian's approval.
  - Pin the descriptor to the 0.2 release commit. Push the updated descriptor, description and README to the PR only after Ian approves the wording.
  - The PM owns the PR from now until it closes and checks it with the PM checks.
- Proof:
  - The exact community build and SQL tests pass on Linux x86-64 and ARM64 against 1.5.6 and the 2.0 alpha, and doc_test passes.
  - Ian approves both drafts.
  - After Ian's go, community CI passes on the PR and the maintainer's review is answered.
- Defers: Windows community targets, unless CI requires them.

## Progress

- 2026-10-13 started

PM amendment, 2026-10-13. The fresh-reader test of the DuckDB README page, the descriptor's `extended_description` and the PR description runs under 0575, and each opens with the site README's pitch word for word, as 0578 says.
