# 0576: Give every library a first-class experience

Status: OPEN.

Milestone: 0.2

Depends on: 0562, 0568, 0574

Reviews: revision 243002f, reject

Reviews: revision 340f966, accept

Reviews: revision bd54638, accept

Reviews: revision f6f4f90, accept

Reviews: revision d84408e, accept
Reviews: revision 721f505, reject

Reviews: revision 87d2896, accept

Reviews: revision c8805a2, accept

Reviews: revision fac3992, accept

Reviews: revision a82cc47, reject

Reviews: revision 6bc15d6, accept
Reviews: revision e1fd5b9, accept

Reviews: revision 3b3d67b, reject

Reviews: revision 1d312d5, accept
Reviews: revision 43923ee, reject

Reviews: revision fab1322, accept
Reviews: revision 394de24, reject

Reviews: revision ed62acd, accept

Reviews: revision 0301bb6, accept

Reviews: revision b0042d9, accept
Reviews: revision cd5ccdb, accept

Reviews: revision 5f9680d, accept
Reviews: revision c63b1fe, accept
Reviews: revision 6f8962f, reject

Reviews: revision 1edec6e, accept

Reviews: revision 17d3301, reject

Reviews: revision 9d0b261, accept
Reviews: revision 0296829, accept
Reviews: revision d6f05b1, accept
Reviews: revision 60eaaef, reject

Reviews: revision 35f104f, accept

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

PM amendment, 2026-10-13. Remaining work and lane plan. This plan supersedes the earlier lists in this ticket where they differ.

Rulings. (1) Native views are required wherever a handwritten JSON parser exists: Swift, C++, Java and Zig delete theirs and read native result views. C#, Go, TypeScript and R may keep the platform's standard decoder over generated types. The coordinator amends the 2026-10-09 decision and ADR 0129 to say so. (2) The coordinator marks the frozen C compatibility sections of ADRs 0125 and 0129 superseded by the no-compatibility ruling. (3) Each database keeps its typed function family, adds named typed parameters for any option only `_complete` carried, and deletes `_complete`. (4) Polars returns typed `List` and `Struct` columns for all ten functions, generated. (5) The SQLite kinds trap is already refused in code (a849e8799); fix its README and close the item. (6) The MCP catalog work is merged (486f0f989, 243002feb, 8b7632282). A fresh reviewer re-reviews 0f6ddccb4 with written reasons, since 679deb3a7 recorded none. (7) The Swift and Objective-C lanes prove themselves on a dispatched, nonpublishing GitHub macOS workflow; the candidate runs it again.

