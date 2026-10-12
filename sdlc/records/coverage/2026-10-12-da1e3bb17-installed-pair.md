# Installed coverage pair at da1e3bb17

Two ordinary installed measurements used the clean fixed revision `da1e3bb17e98ae4cde5ee75573351d36f4e3ae4d`. They ran sequentially with the same explicit toolchain and offline cache paths. The first took 3611.8 seconds; the second took 3768.8 seconds. Both completed the 31 locally runnable collections and returned 77 solely because Objective-C Foundation requires matching macOS execution. This is partial Linux evidence, not a complete baseline or release pass.

Both measured 59,962 mapped nonblank lines across 458 production files. The production denominator count, collection scope, missing reason and continuous-profile mode matched. The reached-line totals differed in seventeen files and sixteen collections. The repeatability comparison failed. All owned children exited and were reaped.

| Collection | First reached | Second reached |
| --- | ---: | ---: |
| c-source-unit | 16654 | 16653 |
| cli-source | 15153 | 15155 |
| installed-ada | 20485 | 20485 |
| installed-c | 21927 | 21928 |
| installed-cobol | 20605 | 20605 |
| installed-cpp | 16994 | 16994 |
| installed-csharp | 18081 | 18075 |
| installed-dart | 15826 | 15828 |
| installed-flutter | 15706 | 15706 |
| installed-go | 16723 | 16722 |
| installed-java | 14722 | 14722 |
| installed-java-usage | 6922 | 6922 |
| installed-javascript | 15020 | 15020 |
| installed-kotlin | 7520 | 7519 |
| installed-kotlin-usage | 6922 | 6922 |
| installed-pandas | 16489 | 16489 |
| installed-php | 16095 | 16095 |
| installed-polars | 13777 | 13778 |
| installed-python | 16489 | 16489 |
| installed-python-polars | 16489 | 16488 |
| installed-python-shared | 16112 | 16109 |
| installed-r | 18113 | 18113 |
| installed-ruby | 17167 | 17168 |
| installed-rust | 13229 | 13228 |
| installed-scala | 8091 | 8090 |
| installed-scala-usage | 6922 | 6922 |
| installed-swift | 16701 | 16702 |
| installed-typescript | 15020 | 15020 |
| installed-zig | 22335 | 22335 |
| mapped_nonblank | 59962 | 59962 |
| observed_unit_only | 3417 | 3422 |
| rust-integration | 40242 | 40240 |
| source-unit | 22264 | 22263 |
| unobserved | 9624 | 9623 |

The first and second JSON measurements retain their independent per-file counts and missing-profile observations. These original reports do not retain line identities. The owner reused the measurement output after preserving the first counts, so the first raw profiles are unavailable. Equal cardinalities therefore do not prove equal denominator or reached-line sets.

Fresh read-only inspection found real scheduling-dependent paths in timed record delivery, recording-lock contention, usage observation, worker coordination and cancellation. The reporter counts whether each line ran, rather than how many times it ran. These changes are genuine differences in observed line membership. The evidence supports scheduling variation but does not identify every changed line or prove profile loss.

The report repair retains sorted production denominator and per-collection hit-line sets in both JSON outputs and derives the existing totals from those sets. Its regression gives two collections equal counts with swapped hit lines and verifies that the saved line evidence distinguishes them. It also retains zero-hit production mappings and excludes blank and test-only mappings. The regression failed before the repair; all fourteen focused coverage checks and offline policy passed afterward. No product behavior, coverage classification, missing-collection exit status or source ceiling changed.

The paired measurements are retained as [first](2026-10-12-da1e3bb17-installed-first.json) and [second](2026-10-12-da1e3bb17-installed-second.json). No later measurement is represented by these files.
