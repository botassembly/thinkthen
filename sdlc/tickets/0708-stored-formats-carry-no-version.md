# 0708: Stored answer rows carry no key version

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Reviews: revision d1dfcc8, accept

Reviews: revision 91f6a46, accept

## Outcome

- Changes: `conformance/children/backend_cases.py`, `crates/thinkthen/src/engine/store.rs`, `crates/thinkthen/src/engine/store/`, `crates/thinkthen/tests/backend/cache_convert.rs`, `crates/thinkthen/tests/backend/default_cache/prune.rs`, `crates/thinkthen/tests/backend/support.rs`, `crates/thinkthen/tests/native_store.rs`, `crates/thinkthen/tests/native_store/`, `specification/cache.md`, `crates/thinkthen/tests/fixtures/`, `demos/01-refund-gate/`, `demos/02-route-a-ticket/`, `demos/03-grep-for-meaning/`, `demos/06-top-search-hits/`, `demos/12-keep-going/`, `demos/14-grade-a-batch/`, `demos/15-find-the-line/`, `demos/16-triage-pipeline/`, `demos/17-rate-and-sort/`, `demos/19-no-or-could-not-ask/`, `demos/21-options-from-the-record/`, `demos/27-test-with-no-network/`, `demos/39-screen-a-message/`, `demos/40-what-yes-and-no-mean/`, `demos/41-tune-a-question-file/`, `demos/43-lint-a-change/`, `demos/44-recognize-names/`, `demos/45-map-relationships/`, `demos/47-split-a-chained-question/`, `demos/48-first-cut-a-long-list/`, `site/examples/beatles/annotate/`, `site/examples/beatles/bench/`, `site/examples/beatles/blind-spots/`, `site/examples/beatles/recognize/`, `site/examples/beatles/relate/`, `site/examples/beatles/score-bands/`, `site/examples/how-tos/bash/`, `site/examples/recipes/rules-propose-model-confirms/`, `site/examples/recipes/verify-a-claim/`, `site/examples/start/answer-cache/`, `site/examples/start/caching/`, `site/examples/start/files/`, `site/recordings/thinkthen.jsonl`, `spec/fixtures/`, `specification/fixtures/recognize/`, `specification/fixtures/relate/`, `transforms/rows/recording/thinkthen.jsonl`.
- Stored answer rows have no `key_version` column or field. Recordings and fixtures are rewritten offline with jq, with no paid call.
- The cache store keeps one store number (`PRAGMA user_version`), which changes only when the store layout changes, because a cache file written by another build is a real mismatch. Dropping the column is such a change, so the number moves once here. Cache-key and answer-ID hash constants stay unchanged, so users' answer IDs survive.
- Usage files, image BLOB tags, ID salts, recording references and internal manifest tags are 0724.

### Proof

- On the M5 through `pm job run --machine m5 0708 SHA --memory 8 -- cargo nextest run -p thinkthen store native_store cache_convert prune`, a fresh store and a fixture without `key_version` open and replay. Before the change, `normalized` in `crates/thinkthen/src/engine/store/versioned.rs` rejects a row whose `key_version` is not 2.
- On the M5 through pm job, `sdlc/scripts/spec` and `sdlc/scripts/demos` replay every recording and documentation example, and `git grep -n key_version -- . ':!sdlc'` finds nothing.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). The 2026-10-19 survey found `key_version` on about 10,000 stored rows across about 80 files, always 2 and already guarded by the store number. Cache-key and answer-ID strings are hash inputs users never see; changing them would turn every user cache into paid misses, so they stay as fixed constants.

Starts from the `answers` table definition in `crates/thinkthen/src/engine/store.rs`, the insert in `crates/thinkthen/src/engine/store/rows.rs`, the `key_version != Some(2)` check in `normalized` in `crates/thinkthen/src/engine/store/versioned.rs`, and the store fixture loader in `crates/thinkthen/src/engine/store/fixture.rs`. The recordings are the `thinkthen.jsonl` files under the directories claimed above.
