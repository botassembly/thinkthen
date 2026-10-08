# 0489: Recognition context for each record

Source revision: `5e8529d03e103ecb1097a644819aff8a4faf88ab`. This record describes the implementation and its checks. Ticket and lane status remain in pm.

Recognition accepts a context selector through the existing record reader. An empty selected value clears shared context. Missing, null and nontext values retain the existing record admission rules. Every boundary, kind, edge and relation question receives its record's context. Context contributes to cache and recording identity.

The command changes live in `crates/thinkthen/src/cli/args.rs` and `cli/recognize/`. Typed recognition admission lives in `public/complete/recognize/`; stage execution uses `engine/facade/recognize.rs`, `context.rs` and `each.rs`. Existing context rendering supplies wire bodies. Native and backend tests cover admission, stage requests, cache changes, replay and concurrent records. `conformance/recognition-context.json`, `cases.json` and `named-inputs.json` supply shared cases. Existing host fixture builders carry contexts through their typed public inputs. The executable example and recognition, record, C and MCP documentation describe the public behavior.

The coordinator reports accepted code reviews at `b590abfd4`, `fa44dff7d`, integrated `085a6e4e6` and integrated `5e8529d03`. The ticket's review of `95fade3863` covers the plan rather than these code revisions.

The implementation addressed two concrete integration findings. Several typed host fixture builders dropped explicit record contexts; `fa44dff7d` preserved them and added the shared cache/replay recipe. Integrating ticket 0483 changed custom entity wording; `085a6e4e6` corrected exact saved request bodies from “a entity” to “an entity”. Existing fixture paths and assertions were reused.

## Retained checks

`target/0489/` contains the existing logs. The six shared recognition cases passed for all 29 public consumers: distinct contexts, empty context, shared context, null context, nontext context and context cache/replay. This is selected feature coverage, not a completed full surface run. `surfaces.log` contains an interrupted full surface attempt; separate `host-*.log`, selected CLI, C, Rust and MCP logs supply the selected results. Host build and fixture logs retain the qualification for each consumer. No paid backend was used.

`final-test.log` records 1,784 passing workspace tests, passing doctests, 344 passing library-only tests, 23 passing consumer tests, 21 passing parity unit tests, the supporting fixture and transformation checks, the live launcher fixture checks and all 19 binding smoke passes. The script ends by waiting for the smoke process, printing its output and checking its exit code. All printed stage summaries and smoke sentinels are present. The previous task was interrupted before its tool completion was retained, so the top-level test exit code is unknown. No remaining test stage is identified. The full suite was not repeated with unchanged inputs.

The completion pass reran `git diff --check` and `python3 -m unittest conformance.test_parity`; both exited 0, and all 21 parity unit tests passed. `completion-parity-unit.log` holds the latter output.

## Final gates

The final completion environment permits loopback sockets, the systemd user bus and the compiler cache. A loopback bind and a systemd scope both exited 0. Heavy gates ran with `MemoryMax=10G`, `MemorySwapMax=1G` and two build jobs. The earlier restricted attempts in `final-lint.log` do not describe this environment.

`completion-lint.log` records a full `sdlc/scripts/lint` run with exit 0, including policy, child-environment checks, fixture checks, surface registry, formatting, Clippy, documentation and public API inventory. Existing source-size and unmatched license-exception warnings remain nonfatal.

`completion-spec.log` records a full `sdlc/scripts/spec` run with exit 0, including settings, executable specifications, transforms, recognition and relation fixtures, probe harness fixtures and all 24 green demos. The page runner reports its existing skipped examples separately.

`completion-mcp-selected.log` records six passing recognition context cases after the 0487 test-helper integration, with exit 0. The existing MCP conformance runner selected the six canonical rows in memory without changing runner or fixture source. Every case used its own loopback backend and the integrated child-environment helper. `completion-final-parity-unit.log` records 21 passing parity unit tests with exit 0. `git diff --check` exited 0. These checks found no new product defect. The earlier full-suite top-level exit code remains unknown as described above.

The lane occupies about 25 GB across `target/`, `libraries/` and `databases/`, below its 40 GB cap. Reuse complete behavior evidence while its inputs apply; retain an explicit missing exit code rather than inventing a passing result or repeating the suite solely to replace lost process metadata.

## What the build taught us

Test execution limits can prevent loopback proof without indicating a product defect. Preserve the actual failed attempt, then run the missing agreed checks when the required execution environment becomes available. Existing stage output remains useful when source and inputs are unchanged.
