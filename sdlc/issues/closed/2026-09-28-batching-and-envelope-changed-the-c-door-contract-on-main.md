# Batching and the envelope changed the C door contract on main

Status: closed 2026-09-30. Replaced by ADR 0111 and ADR 0112: the port pass with ticket 0291 changes each binding once. Per-port release work stays in the port issues.

## What changed, with authority

1. **Record batches share one packed request.** Bulk `decide`/`filter`/`rank` rows now send a single wire request: state `Each question quotes the text it asks about.` with per-record quoted instructions (`The text is "first". Is it?`). Authority: `specification/records.md` "Order and requests" (~line 81), `specification/backends.md` request shape, ADR 0048 item 1, ADR 0055 items 1+3. Consequence: exact arrival multisets shrink per consumer (e.g. Go 47→39, Zig 50→42, Swift 36→30, COBOL 34→30, PHP 48→40, C# 30 with six packed states, JVM 60 with six packed bodies), and the old three-independent-request reverse-completion stress no longer exists at default packing (JVM notes an explicit `batch=1` scenario would restore it).
2. **The C JSON door returns a success envelope.** Asking calls return `{"value":VALUE,"facts":FACTS}`; `{"usage":true}` stays bare; `details` nests the `thinkthen.result/1` object under `value`. Authority: `specification/types.md` "Answers and failures", `specification/result.md`, `specification/result.schema.json` `$defs/callSuccess`/`$defs/facts`/`$defs/details`. Ports pinned before the landing (Zig, Go, JVM, C#, PHP, COBOL, Ada if affected, Swift's packaged example) needed assertion updates; Dart and C++ already asserted it.
3. **Relation pair requests carry entities only** — the previously closed intentional change (ticket 0167), unchanged further.
4. **Default retries mean four total attempts** (initial + 3): `settings.md` Retries, `settings.rs:115`, `http.rs:271-281`. Zig's gate moved 3→4; PHP/COBOL/Swift gates saw no change in their scenarios — the count is per-gate, so every port gate keeps its own source-derived exact number, never a tolerance.
5. **Repeated batch subsets mint new cache keys** (a fresh request for `[first,second]` after `[first,second,third]`): ADR 0048 item 5 and `records.md` cache paragraphs (~lines 89, 105).

## What each port proved at 71f25087

Every re-pin preserved its unchanged-gate FAIL receipt as drift evidence, then passed a `REPIN-ADAPTED-PACKING-AND-RESULT` (Dart: `REPIN-UNCHANGED-PASS`) gate with exact envelope/field/facts assertions, literal new multisets, and all planted negatives still failing for stated causes. Reports: local experiments 273, 274, 289, 290, 291, 292, 294, 300 — `repin-71f25087-REPORT.md` each, with captured wire bodies and spec citations.

## Ask

1. Confirm each numbered change is intended (1, 2, 4, 5 are new confirmations; 3 is already ruled intentional).
2. Decide whether an explicit `batch=1` reverse-completion scenario belongs in a product stress test, since default packing removed the old one (JVM report).
3. Fold the repin package copies (updated READMEs, fixed packaged examples — JVM's Kotlin/Scala mains and Swift's example asserted the old bare values) into the language merge rather than the sealed stage-two folders.

## Queue-owner disposition

The numbered changes are intended under the cited accepted contracts. Packed bulk requests, the C success envelope, shared relation state, one initial attempt plus the default three retries, and complete request identity already have specification and ticket authority. No new API decision or Ian approval is needed to adapt the package copies to those contracts.

Each integration ticket must retain the exact pinned re-run receipts and update its packaged examples at the final core pin. Do not edit the sealed experiment evidence. The integration reviewer checks literal request bodies, public values and facts, typed failures, and the original planted negatives. A changed count must follow the selected source path, not a tolerance that hides unexpected sends.

Do not add a stress campaign to recreate the old reverse-completion setup. Where a port-specific ordering or cancellation boundary still lacks proof, use one small controlled functional case with explicit batch one and held replies, or cite the equivalent existing boundary proof. Default packing and split-request concurrency are separate behaviors.

This record adds no independent item-table row. J8 and each existing consumer-package issue own the remaining copy, pin and installed-consumer outcomes. The new C++ package handoff is supporting evidence for the existing add-a-C++-binding row, not another independent implementation.

The existing product case `parallel::a_backend_that_answers_out_of_order_still_prints_in_input_order` already covers the batch-one command path. Its helper import is `spawn_one`, which sets `THINKTHEN_BATCH=1` in `tests/backend/harness/mod.rs`. It checks six ordered output rows and concurrent admission against a listener that delays earlier records. The case is not ignored, so the full functional checkpoint retains it. No duplicate stress campaign is needed merely because the default request now packs records. A later change can strengthen the same boundary with controlled completion if it demonstrates a gap; per-port integration still verifies its own ordered decoding and exact request multiset.
