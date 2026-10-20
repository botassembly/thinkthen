# 0707: The result tag carries no version

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Reviews: revision 45c240d, accept

Reviews: revision 26daa36, accept

## Outcome

- Changes: `sdlc/scripts/release-smoke-command-test`, `site/src/data/beatles.mjs`, `site/src/pages/functions/audit.astro`, `site/src/pages/functions/diff.astro`, `site/src/pages/functions/transform.astro`, `site/src/pages/start/tune.astro`, `site/src/pages/databases/*.astro`, `demos/16-triage-pipeline/README.md`, `transforms/README.md`, `transforms/rows/record.sh`, `site/examples/reference/audit/1-counts.out`, `site/examples/reference/diff/1-cuts.out`, `site/examples/reference/transform/1-list.out`, `site/examples/reference/transform/2-show.out`, `site/examples/start/tune/1-suggest.out`, `site/examples/start/tune/2-diff.out`, `site/examples/recipes/ask-your-cache-with-duckdb/1-query.out`, `specification/settings.md`, `specification/mcp.md`, `specification/rust-api-0.2.md`, `specification/cache.md`, `specification/backends.md`, `specification/recording.md`, `spec/rank-sets.md`, `crates/thinkthen/src/schema_tests.rs`, `crates/thinkthen/src/core/result/complete.rs`, `crates/thinkthen/src/public/results/complete_schema.rs`, `crates/thinkthen/src/core/result/complete/atomic.rs`, `crates/thinkthen/src/core/result/complete/recognize.rs`, `crates/thinkthen/src/core/result/complete/annotation.rs`, `crates/thinkthen/src/core/result/complete/relate.rs`, `crates/thinkthen/src/core/result/complete/find.rs`, `crates/thinkthen-cli/src/cli/asking/native/presentation.rs`, `conformance/fixtures/packets.json`, `specification/README.md`, `build-tools/results/generate.py`, `conformance/c_parity.py`, `conformance/cases.json`, `conformance/children/fixture_suite.py`, `conformance/cli_parity.py`, `conformance/fixtures/complete.json`, `conformance/session_projection.py`, `crates/thinkthen-cli/src/cli/audit.rs`, `crates/thinkthen-cli/src/cli/measure.rs`, `crates/thinkthen-cli/src/statistics/answer.rs`, `crates/thinkthen-cli/src/statistics/verbs.rs`, `crates/thinkthen/src/core/result/complete/tests.rs`, `crates/thinkthen/src/core/result/complete/tests/aggregate.rs`, `crates/thinkthen/src/core/result/complete/wire.rs`, `crates/thinkthen/src/ownership_tests.rs`, `crates/thinkthen/src/public/results.rs`, `crates/thinkthen/src/public/results/complete.schema.json`, `crates/thinkthen-cli/tests/backend/`, `crates/thinkthen/tests/fixtures/measure/README.md`, `crates/thinkthen/tests/fixtures/recognize-detailed.json`, `crates/thinkthen/tests/native_complete/`, `crates/thinkthen-cli/transforms/compare.jq`, `crates/thinkthen-cli/transforms/trials.jq`, `databases/sqlite/tests/complete/duckdb_child.py`, `databases/sqlite/tests/complete/project.py`, `demos/16-triage-pipeline/fake-rows.jsonl`, `demos/16-triage-pipeline/review-jsonl`, `demos/16-triage-pipeline/self-test`, `libraries/c/tests/installed_call.py`, `libraries/cobol/checks/blocking.py`, `libraries/csharp/tests/source/NativeCases.cs`, `libraries/csharp/tests/source/NativeChecks.cs`, `libraries/csharp/tests/source/Program.cs`, `libraries/jvm/tests/SessionCases.java`, `libraries/mcp/conformance.py`, `libraries/mcp/installed.py`, `libraries/python/tests/native_case.py`, `libraries/python/tests/test_named_calls.py`, `libraries/python/tests/test_series_accessors.py`, `probes/01-find-vs-rank/`, `probes/02-confidence/`, `probes/03-score/`, `probes/04-option-order/`, `probes/05-irrelevant-option/`, `probes/06-hostile-text/`, `probes/07-true-and-false-texts/`, `probes/08-option-descriptions/`, `probes/09-evidence-shape/`, `probes/annotate-0015/`, `probes/find-0040/`, `sdlc/scripts/generate-c-header.py`, `site/examples/beatles/`, `site/examples/recipes/`, `site/examples/reference/audit/files/shown.jsonl`, `site/examples/reference/diff/files/memory.jsonl`, `site/examples/reference/transform/files/shown.jsonl`, `site/examples/start/tune/files/shown.jsonl`, `spec/result.md`, `specification/annotate.md`, `specification/fixtures/types/corpus.json`, `specification/rank.md`, `specification/recognize.md`, `specification/relate.md`, `specification/result.md`, `specification/result.schema.json`, `specification/types.md`, `transforms/band/test.sh`, `transforms/compare/compare.jq`, `transforms/compare/test.sh`, `transforms/rows/runs/`, `transforms/triage/cases.jsonl`, `transforms/trials/test.sh`, `transforms/trials/trials.jq`, `build-tools/results/csharp/CompleteFacts.g.cs`, `libraries/ada/src/thinkthen-answers.adb`, `libraries/ada/src/thinkthen-session_c.ads`, `libraries/c/include/thinkthen.h`, `libraries/c/src/session/views_generated.rs`, `libraries/cobol/copybooks/tt-native-generated.cpy`, `libraries/cobol/src/tt_requests_generated.c`, `libraries/cobol/src/tt_requests_generated.h`, `libraries/cpp/include/thinkthen/results_generated.hpp`, `libraries/dart/lib/src/session/abi_generated.dart`, `libraries/go/owned_results_generated.go`, `libraries/jvm/session/thinkthen/NativeResults.java`, `libraries/jvm/session/thinkthen/Results.java`, `libraries/objective-c/Sources/Foundation/TTResults.g.m`, `libraries/php/src/session/ffi_generated.h`, `libraries/php/src/session/results_generated.php`, `libraries/python/src/results_generated.rs`, `libraries/python/thinkthen/_native_results.py`, `libraries/python/thinkthen/_native_results.pyi`, `libraries/r/thinkthen/src/rust/src/results_generated.rs`, `libraries/ruby/src/ffi/results_generated.rs`, `libraries/swift/Sources/ThinkThen/ResultsGenerated.swift`, `libraries/typescript/results_generated.d.ts`, `libraries/typescript/results_generated.js`, `libraries/zig/src/results_generated.zig`.
- The generated wrapper files, the C header and `build-tools/results/csharp/CompleteFacts.g.cs` come from the Rust result schema (`crates/thinkthen/src/public/results.rs`, `crates/thinkthen/src/public/results/complete.schema.json`) through `build-tools/results/generate.py` and `sdlc/scripts/generate-c-header.py`. This ticket claims those sources, not the outputs. The lander regenerates the outputs at merge with `python3 build-tools/results/generate.py --all` and `python3 sdlc/scripts/generate-c-header.py`, and `lint`'s currency checks confirm them.
- The result type tag keeps its name and loses its number: `thinkthen.result`, and the `trials` transform's own tag becomes `thinkthen.trials`. Readers match the unnumbered tag. The generated result `Version` enums in the C header and every wrapper become a plain tag.
- The `thinkthen.result/1` refusals in `audit` and `measure` are deleted, and every `result/1` fixture, saved run, probe run and site example is converted to the current shape. The `trials` transform reads current output.
- Version-history prose about result tags in the specification (for example in `types.md` and `result.md`) is deleted.
- The change regenerates `libraries/c/include/thinkthen.h` with `sdlc/scripts/generate-c-header.py`, passes `sdlc/scripts/check-c-exports.py`, regenerates wrappers from `build-tools/results/`, and replays every changed page through `sdlc/scripts/spec` and `sdlc/scripts/demos`.
- The `record-error`, `run` and `status` tags and the plan `schema` members are 0723.

