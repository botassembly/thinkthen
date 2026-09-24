---
flow: quick-fix
priority: 100
opens: .github/workflows/gate.yml sdlc/issues sdlc/tickets/0100-manual-gate-and-closed-issues.md sdlc/records/0100-quick-fix-manual-gate-and-closed-issues.md
---

# 0100: Run the gate only by hand and close five fixed issues

Status: landed 2026-09-24; record `sdlc/records/0100-quick-fix-manual-gate-and-closed-issues.md`, review `sdlc/records/0100-review.md`. Owner: Claude.

## Work

1. Set `on:` in `.github/workflows/gate.yml` to `workflow_dispatch:` only. The jobs stay unchanged. Ian ruled on 2026-09-22 that no push or pull request runs GitHub Actions. `sdlc/issues/2026-09-22-stop-github-actions-on-push.md` records the ruling.
2. Verify each fix on main by command. Then set the Status line of each issue to closed, naming the ticket and commit that fixed it:
   - `2026-09-22-command-wording-and-help-fixes-for-0-1` (tickets 0082 and 0090)
   - `2026-09-20-accuracy-round-on-three-public-sets-and-a-speed-rerun` (the `cost.jq` defect, ticket 0044)
   - `2026-09-23-score-spec-says-the-vendor-score-agrees-and-it-differs` (ticket 0087)
   - `2026-09-24-a-target-side-choice-asks-the-reversed-relation` (ticket 0088)
   - `2026-09-22-stop-github-actions-on-push` (this ticket)

Decisions, each of which Ian can overturn:

- `pages.yml` keeps its triggers. It stays disabled on GitHub by hand. Scope was set to `gate.yml` alone.
- No lint rung reads the workflow file. The red-green proof is a YAML check run by command and recorded in the record. A standing lint check is left out, because `sdlc/scripts/lint` is being edited by other tickets.

Stop rule: touch only the files in `opens`.
