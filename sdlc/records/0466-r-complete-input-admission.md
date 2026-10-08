# 0466: Refuse contradictory R complete inputs

The R complete input boundary now tracks whether `records`, `paths`, `options`, and `jsonl` were supplied. Records refuse every file-source field and files refuse `records`, including explicit empty and null values, before reading a source or sending a request. A selected `records`, `paths`, or `jsonl` null keeps its prior usage refusal; omitted fields keep their defaults. Valid records and files retain originals, per-record context and options, and batch behavior. A bounded Quick Fix corrects the installed R `tt_plan` example to the existing conservative two-request bound, already pinned by the independent plan tests.

The new public R test failed on the selected-null case before the correction and passed after it: 24 checks and three counted loopback requests. Policy passed. Fresh read-only whole-change review and a fresh review of the selected-null correction returned ACCEPT. The installed R check exited 0. It passed the tarball install, outside-repository offline build, and both 249-case native parity runs. After merging main at `eafcd3f26`, the focused R test passed 24 checks with three sends. The full test exited 0 with 1,773 workspace tests, 339 library-only tests, 23 external-consumer tests, and 19 binding smokes. Policy, lint, and spec exited 0; spec reported 24 green demos. Planning-only main changes through `242bf3eab` were then merged without another code check.

R Rust source grew from 3,431 to 3,464 nonblank lines. The combined R source and test count is 3,339. Python, Ruby, and TypeScript each count 33 more lines from the shared complete module; C counts only the unchanged source module. The root Rust ratchet matches main at 164,634. The existing private-name list checked 35 entries with no tracked path or file hit. Accepted product correction: `ce732929a`; checked ratchet commit: `1dd8e32b8`; checked code merge: `28b99ce1b`.

## What the build taught us

Tracking field presence must preserve the old validation of fields the selected input kind actually uses. A shared source module also belongs in every binding ratchet that measures it. Release management and landing remain with the coordinator.
