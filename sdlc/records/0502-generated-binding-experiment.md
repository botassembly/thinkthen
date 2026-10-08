# Generated binding experiment

Ticket: [0502](../tickets/0502-generated-binding-experiment.md). Starting revision: `163773c2a5b90532f727c3a8733d47271d399da2`.

Choose the approved generated C-header and per-language struct fallback. Keep Python's direct Rust binding. Do not use unmodified quicktype JSON classes as the complete typed SDK. This recommendation stays within the PM architecture ruling; Ian can overturn it.

## What the experiment tested

The owned folder is `experiments/473-thinkthen-generated-bindings/`. Its OWNER names `thinkthen/0502`. It retains the plan, tool lock, generated classes, input schema and cases, probes, source line inventory, raw logs and report. Outputs remain outside Git.

Inputs came from this MIT repository: [result schema](../../specification/result.schema.json), [complete host fixture](../../libraries/php/fixtures/complete.json), [C header](../../libraries/c/include/thinkthen.h), current readers and accepted [ADR 0125](../planning/adr/0125-one-request-contract-and-native-admission.md). The fixture uses synthetic records and outcomes. The probe adds a schema-valid backend failure with retained facts. A wrapper selects the existing completeDecide, completeAnnotation, completeRank and CompleteError definitions; it changes no definition.

Quicktype 26.0.0 generated C# using System.Text.Json and Dart using dart:convert. The local tool installation fetched npm packages once; generation and probes ran offline. No repository dependency or lock changed. Hosts were .NET SDK 8.0.131, Dart 3.13.4, PHP 8.3.6 and Python 3.12.3.

## Results

| Host | Observed behavior |
| --- | --- |
| C# generated JSON classes | Reads decide, annotate, rank and retained failure facts. An omitted failure facts member and a present null member both decode to the same nullable property and reserialize identically. |
| Dart generated JSON classes | Erases the same omitted/null distinction. Also refuses the schema-valid annotation fixture: AnswerAnswer.fromJson asserts that an omitted optional probabilities map is non-null. Optional attempts omission becomes an empty list. |
| PHP direct access | Dictionary decoding keeps omitted keys distinct from null, false values, failed objects and unknown output members. FFI::cdef loads declarations stripped directly from the current C header, allocates header-declared presence fields and creates, fires and frees a cancellation token. |
| Python direct access | Dictionary decoding keeps the same distinctions. A bounded C-header struct subset generator produces ctypes layouts; their optional-content and facts sizes match PHP at 32 and 144 bytes. Direct native token creation, firing and free pass using the lane's existing libraries/c/target/debug/libthinkthen_c.so. That warm library was not rebuilt or qualified against this revision. The product's direct Rust/PyO3 binding remains the preferred Python path. |

Both generated hosts fail the same required omission/null case. That meets the reviewed stop rule. The experiment stops without custom presence converters or a generation framework. JSON Schema validation reports no errors for the fixture. The Dart failure therefore comes from generated reading behavior, not invalid fixture data.

The C header already distinguishes absent optional content from present JSON content, including authored null. Its explicit presence and discriminator fields give the fallback a representation that generic nullable JSON properties lack. A 19-line local generator demonstrates six structs from the existing header; it does not establish a production parser or Rust-to-C header generator.

The source inventory counts nonblank reader lines: C# 127 across current reader files, Dart 1565 across native value and observation readers, PHP 662 in native views, Python 223 in result.rs. These are the named files in reader-lines.json, not equivalent complete host implementations. Generated C# has 1530 nonblank lines and Dart 1189. The probes use 14 C# and 15 Dart handwritten lines; they do not repair the semantic loss. The header subset generator uses 19 lines. No complete production adapter exists here, so the half-reader glue rule cannot honestly be measured for a migration.

The 100000-row runs decode the same synthetic facts object, retaining records=2 on every iteration. Recorded microseconds per row: C# 2.995, Dart 1.437, PHP 0.962, Python 2.358. These are one local decode run per recorded log, not cache-hit engine calls, a transport benchmark or a performance guarantee. There were no paid calls.

## Chosen mechanics and remaining limits

Generate the C header from the Rust boundary, then derive host struct declarations from that header. Use PHP's FFI parser directly. Generate C# and Dart native layouts while retaining small owned-copy readers, typed public calls and native result lifetimes. Keep Python direct to Rust. Share semantic cases across these paths instead of maintaining separate layout truth.

ADR 0125 supplies the request shape while 0491 implements it. This experiment does not choose or change that production request shape. The early semantic stop leaves generated request types, native decide/annotate/rank execution, record and file readers, one-image execution, lazy streaming, cancellation during execution, complete failed-summary layout generation and 100000 real cache-hit rows untested. Token lifecycle and JSON descriptor support do not prove those behaviors. Production migration still needs their focused checks, full layout comparison and owned-value copying. No rank streaming API is proposed; the current header retains aggregate rank execution.

The attempted pi-job inventory runner exited without a report or DONE file. pi-job wait reported no live runner. Direct source counting replaced that inventory, and no background job remains.

## What the build taught us

Schema-generated nullable properties can silently lose caller presence even when the schema describes null correctly. A generator that accepts a schema is not a semantic reader. Native tagged presence fields provide a simpler starting point for this SDK's failure and authored-content contracts. This bounded result settles the generation choice without claiming production readiness.
