# 0707: The result tag carries no version

Status: OPEN.

Milestone: 0.2.1

Phase: 2

## Outcome

- Changes: `build-tools/results/generate.py`, `conformance/c_parity.py`, `conformance/cases.json`, `conformance/children/native_fixture.py`, `conformance/cli_parity.py`, `conformance/fixtures/complete.json`, `conformance/session_projection.py`, `crates/thinkthen/src/cli/audit.rs`, `crates/thinkthen/src/cli/measure.rs`, `crates/thinkthen/src/core/measure/answer.rs`, `crates/thinkthen/src/core/measure/verbs.rs`, `crates/thinkthen/src/core/result/complete/tests.rs`, `crates/thinkthen/src/core/result/complete/tests/aggregate.rs`, `crates/thinkthen/src/core/result/complete/wire.rs`, `crates/thinkthen/src/ownership_tests.rs`, `crates/thinkthen/src/public/results.rs`, `crates/thinkthen/src/public/results/complete.schema.json`, `crates/thinkthen/tests/backend/`, `crates/thinkthen/tests/fixtures/measure/README.md`, `crates/thinkthen/tests/fixtures/recognize-detailed.json`, `crates/thinkthen/tests/native_complete/`, `crates/thinkthen/transforms/compare.jq`, `crates/thinkthen/transforms/trials.jq`, `databases/sqlite/tests/complete/duckdb_child.py`, `databases/sqlite/tests/complete/project.py`, `demos/16-triage-pipeline/fake-rows.jsonl`, `demos/16-triage-pipeline/review-jsonl`, `demos/16-triage-pipeline/self-test`, `libraries/c/tests/installed_call.py`, `libraries/cobol/checks/blocking.py`, `libraries/csharp/tests/source/NativeCases.cs`, `libraries/csharp/tests/source/NativeChecks.cs`, `libraries/csharp/tests/source/Program.cs`, `libraries/jvm/tests/SessionCases.java`, `libraries/mcp/conformance.py`, `libraries/mcp/installed.py`, `libraries/python/tests/native_case.py`, `libraries/python/tests/test_named_calls.py`, `libraries/python/tests/test_series_accessors.py`, `probes/01-find-vs-rank/`, `probes/02-confidence/`, `probes/03-score/`, `probes/04-option-order/`, `probes/05-irrelevant-option/`, `probes/06-hostile-text/`, `probes/07-true-and-false-texts/`, `probes/08-option-descriptions/`, `probes/09-evidence-shape/`, `probes/annotate-0015/`, `probes/find-0040/`, `sdlc/scripts/generate-c-header.py`, `site/examples/beatles/`, `site/examples/recipes/`, `site/examples/reference/audit/files/shown.jsonl`, `site/examples/reference/diff/files/memory.jsonl`, `site/examples/reference/transform/files/shown.jsonl`, `site/examples/start/tune/files/shown.jsonl`, `spec/result.md`, `specification/annotate.md`, `specification/fixtures/types/corpus.json`, `specification/rank.md`, `specification/recognize.md`, `specification/relate.md`, `specification/result.md`, `specification/result.schema.json`, `specification/types.md`, `transforms/band/test.sh`, `transforms/compare/compare.jq`, `transforms/compare/test.sh`, `transforms/rows/runs/`, `transforms/triage/cases.jsonl`, `transforms/trials/test.sh`, `transforms/trials/trials.jq`.
- The generated wrapper files, the C header and `build-tools/results/csharp/CompleteFacts.g.cs` come from the Rust result schema (`crates/thinkthen/src/public/results.rs`, `crates/thinkthen/src/public/results/complete.schema.json`) through `build-tools/results/generate.py` and `sdlc/scripts/generate-c-header.py`. This ticket claims those sources, not the outputs. The lander regenerates the outputs at merge with `python3 build-tools/results/generate.py --all` and `python3 sdlc/scripts/generate-c-header.py`, and `lint`'s currency checks confirm them.
- The result type tag keeps its name and loses its number: `thinkthen.result`, and the `trials` transform's own tag becomes `thinkthen.trials`. Readers match the unnumbered tag. The generated result `Version` enums in the C header and every wrapper become a plain tag.
- The `thinkthen.result/1` refusals in `audit` and `measure` are deleted, and every `result/1` fixture, saved run, probe run and site example is converted to the current shape. The `trials` transform reads current output.
- Version-history prose about result tags in the specification (for example in `types.md` and `result.md`) is deleted.
- The change regenerates `libraries/c/include/thinkthen.h` with `sdlc/scripts/generate-c-header.py`, passes `sdlc/scripts/check-c-exports.py`, regenerates wrappers from `build-tools/results/`, and replays every changed page through `sdlc/scripts/spec` and `sdlc/scripts/demos`.
- The `record-error`, `run` and `status` tags and the plan `schema` members are 0723.

### Proof

- On the M5 through `pm job run --machine m5 0707 SHA --memory 8 -- sh transforms/trials/test.sh`, a test pipes real `--details` output into `trials` and gets rows. Before the change, `trials` refuses it with "every row must use schema thinkthen.result/1".
- On the M5 through pm job, `git grep -nE 'thinkthen\.(result|trials)/[0-9]' -- . ':!sdlc' ':!CHANGELOG.md'` finds nothing, and `sdlc/scripts/lint`, `sdlc/scripts/spec`, `sdlc/scripts/demos` and `sdlc/scripts/surfaces` pass for every library that runs on macOS.
- The SQLite DuckDB child projection under `databases/sqlite/tests/complete/` runs at the 0619 container checkpoint.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). The tag has one current value, `thinkthen.result/2`; the number guards nothing, while readers compare exact strings.

Starts from:

- The `thinkthen.result/2` serde rename in `crates/thinkthen/src/core/result/complete/wire.rs` and the tag in `crates/thinkthen/src/public/results/complete.schema.json`.
- The exact-string readers in `crates/thinkthen/src/core/measure/answer.rs`, `crates/thinkthen/src/core/measure/verbs.rs` and `crates/thinkthen/transforms/compare.jq`.
- The `result/1` refusals: `saved_results` in `crates/thinkthen/src/cli/audit.rs` and the old-format message in `crates/thinkthen/src/cli/measure.rs`.
- `crates/thinkthen/transforms/trials.jq` and its copy `transforms/trials/trials.jq` refuse any row that is not `result/1`, so they cannot read current output; `transforms/trials/test.sh` passes only because it builds `result/1` rows.
- The `THINKTHEN_COMPLETE_VERSION_THINKTHEN_RESULT_2_V1` constant in `libraries/c/include/thinkthen.h` and its generated mirrors.

Collides with 0719 and 0709 on generated wrappers and the C header, and with 0642 on `conformance/fixtures/complete.json`; land them one after another. Paths under `crates/thinkthen/src/cli/` move when 0639 lands; the claim follows the move.
