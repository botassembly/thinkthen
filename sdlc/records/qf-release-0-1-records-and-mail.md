# Quick Fix qf-release-0-1-records-and-mail: record the 0.1 release and file the open mail asks

Status: built in lane claude-2, branch `ticket/quick-fix-release-0-1-records`, off origin/main `81062bd31`. Records only. Ian can overturn any milestone placement or proposed default here.

## Why

0.1.1 published on every registry on 2026-10-02 and 2026-10-03, and GitHub release v0.1.1 is public. Ticket 0128 phase 4 names a release record that did not exist. milestones.md still listed exit criterion 5 and path step 6 as open. The inbox held open asks from the docs team, the pm team and Beatles Bench. Each needed an answer or a filed issue. Ian approved finishing all of it.

## Change

- `sdlc/records/0128-release-0-1.md` records the 0.1.0 run 37035814818 and the 0.1.1 run 37059415069, what each registry holds, and what stays open. Its "Public install checks" section is the install-check builder's to fill.
- Ticket 0128's status says it is done except Phase 4 step 8, the public install checks; step 7, the Pages deploy; Ian's confirmation that the local registry tokens are deleted; and Ian's decision on the retained history step.
- milestones.md marks the 0.1 exit criteria met and lists the new 0.2 and later items.
- Ticket 0393, reserved with `pm ticket new`: npm publishes through staged publishing.
- Sixteen new issues, each answering a mail ask: the `rank --details` null value, the search features, the help gaps, the glossary, outside-agent reporting, the doc-test gate, `native_install` and `CARGO_TARGET_DIR`, the pandas and DuckDB binding gaps, the RubyGems placeholder, five draft functions and tools, the command's overhead, and the bench's moved runs.
- The proxy issue is rewritten with Ian's decisions of 2026-10-02. `ten-use-cases.md` and `libraries/command.md` point at it. `specification/roadmap.md` changes in the first change outside `sdlc/`, as the issue says.
- The run-facts issue gains the missing `server_ms` item.

No code, workflow, site or specification file changes.

## Checks

- `python3 sdlc/scripts/tickets`: 0 failures.
- `pm lint` reports no finding in a file this change touches.
- The private-name guard over the changed files finds nothing, and no added line names a home path.
- `gh release view v0.1.1` lists the extension archives the record names. `gh api` job logs give the npm 403 line of job 111040245433 and the publish line of job 111193360031.
