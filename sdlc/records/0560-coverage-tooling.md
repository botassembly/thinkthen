# Coverage tooling measurements

The collector validations measured installed packages against instrumented Rust core and C objects. These separate validations establish that the collectors produce usable profiles; they are not the combined baseline or release qualification. Product Rust and its source ceiling did not change.

## Installed measurements

The C validation passed its owner and documented examples and all 42 routine shared cases. Python's shared validation passed 51 tests and type checks; Python, pandas and Python Polars each passed 40 routine cases. Their five collections each produced a valid pooled profile. Final C consumer executables and the installed Python extension were retained for export.

Java, Kotlin and Scala each passed separate session and usage validations. All six collections produced valid profiles containing Rust-core and C-library mappings. The native bytes packaged in the JAR matched the measured shared library.

| Collector | Valid profiles |
| --- | ---: |
| C++ | 94 |
| Go | 33 |
| Zig | 61 |
| Ada | 1 pooled |
| PHP | 1 pooled |
| COBOL | 1 pooled |
| JavaScript | 57 |
| TypeScript | 57 |
| Ruby | 65 |
| R | 141 |
| Rust | 63 |
| Rust Polars | 397 |
| C# | 69 |
| Dart | 6 |
| Flutter | 1 |
| Swift | 48 |

These validations preserved routine assertions and privacy guards. Final successful exports rejected no profiles. Swift's normal installed check passed 42 cases and ownership checks after the existing generator restored nine bridge lines for the current named-session export.

Polars supplied twelve additional mapped core files, including 432 observed lines in frame request and typed code. Its retained executable therefore contributes to the common production denominator. Counting only the default command and C shared library would omit those mappings.

The separate C source-unit validation passed 25 tests and produced two valid profiles. Its export reached 2,357 C production lines. The production-only denominator excluded 1,981 additional test-only mapped lines. C source-unit hits are classified with unit tests, separately from installed callers.

## Fixture limits

Fresh read-only reviews measured and accepted these increases for final-object retention, owned backend routing and writable profile mounts. They change test fixture limits, not the Rust source ceiling.

| Fixture limit | Before | Measured limit | Change |
| --- | ---: | ---: | ---: |
| Go Python | 271 | 278 | +7 |
| Zig Python | 167 | 178 | +11 |
| Ada Python | 551 | 554 | +3 |
| PHP Python | 113 | 114 | +1 |
| C# Python | 781 | 797 | +16 |
| Swift Python | 284 | 289 | +5 |
| R source and fixtures | 1901 | 1915 | +14 |

The seven increases total 57 lines. Go and Zig retain the exact final static consumers. Ada retains its session executable; the shared usage builder retains the other executable. PHP forwards the owned backend target. C# and Swift expose writable profile directories inside isolated consumers. R adds fifteen nonblank lines for the shared waiting-shell helper, removes two duplicated launcher lines, and adds one line for readable clean environment isolation. Production R is unchanged. A fresh reviewer measured and accepted the 14-line increase without deleting assertions or changing formatting to reduce the count.

## Combined checkpoint

At `80a031bc4`, the routine test returned 1. The workspace passed 2,002 tests with 73 skipped; the consumer suite passed 21 with four skipped; the doctest passed. Python passed 43 of 44 tests. The failing bare Rust campaign fixture omitted the repository toolchain file and the compiler's new `extra_env` argument.

Lint returned 1 at the child-environment check because the R waiting-shell launcher inherited the whole environment. Later lint stages did not run. Six focused coverage regressions passed. All checkpoint children exited and were reaped.

## What the build taught us

An installed static archive needs LLVM's runtime initializer as well as coverage sections. The initial C calls passed while producing no profile; retaining the runtime anchor before localization restored profile writes without changing ordinary archive exports.

Normal Go exits and signal termination can bypass exit-time profile writes. A pinned LLVM probe demonstrated valid continuous profiles from concurrent normal, direct and signal exits. Finite file-size limits remained unchanged and produced no regular profile or extra stderr. Reports retain those missing observations rather than treating zero hits as proof that code is unreachable.

Native dependency assembly can retain private build paths after Rust remapping. Matching native compiler remaps made the unchanged C++ and Zig privacy guards pass. A copied package source contributes to canonical coverage only when its bytes match the repository source.

The R startup assertions exposed an exited orphan child that remained a zombie under the collector's subreaper. A waiting shell restored ownership and reaping while preserving both assertions. The routine checkpoint then caught the separate environment-isolation defect in that launcher.

The default routine checks and the actual installed checks exercise different plumbing. The combined checkpoint caught stale fixture contracts that separate installed validations did not cover. These results support the wave checkpoint rule and do not replace the common-revision baseline, repeatability or candidate proof.
