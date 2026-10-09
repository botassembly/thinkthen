# Writing a thinkthen binding

This guide defines the 0.2 target for authors of language bindings. [The 2026-10-09 ruling](../sdlc/decisions/2026-10-09-thin-first-class-bindings.md) sets the rule: Rust owns every rule; each language owns only its idiom. It supersedes the plain-host-value requirement and typed-host-result ban in [ADR 0112](../sdlc/planning/adr/0112-rust-owns-the-result-schema.md). The ADR's Rust schema ownership and compatible unknown-result-member handling remain. This guide describes the approved target; it does not certify that each package already meets it.

## Shared ownership and contracts

The [versioned Request contract, ADR 0125](../sdlc/planning/adr/0125-one-request-contract-and-native-admission.md), defines the public input edge. The public Request decodes host input, admits it once, and converts admitted values to pure core values. Core owns grammar, limits, defaults and semantic validation; it imports no public Request, filesystem reader or runtime handle and touches no file, environment, socket, clock or process. [0511](../sdlc/tickets/0511-core-owns-request-grammar.md) consolidates the remaining admission rules within that boundary. A binding translates language values and forwards core errors; it never restates the grammar or rejects a semantic input through a private validator.

Native callers construct typed Request values without a JSON round trip. Direct Rust bindings expose Rust-owned results with ordinary language object behavior. C-interface languages use the owned JSON session specified by [0503](../sdlc/tickets/0503-narrow-json-session-interface.md), then expose generated typed results. That session owns input and result buffers, bounds both queues, preserves whole-set admission and completed prefixes where specified, and permits cancel or free without waiting for a blocked provider. Its reviewed handle ADR defines signatures and lifetimes; do not invent session symbols here or stage an entire feed as an array.

[0513](../sdlc/tickets/0513-generated-typed-results-with-presence.md) generates host results from Rust, including explicit presence wherever missing differs from present null. Ordinary nullable properties cannot encode both states alone. Results retain unresolved answers, failed answers and all observed facts, including attempts, cache observations, identifiers, provenance and final failure facts under [the result contract](../specification/result.md). They never fabricate observations or turn a failure into null. Generated readers accept unknown output members. Request closure never closes caller-authored JSON or result objects. The shared result fixture is the oracle for absence, null, false, failure and unknown fields.

C, Zig, Ada and COBOL use fixed layouts generated from Rust. Other C-interface languages use the session and generated host result types, rather than hand-copied native layouts and readers. Generate declarations and presence discriminators from the source of truth; keep no private schema or field list. The generated C header remains the ABI contract.

[0515](../sdlc/tickets/0515-one-public-api-per-language.md) removes dead and unshipped binding code. Each language's migration ticket removes that language's 0.1 JSON-string calls and duplicate APIs after its replacement passes. Frozen 0.1 C symbols remain for C compatibility and are documented as deprecated. Their accepted legacy grammar and behavior remain protected by ADR 0125; this exception does not create a second host-language API.

[0517](../sdlc/tickets/0517-prebuilt-native-libraries-in-every-package.md) designs how each package carries its prebuilt native library for supported platforms. Each migration ticket builds its own package under that design. Installation and the first call require no manual library path. Follow the language package's normal install command; package inventories come from the shared generated definition. Public installation text continues to describe the published release until 0.2 ships.

## Caller acceptance and code ownership

The [native package design](../sdlc/decisions/2026-10-09-native-package-design.md) fixes platform pairs, runtime floors, native layout and automatic loading. The PM approved its pub build-time asset route in [the pub ruling](../sdlc/decisions/2026-10-09-pub-build-time-native-asset.md).

An ordinary caller installs a package, supplies native host values to a named function, reads a native answer or complete result, handles a typed failure, cancels work where the host supports it, and releases resources through the host's normal idiom. The caller does not build transport JSON, decode result JSON or find a native library by hand. Structured JSON supplied as evidence remains valid data. Internal JSON transport does not determine the public interface.

