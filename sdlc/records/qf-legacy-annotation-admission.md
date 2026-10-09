# Legacy annotation document admission Quick Fix

ADR 0125 freezes the accepted unversioned C request grammar. Legacy annotation records contain strings that execution reads as structural documents when they are valid JSON. Shared admission translated every string to literal text, so an accepted question with `on` refused before execution. This repair restores that accepted behavior without changing the canonical request grammar or the C ABI.

## Changes

`libraries/c/src/call/legacy.rs` uses `QuestionInput::annotation_document` for legacy annotation strings and retains `RecordEvidence::original()` as JSON or literal text according to `RawRecord::literal()`. Other legacy string records remain literal. The existing ordered parser owns syntax fallback, duplicate and depth refusals. The execution method, result writer, shared Request code, parser, headers and JVM source remain unchanged.

`libraries/c/tests/door/canonical.rs` adds one regression through the existing installed-header C driver and loopback listener. Its projected request exactly matches the unchanged `Matrix.requests[11]` in `libraries/jvm/tests/Matrix.java`. The test pins the selected outgoing sentence, absence of hidden content, returned answers, repeat-cache facts and zero extra requests, duplicate refusal, syntax-invalid literal fallback, and the exact canonical literal-text `on` refusal. The existing `cases::batching::direct_c_annotate_record_nests_at_most_127_levels` retains the 127/128 boundary proof without duplicate new cases.

`libraries/c/ratchet.json` records the measured increase from 18178 to 18273 nonblank Rust lines. The root source ceiling stays 176011. The repair reuses the native parser and C driver. No checker, fixture parser, exported symbol, dependency or test hook was added.

## Evidence

The new regression failed on the pre-fix translator: the projected request and the then-included depth-127 case returned usage errors. The repaired translator passed. After removing duplicate depth cases and extracting the private string-record helper, five final focused C tests passed: both canonical tests, the unchanged depth test, and the exact dynamic/static header-symbol checks. The C driver runs under AddressSanitizer with leak checking and a cleared environment, fresh cache and loopback fake key.

Offline C all-target Clippy with warnings denied, C formatting, both existing Rust source ratchets, whitespace checks and repository policy passed. Policy retains existing file-size warnings outside this change. All heavy checks used an owned unit with MemoryMax 8G, MemorySwapMax 1G and two Cargo build jobs. The lane stayed below 40 GiB; warm builds were retained. No paid calls, release actions or remote-machine checks ran.

The broader C run passed 22 unit tests and 75 of 79 door tests. It exposed four failures outside the changed annotation path. Three reproduce with the pre-fix translator from base `2ca061f51` and identical test inputs: `current::public_batch_metadata_uses_header_kinds_for_setting_and_both_warning_sides` disagrees with the batch-warning expectation; `golden::every_door_reply_keeps_its_bytes` and `plan::p1_plans_with_no_key_and_no_send_and_refusals_keep_the_outputs` expect output without the current request-estimate fields. `source_controls::source_recognition_uses_one_deadline_for_every_record` exceeds its one-second deadline in the wider run and again without concurrent tests. Their expectations were not changed. These failures prevent claiming a green full C suite.

## What the build taught us

Translation must preserve the input's established meaning before shared admission. Reusing the execution parser restores one source of truth for annotation document classification; parsing canonical text as JSON would change a separate caller contract. Tests are registered through nested modules: the existing batching depth case belongs to `cases`, so adding another depth table would duplicate its owning proof.

## Limits

This is Linux C evidence. The complete JVM suite, other installed language surfaces, macOS, Windows and release qualification were not run. Exact fixture equivalence and the real C door cover the unchanged JVM request at its native boundary. The three baseline failures and repeated source-recognition deadline failure remain outside this repair. Fresh independent review accepted revision `8840dafab1b0003f6dc2a87c0d1d809fc3bc0970`, including parser reuse, canonical text boundaries, projection and cache behavior. The repair landed at `d79272adbcd12ab771c4d3f2b20e84167dd9e3db`.