Stage 1 runs these lanes at once. G1, generator (0562's developer; owns `generate.py`, `registry.py`, `host_answers.py`, `layouts.py`, `c.py`, `native_request.py`, `operations.py`): first restore Zig generation from e0afccd3a's template, because aa4d35726 calls `zig.calls` and `zig.render_results` that do not exist and lint's `generate.py --all --check` stops at Zig; then give every schema type and arm one stable name with no `AlternativeN`, `arm_N`, `ReadableQuestionN`, `Owned` or `Input`; make rank a plain list of indexes and filter a plain list of inputs in the shared answer model; add a shared typed-facts model for errors; add a settings container for every target. Proof: `generate.py --all --check` and `sdlc/scripts/lint`. G2, core and Rust (owns `crates/thinkthen/src/public`, `libraries/rust`, `libraries/c`): keep one call family with `Engine::filter`, delete the 28 parallel `Complete`, `Owned` and `Source` result types and the `compatibility` field from 0565, remove `from_json` from the public API, and delete C's raw JSON calls once nothing calls them. Rust and C hold their ceilings of 392 and 1,905. G3, tooling (owns `conformance/**`, `site/scripts/smoke-bindings.mjs`, `sdlc/scripts/library-size`, `libraries/BINDING-AUTHOR.md`): the 0568 and 0573 tooling items. MCP (owns `crates/thinkthen/src/mcp`, `libraries/mcp`): the re-review, the candidate `tools/list` measurement and one fresh-agent call per tool. SQL, one lane per database (owns `databases/<db>`): ruling (3) and the SQLite README.

Stage 2 starts when G1 and G2 land. One lane per language owns `libraries/<lang>/**`, its own `sdlc/generators/results/templates/<lang>*.py` files and its own ratchet entries. Each lane regenerates rather than hand-editing generated files, sets its ratchets to handwritten-production scope, runs `libraries/<lang>/check.sh` with its example replayed, passes `conformance/library-surface.py --library <lang>`, and stays within its handwritten budget. Each lane also carries its items from 0563, 0564, 0568, 0573 and 0574.
- Python (≤1,043 py, ≤1,175 rs): typed option stub and `Result[T]`; Polars typed columns for the six remaining functions; hide `to_dict`.
- R (R ≤487; rs ≤1,643 and falling): `tt_item`; blocking calls on the shared session, deleting `worker.rs`, `receipt.rs` and `account.rs`; one `.Rd` page per export; no jsonlite reads; `check.sh` runs `examples.R`.
- TypeScript (total ≤494): generated `RequestOptions` and `EngineSettings`; `plan`; ten answer accessors; recognize and relate builders; hide `Results.parse` and `packet`; rewrite examples.
- Ruby (total ≤965): typed keyword options from the template; generated RBS under `sig/`; hide `dump`; example replay in `check.sh`.
- PHP (≤420): drop `array` options; `plan`; hide `toObject` and `jsonSerialize`; replace the 50 ms `wait()` loop with `thinkthen_session_next_packet`.
- JVM (Java ≤687 and falling; Kotlin ≤66; Scala ≤59): ordered-option builder; one executor per engine; delete `Json.java` and read native views; hide `json()`, `fromJson` and `Values`; generated Kotlin facade with Kotlin lists, nullable types and `suspend`; generated Scala facade with `Option`, Scala collections and `Future`; shared cases written once.
- C# (≤415): typed facts; hide `ToJson`, `ToPlain` and `ParseQuestion`; list-of-pairs options.
- Go (≤628): typed per-function inputs instead of `any`; typed `Error.Facts`; hide `MarshalJSON`.
- Dart and Flutter (≤623): `plan`; `check.sh` runs `--check`.
- Swift (≤593 and falling): delete `OwnedJSON.swift` and read native views; typed facts; hide `JSONValue` and `parseQuestion`; start sessions through `thinkthen_session_new_named_with_surface`.
- Objective-C (.h ≤95, .m ≤292): typed public calls; expand the blocking-answer macros; encode `TTOrderedMap`; wait on a condition instead of the busy loop; keep `thinkthen.h` private; one example; fix the duplicate `TTUsagePersistenceState`; start sessions through `thinkthen_session_new_named_with_surface`.
- Zig (≤399): review and merge e0afccd3a and repair its tests; ten accessors; typed settings and facts; rewrite the examples on the blocking call; hide `pub const c`; start sessions through `thinkthen_session_new_named_with_surface`.
- C++ (≤466, falling by about 243): delete `json.hpp` and read native views; typed `Client` settings; private C header; one failure type; ASan over the ten examples.
- Ada (adb ≤200, ads ≤80): `T_EngineSettings`; make `Thinkthen_Session_C` and `Requests` private.
- COBOL (c ≤12, h ≤12): settings record; one `CALL` per answer; rank and filter tables; the session check uses the blocking call; correct the README.
- 2026-10-13 started
- 2026-10-13 next: Merged the reviewed native helper slice as 8b2a47981a416a324514920e8b72197a89b263e5 after installed C ownership, all 42 shared cases, fourteen ASan door checks and unchanged Ada prerequisite passed; continue the language slices and combined checkpoint.
- 2026-10-14 next: Merged the freshly accepted located-feed file selection repair at 959ed3c7 with 40 request and 39 session checks passing; continue the database and language migrations on the combined branch.
- 2026-10-13 next: Merged reviewed Zig native helpers and strengthened find receipt as 1d9fb9ccccb047b8a3d1717fd959fde8831c05ab with installed ownership and all forty-two routine cases passing; continue remaining language quality work.

PM amendment, 2026-10-14. Ordinary `thinkthen_details` returns the existing typed receipt projection, not JSON. This supersedes the 0538 retention of ordinary details JSON. The approved packed try-details recovery envelope stays as it is.
