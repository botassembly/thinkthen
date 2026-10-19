# 0617: Run harness fixtures on the tracked checkout, not copied scripts

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0615

Reviews: revision bdff727, accept

Reviews: revision 2389e7c, accept

Reviews: revision ea18d31, accept

Reviews: revision 02dc706, accept

Reviews: revision d4d3ccb, accept

Reviews: revision 4973fd1, accept

## Outcome

- Fixtures that exercise `test`, `test-full-cases`, `smoke` or `surfaces` run against a worktree of the tracked checkout (`git worktree add` or a `git archive` of `HEAD`), never against a hand-picked list of copied files.
- The inline `for tool in (...)` lists are deleted. Fixtures take their tool set from the 0615 manifest.
- Fixtures that only check how these scripts are laid out internally are deleted. The remaining ones assert results a user or release sees: exit codes, printed verdict lines, written receipts.
- `sdlc/scripts/heavy-lock` is deleted, and its callers source `allow-list` directly.
- These checks must stay: the archive clean-tree guard (bfdc8d1ce) and the retention of failed parity logs (a334b3bf2).

### Proof

- On the M5 through pm job, `sdlc/scripts/test` passes.
- Done once by hand and not committed: a commit that adds a new sourced helper to `sdlc/scripts/test` leaves the fixtures passing with no fixture edit.
- A grep finds no tuple of script names being copied into a fixture root in `conformance/`, and no `for tool in (` list under `sdlc/scripts`.

## Evidence

Several conformance fixtures copy named entrypoint scripts into a fake checkout and stub the rest. When an entrypoint starts sourcing a new file, the copy lacks it and the fixture fails, one file per round. On 2026-10-17, three commits each added one missing file to the same hand-written lists: `cb9c71eb8` at 13:15 added `heavy-lock`, `2d5808174` at 13:17 changed the stress fixture's Python, and `3a4d5f0b3` at 13:58 added `libraries/ruby/toolchain.env` and `ruby`. All three touch `conformance/test_parity.py` and `conformance/test_stress_selection.py`. `de1013a59` (2026-10-16) is the same class. These fixtures check how the harness is laid out, not what the product does. `sdlc/scripts/heavy-lock` is now a two-line shim that sources `allow-list`, yet fixtures still have to copy it (`067453bd1`).

The fixtures also build PATH from tool lists typed inline: there are 15 `for tool in (` loops under `sdlc/scripts`, for example `install-check-self-test.py:110`, `coverage-checks.py:167` and `coverage_installed_extensions.py:23`. `mktemp`, `sha256sum` and `shasum` were added to two of these lists only after a run failed without them (`7f1d4b7fb`).

## Progress

- 2026-10-18 started
- 2026-10-19 next: The full gate exposed database routes omitted from the earlier library-only guard table; fresh review accepts4973fd113 after all three database guards and PostgreSQL shell ordering were repaired, portable cases and extended guard checks passed on M5, and the failed d612720 receipt remains retained before one closing gate.
