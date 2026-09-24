---
flow: quick-fix
priority: 95
opens: sdlc/scripts/live sdlc/live-test sdlc/issues/2026-09-20-packing-rows-into-one-request-measured.md sdlc/records/0101-live-refuses-non-shell-jobs.md
---

# 0101: Refuse a non-shell live job before any charge

Status: in progress. Owner: Claude.

## Problem

`sdlc/scripts/live` hands every job to `/bin/sh`. It appends the `charge N` row first and checks nothing about the job's language. A Python job dies at once and keeps its whole reservation, because the ledger has no refund. `sdlc/issues/2026-09-20-packing-rows-into-one-request-measured.md`, section "A defect in the live launcher", records a lost 560,000-token reservation.

## Fix

The launcher reads the job's first line before it takes the lock. The line must be exactly `#!/bin/sh`, the interpreter the launcher runs. Any other first line, an empty file, or an unreadable file refuses with status 2 and no ledger row. All 46 committed jobs under `demos/` and `probes/` already start with `#!/bin/sh`.

Decisions Ian can overturn:

- The rule is an exact `#!/bin/sh` line. A `#!/bin/bash` or `#!/bin/sh -e` line is refused too, because `/bin/sh` would ignore that shebang and run the file with other rules than it names.
- The issue's second guard, a sentence in `sdlc/scripts/README.md`, waits. Ticket 0083 edits that file. The refusal message names the rule, so a user learns it at the door.

## Proof

`sdlc/live-test` gains a case in its own temporary checkout and ledger. A job whose first line is `#!/usr/bin/python3` must exit 2 with the refusal, leave `charged_tokens 0`, and not run. The case fails on `origin/main` because the charge lands first. No real ledger and no real backend are touched.
