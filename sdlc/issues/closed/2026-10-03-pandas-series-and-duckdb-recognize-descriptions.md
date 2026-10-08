# The pandas binding refuses a Series on four functions, and DuckDB's recognize takes no kind descriptions

Status: closed.
Resolution: e056fa21c
Milestone: 0.2

Every function page opens with the same simple call in every language. Two gaps stop a sample from matching the command-line opening.

1. The pandas binding refuses a Series for `filter`, `rank`, `find` and `relate`, with the error "Pass column.to_list()". `decide`, `choose`, `score`, `tag` and `annotate` accept a Series. A reader expects a column to work the same way for every function. The site's pandas samples for those four pass `.tolist()` today.
2. DuckDB's `thinkthen_recognize` takes only kind names. The command line and the other bindings can send a description for each kind. The `recognize` page opens on kind names only.

Neither belonged in 0.1. Each is a small change on one binding, and each keeps the old form working. When either lands, the site moves its samples back to the plain form.

## 2026-10-06 ownership

0410 owns pandas function/input parity and updates the affected plain-Series examples. 0434 owns DuckDB kind descriptions and its recognize example. Both are required in 0.2; 0296 owns accessor dispatch separately.

## Verified fixes

The pandas Series behavior landed with 0410 at `730aa68ed`. SQL kind descriptions landed with 0434/0435 at `7b3f7729b`; `e056fa21c` records the final integration results. [Dataframe record](../../records/0410-dataframes-ten-functions.md) and [SQL record](../../records/0434-sql-question-and-option-parity.md) retain the actual source/installed checks. The failure descriptions above describe the earlier source.
