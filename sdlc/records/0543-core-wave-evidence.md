# Core wave evidence

Measured on 2026-10-12 and 2026-10-13. The core wave uses the accepted installed coverage pair at `da1e3bb17` as its baseline. No additional full coverage run was made.

The accepted main Rust ceiling was 187,606 nonblank lines under `crates/` and `conformance/`. The combined core measured 186,959 at `39a9d05ab`. Removing repeated cache-test setup reduced it to 186,956. The atomic cache-admission repair and its retained regression assertions measured 186,955, a decrease of 651 from main.

The C examples and fixture count is 661. The standalone deletion count of 650 preceded the reviewed Windows fix, which added four guarded include lines and seven lines setting stdout and stderr to binary mode. The complete count lowers main's ceiling from 3,395 to 661. A fresh reviewer explicitly accepted that reconciliation. Every changed source ceiling remains below main's corresponding ceiling.

## Library builds

At `39a9d05ab`, build-only checks passed for native C and its exports, Rust, the Polars feature and external consumer, Python, Ruby, Node and TypeScript, R, C++, C#, Go, Java, Kotlin, Scala, Linux Swift, Zig, PHP, Dart, Linux Flutter, Ada, COBOL, and the CLI and MCP binary. Native imports and the Python dataframe imports passed. Apple Foundation and Windows were outside those Linux build checks.

The build owner corrected three invocation mistakes: Ruby needed the extension basename matching its initializer, C# needed its local offline restore, and Dart needed the staged consumer's imported sibling files. The corrected build commands passed without product changes. Those results describe `39a9d05ab`; the later atomic cache-admission change modifies production Rust.

## Findings from the combined checks

The first routine run timed out in the old-cache maintenance matrix. It created the same valid cache separately for each of eight formats and four commands. Sharing one fixture per format retained all 32 refusal checks, exact errors, bytes, modification times, held WAL ownership and counted zero sends. The owning case decreased from 3.624 seconds in isolation to 1.458 seconds under the unchanged timeout.

The version checker still required a removed comment in the C module root. It now checks the surviving version owner and constants, with all original 29 cases retained and four C mismatch and refused-write cases added. Its owning self-test passed 33 cases.

An active find replay test still read a historical probe's old-format recording. Its two saved exchanges were extracted into a current-format fixture. Request and response bodies, models, usage, timestamps, provenance, selected-line assertions and the no-answer assertion were preserved. The historical experiment was unchanged. The focused find and cache collection passed 25 cases.

The C process-cleanup fixture still expected stdin from the retired consumer. Its assertion now matches the current session consumer's absent input and text options. Deadlines, secrecy, descendant cleanup and unrelated-process survival checks were preserved. The workflow self-test passed 113 cases.

The zero-budget test treated a legacy digest file as a cache miss. It now separately checks the required Local refusal, exact deletion diagnostic, absent budget denial, zero requests and unchanged old bytes and entry count. Other-address misses, the ignored marker and first-use budget precedence retain their own assertions. Seven owning checks passed.

Parity metadata retained one schema-only row after its recognition fixture and schema were retired. Removing that row retained all 24 required recognition call cases and all 100 current schema fixtures. Validation reported 29 consumers and 258 required cases. All 36 parity tests passed, including rejection of a missing required binding case.

The checkpoint supervisor initially reaped adopted children only after the gate exited. A timeout assertion therefore observed a killed grandchild as a zombie. Continuous child reaping preserved the primary exit status and made the unchanged timeout and cleanup assertions pass. The corrected supervisor reaped 40 test descendants and 10 lint descendants while the gates ran, then drained all remaining children.

The complete API inventory retained five declarations for intentionally removed public compatibility methods. The specification now describes the current typed accessors, result/2 serialization, scalar result/1 projections and private Details producer. Inventory checked 1,932 declarations and rejected all four negative plants.

The fork consumer then exposed a real cache-initialization race. A parent call failed with an old-store refusal after sending once. Separate version and table-count queries could straddle another writer's valid schema commit. One SQL statement now reads both values in the same snapshot before writable pragmas; initialization retains its write-lock recheck. The owning eight fork cases passed. Five additional regressions covered old-store refusal, active WAL and concurrent cache writers. The old-store table includes unversioned SQLite and retains exact byte, modification-time and zero-send assertions.

At `aa32bc500`, all 2,002 workspace tests, one doctest and 21 external consumer tests passed before the stale parity row stopped the routine script. At `4187991f5`, all workspace tests and the doctest passed; the fork consumer exposed the cache race. Its lint run passed the registry, timeout, formatting, Clippy and documentation stages before the stale public API declarations stopped the final inventory check. Each failed gate's exit and evidence were retained. Source review verdicts apply to their narrow changes; these measurements do not record whole-ticket acceptance.
