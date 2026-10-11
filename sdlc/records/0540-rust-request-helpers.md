# Shared Rust request helpers

Rust request execution now uses one result accessor per kind, including the 27 former hand-written kind checks. Source and feed framing share admission and text decoding. One terminal-error iterator owns stopping after the first error, one function table owns streaming eligibility, table Rows owns its reset, and rank's error mapping belongs to the definitions table. Public behavior, diagnostics and API signatures are unchanged. The root Rust source ceiling fell from 189472 to 189458, a reduction of 14 lines.

Fresh code review accepted the complete refactor and checked all 27 diagnostics, mixed-kind projections, framing, terminal behavior, table ownership and rank mapping. Focused evidence includes 199 request/table/rank cases, 66 final request/session cases, 46 Polars cases, supported Clippy and compilation of the newly landed Python adapter. Initial check commands omitted fake backend settings, combined mutually exclusive SQLite features, or used the scratch decoy as the Polars usage folder. Correct commands passed; the original setup failures remain in the logs.

The routine closure run passed all 1995 Rust cases, doctests and all 21 public consumer cases under the unchanged timeout and thread limits. Its support suite then exposed an unrelated checkpoint mock missing the site folder required by 0543's new mandatory docs replay. The bounded fixture correction landed as `fa49e95b9` after fresh code review. Rebasing introduced no Rust change. Resuming from that support suite passed all 26 cases and the remaining schema, child-consumer, transform, triage and live checks. Full lint passed once. These staged results are not described as an uninterrupted full test run; 0542 owns that final-commit requirement.

The main checkout retains developer checks and closure logs in `target/0540-rust-request-helpers/`, including the original support failure, passing support continuation and full lint output. These build outputs are not committed.

## What the build taught us

Consolidation must retain the caller's admitted kind and exact error wording. Accessors can serve both scalar and mixed projections without a second dispatch rule. A fixture that models checkpoint commands must model a newly required docs replay and its failure path; silently skipping it would weaken the release gate. Keep passing unchanged core evidence when correcting that fixture.
