# Final 0.2 code review

Five fresh read-only reviewers checked the changed code after `38fa0cdfa2b41ebedcafe8973ccd65c7bf1b8a67`. Their scopes covered the Rust core and host adapters, Rust and Polars consumers, dynamic language bindings and generators, native language bindings, databases and MCP callers, and release workflows, shared fixtures and executable site code. They traced surviving callers and replacement wiring after deletions. Reviews covered code; they ran no builds or tests and audited no ticket status or receipts.

The core and release reviews pinned `35432b189a688f5cbac87b4bb761be6bab2f26b4`. The dynamic, native and database reviews pinned `a70844aa5232ee6cab5547ba0e7bc580badc0118`. The intervening code was the PHP change, which the dynamic reviewer covered. All scopes received acceptance after the corrections below. Platform execution remains the candidate's proof.

## Findings and corrections

The native binding reviewer found that the Foundation consumer required a concrete value for every successful decision. The existing unsure decision case returns JSON null, which the public result correctly represents as Null presence. The fixture now checks that presence against raw NSNull and compares concrete values only when present. Fresh review accepted the correction and the one-line fixture ceiling increase from 1758 to 1759. Generated Objective-C checks and the existing parity inventory passed; macOS execution belongs to the candidate.

A preceding routine gate exposed reused process-ID scratch folders in native-store tests. The common scratch helper now gives each routine, full-case, stress and package runner an owned temporary root. Windows TMP and TEMP receive native paths through the existing cygpath tool when available. Targeted native tests passed 142 cases, including the original failed replay case. Ownership checks preserved foreign files and failure status while removing owned scratch output. The package check also imports the shared native fixture from its relocated conformance directory. Fresh release review accepted both corrections.

The first uninterrupted routine attempt on those corrections passed all 1995 workspace cases, doctests and 21 installed Rust consumer checks, then failed seven supporting fixture subcases because their mocked scratch helper omitted temp_home. The existing routing fixture now supplies that mock function, preserving its assertions. All 26 supporting tests passed, and fresh bounded review accepted the change.

## Final proof

On `f02cadd78c0c2385fc3d5eb729a5a95c5639f43c`, `sdlc/scripts/lint` and `sdlc/scripts/test` each passed in one uninterrupted run. Lint checked all 1963 public declarations and refused its four inventory plants. The routine suite passed 1995 workspace cases, doctests, 21 installed Rust consumer checks, 26 parity fixtures, schema and child checks, transforms, the triage pipeline and live checks. Its 73 skipped workspace cases are reserved for the release suite. All processes exited and were awaited.

Original failures, targeted corrections and both closure attempts are retained under `target/0542-final-code-review/` in the main checkout. The folder includes the checked commit, original lint and routine logs, final uninterrupted logs and developer correction logs. This local evidence is not pushed. The original process-ID collision log also remains under `target/0538-duckdb-shared-requests/closure/`.

## What the build taught us

Review the installed consumer's presence assertions alongside the generated public result types. Null is a successful decision value. Allocate temporary roots at the common runner boundary, and update fixtures that replace that boundary. Keep the original failed gate and run the required final command uninterrupted after the correction.
