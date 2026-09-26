# The sdlc history names private repositories

Status: Open

Filed on 2026-09-25 by the coordinator. Ticket 0126's builder found two planning pages that name a private caller's repository and its paths. A sweep of main then found 34 files that name one of three private repositories or their paths. Most are "Starts from" lines in the surface-port tickets, paths into the private marketing repository in closed issues, and a private experiment repository in `how-to-portfolio-study.md`. This repository goes public, and the workspace rule says a public repository never names a private project.

This issue names no private project. The coordinator's sweep list sits outside the repository. The files are:

- `sdlc/issues/2026-09-25-command-wording-and-help-fixes-before-0-1.md`
- `sdlc/issues/2026-09-25-site-samples-and-pages-after-the-surfaces-land.md`
- `sdlc/issues/closed/2026-09-21-is-there-an-eleventh-function-a-sweep-of-the-three-primitives.md`
- `sdlc/issues/closed/2026-09-21-recognize-is-the-ninth-function-and-the-deck-needs-one-real-output.md`
- `sdlc/issues/closed/2026-09-21-the-public-word-for-the-judged-thing-needs-one-ruling.md`
- `sdlc/issues/closed/2026-09-21-the-recognize-brief-for-the-experiment-team.md`
- `sdlc/issues/closed/2026-09-21-update-for-the-library-team-recognize-and-relate.md`
- `sdlc/issues/closed/2026-09-22-surfaces-branch-review-the-full-findings.md`
- `sdlc/planning/build-queue-2026-09-21.md`
- `sdlc/planning/handoff-to-the-architect-2026-09-21.md`
- `sdlc/planning/handoff-to-the-build-team-2026-09-21.md`
- `sdlc/planning/how-to-portfolio-study.md`
- `sdlc/planning/interface-audit.md`
- `sdlc/planning/issue-backlog-2026-09-25.md`
- `sdlc/planning/one-line-plan-2026-09-24.md`
- `sdlc/planning/quality-plan.md`
- `sdlc/planning/recognize-design.md`
- `sdlc/tickets/0086-expose-the-public-rust-api.md`
- `sdlc/tickets/0098-build-the-binding-members.md`
- `sdlc/tickets/0105-port-the-python-surface.md`
- `sdlc/tickets/0106-port-the-python-polars-data-frame-layer.md`
- `sdlc/tickets/0107-port-the-typescript-surface.md`
- `sdlc/tickets/0108-port-the-r-surface.md`
- `sdlc/tickets/0109-port-the-sqlite-surface.md`
- `sdlc/tickets/0110-port-the-duckdb-surface.md`
- `sdlc/tickets/0111-port-the-postgresql-surface.md`
- `sdlc/tickets/0112-port-the-ruby-surface.md`
- `sdlc/tickets/0113-add-the-audit-command.md`
- `sdlc/tickets/0114-add-the-diff-command.md`
- `sdlc/tickets/0118-relate-on-the-callers-duckdb-database.md`
- `sdlc/tickets/0122-support-pandas-columns-and-frames.md`
- `site/README.md`
- `site/scripts/pull-examples.mjs`
- `site/src/pages/blog/code-that-understands.astro`

## Fix

A Quick Fix before the repository goes public replaces each name with a generic consumer, such as "a private experiment repository", "the marketing deck", or "a private caller". Where a path matters as evidence, it says what the path held. Dated records keep their meaning, and only the name changes. Done when a sweep for the private repository names on this machine finds nothing in `git grep`.

A check inside this repository cannot hold the names it bans without naming them. So the sweep runs from the workspace, not from a rung.
