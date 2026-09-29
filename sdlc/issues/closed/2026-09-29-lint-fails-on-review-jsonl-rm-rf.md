Status: Closed. Reviewed cleanup `da126032` is integrated, and the coordinator's full lint checkpoint passed on main `45973d198`. Filed 2026-09-29 by the marketing lead from workspace experiment 2038's sync run.

# lint fails on main: `review-jsonl` removes a path outside `scratch.sh`

After merging `origin/main` into `design/2038-sql-dataframe-interface` at `e36ea3b6`, the experiment reran the offline gates. `pages` and `policy` passed. `lint` failed on a file from main, not from the design branch:

- `demos/16-triage-pipeline/review-jsonl:17`: `if ((status != 0)); then rm -rf -- "$output"; fi`
- It landed in `562a5e25` ("Add narrow annotate continuation for missing pointers").
- It breaks `sdlc/planning/worktrees.md` rule 11: a script deletes only a path it made with `mktemp` in the same run, through the checked cleanup.

## Done when

`lint` passes on main, and the demo removes only the path it created, through the guarded cleanup.

## Resolution

The `ticket/qf-triage-cleanup` candidate builds the two JSONL files in a
same-parent directory made by `scratch_dir`. Its trap removes that directory
through `scratch_clean` on failure. A native atomic no-replace rename publishes only
after the expected annotate exit 7, six schemas, and both transforms pass.
An existing output file, directory, or symlink is refused; a symlink added
during replay is also refused without touching its target. The focused replay
and failure checks are recorded in
[`qf-triage-cleanup.md`](../../records/qf-triage-cleanup.md).

The focused cleanup proof passed before integration. The final full lint checkpoint now passes, including its recursive-cleanup guard. This closes the original criteria; it does not claim an additional M5 demo replay or full package campaign.
