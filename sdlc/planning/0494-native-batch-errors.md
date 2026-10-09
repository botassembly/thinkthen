# R native batch and error conversion

Build from `fc55c0c3f3f11bd7036c4ee9d860a13aaf0eb346`. Convert incremental results, terminal facts and error facts through the shared generated R result graph on R's main thread. Preserve missing values, null, original inputs, typed identities, safe printing, interruption and native finalization.

Claim `libraries/r/thinkthen/src/rust/src/native_results.rs`, `libraries/r/thinkthen/src/rust/src/ffi/complete.rs`, the generated extendr wrappers, `libraries/r/thinkthen/R/native_complete.R`, the error conversion seam in `libraries/r/thinkthen/R/thinkthen.R`, and `libraries/r/tests/native_case.R`. Also claim the R rustfmt child in `sdlc/generators/results/generate.py`: use the existing shared child environment and C target allowlist to prevent inherited credentials and configuration. Keep shared compatibility execution and stream helpers because other hosts still import them.

First reuse generated conversion for incremental rows and final facts. Then use the same converter for native errors, retaining the existing R condition classes. Verify tiny installed eager, batch and refusal calls through the public functions, and run generator freshness, formatting and source policy. Hold full parity, load and release checks for the coordinator.

Canonical Request input conversion and public function consolidation require a separate claim after this checkpoint review. This sequence avoids replacing the input grammar and every result boundary in one unreviewed change.
