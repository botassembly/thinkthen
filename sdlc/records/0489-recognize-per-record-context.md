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

## Execution limits

The completion environment blocks socket creation and the systemd user bus with `Operation not permitted`. It also blocks the compiler cache. The first lint attempt exited 1 at the compiler cache. A cache-disabled lint retry passed policy, catalog, ticket checks, recognition keys, child environment checks, C ABI checks and version checks. It was stopped with exit 130 after the scope restriction was confirmed. `final-lint.log` holds the retry output; it is not a passing full lint gate.

Full lint, full specification checks and the requested six-case MCP rerun after the 0487 runner integration remain uncompleted in this environment. Earlier `integrated-mcp-selected.log` records six MCP passes before that runner integration. The current sandbox cannot supply the required loopback access or process scope, and permits no approval escalation. These checks require an authorized execution environment. No new product defect was found by the permitted completion checks.

The lane occupies about 25 GB across `target/`, `libraries/` and `databases/`, below its 40 GB cap. Reuse complete behavior evidence while its inputs apply; retain an explicit missing exit code rather than inventing a passing result or repeating the suite solely to replace lost process metadata.
