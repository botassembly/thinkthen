---
flow: build
priority: 88
opens: crates/thinkthen/src/core crates/thinkthen/src/cli crates/thinkthen/tests specification spec demos conformance sdlc/ratchet.json sdlc/planning
---

# 0088: Build the public `relate` command

Status: design accepted by independent Sol Medium review at `113642b2`; landed 0081 clears implementation.

## Outcome

Add `thinkthen relate` over the shared relation foundation from 0081. It reads one complete entity set, accepts the settled inline or `@entities` relation grammar, emits self-contained name-and-kind edges, and exposes the exact dry-run and Option A detailed schemas. Recoverable mixed logical failure emits the ruled partial output and exits 6.

The complete settled contract is `sdlc/planning/relate-design.md`. This ticket implements that contract without changing the shared planner, relation state, fallback rules, request identity, or recognition behavior landed by 0081.

## Public contract

The command forms are `thinkthen relate [OPTIONS] RELATION...` and `thinkthen relate [OPTIONS] @entities.json`. The referenced file is the complete relation question file. It carries ordered relations, optional `reads` and `either`, threshold, saved calibration `profile`, model, and the two structured pointers under the ordinary question-file precedence rules. Saved `profile` has no command-line override and changes canonical question identity. `--profile FILE` is a separate runtime backend-profile file that supplies a name and local limits; it never replaces the saved calibration identity. Framing and input path remain command-only. Inline rules use `NAME=SOURCE_KIND:TARGET_KIND`; bare `NAME` means `NAME=*:*`; `--either` marks inline rules unordered. Inline rules and `@entities` are mutually exclusive.

A structured document selected by the pointers, plus JSONL, CSV, and TSV records, resolves `/name` and `/kind` independently against each original object. `--field` replaces the name pointer and `--kind-field` replaces the kind pointer. CSV and TSV headers form the addressed object. Each selected value is a nonempty string. `--lines` refuses both pointers, uses each complete nonempty line as the name, assigns synthetic kind `*`, and accepts only bare or `*:*` rules. The complete set validates before any request. A malformed inline rule, missing or mistyped pointer, blank value, duplicate name-and-kind identity, absent concrete kind, incompatible line rule, or 256th entity exits 2 with zero sends. An unreadable, invalid, extra-key, wrong-version, or wrong-shape question file exits 5; a valid file for another verb exits 2.

Empty bytes under `--lines` or `--jsonl` succeed with no output and no request. A blank line is an invalid empty entity under `--lines` at exit 2; JSONL permits no blank line and refuses one at exit 2. Empty bytes under `--csv` or `--tsv` exit 2 because the header is missing. A valid CSV or TSV header with no data rows succeeds with no output and no request. Empty document input exits 2. These outcomes apply equally to a normal run and `--dry-run`; successful empty streams and header-only tables print no dry-run object.

Bare output writes one compact edge per line in ruled order. Endpoints contain only `name` and `kind`. One-way output keeps declared source-to-target direction even when the target side asked. Either output normalizes to input order. No edge appears twice.

`--dry-run` sends nothing and emits the exact `thinkthen.relate-plan/1` schema in `relate-design.md`: resolved runtime backend-profile name, framing and fields, question-file `from` provenance, entity count, expanded concrete relations, selected method and fallback reason per concrete relation, exact logical and split request counts, and each exact UTF-8 request body with its byte count and digest. It makes no token or price claim.

`--details` emits the ruled Option A `thinkthen.result/1` object with key order `schema`, `value`, `question`, `answer`, `meta`. The resolved question, digest boundary, four ordered answer-entry unions, target-side asker roles, candidates, `none`, pre-threshold `pick`, request digests, aggregate metadata, and always-present `failed_questions` follow `relate-design.md` exactly.

When at least one logical relation answer is valid and at least one is recoverably failed, the command buffers the aggregate, emits successful bare edges or the complete Option A object with failed entries, and exits 6. No valid logical answer remains exit 4 with no output. Transport, status, replay, local, output, cancellation, and defect failures retain their current codes and never become partial success. Exit 4 with no output and exit 0 with partial output remain rejected alternatives.

## Scope and exclusions

Scope includes relate arguments and help; `@entities` parsing, canonical digest, precedence, and JSON Schema; one structured document and all four record framings; complete-set validation; command orchestration through 0079 and 0081; bare, Option A, dry-run, and exit-6 rendering; offline fixtures; replay/cache integration; full shared secrecy and refusal coverage; specification, executable page, and one replay-only ADR 0011 how-to; exact ratchet and durable record updates.

Exclude changes to the planner, relation state, fallback, recognition, generic splitter or scheduler; incremental or two-set input; public method or one-to-many controls; runner-up questions; library, database, C, or `surfaces` APIs; dependencies; credentials; production data; retained artifacts; live calls; paid calls; publication.

