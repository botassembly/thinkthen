# 0.2 quality review, 2026-10-18

Ian asked for an adversarial review of 0.2 across every language, surface and binding, with grades, the root causes of the thrashing, and long-term stability. Four read-only reviewers read main at 393f4bec1: the Rust core and binding contract, two halves of the language libraries, and the database extensions with the release machinery. The PM checked the main claims against the code.

## Grades

| Area | Grade | Main reason |
|---|---|---|
| Rust core | B- | Strict lints, pure, heavily tested; too large, two hand-locked session types, leftover compatibility code |
| Host crate | C | Calls itself "private compatibility transports" (`crates/thinkthen-host/src/lib.rs:1-7`) |
| C interface | C+ | Generated header and panic guard; per-thread last error, typed and JSON entry points, a COBOL cap in shared code (`libraries/c/src/ffi/native_request/ffi.rs:76`) |
| Generator and binding contract | C | Freshness checks are real; call answer rules live in Python templates and are re-implemented per language (`sdlc/generators/results/templates/host_answers.py:28-30`) |
| Conformance | B+ | One case file, loopback backend, 29 consumers |
| Rust library | A- | Examples and a consumer over the crate |
| C | B+ | Generated header and views; fragile symbol hiding, link command needs `sed` |
| SQLite | B | Rust, tests load the real extension; settings struct duplicated |
| Command-line tool | B+ | Real-binary tests |
| Installers | B | Checksums and receipts; musl-on-glibc-headers workaround |
| MCP | B | Rust server; the Python client is unpackaged |
| Zig | B- | Generated; refuses every target but x86_64-linux-gnu (`libraries/zig/build.zig:44-45`) |
| C# | B- | The pilot shape; native declarations still hand-written |
| Polars | B- | In the crate; consumer harness rewrites Cargo.toml by string |
| PostgreSQL | B- | Pure Rust; duplicated settings validation; 1,639-line shell test |
| Python | C+ | Real wheel; `_calls.py` re-derives answers and differs from TypeScript |
| Go | C+ | Thin; native library absent for non-linux/amd64 module users |
| Ada | C+ | Generated; two call families |
| COBOL | C+ | Generated; documented build disables a pointer-type warning |
| JVM | C+ | Reads native layouts directly; no Maven or Gradle build; Java 22 minimum |
| Swift | C+ | `path:` binary target; hand projection in `CallProjection.swift` |
| PHP | C+ | FFI over generated header; hand projection; Linux x86-64 only |
| C++ | C | Not an installable CMake package; examples call `detail` internals |
| TypeScript | C | JSON strings decoded by 4,000 generated lines; Node pinned exactly to 22.22.3 in the gate |
| Dart and Flutter | C | `native-assets.json` has `"url": null` and Linux x64 only |
| Objective-C | C | `.unsafeFlags` in `Package.swift:11` blocks use as a remote dependency |
| Ruby | C | Right shape (magnus); Ruby built from source, projection in Ruby |
| DuckDB | C | Rank kind shared as a bare number across C++ and Rust; `removed.cpp` stubs; retired workspace |
| R | C- | Compiles Rust at install; sends itself SIGINT; keeps a rename message (`thinkthen/R/thinkthen.R:56`) |
| Test harness | D | About 70,000 lines against 116,000 of product; fixtures copy the scripts they test |
| Release workflows | D | No job sets `timeout-minutes`; 693-line `release.yml` |

## Root causes of the thrashing

1. Rust does not compute each call's answer and probability for the C session, so about 15 to 18 languages each carry their own copy of the rules, and the copies already differ.
2. Bindings were built while the result contract changed: four binding-design decisions in two weeks, 70 header changes since 2026-09-15.
3. There is no single native path: C session, fixed layouts, JSON session, and four direct Rust bridges. Each contract change lands three or four ways.
4. Every language runs its own session loop over a per-thread last error.
5. The harness tests the shape of the environment: hand-written tool lists and PATHs (15 loops in `sdlc/scripts`, machine folders in the `sdlc/pm.json` gate), fixtures that copy entrypoints, checks that exit 77 when a tool is missing. Each new machine or tool brings another round.
6. Source-size ceilings: 44 ratchet files and over 400 ceiling-only commits since 2026-09-15, against the binding guide's own rule to set no line target.

## For 0.2

These shorten qualification without changing the product:

- `sdlc/scripts/smoke:104` runs binding smokes four at a time while the Rust release tests share the machine; four groups near 4 GB each is the leading explanation for the Linux timeouts in C, Go, Zig and SQLite.
- Add `timeout-minutes` to every hosted job.
- Replace the machine-specific PATH in the `sdlc/pm.json` gate with the login environment.

## After 0.2

1. Compute each call's answer, probability, pick and resolution in Rust and put them in the final packet; delete `host_answers.py` rules and the per-language projections.
2. Add one blocking run-call C function with an error out-parameter; shrink every session loop to a wrapper.
3. Merge the two session types onto one tested channel.
4. Replace pattern-based secret hiding with an allow-list of provider error fields.
5. One input path, a runtime version check, and the COBOL cap moved into COBOL.
6. Remove the line ceilings; one declared tool manifest; fixtures run on the real checkout; one pinned Linux container with hosted runner limits.
7. Finish packaging: Dart asset URLs, Swift `url:` and checksum, Objective-C without unsafe flags, Go native library, C++ CMake config, prebuilt R.
