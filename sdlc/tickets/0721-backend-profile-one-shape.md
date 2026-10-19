# 0721: Backend profiles have one unversioned shape

Status: OPEN.

Milestone: 0.2.1

Phase: 2

## Outcome

- Changes: `conformance/backend-profiles.json`, `conformance/calibration.json`, `conformance/consumer/consumer/tests/public/settings.rs`, `conformance/settings.json`, `crates/thinkthen/src/cli/edge/deadline_tests.rs`, `crates/thinkthen/src/cli/profile.rs`, `crates/thinkthen/src/cli/schedule/width_tests.rs`, `crates/thinkthen/src/config/backend_setups_tests.rs`, `crates/thinkthen/src/core/adapters/systemone/images/local_tests.rs`, `crates/thinkthen/src/core/backend_profile.rs`, `crates/thinkthen/src/core/batch/tests.rs`, `crates/thinkthen/src/core/pack/packer_tests.rs`, `crates/thinkthen/src/engine/facade_tests.rs`, `crates/thinkthen/tests/backend/`, `crates/thinkthen/tests/library/`, `crates/thinkthen/tests/native_complete/`, `crates/thinkthen/tests/public_batches/`, `databases/duckdb/tools/backend_setups.py`, `databases/duckdb/tools/settings_suite.py`, `databases/postgresql/check.sh`, `databases/sqlite/tests/test_named_backends.py`, `spec/decide.md`, `specification/backends.md`, `specification/fixtures/images/local/clef-flash-profile.json`, `specification/fixtures/images/local/clef-profile.json`, `specification/fixtures/images/local/imajev-profile.json`.
- Backend profiles are one shape with an optional `image_profile`. `thinkthen.backend-profile/1` and `/2` and their `schema` member are gone. A profile that still has `schema` is refused with one sentence naming the member to delete.
- The backends specification, `spec/decide.md`, the conformance profile files and the image profile fixtures drop the member.

### Proof

- On the M5 through `pm job run --machine m5 0721 SHA --memory 8 -- cargo nextest run -p thinkthen backend_profile profile`, a profile without `schema` loads with and without `image_profile`, and a profile with `schema` is refused with the exact sentence. Before the change, the first is refused as "has schema `thinkthen.backend-profile/1`".
- On the M5 through pm job, `sdlc/scripts/spec` and the SQLite named-backend tests in `sdlc/scripts/surfaces` pass. The DuckDB settings suite and the PostgreSQL check run at the 0619 container checkpoint.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). `profile` accepts versions 1 and 2, where 2 is 1 plus the optional `image_profile`. Split from 0706.

Starts from `BackendProfile::parse` in `crates/thinkthen/src/core/backend_profile.rs`, which branches on `version_two` and refuses a missing schema with the `ProfileError` "has schema" variant, and the profile command in `crates/thinkthen/src/cli/profile.rs`. About 40 Rust test files write profiles with the member.

Shares `conformance/backend-profiles.json` with 0724, which drops that file's own manifest tag. Paths under `crates/thinkthen/src/cli/` move when 0639 lands; the claim follows the move.