One public API means one coherent function family. Bare answers, complete details, synchronous calls and host-appropriate async calls may be views of that family. It does not mean one untyped generic call or removing useful typed methods to meet an export count. Each migration declares its recommended calls and the old names it replaces; The migration removes the old names only after replacement parity. Frozen C compatibility remains separate.

Minimize maintained logic across the engine, generator, templates and host glue together. Moving handwritten rules into per-language templates does not remove duplication. Generate declarations and mechanical conversions from Rust; host code owns naming, value conversion, representation checks, scheduling, authority and cleanup. Share transport and ownership within JVM and Dart/Flutter families. A small amount of explicit host code is appropriate when its removal would make ordinary calls, errors or cleanup harder. Record actual handwritten code removed and added, including generator/template cost, in the existing landing record; set no arbitrary line target and add no recurring measurement process.

0513 completes the shared semantic graph, generator mechanics and C# reference. Host migration tickets own their target templates, outputs and installed adoption. 0505 supplies additive complete C session views for C and Zig. Ada (0528) and COBOL (0529) generate their declarations from those views. A generated header over an incomplete old carrier is not complete typed access. Known observations, presence, partial usage and failure facts must survive native serialization before any host round trip can prove them.

For an async host, a Task, Future or Promise type alone proves little. The installed held-provider case must show unrelated host work progressing and caller cancellation/cleanup returning before the provider is released. Native settlement may follow later; pending final facts stay pending. 0516 establishes the waiting and cancellation pattern; each migration proves its language's idiom without duplicating the native session test matrix. Synchronous hosts retain their normal execution model.

Packaging design precedes the affected host migration. 0516 proves the first local NuGet install. Each migration ticket builds its own package under 0517's design. 0501 owns the common inventory, and 0530 assembles the final distributions. Supported targets and runtime floors come from that design, not from a language name alone. Install the final consumer artifact; a development archive, source import or generated declaration does not prove the shipped package.

## Ten first-class items by language

Each table gives the expected form, not a new signature design. Named calls cover all ten functions through one API. Inputs cover the admitted typed questions, text, records, sources, images and bounded feeds in Request. Complete typed results carry every function's value and observations. The absence row always includes the distinction between a failed answer and an unresolved answer. Error kinds, retryability and facts come from Rust. Async and cancellation preserve bounded ownership; synchronous languages need not acquire an async runtime. Editor support includes generated declarations and documentation. Thinness permits naming, value conversion, scheduling and cleanup idiom only.

C# uses the [0516 pilot](../sdlc/tickets/0516-csharp-thin-first-class-pilot.md): session calls, generated presence-aware typed results, Task with CancellationToken, SafeHandle, nullable reference types and typed exceptions through one entry point. Dart follows the same session pattern with Future, generated presence-aware Dart classes, explicit cancellation and deterministic cleanup; Flutter packages it as a real FFI plugin. Neither language keeps the old native layout readers or substitutes nullable JSON properties for explicit presence.

### Rust

[Package and existing checks](rust/README.md).

| Item | Expected form |
| --- | --- |
| Calls | Named typed methods through one crate API. |
| Inputs | Typed Request and native iterators. |
| Typed results | Rust-owned structs and enums. |
| Absence | Option plus explicit presence where required. |
| Errors | Result with typed Error. |
| Async and cancellation | Native execution and owned cancellation; no required async runtime. |
| Cleanup | RAII and scoped workers. |
| Editor support | Rustdoc and compiler-visible public types. |
| Install | Cargo package. |
| Thinness | Direct Rust; no host reader. |

### Python

[Package and existing checks](python/README.md).

| Item | Expected form |
| --- | --- |
| Calls | Named methods on one Python API. |
| Inputs | Python values and iterables at the Rust edge. |
| Typed results | Rust-owned Python objects and generated stubs. |
| Absence | None plus generated presence representation. |
| Errors | Typed Python exceptions. |
| Async and cancellation | Python async idiom and explicit cancellation. |
| Cleanup | Context manager and native ownership. |
| Editor support | Generated type stubs and docstrings. |
| Install | Wheel with its native extension. |
| Thinness | Direct Rust; no hand-copied result reader. |

