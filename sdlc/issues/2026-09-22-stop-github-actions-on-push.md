# Stop GitHub Actions from running on every push

Status: Open. High priority. Ian's ruling: no GitHub Actions for now. All testing runs on the local machines.

`.github/workflows/gate.yml` runs on every push and every pull request, on every branch. ThinkThen is private, so each run spends the botassembly organization's free-plan allowance of 2,000 Actions minutes a month. GitHub warned Ian at 90 percent on 2026-09-22.

Observed with `gh run list` on 2026-09-22: 293 gate runs since 2026-09-21 14:03 UTC, about 630 runner minutes. `main` used about 244 and `surfaces` about 197. The rest came from ticket branches. `pages.yml` ran 7 times in the same span.

GitHub runs the workflow file from the pushed branch. A fix on `main` alone leaves `surfaces` and every ticket branch still running the gate until each one merges `main`.

## Fix

1. Now: `gh workflow disable gate -R botassembly/thinkthen` and `gh workflow disable pages -R botassembly/thinkthen`. This stops every branch at once and changes no file.
2. Then: set `on:` in `gate.yml` to `workflow_dispatch:` only, so a run happens only by hand. Merge `main` into `surfaces` and any live ticket branch.
3. Keep the ladder local. `sdlc/scripts` stays the gate of record.
