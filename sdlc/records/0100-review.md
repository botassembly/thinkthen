FINDINGS

# Review of ThinkThen Quick Fix 0100 at 664f07ed

Reviewer: fresh read-only Claude session. Worktree `/home/ian/workspace/worktrees/thinkthen-0100`, branch `ticket/0100-manual-gate-and-closed-issues`.

## Finding 1: the open default-width question points at a ticket that does not carry it

The accuracy issue's new Status says "Closed" and "The default-width question stays open for ticket 0077." Observed:

- Ticket 0077 exists only on `origin/ticket/0077-process-width-cap` (`sdlc/tickets/0077-share-one-process-width-cap.md`). No 0077 ticket file is on main.
- That ticket keeps the fallback at 4 on purpose. Line 29: "The command still schedules an unbound batch at the existing fallback width 4, so command behavior and help remain unchanged." Line 23: "This ticket does not change the accepted range or add a pacer."
- `grep` finds no mention of the 2026-09-20 issue, the 1,267/1,319/1,272 measurements, or the "default drops to 3" question in that ticket.
- The issue also asked that the `--jobs` reference page give the measured numbers. `specification/records.md:129` and `site/src/pages/reference.astro:145` on main still carry no measured numbers.

So the pointer is one-way, and the target ticket decides the opposite scope. The question now lives only inside a closed issue's status line. Fix options, pick one: keep the accuracy issue Open with a status naming the `cost.jq` defect as fixed in `ce96895b` and the default-width question plus the `--jobs` page numbers as still open with no owner yet; or move the question to its own open issue and point the status there. Do not name 0077 as the owner unless 0077 is amended to carry it.

## Checks that passed

1. `gate.yml`: `git diff origin/main...HEAD` changes only the header comment and `on:`. `yaml.safe_load` gives triggers `['workflow_dispatch']`. The jobs are untouched.
2. Ruling: `sdlc/issues/2026-09-22-stop-github-actions-on-push.md` says "Ian's ruling: no GitHub Actions for now. All testing runs on the local machines." Its Fix step 2 says to set `on:` to `workflow_dispatch:` only. This matches the ticket.
3. Fixes on main, by command:
   - `git merge-base --is-ancestor` against `origin/main`: `331e85f7`, `4662310a`, `565f8b4f`, `ce96895b`, `37746b36`, `c84051f1` are all on main.
   - `cost.jq`: a text-input row through the file at `ce96895b~1` fails `Cannot index string with string "id"`, exit 5. The same row through main's `transforms/cost/cost.jq` prints totals, exit 0.
   - `specification/score.md` line 25 says the fields differ and names the tool value. `grep "so the two agree"` finds nothing.
   - `crates/thinkthen/src/core/relation.rs` line 237 is `format!("___ {reads} {asking}")`.
   - Command wording: ticket 0082 lists item 27 as stale and item 44 as moved to 0090. Ticket 0090 status is landed at `565f8b4f`. `git grep -w calibrated` on main's `site/src` finds nothing.
   - Stop-actions issue: closed by this ticket's `gate.yml` change.
4. Scope: the diff touches `gate.yml`, five files in `sdlc/issues/`, the ticket, and the record. All are in `opens`. `git merge-tree --write-tree origin/main HEAD` exits 0. The branch base is `ce0e3d6f`, and origin/main is at `ac69ec5f`. The merge is clean.
5. Gates, all `THINKTHEN_` variables unset: `sdlc/scripts/lint` exit 0. `sdlc/scripts/spec` exit 0, demos 21 green, 0 red. `sdlc/scripts/live` was not run. The worktree stayed clean.

## Notes, not blocking

- Ticket 0087's Status line on main still reads `accepted` though `37746b36` landed its record. That is outside this ticket's `opens`.
- The record cites a `gh workflow list` observation. I did not re-run it.