The generated output paths above name the exact lander-regenerated outputs already required; they add no handwritten scope.

### Proof

- On the M5 through `pm job run --machine m5 0707 SHA --memory 8 -- sh transforms/trials/test.sh`, a test pipes real `--details` output into `trials` and gets rows. Before the change, `trials` refuses it with "every row must use schema thinkthen.result/1".
- On the M5 through pm job, `git grep -nE 'thinkthen\.(result|trials)/[0-9]' -- . ':!sdlc' ':!CHANGELOG.md'` finds nothing, and `sdlc/scripts/lint`, `sdlc/scripts/spec`, `sdlc/scripts/demos` and `sdlc/scripts/surfaces` pass for every library that runs on macOS.
- The SQLite DuckDB child projection under `databases/sqlite/tests/complete/` runs at the 0619 container checkpoint.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). The tag has one current value, `thinkthen.result/2`; the number guards nothing, while readers compare exact strings.

Starts from:

- The `thinkthen.result/2` serde rename in `crates/thinkthen/src/core/result/complete/wire.rs` and the tag in `crates/thinkthen/src/public/results/complete.schema.json`.
- The exact-string readers in `crates/thinkthen-cli/src/statistics/answer.rs`, `crates/thinkthen-cli/src/statistics/verbs.rs` and `crates/thinkthen-cli/transforms/compare.jq`.
- The `result/1` refusals: `saved_results` in `crates/thinkthen-cli/src/cli/audit.rs` and the old-format message in `crates/thinkthen-cli/src/cli/measure.rs`.
- `crates/thinkthen-cli/transforms/trials.jq` and its copy `transforms/trials/trials.jq` refuse any row that is not `result/1`, so they cannot read current output; `transforms/trials/test.sh` passes only because it builds `result/1` rows.
- The `THINKTHEN_COMPLETE_VERSION_THINKTHEN_RESULT_2_V1` constant in `libraries/c/include/thinkthen.h` and its generated mirrors.

