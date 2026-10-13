# Installed coverage baseline at da1e3bb17

Measured source: `da1e3bb17e98ae4cde5ee75573351d36f4e3ae4d`. Both Linux runs completed 31 collections; Objective-C Foundation was omitted because it requires macOS. The first took 3611.8 seconds; the second took 3768.8 seconds. Each measured 59,962 mapped nonblank lines across 458 production files.

The [0560 PM amendment](../../tickets/0560-measure-coverage-on-every-surface.md#evidence) accepts this pair as the 0.2 baseline and waives reproducibility. Each candidate collection must reach at least its lower baseline count below, minus lines deleted by the cleanup waves. Objective-C joins at the candidate on macOS. No full coverage measurement runs before the candidate.

| Collection | First reached | Second reached | Lower baseline count |
| --- | ---: | ---: | ---: |
| c-source-unit | 16654 | 16653 | 16653 |
| cli-source | 15153 | 15155 | 15153 |
| installed-ada | 20485 | 20485 | 20485 |
| installed-c | 21927 | 21928 | 21927 |
| installed-cobol | 20605 | 20605 | 20605 |
| installed-cpp | 16994 | 16994 | 16994 |
| installed-csharp | 18081 | 18075 | 18075 |
| installed-dart | 15826 | 15828 | 15826 |
| installed-flutter | 15706 | 15706 | 15706 |
| installed-go | 16723 | 16722 | 16722 |
| installed-java | 14722 | 14722 | 14722 |
| installed-java-usage | 6922 | 6922 | 6922 |
| installed-javascript | 15020 | 15020 | 15020 |
| installed-kotlin | 7520 | 7519 | 7519 |
| installed-kotlin-usage | 6922 | 6922 | 6922 |
| installed-pandas | 16489 | 16489 | 16489 |
| installed-php | 16095 | 16095 | 16095 |
| installed-polars | 13777 | 13778 | 13777 |
| installed-python | 16489 | 16489 | 16489 |
| installed-python-polars | 16489 | 16488 | 16488 |
| installed-python-shared | 16112 | 16109 | 16109 |
| installed-r | 18113 | 18113 | 18113 |
| installed-ruby | 17167 | 17168 | 17167 |
| installed-rust | 13229 | 13228 | 13228 |
| installed-scala | 8091 | 8090 | 8090 |
| installed-scala-usage | 6922 | 6922 | 6922 |
| installed-swift | 16701 | 16702 | 16701 |
| installed-typescript | 15020 | 15020 | 15020 |
| installed-zig | 22335 | 22335 | 22335 |
| rust-integration | 40242 | 40240 | 40240 |
| source-unit | 22264 | 22263 | 22263 |

| Total | First | Second |
| --- | ---: | ---: |
| mapped_nonblank | 59962 | 59962 |
| observed_unit_only | 3417 | 3422 |
| unobserved | 9624 | 9623 |

File counts below come from the second run.

## Most lines observed only in source-unit profiles

| File | Lines |
| --- | ---: |
| libraries/c/src/ffi/current/question/ffi.rs | 244 |
| crates/thinkthen/src/mcp/request.rs | 176 |
| libraries/c/src/complete/execute.rs | 167 |
| crates/thinkthen/src/mcp/tools.rs | 160 |
| crates/thinkthen/src/mcp/runtime.rs | 129 |
| libraries/c/src/current/descriptors.rs | 113 |
| libraries/c/src/complete/metadata.rs | 111 |
| libraries/c/src/current/mod.rs | 103 |
| libraries/c/src/complete/observations.rs | 101 |
| libraries/c/src/complete/questions.rs | 96 |
| libraries/c/src/complete/details.rs | 92 |
| libraries/c/src/current/values.rs | 89 |
| crates/thinkthen/src/mcp/admission.rs | 85 |
| libraries/c/src/complete/structured.rs | 84 |
| libraries/c/src/ffi/current/ffi.rs | 76 |
| libraries/c/src/ffi/complete/ffi.rs | 73 |
| crates/thinkthen/src/public/request/composition.rs | 62 |
| libraries/c/src/ffi/current/read/ffi.rs | 56 |
| libraries/c/src/failures.rs | 50 |
| crates/thinkthen/src/public/request/transport.rs | 49 |
| libraries/c/src/complete.rs | 47 |
| crates/thinkthen/src/cli/args/debug.rs | 46 |
| libraries/c/src/ffi/current/author/ffi.rs | 43 |
| libraries/c/src/current/question.rs | 42 |
| libraries/c/src/ffi/current/recognition/ffi.rs | 41 |
| crates/thinkthen/src/core/answer.rs | 37 |
| crates/thinkthen/src/mcp/executor.rs | 37 |
| crates/thinkthen/src/core/settings.rs | 35 |
| libraries/c/src/complete/inputs.rs | 35 |
| crates/thinkthen/src/public/results/aggregate_reading.rs | 33 |

## Most lines with no observed hits; missing child paths are unmeasured

| File | Lines |
| --- | ---: |
| libraries/c/src/session/views_generated.rs | 1887 |
| libraries/c/src/ffi/complete/ffi.rs | 379 |
| crates/thinkthen/src/public/frame/eager.rs | 338 |
| libraries/c/src/typed.rs | 288 |
| libraries/c/src/ffi/typed_facts/ffi.rs | 277 |
| libraries/c/src/ffi.rs | 261 |
| libraries/c/src/ffi/batch/ffi.rs | 255 |
| libraries/c/src/complete/rows.rs | 226 |
| libraries/c/src/complete/structured.rs | 216 |
| libraries/c/src/call/projection.rs | 209 |
| libraries/c/src/call.rs | 203 |
| crates/thinkthen/src/public/frame/column.rs | 177 |
| crates/thinkthen/src/public/frame/complete.rs | 176 |
| crates/thinkthen/src/public/frame/lazy.rs | 168 |
| crates/thinkthen/src/public/frame/typed_series.rs | 130 |
| libraries/c/src/call/frozen.rs | 120 |
| crates/thinkthen/src/public/frame/inputs.rs | 119 |
| libraries/c/src/complete/inputs.rs | 114 |
| libraries/c/src/ffi/current/author/ffi.rs | 90 |
| libraries/c/src/complete/located.rs | 86 |
| crates/thinkthen/src/public/frame/request.rs | 82 |
| libraries/c/src/complete/metadata.rs | 79 |
| libraries/c/src/plan.rs | 79 |
| libraries/c/src/complete/streaming.rs | 77 |
| libraries/c/src/complete/rank_set.rs | 73 |
| crates/thinkthen/src/public/frame/options.rs | 65 |
| crates/thinkthen/src/mcp/executor.rs | 62 |
| crates/thinkthen/src/public/frame/typed_batches.rs | 61 |
| crates/thinkthen/src/public/request/definition.rs | 60 |
| libraries/c/src/current/question.rs | 58 |
