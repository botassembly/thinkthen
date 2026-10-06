# The pandas binding refuses a Series on four functions, and DuckDB's recognize takes no kind descriptions

Status: open. Filed 2026-10-03 from the docs message "Two binding gaps the docs found" of 2026-10-01 (site ticket 0052 slice A). Owner: the queue owner.
Milestone: 0.2

Every function page opens with the same simple call in every language. Two gaps stop a sample from matching the command-line opening.

1. The pandas binding refuses a Series for `filter`, `rank`, `find` and `relate`, with the error "Pass column.to_list()". `decide`, `choose`, `score`, `tag` and `annotate` accept a Series. A reader expects a column to work the same way for every function. The site's pandas samples for those four pass `.tolist()` today.
2. DuckDB's `thinkthen_recognize` takes only kind names. The command line and the other bindings can send a description for each kind. The `recognize` page opens on kind names only.

Neither belonged in 0.1. Each is a small change on one binding, and each keeps the old form working. When either lands, the site moves its samples back to the plain form.

## 2026-10-06 ownership

0410 owns pandas function/input parity and updates the affected plain-Series examples. 0434 owns DuckDB kind descriptions and its recognize example. Both are required in 0.2; 0296 owns accessor dispatch separately.
