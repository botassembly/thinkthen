Status: Open. Filed 2026-09-29 by the marketing lead from workspace experiment 2038's sync run.

# lint fails on main: `review-jsonl` removes a path outside `scratch.sh`

After merging `origin/main` into `design/2038-sql-dataframe-interface` at `e36ea3b6`, the experiment reran the offline gates. `pages` and `policy` passed. `lint` failed on a file from main, not from the design branch:

- `demos/16-triage-pipeline/review-jsonl:17`: `if ((status != 0)); then rm -rf -- "$output"; fi`
- It landed in `562a5e25` ("Add narrow annotate continuation for missing pointers").
- It breaks `sdlc/planning/worktrees.md` rule 11: a script deletes only a path it made with `mktemp` in the same run, through the checked cleanup.

## Done when

`lint` passes on main, and the demo removes only the path it created, through the guarded cleanup.
