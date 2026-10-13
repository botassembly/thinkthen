# 0576: Give every library a first-class experience

Status: OPEN.

Milestone: 0.2

Depends on: 0562, 0568, 0574

## Outcome

Every language library, database extension and the MCP server feels native to a senior developer on that platform and grades B or better on the library rubric below. The public API of each exposes no transport JSON, no schema-path names and no raw C layouts.

## Evidence

- Starts from: a 2026-10-13 read-only grading of every library on `ticket/0543-final` at acf368f06 against seven points: typed requests, one-line typed answers, typed question builders, idiom, helpers, examples and tests, and thinness.
  - Grades: Rust A-, Python B+, C# B+, MCP B+, Swift B, Java B-, Go B-, PostgreSQL B-, Ruby C+, Kotlin C+, C++ C+, Ada C+, DuckDB C+, TypeScript C, Scala C, C C, COBOL C, SQLite C, R C-, PHP D+, Dart and Flutter D+, Zig D+, Objective-C D.
  - 0568 and 0574 already cover typed options and settings everywhere, one typed answer accessor per function, typed recognize and relate builders, safe C++ accessors, the Objective-C blocking call, Zig's session wait and running every example. 0562 covers generated names.
  - Gaps no ticket covers:
    - The three databases' 0.2 complete functions take and return JSON text, while their older functions return typed columns. A caller of the newer API gets a worse experience.
    - The MCP server embeds the whole request schema's 52 definitions in every one of its ten tools. The marketing lead measured `tools/list` on the 2026-10-11 debug build at 1,140,455 bytes, about 280,000 tokens: about 43 KB of input schema and 71 KB of output schema per tool. A fresh agent cannot load it, and the 0.2 release gate expects a fresh agent to run the server.
    - Error facts reach callers as a JSON string in C#, Go, Swift, Kotlin and others, and several libraries have one catch-all error type.
    - Generated `Alternative0`, `alternative3(.null)`, `arm_1`, `Owned` and `Input` names remain in public types in C#, Swift, C, Go and Ada.
    - Several libraries keep their own handwritten JSON parser: Swift (about 220 lines), C++ (about 240), Java (about 190) and Dart.
    - Every type has a public `json()` in Java and Dart. Objective-C's public header exposes the C header. Zig exposes `pub const c`.
    - Kotlin and Scala are thin Java wrappers: no named arguments or DSL, `Any?` facts, CompletableFuture and `null` in public signatures.
    - Dart waits through a new isolate per call. Java starts two raw threads per call. The Objective-C feed path busy-spins.
    - Rank and filter answers come back wrapped (`AtomicNonZeroUsize`, `Presence[json.Number]`) instead of plain lists.
    - Rust keeps two call families and parallel Complete, Owned and Source result types, and filter needs `Request::named("filter", ...)`.
    - Zig does not compile as written: `session.zig` names a type the generator no longer emits.
    - R has 4 help pages for about 25 exports. Ruby has no RBS signatures. Python's stub types every option as `Any`. Polars answers come back as Object columns.
    - SQLite reads `'["person"]'` as one literal kind.
- Keeps: Python's dataframe and pydantic adapters, and C as the shared native layer under its 1,982-line ratchet.
- Changes:
  - Return typed rows, structs or composite types from the database complete functions, and take named typed parameters. Keep one function family per database.
  - Give each MCP tool a short input schema with only the fields an agent writes, drop the output schema from `tools/list`, and let descriptions point to the server's help for the full grammar. The engine still validates every call against the full request schema.
  - Generate one typed error per library with typed facts and native error kinds.
  - Decode results once in Rust and hand libraries native values. Delete each library's handwritten JSON parser.
  - Hide transport JSON, the C header and schema-path names from every public API.
  - Make Kotlin and Scala idiomatic: named arguments or builders, nullable or Option results, coroutines or Scala futures, and native collections.
  - Return plain lists for rank and filter answers in every library.
  - Merge Rust onto one call family with one result family, and add a named `filter` call.
  - Fix the Zig build, the Dart wait isolate, the Java threads and the Objective-C busy-spin.
  - Add the missing R help pages, Ruby RBS signatures, the Python option stubs and typed Polars columns. Fix the SQLite kinds trap.
- Proof:
  - A fresh read-only grading on the candidate gives every library, database and the MCP server B or better on the same seven points.
  - `tools/list` measured on the candidate is under 25 KB in total, and a fresh agent loads the server and answers one call per tool. A larger catalog blocks 0.2.
  - Every language library's routine installed checks pass.
- Defers: nothing.

## Progress
