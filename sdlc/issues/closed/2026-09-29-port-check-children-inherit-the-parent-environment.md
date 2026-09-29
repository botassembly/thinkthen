# Port checks inherit the parent environment

Status: Closed after the reviewed 0276 library batch and final DuckDB Quick Fix `5b178c44e`. Originally confirmed on main `1b1f09d70` with `python3 sdlc/scripts/children` and source inspection.

The existing child-environment guard reports 46 locations: one in the DuckDB bridge test and 45 in the Ada, COBOL, C++, C#, Dart, Go, JVM, Objective-C, PHP, R, Swift and Zig checkers. Examples include `libraries/ada/checks/exports.py` invoking `nm` with the full environment, and `libraries/php/fixtures/portable_batch.py` starting the conformance backend with inherited variables and copying the environment for a consumer.

These test tools should receive explicit required variables and controlled scratch homes under the accepted 0127 isolation rule. Inheriting unrelated caller settings or credentials makes a test depend on the shell that launches it. The finding does not claim a leaked credential or a product API defect. Confirm each reported call before changing it; preserve pinned compiler/runtime selection, loader paths and accepted source/archive modes.

## Outcome and proof

Route the affected test children through the existing language helpers or an explicit minimal environment. Keep the real installed-consumer and exact request-count/body assertions. Use a small planted unrelated variable to verify isolation without a broad consumer rebuild, then run only affected focused checks needed for changed environment selection. The existing guard must accept corrected call sites; do not add exemptions or weaken its tables.

The independent library batch preserved the DuckDB finding during Ian's SQL/DataFrame hold. Ian later handed over the accepted redesign, permitting the final narrow test-child correction. No full package qualification follows from this source isolation fix.

## Library batch complete

Ticket 0276 at independently accepted `9675d47f` fixes all 45 library findings in the 27 claimed files. The planted real PHP backend and consumer, selected source and extracted-package checks, JVM launcher environment stub and ten measured Python ratchets passed. That library-stage guard scan left only the DuckDB location. The final Quick Fix `5b178c44e` passed fresh Medium review, the existing bridge child with a planted unrelated parent variable, and the unchanged guard with zero findings. The DuckDB source ratchet is6609. No full-lint or all-host qualification is claimed.

The integrated routine lint checkpoint passed child isolation and reached a separate known Dart README fence failure. That issue remains open; it does not reopen these corrected child environments. See [the final build record](../../records/qf-duckdb-test-child-environment.md).
