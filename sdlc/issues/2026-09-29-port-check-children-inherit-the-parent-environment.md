# Port checks inherit the parent environment

Status: Open. Confirmed on main `1b1f09d70` with `python3 sdlc/scripts/children` and source inspection.

The existing child-environment guard reports 46 locations: one in the DuckDB bridge test and 45 in the Ada, COBOL, C++, C#, Dart, Go, JVM, Objective-C, PHP, R, Swift and Zig checkers. Examples include `libraries/ada/checks/exports.py` invoking `nm` with the full environment, and `libraries/php/fixtures/portable_batch.py` starting the conformance backend with inherited variables and copying the environment for a consumer.

These test tools should receive explicit required variables and controlled scratch homes under the accepted 0127 isolation rule. Inheriting unrelated caller settings or credentials makes a test depend on the shell that launches it. The finding does not claim a leaked credential or a product API defect. Confirm each reported call before changing it; preserve pinned compiler/runtime selection, loader paths and accepted source/archive modes.

## Outcome and proof

Route the affected test children through the existing language helpers or an explicit minimal environment. Keep the real installed-consumer and exact request-count/body assertions. Use a small planted unrelated variable to verify isolation without a broad consumer rebuild, then run only affected focused checks needed for changed environment selection. The existing guard must accept corrected call sites; do not add exemptions or weaken its tables.

The DuckDB test remains under Ian's SQL/DataFrame hold and is not authorized by the independent library batch. Keep that location explicit until the hold ends. A corrected library batch must not claim whole-tree lint green while a held or other reported location remains.