## Owners and budget

Production owners are a new `core/relate_file.rs` with behavior-local parsing/digest support; `core/mod.rs`; new `cli/relate.rs` submodules for config, input, run, dry run, and result; a new relate argument owner called by `cli/args`; command dispatch; and behavior-local failure mapping. Documentation owners are `specification/{README,channels,question-file,result,records,recording,relate}.md`, the question-file schema/fixtures, `spec/relate.md`, and one replay-only demo.

Test owners are new core relate-file tests, `tests/relate_edge.rs`, and backend relate modules plus the shared profile, cache identity, recording/replay, secrecy, refusal, conformance, and question-file suites. Split the 500-line secrecy file and 497-line failure file before adding cases. Do not add relate results to 499-line `core/result.rs` or fill 408-line `cli/args.rs` when a behavior-local owner is available.

Change or add at most 15 production Rust files and 11 test-only Rust files. Add at most 1,200 nonblank production Rust lines and 1,100 nonblank test Rust lines, 2,300 gross. Keep every Rust file at or below 500 nonblank lines and add no dependency. The implementation record lists actual files and gross additions and explains any variance before code review.

## Acceptance gates

1. An independent Sol reviewer returns `ACCEPT` on this ticket and the 0088 sections of `relate-design.md` before product code starts. Implementation still waits for landed 0081.
2. Red then green: `cargo test --locked -p thinkthen --lib relate_file` pins the full `@entities` and inline grammar, precedence, canonical bytes and digest, saved calibration identity in that digest, relation order, pointer defaults and overrides, line restrictions, duplicates, absent kinds, and the 255-entity boundary.
3. Red then green: `cargo test --locked -p thinkthen --test backend relate` pins document/lines/JSONL/CSV/TSV behavior, exact bare ordering, reverse asking, either normalization, wildcard expansion as consumed from 0081, exact dry-run JSON, exact Option A JSON for successful and failed choice and H entries, inclusive cuts, partial output at exit 6, no-valid-answer exit 4, cache, record, replay, cancellation, and output failure. Dry-run snapshots prove inline rules omit `from`; a file-backed mixed-precedence case writes exact `question`, `threshold`, `model`, `field`, `kind_field`, and saved `profile` provenance; and `backend_profile` is null without `--profile FILE` and the resolved file name with it. The matrix separately pins empty line and JSONL streams, a blank line in each, empty CSV and TSV, valid header-only CSV and TSV, and an empty document, including normal and dry-run output and zero-send counts.
4. Red then green: `cargo test --locked -p thinkthen --test question_file relate` pins the published grammar, schema, precedence, unreadable and invalid question-file exit 5, wrong-verb exit 2, and saved calibration identity distinct from runtime `--profile FILE`. `cargo test --locked -p thinkthen --test relate_edge help` pins help text, the beta label, and secrecy wording.
5. The full shared secrecy route covers bare/details, document/lines/JSONL/CSV/TSV, success, dry run, default and explicit cache, record, replay, replay miss, missing key, transport/status/decode and mixed logical failures, backend-profile and field refusals, hostile and damaged requested recordings, storage failure, and the authorization-header-only key path. It inspects stdout, stderr, every `Debug` value, request bodies, recording/cache files, and fixtures. The shared refusal sweep pins exact safe messages and zero loopback sends for every local refusal. `cargo test --locked -p thinkthen --test backend secrecy` and `cargo test --locked -p thinkthen --test backend refusals` both exit 0.
6. `spec/relate.md`, the relation question-file fixtures, and the replay-only how-to pin the public examples, exit 6, replay secrecy, no token or price claim, and no public method control. `sdlc/scripts/spec` runs all of them.
7. Independent Sol code review checks every public schema, failure class, no-send path, secrecy route, 0081 reuse, ordering rule, documentation claim, and budget. It rejects a second planner, assembler, fallback, threshold, splitter, edge serializer, or request-state owner.
8. The coordinator runs `sdlc/scripts/install`, `sdlc/scripts/lint`, `sdlc/scripts/test`, and `sdlc/scripts/spec` sequentially from the exact candidate revision with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset and no competing Rust build, then runs `git diff --check`. Every command exits 0. No live or paid call runs.

## Dependencies and route

Landed 0079 and landed 0081 are required. No other ticket blocks implementation.

Contract 2; state and timing 2; reach 1; proof 2; cost of error 1; total 8; minimum and final level 3 because partial failure, cache/replay identity, exact public schemas, and credential secrecy interact. Ian ended the Luna trial after two 0081 remediation passes and ordered Sol Medium to drive redesigned tickets 0081 and 0088. A separate Sol reviewer remains independent. Stop and re-score if implementation changes the settled public contract, 0081, a dependency, another surface, or this budget.
