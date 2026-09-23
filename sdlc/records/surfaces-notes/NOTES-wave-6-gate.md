# Wave 6, the gate lane (surfaces-review-5)

The fifth review's gate, process, and records findings, fixed on branch
`w6/gate`. Each fix carries its red output, its green output, and a check
the gate runs. The commits end "(refs surfaces-review-5)".

## Decisions Ian can overturn

- **Diverge means not run.** `conformance/skiptable.py` defines it once.
  A diverge entry must cite `conformance/DIVERGENCES.md`, and a runner
  prints FAIL for any case it ran and got wrong. The table carries no
  diverge entry today.
- **A lost wire suite fails the gate.** When the gate started or found a
  surface's stub, that surface's wire skip is a failure. The summary
  counts wire suites run and lost.
- **The gate workflow fetches full history** (`fetch-depth: 0`). The
  surfaces ratchet reads the commit that last moved its ceiling. In a
  shallow clone it now says the history is too shallow.
- **The private-reference checker holds a hash.** The public tree never
  spells the private name, even as pattern data.
- **cargo-pgrx runs only through `scripts/pgrx-package-locked.sh`.**
  cargo-pgrx 0.17 has no lock flag, so the wrapper proves the lock before
  and after packaging.

## Gaps this lane records and does not fix

- **The ceiling raises name no real second-agent review.** Commits
  `78315ba`, `d9cc3db`, and `252f530` each carry a second-agent line.
  Each line cites the fourth review's probes or the verifier's table.
  Neither reviewed the raise itself. The history stays as it is. The next
  ceiling move needs a review that names the raise and what it checked.
- **ADR 0041's review line and its context disagree with the record.**
  The review line cites "finding 21's reconciliation", and finding 21 is
  the skips. It says the verifier re-ran the hostile-deadline probes, and
  the verifier's row 15h reads PARTIAL. The context names a computed
  budget landing on -1 as the hazard, and the decision makes -1 the
  sentinel. An amendment by a second agent should settle both.
- **SQLite compares no recognize relations.** The surface ships no
  `thinkthen_relations` function, so its runner notes the gap and
  compares the entities only. The SQLite lane owns it.
- **DuckDB conformance did not run offline here.** No DuckDB 1.5.5 CLI
  was on this host, and the extension venv needs the network. The runner
  changes (the relations half, the in-process table reader) are unrun.
  The DuckDB lane owns the first run.
- **The per-workspace deny check still fetches once when cold.** The
  function lives in `sdlc/scripts/lint-workspaces`, which the DuckDB lane
  is editing. So does the BSD-unsafe `sed` newline in the same script.
- **This lane edited files other lanes own.** The wire-skip lines in each
  surface's `check.sh`, `libraries/python/build-wheel.sh`, the R vendor
  step and its check line, `libraries/r/thinkthen/src/Makevars.in`, and
  each runner's wire flag. Each edit is a few lines.
