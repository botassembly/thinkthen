# Upgrade calls to development 0.2

Public installation remains 0.1.2. Use these mappings with a matching reviewed development package. The [binding guide](BINDING-AUTHOR.md) links the generated contracts. Each package README owns its exact calls, input types, generated results and installation route. Installed routine checks establish the tested local packages; final distribution assembly, registry installation, full parity and platform qualification belong to the candidate.

## Replace removed calls

All judging surfaces expose the ten named functions: `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate`. Use the owning guide for each language's spelling and argument order. The replacements below are the sole recommended host APIs. The native C package retains its frozen 0.1 compatibility exports separately.

| Previous call or carrier | Development replacement and owning guide |
| --- | --- |
| Python curried `tt.decide(q)(rows)`, `Judge`, `engine.complete`, `tt.details`, `complete.Item/Image/Files`, legacy batch and module `plan` | Named `Engine` calls; `details=True` retains generated rows; `Item`, `Image`, `Files` and `QuestionSource` supply native inputs; `engine.iterate` streams and `engine.plan` previews. Module verbs accept question and input together. [Python](python/README.md) |
| TypeScript `Engine`, default verbs, `complete`, `*_many`, `details`, `plan`, `CompleteTypes` and `call.value` | Ten named `Client` methods; `Client.item/files/questionFile/questionName/questionReference`; `client.start` streams; generated `Results`; read `call.results` and `call.facts`. [TypeScript](typescript/README.md) |
| Ruby module verbs, `Engine`, `Engine#complete`, `*_many` and copied question/input/complete carriers | Named `Client` methods; `Client.open` scopes cleanup; arrays replace many calls; ordinary hashes and `Client.item/files/question_file/question_name/question_reference` replace old carriers; `client.plan` previews. [Ruby](ruby/README.md) |
| R `tt_*_complete`, `tt_*_batch`, `tt_details`, judging closures and simplified frames | Named `tt_*` calls with authored question lists and `options = list()`; `tt_batch("*", question, input)` streams; `tt_files` supplies sources; read generated `$results` and `$facts`. [R](r/README.md) |
| C++ `tt::create`, `tt::call`, `tt::Engine`, `tt::many`, `tt::native`, `tt::complete` and copied JSON views | `tt::Client` named methods with generated `tt::inputs` and `tt::results`; `Call` owns polling, cancellation and cleanup. [C++](cpp/README.md) |
| Go `Engine`, generic `Call`, old named judgments and handwritten readers | `NewClient(settings)` and ten named `Client` methods with generated inputs, results and `RequestSource`; `Client.Plan(Request)` previews. [Go](go/README.md) |
| Java `Door`, complete/map overloads and preview facades; Kotlin/Scala facades | Named `thinkthen.Engine`, `KotlinEngine` and `ScalaEngine` methods with generated declarations and session packets. [JVM](jvm/README.md) |
| C# `Engine.Open(settingsJson)`, generic `Call/CallTyped`, old batch readers and plan arguments | `Engine.Open(InputEngineSettings)`, named generated-input `*Async` methods, streaming descriptor overloads, `ExecuteAsync/StartSession(InputRequest)` and `Plan(InputRequest)`; generated `ThinkThen.Results`. [C#](csharp/README.md) |
| Swift `Engine`, JSON calls, `NativeQuestion/NativeSource` and handwritten complete readers | `Client` named async methods with generated `Input*` and `Owned*` values; `SessionFailure.call` retains failure settlement. [Swift](swift/README.md) |
| Zig older JSON and complete families | Named typed sessions with generated questions, request inputs and owned packet values; `Request.plan` returns an owned native plan. [Zig](zig/README.md) |
| PHP `ThinkThen`, `ThinkThen\Native\Engine` and handwritten native readers | `Client` named methods, explicit input selectors and generated `Completed` results with terminal facts. [PHP](php/README.md) |
| Dart `Door`, older named judgments and copied complete readers | Named `Engine` methods over the packaged native asset and session route. [Dart](dart/README.md) |
| Objective-C `TTClient`, GNU sources and JSON/files calls | Apple-only `TTFoundationClient` named methods, generated results, `TTTask` cancellation, ARC and `NSError`. GNU support has ended. [Objective-C](objective-c/README.md) |
| Ada root judging APIs, handwritten C carriers and JSON `Plan` wrapper | `Thinkthen.Sessions.Calls` named procedures and generated native declarations. Direct ABI preview remains; the package claims no named typed Ada Plan result. [Ada](ada/README.md) |
| COBOL `TT-DECIDE`, `TT-CALL`, descriptor calls and handwritten copybooks | Ten `TT_SESSION_*` functions with generated question/input records and owned session values. Platform layouts require their matching generated declarations. [COBOL](cobol/README.md) |

The development JVM runtime requires stable JDK 22 or later and `--enable-native-access=ALL-UNNAMED`, without preview features or a manual library path. Its local package includes the selected native target; ordinary Maven distribution still needs final assembly. Objective-C uses Apple Foundation and ARC. Apple SDK compile checks exist, but matching installed Foundation execution remains unrun. Windows execution and registry installs remain candidate checks. The [package design](../sdlc/decisions/2026-10-09-native-package-design.md) defines intended targets and runtime floors; each package guide records narrower observed evidence.

## Files, questions and results

Use each language's explicit native file-source selector. The [file contract](files.md) defines physical locations and ordered folder reading. Paths remain source locations rather than model evidence. Saved files, names and references use explicit question selectors; literal text stays literal. Authored question dictionaries replace removed question builders where the owning language admits ordinary values.

Generated results preserve absence, explicit null, false, identifiers and unknown extension members. Read settled call facts for observed requests and provider usage. `meta.usage` attributes token shares to logical questions; coalesced question occurrences can repeat those shares. Summing row metadata does not recover live provider usage. Call facts count each live provider response once and provide authoritative observed usage and any available estimated cost. The estimate requires complete reported usage and explicit prices; it makes no claim about a provider's invoice. See the [result contract](../specification/result.md).

Pending equal questions can coalesce and repeat their attributed shares. A repeated call after completion can send again; record mode does not read stored answers. The recording SQLite store upserts the latest response by request digest and answer key. It is not an append-only attempt ledger, so summing retained response bodies cannot reconstruct historical live usage. See the [cache contract](../specification/cache.md) and [planning decision](../sdlc/planning/adr/0123-conservative-plan-request-bounds.md).

## MCP and SQL

MCP uses the command's ten tools and complete results through local stdio. Its [contract](../specification/mcp.md) defines exclusive evidence, records, source and finite input descriptors. Incoming framing also bounds aggregate original compressed attachments, including duplicates. Explicit native text-file inputs carry captions too large for a frame. The [MCP guide](mcp/README.md) gives the development launch and recorded first call.

SQL uses named scalar judgments and generated complete `*_details` results, named judgment tables where supported, native plans and process usage totals. Use the owning [DuckDB](../databases/duckdb/README.md), [SQLite](../databases/sqlite/README.md) or [PostgreSQL](../databases/postgresql/README.md) guide for removed spellings, schema rules and installed version limits. Process totals cannot isolate concurrent statements. Apply explicit tariffs to complete observed usage outside SQL and label the result as an estimate.
