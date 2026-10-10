# Upgrade calls to development 0.2

Public installation remains 0.1.2. Use these mappings with a matching reviewed development package. The [binding guide](BINDING-AUTHOR.md) defines the recommended typed function family for each language. Each package README owns its exact calls, input types, generated results and installation route. This guide does not establish final installed parity or platform qualification.

## Removed calls

The [R mapping](r/README.md) covers every replaced R call, including question options, details, files and plans. Replace `tt_*_complete(question, input)` with the corresponding `tt_*(question, input)`. Replace named `tt_*_batch` calls with `tt_batch("*", question, input)`, substituting the function name for `*`. Simplified frames and judge closures are no longer public. Read the owned result and its native facts rather than decoding transport JSON.

The [C# mapping](csharp/README.md) replaces `Engine.Open(settingsJson)` with `Engine.Open(InputEngineSettings)`, named judgments with their generated-input `*Async` methods, and batch readers with the corresponding `IAsyncEnumerable<InputRequestSessionDescriptor>` overload. Replace generic `Call` or `CallTyped` with a named function; explicit session control uses `ExecuteAsync(InputRequest)` or `StartSession(InputRequest)`. Replace the old plan arguments with `Plan(InputRequest)`. Generated `ThinkThen.Results` objects replace compatibility readers and native layouts.

## Additive replacements

These APIs exist alongside compatibility calls. Apply the owning README's migration instructions before removing a dependency on old names:

| Existing call family | Development replacement and owning guide |
| --- | --- |
| TypeScript `engine.complete.decide` | `Client.decide` and the other named methods with generated results. [TypeScript guide](typescript/README.md) |
| C++ `tt::create`, `tt::call`, `tt::many` and `tt::native` | `tt::Client` named methods with generated inputs; `Call` owns cancellation and cleanup. [C++ guide](cpp/README.md) |
| PHP `ThinkThen` and `ThinkThen\Native\Engine` | `Client` named methods returning `Completed` results and terminal facts. [PHP guide](php/README.md) |
| Objective-C `TTClient` and JSON/files calls | `TTFoundationClient` named methods with generated results, `TTTask` cancellation and ARC. [Foundation guide](objective-c/README.md) |
| Go `Engine.Decide`, generic `Call`, and `Files` | `Client.Decide`, the other named methods, and named methods with `FileRecords`; construct with `NewClient(settings)`. [Go guide](go/README.md) |
| Ruby `ThinkThen::Engine` | Ten named calls on `ThinkThen::Client`, with owned native results. [Ruby guide](ruby/README.md) |
| JVM `Door.decideComplete` and other complete calls | Named `thinkthen.Engine` methods returning session packets; Kotlin uses `KotlinEngine`, Scala uses `ScalaEngine`. [JVM guide](jvm/README.md) |
| Dart `Door.decide` and other judgments | Corresponding named `Engine` methods over the native asset/session route. [Dart guide](dart/README.md) |

The [Python mapping](python/README.md) moves complete and details callers to ordinary named `Engine` calls with generated results and `details=True`. Direct iterator calls collect their result; curried `Judge` streams and the documented complete compatibility forms retain their own behavior. The Python guide also maps Series annotation to an indexed Series of typed values.

The ordinary JVM builder now assembles the stable JDK 22 session package with packaged native resources. A local build supplies only its selected native target; ordinary Maven resolution still requires final distribution assembly. Its old preview API remains behind explicit compatibility scripts. The development Objective-C implementation is Apple-only with Foundation, generated objects, ARC and `NSError`. Apple SDK compile evidence exists; matching installed package execution remains required. Legacy GNU sources stay protected until that installed consumer passes. Use the [package design](../sdlc/decisions/2026-10-09-native-package-design.md) for intended targets and runtime floors. Retain explicit restrictions from package documentation.

## Files and MCP

Use each language's native file-source selector. The [file contract](files.md) describes explicit source selection, physical locations and ordered folder reading. Paths remain source locations rather than model evidence. Saved files, names and references use explicit question selectors; literal text stays literal.

MCP uses the command's ten tools and complete results through local stdio. Its [contract](../specification/mcp.md) defines exclusive evidence, records, source and finite input descriptors. Incoming JSON-RPC framing also bounds aggregate original compressed attachments, including ordered duplicates. Explicit native text-file inputs carry captions too large for a frame; native and provider body limits still apply. The [MCP guide](mcp/README.md) gives the development launch and recorded first call.
