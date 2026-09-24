# 0088: Build the public `relate` command

Status: Implemented in the ticket worktree. Independent Sol review and landing remain open.

## Result

The tenth command reads one complete entity set from a JSON document, lines, JSONL, CSV, or TSV. It accepts ordered inline relation rules or one closed version-one `@entities` question file. Command-line field, kind, threshold, and model values independently override file values. Saved calibration profile identity remains distinct from the runtime backend profile.

The command validates the complete set before it sends anything. It delegates wildcard expansion, method selection, option and request-byte fallback, relation state, splitting, and edge assembly to the landed 0081 and 0079 owners. Bare output writes self-contained name-and-kind edges. Dry run writes the exact `thinkthen.relate-plan/1` report. Detailed output writes the ruled Option A `thinkthen.result/1` aggregate with exact successful and failed choice and yes/no entry unions.

Mixed recoverable logical failure buffers and prints successful edges or the complete detailed aggregate, then exits 6. A reply with no valid logical answer exits 4 without output. Record, replay, explicit cache, default cache, request identity, interrupt behavior, closed output, and fixed safe diagnostics use the existing shared infrastructure.

The public specification, question-file schema and corpus, executable page, and replay-only page 45 now cover the command. Ticket 0088 amends ADR 0018's original twenty-page portfolio to 21 because no existing page teaches complete-set relations.

## Red and green evidence

Before implementation, `cargo test --locked -p thinkthen --lib relate_file` failed because `RelateSpec` and its digest support did not exist. `cargo test --locked -p thinkthen --test relate_edge help` failed because Clap had no `relate` command and exited 2. The first backend relation tests reached a deliberate unimplemented failure and exited 70.

The first exact Option A choice assertion later failed because the manual entry serializer emits shortest JSON numbers `1` and `0`, while the test expected `1.0` and `0.0`. Correcting the expected public bytes made the focused test pass without a production change.

Focused green results before lint: 2 pure relation-file tests passed; 1 help test passed; 2 relation question-file tests passed; the shared question-file corpus test and schema self-test passed; 12 relation and relation-security backend tests passed together; the exact successful and failed choice-entry test passed; the aggregate interrupt test passed; 4 executable relation cases passed; and all 3 page-45 replay cases passed. These tests cover canonical bytes and digest, precedence, complete-set framings and empty outcomes, zero-send refusals, exact dry-run request identity, bare ordering, exact Option A unions, inclusive cuts, exit 6, exit 4, cache, record, replay, replay miss, all-framing secrecy, authorization-header-only key use, backend failures, closed output, and cancellation.

No live, paid, or external backend call ran. The implementer did not run the full four-rung ladder, as the ticket handoff required.

## Budget

The implementation changes exactly 15 production Rust files: `cli/args.rs`, `cli/args/command.rs`, `cli/args/relate.rs`, `cli/failure.rs`, `cli/failure/relate.rs`, `cli/mod.rs`, `cli/relate.rs`, the five files under `cli/relate/`, `core/mod.rs`, `core/records.rs`, and `core/relate_file.rs`.

It changes 9 test-only Rust files: `core/relate_file/tests.rs`, `tests/relate_edge.rs`, `tests/backend/{main,interrupt,relate,relate_security}.rs`, and `tests/question_file/{main,corpus,relate}.rs`.

The gross additions are 1,199 nonblank production Rust lines and 785 nonblank test Rust lines, 1,984 total. The accepted ceilings are 1,200, 1,100, and 2,300. The largest changed Rust file is `cli/failure.rs` at exactly 500 nonblank lines. The Rust ratchet rises from 41,045 to the measured 43,020 lines. The implementer checked the shared planner, state, fallback, splitter, threshold, edge assembly, recording, and result owners before raising it; no second owner was added.

The first full lint attempt reached Clippy and rejected nested outcome handling, two eight-argument render functions, default-field reassignment, and unscoped fixture assertions. The remediation extracted one local outcome helper, passed each renderer one compact behavior-local context, initialized replay state directly, and scoped the existing test-fixture lint policy at the module declarations. Focused Clippy then exited 0. Production behavior and shared relation owners did not change.

The complete lint rung then exited 0 with both backend environment variables unset. Policy, package graph and verification, doctests, documentation, page checks, dependency checks, Clippy, and the exact `43020/43020` ratchet passed. The final focused matrix also passed the shared secrecy and refusal filters, relation interrupt proof, schema self-test, 4 executable relation cases, 3 page-45 replay cases, formatting, page inventory, 500-line scan, and `git diff --check`.

## Open review work

Independent Sol code review remains the next gate. The coordinator still owns the complete sequential install, lint, test, and specification ladder from the exact reviewed candidate. Ian can overturn page 45's addition to the documentation portfolio; doing so requires another home for the accepted replay-only how-to.