Collides with 0719 and 0709 on generated wrappers and the C header, and with 0642 on `conformance/fixtures/complete.json`; land them one after another. Paths under `crates/thinkthen/src/cli/` move when 0639 lands; the claim follows the move.

## Progress

- 2026-10-20 started
- 2026-10-20 next: The combined 0707 and 0735 gate stopped in the installed MCP smoke at candidate fee0216; receipt gate-runs/thinkthen/0707/m5/7333c30d4257ed0ceae2d54655e6e8a31edfec03 is retained, and a fresh developer is diagnosing initialization or tool execution before repair and review.

## Current implementation paths, 2026-10-20

The claim follows the landed CLI move and the existing complete-result serializer readers. Generator-owned Ada and other wrapper outputs remain generator-owned. Use existing replay recordings to regenerate historical examples with truthful provenance; make no paid calls and invent no historical identifiers. The existing generators may prepare job-local outputs for checks; the lander commits refreshed outputs as already required above.

The private serde carrier becomes ResultTag so its generated completeResultTag does not collide with the existing completeTag result definition. It remains the required single literal tag, with no additional carrier or call family.

## Saved-example provenance ruling, PM 2026-10-20

Replace unmatched historical samples with retained replayable data while keeping each page's lesson, then regenerate expected teaching output. Where no retained recording teaches that lesson, use labeled synthetic packets from the existing loopback fixtures. Make no paid calls, invent no historical provenance, and keep no reduced legacy shape.

The added page and output paths serve only the approved provenance labels and regenerated examples. Existing loopback fixtures may be invoked in an admitted one-off command; add no retained generator or server. Preserve each distinct policy assertion when expressing synthetic samples in the current native shape, and document any fixture-only question-kind adjustment for review.
