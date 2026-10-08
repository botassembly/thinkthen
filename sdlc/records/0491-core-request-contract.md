# 0491: Share one typed request contract

Source revision: `d00961c63`. The final implementation uses landed main `b62c6763f5491ac0396516c443b3e7193f799d9b`, including the independently landed recognition carrier, approved Request inventory, recognition observer documentation and its executable Rust example. The final rebase had no conflicts and skipped the already-landed inventory metadata. Development carrier commits were removed in the earlier rebase. This record describes source and observed checks. Ticket and lane status remain in pm.

`crates/thinkthen/src/public/request.rs` and its modules define the closed `thinkthen.request/1` contract for ten functions, typed selectors, records, sources, feeds, attachments and controls. Pure header admission precedes question resolution and evidence reads. Loaded definitions receive separate native admission after their necessary read. Inline content and context declarations receive native validation before attachment reads. Engine execution retains existing typed complete methods, scheduling, controls, completed prefixes and errors. Prepared native definitions enter directly without JSON conversion.

`cli/request.rs` constructs the same typed header before environment lookup, source reading or saved-question reading. Existing edge formatters retain CLI diagnostics. Core image route admission shares the existing capability fact and refuses unavailable attachment routes before file reads. The native frame and Request threshold overrides share cut and band kind updates.

The C call boundary retains effective-last legacy envelope routing. Only an effective string beginning `thinkthen.request/` selects strict decoding, which receives the original JSON bytes. Canonical duplicates and unknown versions reject; unrelated legacy schema values remain ignored. Legacy translation admits the same native definitions. The C writer and shared R/Python source bridge consume those prepared definitions without another question parse. Frozen bare replies and record diagnostics retain their existing writers.

`specification/request.schema.json` comes from test-only deserialization schema generation. Its authored-definition override reuses the committed question-file grammar for primitive definitions, declarations, descriptions and pointers, with closed recognition and set projections. Native admission retains semantic rules that JSON Schema cannot express. Canonical evidence and descriptions retain authored data shapes and order. Present null controls refuse; empty contexts and example lists remain explicit overrides.

## Retained checks

All checks below use offline locked dependencies and isolated loopback fixtures. Heavy native and C checks ran in systemd scopes with an 8 GB memory limit. No paid backend was used.

- `target/0491-final3-native.log`: one Linux CLI authority test and three native Request tests pass. The authority test proves working file watches, then counts zero saved-question reads, source reads, enumeration and sends for invalid headers. Request tests compare all ten functions with native calls, exact outgoing bodies and facts, preserve cached answer and observation identities, retain shared recognition examples and per-record empty overrides, and compare a text Request band with native band reading.
- `target/0491-final-schema.log`: three strict-decoder, generated-byte and independent Draft 2020-12 validator tests pass, including fourteen authored-definition cases. `target/0491-schema-rewrite3.log` retains the intentional failing rewrite before the successful comparison.
- `target/0491-final-backend.log`: 930 CLI backend tests pass; three entries remain ignored in the top-level run. `exchange::a_server_retry_floor_is_waited_once` requires the separate wall-clock stress runner. `named_backends::backend_setups::backend_setup_builder_child` is executed by its parent with an isolated captured environment; `named_backends::builder::builder_child` is the child half that its parent runs with `--ignored`. This run follows the window, projection and diagnostic compatibility corrections. The later shared threshold helper receives the focused native band test and all-target Clippy check.
- `target/0491-final-c.log`: 74 C tests pass, including frozen replies, bare output bytes, canonical routing and sanitizer checks. The shared C fixture runner passes 55 cases and explicitly does not run its one internal defect-injection case.
- `target/0491-final2-clippy.log`: package all-target Clippy passes with warnings denied. `target/0491-final-cclippy.log`: C all-target Clippy passes with warnings denied. The C check precedes the small shared threshold helper; the final C regression rebuild includes that helper.
- `target/0491-integrated-policy.log`: policy exits 0 for 268 resolved packages and confirms every accepted table, ban list and dependency matches. Existing package-size and license-exception warnings remain visible. Formatting and whitespace checks pass; `target/0491-final-children.log` reports zero child-environment findings. Source ceilings equal the measured source totals and their commit explains the added edge code and removed duplication.

The checks do not claim a full surface run, a release or live backend validation. Other language migrations and generated binding access remain separate tickets.

## Review corrections

