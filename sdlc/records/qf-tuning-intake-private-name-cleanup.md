# Quick Fix qf-tuning-intake-private-name-cleanup: generic wording in the tuning intake issues

Status: prepared on `ticket/qf-tuning-intake-private-name-cleanup` from main `eb49b1a7` as a bounded mechanical cleanup of the six issue files that commit `bbde1e9b` added. Root verifies the exact diff and routes it to independent review before it lands. No issue changes status and none counts as completed. Ian can overturn the wording of any replacement.

## Why

This repository goes public, so no tracked file may hold a name from the external private-name guard. The tuning intake prose reintroduced the machine-specific prefix of two evidence paths.

## Change

- 6 files touched, 8 guard matches removed, one line per file (line 3, the status line). Every match was the machine-specific prefix ending immediately before `/experiments/` inside a backticked evidence pointer; none was prose.
- Each pointer now reads the established workspace-relative form: "Evidence lives in local experiment 296" and "local experiment 297", matching how clean issue files already cite workspace experiments. Both folders exist on this machine, so the pointers resolve.
- Preserved without change: the filing date, the GEPA tuning experiment numbers 296 and 297, both folder names, the `notes/2026-09-27-optimization-lessons.md` write-up pointer, every issue status, and every other line of every file. `git diff --numstat` shows exactly 1 insertion and 1 deletion per file.
- Boundary note for root: the guard matched inside evidence paths, not prose. The fix drops only the private prefix and keeps the pointer resolvable under the repo's existing convention, the same treatment as the earlier private-name cleanups. Root rules on that boundary.

## Checks

- Standalone guard under the exact `sdlc/scripts/lint` semantics (one fixed string per nonblank list line, case-insensitive, tracked paths and tracked file text): before the edit, 6 matching lines in 6 tracked files and 0 tracked-path hits, the only hits in the whole tracked tree; after the edit, 0 matching lines in 0 tracked files and 0 tracked-path hits across the whole tracked tree.
- The same scan over the six files plus this record: 0 matches.
- `git diff --check`: clean. `python3 sdlc/scripts/tickets`: exit 0 ("0 evidence failures from ticket 0120 on"). `pages` was not run; no `site/` file changed. No build, tests, runtime, or network.

## Root verification and review

Root independently verified that only line3 changed in each issue, every numeric sequence and experiment path tail is identical, and both workspace folders exist. Root also ran the page and ticket checks successfully. Fresh independent Sol Medium review accepted `5ccf06b1dce0954c12e23c16dd8c321e35297a0f`; its whole tracked-tree scan found zero path or content matches with the actual30-entry guard across7001 tracked paths. This normalizes references without renaming evidence or changing an outcome. No history rewrite is claimed.
