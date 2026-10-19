# 0708 — Stored formats carry no version

Status: OPEN.

Milestone: 0.2.1

## Outcome

- Changes: `crates/thinkthen/src/engine/store/`, `crates/thinkthen/src/engine/store.rs`, `crates/thinkthen/src/engine/usage/`, `crates/thinkthen/src/engine/invocation.rs`, `databases/sqlite/src/images.rs`, `specification/`, `site/recordings/`, `site/examples/`, `conformance/`, `crates/thinkthen/tests/`.
- Stored answer rows have no `key_version` column or field; recordings and fixtures are rewritten offline with jq, with no paid call.
- Monthly usage files have no `schema` member. The leniency for old files (ignored `usage.retries` files, missing `retries` read as 0) is deleted; an unreadable file is refused with one sentence saying to move it.
- SQLite image BLOB tags drop `/1` while no release has stored them.
- The random call, request, observation and failure ID salts drop `/1`.
- Dead `thinkthen.recording/1` references and the internal conformance and fixture manifest tags (`settings-cases/1`, `type-corpus/1` and the rest) drop their numbers.
- The cache store keeps one store number (`PRAGMA user_version`), which changes only when the store layout changes, because a cache file written by another build is a real mismatch. Cache-key and answer-ID hash constants stay unchanged so users' caches and answer IDs survive.

### Proof

- Store tests open a fresh store and a fixture without `key_version` and replay; a usage file without `schema` is read; an old-format usage file is refused with the exact sentence.
- Replay of every recording and documentation example passes on the M5, and `git grep key_version` outside `sdlc/` finds nothing.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). The 2026-10-19 survey found `key_version` on about 10,000 stored rows across about 80 files, always 2 and guarded already by the store number; usage files carry `thinkthen.usage/1` plus leniency for 0.1 files that the no-compatibility ruling forbids. Cache-key and answer-ID strings are hash inputs users never see; changing them would turn every user cache into paid misses, so they stay as fixed constants.