The fresh review at `2edd8049c` found three defects. The first policy result in this record was incorrect: a multi-command shell returned the later formatting command's success and hid the policy command's nonzero exit.

- Extracting offline dispatch moved the Audit/Diff markers after `Environment::read` in the standing policy's router check. Restore direct early dispatch without changing the checker; extract the existing held-model warning to keep the entry function within its limit. `target/0491-integrated-policy.log` records the separate passing policy exit. The first correction Clippy run refused 94 lines; `target/0491-correction2-clippy.log` records exit 0 after the warning extraction.
- The recognition CLI adapter omitted its context selector. Carry `arguments.context_field` into RequestOptions. Extend the existing watched-authority case with invalid `--context-field`, exact established stderr, and zero question/source access or sends. `target/0491-correction-authority.log` records exit 0.
- Shared admission replaced the frozen legacy filter-band diagnostic with a generic kind mismatch. Refuse the translated band at the legacy edge before shared admission and extend the existing C canonical-routing fixture. `target/0491-correction-c.log` records exit 0 with the exact message and no extra sends.

## Full gates and fixture review

The fresh correction review accepted product source `f32a0211d`. A separate fresh fixture review accepted `850ae5a66c51232515bf66807d3ec36b4099b834`. The final clean rebase preserves those reviewed changes and incorporates only separately reviewed main documentation and metadata; no product behavior changed during integration.

`target/0491-full-lint.log` exits 1 at its final inventory check because the approved Request declarations were absent from the ticket. Its earlier policy, fixture, ceiling, formatting, dependency, workspace Clippy and documentation stages pass. The coordinator adds the exact 203 declarations through the existing ticket block; `target/0491-full-inventory.log` then exits 0 with 1,815 declarations checked and all four plants refused. Unaffected earlier lint stages were not repeated.

`target/0491-full-test.log` retains the first actual exit 100: 1,753 workspace tests pass and the native image duplicate case observes two sends instead of one. The failure stops 57 remaining candidate tests; 27 configured skips are separate. `target/0491-image-isolated.log` retains the unchanged exact test's exit 0. These results did not trace the failed run's event ordering.

`target/0491-full2-test.log` retains the unchanged normal rerun's exit 0: 1,811 workspace tests pass with 27 configured skips, 351 library-only tests pass with four configured skips, 23 external consumer tests pass with three configured skips, 21 parity unit tests pass, supporting fixture and transformation checks pass, and all 19 binding smokes pass. `target/0491-full-spec.log` exits 0 with the executable specifications, supporting fixtures and all 24 green demos. Existing skipped examples remain explicit in the logs.

Read-only inspection finds that ADR 0123 permits an equal no-cache key to send again after its earlier answer completes. The image fixture had not established pending overlap. Hold only the first response with the existing rendezvous, and release it on the two-item iterator's third pull, after both inputs have been admitted. Keep the exact one-send and two-send assertions, outgoing bodies, attachments and original rows. This is a deterministic test precondition; the first failed run's ordering is inferred rather than traced. Existing `backend/batching/ceiling.rs` coverage for completed no-cache repeats remains unchanged.

`target/0491-image-handshake2.log` records the corrected exact default-feature test passing. `target/0491-image-workspace.log` records the normal workspace stage under load after the final fixture correction: exit 0, all 1,811 tests pass with 27 configured skips, including the image case. `target/0491-image-library-only.log` records the affected exact library-only image test passing with exit 0. `target/0491-image-clippy2.log` and `target/0491-image-policy.log` record separate exit 0 results for all-target package Clippy and policy. The final root ceiling is 170,787 measured nonblank lines. The unchanged consumer, smoke and specification checks were not repeated for the fixture or documentation integration.

## What the build taught us

Effective-last legacy routing must select a decoder without normalizing the original canonical document. Otherwise a repeated canonical marker or nested control can disappear before strict admission.

Native default threshold provenance matters in canonical roundtrips. Serializing a synthetic default as authored changes batch warnings and facts. Request options carry native call overrides while authored definitions retain their existing provenance.

A pure admission test must observe real authority use. Working file-access watches and a loopback request counter expose reads and sends that a plan-only check cannot prove. Saved-definition validation necessarily follows its read and deserves a separate claim.

The initial broad definition schema hid a structural mismatch. Reusing the authored schema and checking malformed definitions against actual decoding removed that mismatch without another primitive grammar. Optional recognition parser aliases also proved unnecessary; the incorporated carrier keeps the requested text-first notation and structured examples.
