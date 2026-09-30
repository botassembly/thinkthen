# 0309: Docs and records hygiene

Status: in progress. Lane claude-1. Branch `ticket/0309-docs-and-records-hygiene`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, ruling 8.

## Outcome

The front pages say what the tool does. The issue folder holds open issues only, each with a true status line. The planning index names the current plan. The Swift and Objective-C packages cannot drift from the C header.

## Evidence

- Starts from: main `1fc0075e9`, the cleanup plan's "next" list, and a read of `README.md`, `specification/README.md`, `sdlc/issues/` and `sdlc/planning/README.md`.
- Keeps: every specification rule, every issue's history, every historical plan, and `site/`, which marketing owns.
- Changes: the filter row in `README.md`; the command and audit rows in `specification/README.md`; the ADR 0050 citations; five closed issues move to `closed/`; five status lines added or corrected; one issue filed for marketing; the planning index; the vendored C header copies.
- Proof: `sdlc/scripts/lint`, `python3 sdlc/scripts/tickets`, a grep for links to moved files, and the Swift and Objective-C builds that exist here.
- Defers: the site fixes, which marketing owns; the open work each touched issue still names.

## Changes

1. `README.md` says filter prints kept records in input order, with CSV and TSV rows as JSON, as `specification/filter.md` says. `specification/README.md` lists `recognize`, `audit`, `diff` and `transform`, and says `audit` grades all ten commands.
2. ADR 0050 was planned by ticket 0147 and never written; ADR 0056 replaced it. The citations say so.
3. Issue `2026-09-30-reference-page-exit-codes-and-key-rule-drift.md` asks marketing to fix the reference page's exit codes and key rule.
4. Closed issues move to `sdlc/issues/closed/`, and links follow them. The panic, C++ and R issues gain status lines. The System One null-criteria and architect review 10 statuses say what remains.
5. `sdlc/planning/README.md` names `cleanup-2026-09-30.md` as the status authority and lists the older plans as historical.
6. The Swift and Objective-C packages stop carrying their own copy of `libraries/c/include/thinkthen.h`.