### pandas

[Package and existing checks](python/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One Series accessor using the Python API. |
| Inputs | Series and ordinary Python values. |
| Typed results | Typed Python results with indexed Series output. |
| Absence | Preserve presence separately from pandas missing values. |
| Errors | Python exceptions. |
| Async and cancellation | Delegate execution and cancellation to Python. |
| Cleanup | Delegate native ownership to Python. |
| Editor support | Accessor documentation and Python stubs. |
| Install | Python package and native extension. |
| Thinness | Only dataframe conversion over Python. |

### Python Polars

[Package and existing checks](python/README.md). Ticket 0496 owns this surface.

| Item | Expected form |
| --- | --- |
| Calls | One expression or namespace API. |
| Inputs | Polars columns and typed values. |
| Typed results | Typed results with Polars column output. |
| Absence | Preserve presence separately from Polars nulls. |
| Errors | Python exceptions for Python callers. |
| Async and cancellation | Delegate execution and cancellation to native binding. |
| Cleanup | Owned native resources. |
| Editor support | Namespace declarations and documented column types. |
| Install | Package with native extension. |
| Thinness | Only dataframe conversion and native calls. |

### Rust Polars

[Package and existing checks](polars/README.md). Ticket 0527 owns the implementation under `crates/thinkthen/src/public/frame.rs` and `frame/`, plus its installed consumers.

| Item | Expected form |
| --- | --- |
| Calls | One named Rust column API. |
| Inputs | Native Polars columns and typed Rust values. |
| Typed results | Native columns and Rust-owned complete results. |
| Absence | Preserve column nulls, row identity and complete-result presence. |
| Errors | Typed Rust Result and Error. |
| Async and cancellation | Native cancellation; no required async runtime. |
| Cleanup | Rust ownership and scoped resources. |
| Editor support | Rustdoc and compiler-visible types. |
| Install | Cargo package with the declared Polars feature. |
| Thinness | Direct Rust conversion; no JSON result reader. |

### TypeScript

[Package and existing checks](typescript/README.md).

| Item | Expected form |
| --- | --- |
| Calls | Named methods on one exported API. |
| Inputs | Typed objects, byte arrays and bounded async inputs. |
| Typed results | Generated TypeScript result declarations. |
| Absence | undefined or explicit presence distinct from null. |
| Errors | Typed Error subclasses. |
| Async and cancellation | Promise and explicit cancellation. |
| Cleanup | Explicit close and owned native resources. |
| Editor support | Generated declarations and editor completion. |
| Install | npm package with native addon. |
| Thinness | Direct Rust addon; no hand-copied JSON readers. |

### Ruby

[Package and existing checks](ruby/README.md).

| Item | Expected form |
| --- | --- |
| Calls | Named methods on one Ruby API. |
| Inputs | Ruby objects and enumerable inputs. |
| Typed results | Rust-owned Ruby objects with generated accessors. |
| Absence | nil plus explicit presence. |
| Errors | Typed exception classes. |
| Async and cancellation | Ruby scheduling idiom and explicit cancellation. |
| Cleanup | Block-scoped cleanup and native ownership. |
| Editor support | Generated accessors and API documentation. |
| Install | Gem with native extension. |
| Thinness | Direct Rust; no private result grammar. |

### R

[Package and existing checks](r/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named function family. |
| Inputs | R vectors, lists and data frames. |
| Typed results | Rust-owned typed R result objects. |
| Absence | Preserve absent versus explicit null separately from NA. |
| Errors | Typed R conditions. |
| Async and cancellation | R execution idiom and explicit cancellation. |
| Cleanup | Explicit close with native finalization safety. |
| Editor support | Documented result classes and function help. |
| Install | R package with native library. |
| Thinness | Direct Rust; only R value conversion. |

### C

[Package and existing checks](c/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One current typed API; deprecated frozen ABI retained. |
| Inputs | Generated descriptors and owned feeds. |
| Typed results | Generated fixed result layouts. |
| Absence | Generated presence tags distinct from null. |
| Errors | Generated error kinds and retained failure facts. |
| Async and cancellation | Owned cancellation and bounded sessions. |
| Cleanup | Explicit generated handle and buffer release. |
| Editor support | Generated header documentation. |
| Install | C archive with header and prebuilt library. |
| Thinness | Generated layouts; no copied validation. |

### C++

[Package and existing checks](cpp/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named C++ API. |
| Inputs | C++ values, containers and bounded feeds. |
| Typed results | Generated C++ result types. |
| Absence | Optional values plus explicit presence. |
| Errors | Typed exceptions or declared typed error result. |
| Async and cancellation | C++ async idiom and owned cancellation. |
| Cleanup | RAII session and result ownership. |
| Editor support | Generated headers and documented types. |
| Install | CMake package with prebuilt library. |
| Thinness | JSON session; only C++ conversion and ownership. |

### C#

[Package and existing checks](csharp/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named entry point. |
| Inputs | Typed C# objects and bounded enumerable inputs. |
| Typed results | Generated C# classes. |
| Absence | Nullable reference types plus explicit presence. |
| Errors | Typed exceptions with kind, retryability and facts. |
| Async and cancellation | Task with CancellationToken. |
| Cleanup | SafeHandle and deterministic disposal. |
| Editor support | Generated types and nullable annotations. |
| Install | NuGet runtime assets with native library. |
| Thinness | JSON session; remove native layout and reader copies. |

### Go

[Package and existing checks](go/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named Go API. |
| Inputs | Go structs, slices and bounded inputs. |
| Typed results | Generated Go result types. |
| Absence | Pointers or optional values plus explicit presence. |
| Errors | Typed error values. |
| Async and cancellation | Go cancellation idiom and goroutine-compatible execution. |
| Cleanup | Explicit Close and owned handles. |
| Editor support | Generated Go declarations and documentation. |
| Install | Go package with prebuilt native library. |
| Thinness | JSON session; only Go conversion and scheduling. |

### Java

[Package and existing checks](jvm/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named Java API. |
| Inputs | Java objects, collections and bounded inputs. |
| Typed results | Generated Java result classes. |
| Absence | Optional values plus explicit presence. |
| Errors | Typed exceptions. |
| Async and cancellation | Java async idiom and explicit cancellation. |
| Cleanup | AutoCloseable and owned native handles. |
| Editor support | Generated classes and Javadoc. |
| Install | Maven native jars; stable APIs, no preview flags. |
| Thinness | JSON session; shared JVM ownership. |

### Kotlin

[Package and existing checks](jvm/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named Kotlin API. |
| Inputs | Kotlin values, collections and bounded inputs. |
| Typed results | Generated Kotlin result classes. |
| Absence | Nullable types plus explicit presence. |
| Errors | Typed exceptions. |
| Async and cancellation | Coroutine idiom and explicit cancellation. |
| Cleanup | use and shared owned JVM resources. |
| Editor support | Generated classes and Kotlin documentation. |
| Install | Maven artifacts with native library. |
| Thinness | JSON session through shared JVM mechanics. |

### Scala

[Package and existing checks](jvm/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named Scala API. |
| Inputs | Scala values, collections and bounded inputs. |
| Typed results | Generated Scala result types. |
| Absence | Option plus explicit presence. |
| Errors | Typed exceptions or declared typed error result. |
| Async and cancellation | Future idiom and explicit cancellation. |
| Cleanup | Scoped close over shared JVM ownership. |
| Editor support | Generated types and Scala documentation. |
| Install | Maven artifacts with native library. |
| Thinness | JSON session through shared JVM mechanics. |

### Swift

[Package and existing checks](swift/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named Swift API. |
| Inputs | Swift values, collections and bounded inputs. |
| Typed results | Generated Swift result types. |
| Absence | Optional plus explicit presence. |
| Errors | Typed thrown errors. |
| Async and cancellation | async/await with cancellation. |
| Cleanup | Deterministic owned resource cleanup. |
| Editor support | Generated types and Swift documentation. |
| Install | SwiftPM binary target with native library. |
| Thinness | JSON session; only Swift conversion and ownership. |

### Zig

[Package and existing checks](zig/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named Zig API. |
| Inputs | Generated descriptors and bounded inputs. |
| Typed results | Generated fixed layouts. |
| Absence | Optional values with generated presence tags. |
| Errors | Typed error sets with retained facts. |
| Async and cancellation | Owned cancellation; explicit synchronous execution. |
| Cleanup | defer and explicit owned release. |
| Editor support | Generated declarations and documentation. |
| Install | Package with prebuilt library. |
| Thinness | Generated layouts; no private grammar. |

### PHP

[Package and existing checks](php/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named PHP API. |
| Inputs | PHP values, arrays and bounded inputs. |
| Typed results | Generated PHP result classes. |
| Absence | null plus explicit presence. |
| Errors | Typed exceptions. |
| Async and cancellation | PHP execution idiom and explicit cancellation. |
| Cleanup | Deterministic close with owned native handles. |
| Editor support | Generated declarations and PHP documentation. |
| Install | Composer package with prebuilt library. |
| Thinness | JSON session; no native view copies. |

### Dart and Flutter

[Package and existing checks](dart/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named Dart API shared by Flutter. |
| Inputs | Dart values, collections, bytes and bounded streams. |
| Typed results | Generated Dart result classes. |
| Absence | Nullable types plus explicit presence. |
| Errors | Typed exceptions. |
| Async and cancellation | Future and explicit cancellation. |
| Cleanup | Deterministic close and owned native handles. |
| Editor support | Generated declarations and Dart documentation. |
| Install | pub package with native assets; real Flutter FFI plugin. |
| Thinness | JSON session; remove native layout and reader copies. |

### Ada

[Package and existing checks](ada/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named package API. |
| Inputs | Generated descriptors and bounded inputs. |
| Typed results | Generated fixed records and discriminated types. |
| Absence | Generated presence discriminants. |
| Errors | Typed exceptions or declared status with error facts. |
| Async and cancellation | Owned cancellation; Ada task idiom when needed. |
| Cleanup | Controlled ownership or explicit release. |
| Editor support | Generated package specifications and documentation. |
| Install | Package with prebuilt native library. |
| Thinness | Generated layouts; no label grammar copy. |

### Objective-C

[Package and existing checks](objective-c/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named Foundation API. |
| Inputs | Foundation collections and byte data. |
| Typed results | Generated Foundation result objects. |
| Absence | Nullability annotations plus explicit presence. |
| Errors | NSError with retained typed details. |
| Async and cancellation | Blocks for async and cancellation. |
| Cleanup | ARC and owned native resources. |
| Editor support | Generated headers and nullability annotations. |
| Install | Apple package with prebuilt native library. |
| Thinness | JSON session on Foundation; follow 0518. |

### COBOL

[Package and existing checks](cobol/README.md).

| Item | Expected form |
| --- | --- |
| Calls | One named callable interface. |
| Inputs | Generated input records and bounded feeds. |
| Typed results | Generated fixed records. |
| Absence | Generated presence and discriminator fields. |
| Errors | Typed status records with retained facts. |
| Async and cancellation | Owned cancellation; explicit synchronous execution. |
| Cleanup | Explicit owned session and buffer release. |
| Editor support | Generated copybooks and interface documentation. |
| Install | Package with prebuilt native library. |
| Thinness | Generated layouts; no private input grammar. |

## Enforcement and proof

Use the existing checks below. Their present scope does not establish future generation, session or packaging behavior. The owning tickets extend these checks as their contracts land; this guide adds no new checker or receipt system.

| What to enforce | Existing check and source | Required evidence |
| --- | --- | --- |
| Pure core, dependency direction and source boundaries | [policy.py](../sdlc/scripts/policy.py), run by [lint](../sdlc/scripts/lint); [Rust standards](../sdlc/planning/rust-standards.md) | No inward edge dependency or copied rule; 0511 adds checks for repeated limits and cross-crate path includes. Review catches semantic validation copies. |
| Rust schema ownership and presence | [schema tests](../crates/thinkthen/src/schema_tests.rs), [request schema](../specification/request.schema.json), [result corpus](../specification/fixtures/types/) | Generated bytes agree with Rust; missing, null, false, failure, unknown fields and every function survive host conversion. 0513 adds stale-output enforcement for generated host types. |
| Shared behavior and complete observations | [conformance cases](../conformance/cases.json), [conformance runner](../conformance/parity.py), each package's existing check.sh | Run the shared cases through the real host API. Retain all result observations and failure facts. Plans and invalid-input refusals prove zero sends by loopback request counts. |
| C ABI and constrained layouts | [header generator](../sdlc/scripts/generate-c-header.py), [export check](../sdlc/scripts/check-c-exports.py), [C door tests](c/tests/door/main.rs) | Generated declarations match the Rust ABI; frozen symbols and behavior remain. Measure layout through the target compiler rather than trusting a copied declaration. |
| Async, cancellation and cleanup | Each package's existing check.sh and [shared native consumers](../conformance/consumer/) | Exercise cancellation, blocked-provider cleanup, stopped readers, backpressure and owned buffer lifetimes at the public boundary. 0503 owns session lifetime cases; host migration adds its idiom cases. |
| Editor support and one API | Each package's compiler/import checks and existing export inventories | A real consumer compiles or imports generated types and uses named calls; deprecated frozen C exports do not authorize duplicate host entry points. Each migration removes its old host calls. Each migration's installed check compares the package's public symbols with its declared recommended API. |
| Installed package and native assets | Existing package checks, including [C#](csharp/tests/package_check.py) and [JVM](jvm/tests/package_check.py), and the [release package checks](../sdlc/scripts/README.md) | Install a local artifact in a clean environment and run a real call without a library path override. Migration packaging slices and 0501 share the generated product inventory for builders and installed checks. |

Test children use a cleared, explicit environment with owned cache/state directories, the loopback address and a fake key only when needed. A plan test uses no key. Source-tree checks and installed-artifact checks report separate evidence. Replay changed documentation examples. Run the smallest relevant format, policy and functional checks. The coordinator runs the routine suite once at ticket closure; large-input and full parity runs wait for the release suite. Hosted qualification and publication retain the release hold.

The reviewer applies all ten rows to each language and checks the generated source and actual installed consumer. C# and Dart must reach the session, presence, async and cleanup shapes stated above. The reviewer treats missing observations, lost presence, duplicated rules, borrowed buffers outliving their owner and missing packaged native libraries as caller failures. No private copy of the corpus or hand-kept result field inventory stands in for this evidence.

SQL and MCP retain their own surface contracts. [0519](../sdlc/tickets/0519-sql-surfaces-consistent-and-described.md) owns SQL NULL, binary image, native JSON and description behavior; this guide does not settle an additional NULL rule. [The MCP contract](../specification/mcp.md) owns its ten tools. Both surfaces use shared Rust admission and complete results.

For SQL, ordinary database values and documented complete-result calls must preserve NULL, errors, native binary images and the database's supported JSON representation. Verify return types and in-database descriptions through the installed extension. For the command, preserve existing pipes, output, interruption and exit codes while moving judgment execution to Request. For MCP, retain tool discovery, generated input schemas, protocol errors, cancellation and file authority. These are 0519 and 0512 acceptance boundaries; they do not add new commands or protocol features.
