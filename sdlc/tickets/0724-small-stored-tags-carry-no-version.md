# 0724: Usage files, image tags, ID salts and internal manifests carry no version

Status: OPEN.

Milestone: 0.2.1

Phase: 2

## Outcome

- Changes: `crates/thinkthen/src/engine/usage/counts.rs`, `crates/thinkthen/src/engine/usage/storage.rs`, `crates/thinkthen/src/engine/usage/no_lock_tests.rs`, `crates/thinkthen/src/engine/usage/tests.rs`, `crates/thinkthen/src/windows/checkpoint/tests.rs`, `crates/thinkthen/tests/backend/default_cache/usage.rs`, `crates/thinkthen/tests/backend/status.rs`, `specification/recording.md`, `databases/sqlite/src/images.rs`, `crates/thinkthen/src/engine/invocation.rs`, `crates/thinkthen/tests/backend/secrecy.rs`, `crates/thinkthen/tests/backend/secrecy/routes.rs`, `specification/fixtures/images/0034-liquid-decide-exchange.json`, `specification/fixtures/images/0034-local-decide-exchange.json`, `conformance/backend-profiles.json`, `conformance/calibration.json`, `conformance/cases.json`, `conformance/consumer/consumer/tests/public/settings.rs`, `conformance/parity.py`, `conformance/README.md`, `conformance/record-values.json`, `conformance/settings.json`, `crates/thinkthen/src/cli/conformance_tests/profile_cases.rs`, `crates/thinkthen/src/cli/conformance_tests/support/conformance.rs`, `crates/thinkthen/src/core/result/record_value.rs`, `crates/thinkthen/tests/backend/settings_cases.rs`, `databases/duckdb/tools/settings_cases.py`, `databases/duckdb/tools/verbs_portable.py`, `databases/postgresql/tests/portable_batch.py`, `databases/postgresql/tests/settings_cases.py`, `databases/sqlite/tests/test_portable_batch.py`, `databases/sqlite/tests/test_settings.py`, `libraries/r/tests/settings_cases.R`, `specification/fixtures/batching/portable-records.json`, `specification/fixtures/batching/portable-structured.json`, `specification/fixtures/question-file/corpus.json`, `specification/fixtures/types/check.py`, `specification/fixtures/types/corpus.json`.
- Monthly usage files have no `schema` member (`thinkthen.usage/1`). The leniency for old files (ignored `usage.retries` files, missing `retries` read as 0) is deleted. An unreadable file is refused with one sentence saying to move it.
- SQLite image BLOB tags drop `/1` (`thinkthen.image`, `thinkthen.images`) while no release has stored them.
- The random call, SDK request, observation and failure ID salts drop `/1`.
- Dead `thinkthen.recording/1` references in the secrecy tests and the image exchange fixtures drop their numbers.
- The internal conformance and fixture manifest tags drop their numbers: `settings-cases`, `type-corpus`, `conformance`, `parity-cases`, `calibration`, `backend-profile-conformance`, `record-value-conformance`, `question-file-corpus`, `portable-batch-records` and `portable-structured-batch`.

### Proof

- On the M5 through `pm job run --machine m5 0724 SHA --memory 8 -- cargo nextest run -p thinkthen usage status settings_cases secrecy`, a usage file without `schema` is read, and an old-format usage file is refused with the exact sentence. Before the change, the first is refused and the second is read.
- On the M5 through pm job, `git grep -nE 'thinkthen\.(usage|image|images|call-id|sdk-request-id|observation-id|failure-id|recording|settings-cases|type-corpus|conformance|parity-cases|calibration|backend-profile-conformance|record-value-conformance|question-file-corpus|portable-batch-records|portable-structured-batch)/[0-9]' -- . ':!sdlc' ':!probes'` finds nothing, and `sdlc/scripts/spec` and the SQLite and R checks in `sdlc/scripts/surfaces` pass.
- The DuckDB and PostgreSQL settings and portable-batch suites run at the 0619 container checkpoint.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). Usage files carry `thinkthen.usage/1` plus leniency for 0.1 files that the [no-compatibility ruling](../decisions/2026-10-12-no-compatibility-before-1-0.md) forbids. Split from 0708; these tags share no files with the stored answer rows.

Starts from:

- The `thinkthen.usage/1` rename in `crates/thinkthen/src/engine/usage/counts.rs` and `read`, `scan` and `read_counts` in `crates/thinkthen/src/engine/usage/storage.rs`.
- The `IMAGE` and `IMAGES` constants in `databases/sqlite/src/images.rs`.
- The `fresh` salts in `crates/thinkthen/src/engine/invocation.rs`.
- Each manifest tag's writer and reader in the claimed files.

Keeps: `thinkthen.image-state/1` and `thinkthen.question-key/2` are hash inputs in `crates/thinkthen/src/core/pack.rs` and `crates/thinkthen/src/core/pack/key.rs`, like the cache-key constants 0708 keeps, so they stay. The probe run records under `probes/` carry `thinkthen.recording/1` about 1,950 times as saved measurement output that ThinkThen does not read; they stay.

Shares `conformance/backend-profiles.json` with 0721 and `conformance/cases.json` with 0706, 0707 and 0719. Paths under `crates/thinkthen/src/cli/` move when 0639 lands; the claim follows the move.
