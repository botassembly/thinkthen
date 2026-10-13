# 0577: Pass DuckDB community CI and answer its review

Status: OPEN.

Milestone: 0.2

## Outcome

The ThinkThen DuckDB extension builds in DuckDB's community CI on Linux x86-64 and ARM64. The listing pull request and the DuckDB README page read plainly, in a few human sentences, and say what the extension costs. Ian approves every public word before anyone posts it. This work runs beside 0.2 and does not hold its release.

## Evidence

- Starts from: the marketing lead's 2026-10-13 report on duckdb/community-extensions#2936, which 0415 opened on 2026-10-11 and marked complete.
  - The PR has failed community CI since 2026-10-06. The linux_arm64 job stops at `Failed to load manifest from directory /duckdb_build_dir ... vcpkg.json: No such file or directory`. Community CI runs with `VCPKG_BUILD=1` and expects `vcpkg.json` at the repository root, which this repo lacks. The linux_amd64 job was cancelled when ARM failed, so neither Linux target has passed.
  - 0415's offline rehearsal did not use the vcpkg toolchain or the CI's Rust version.
  - A DuckDB maintainer's review is unanswered. It says the PR text and the linked README read as machine-written and hard to follow, pricing is unclear, and the ARM build fails.
  - Ian did not know the PR was open.
- Keeps: the extension's SQL surface and 0415's adapter.
- Changes:
  - Add what community CI needs at the repository root, at least `vcpkg.json`, checked against DuckDB's extension template.
  - Reproduce the community build exactly as its CI runs it, with the vcpkg toolchain and the CI's Rust version, on Linux x86-64 and ARM64. Record each result.
  - Draft a replacement PR description of at most five plain sentences.
  - Draft a DuckDB README page that says what the extension does, shows one example, and states that the extension is free and MIT licensed, hosted models charge per token, and local models are free.
  - The PM sends both drafts to the marketing lead for Ian's approval. Nobody posts or pushes to the PR without Ian's go.
  - The PM owns the PR from now until it closes and checks it with the PM checks.
- Proof:
  - The exact community build passes on Linux x86-64 and ARM64.
  - Ian approves both drafts.
  - After Ian's go, community CI passes on the PR and the maintainer's review is answered.
- Defers: macOS and Windows community targets, unless CI requires them.

## Progress
