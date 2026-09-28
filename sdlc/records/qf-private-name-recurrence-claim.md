# Private-name recurrence cleanup claim

Status: claimed for codex-7 as a release-priority Quick Fix. The existing external list at `~/.config/thinkthen/private-names.txt` holds 30 entries; it must remain outside this repository and must not be printed. The environment variable was unset, but the input itself existed. At main `8afb260a`, the standalone guard found 70 matching lines in 41 tracked files and no tracked-path hit. It found no website file.

Replace private consumer names and machine-specific home prefixes with accurate generic descriptions or workspace-relative evidence paths. Preserve source pins, artifact hashes, failure history and observed behavior. Do not rewrite Git history, edit raw experimental artifacts, weaken the guard or alter its list. Review any executable probe change for unchanged behavior; this is not permission to change measured input or fabricate a path. No website or user-facing documentation migration is included. The DuckDB proof README below is an internal build receipt.

The active DuckDB owner retains `sdlc/records/0247-duckdb-complete-question-forms-build.md` and will normalize its own record before review. Merge that result and rerun the guard before claiming zero whole-tree hits. Other active builders should check their new records using the same existing input.

## Claimed files

- `databases/duckdb/proof/0201/README.md`
- `probes/choose/measure.py`
- `sdlc/issues/2026-09-25-release-and-install-for-0-1.md`
- `sdlc/issues/closed/2026-09-28-recognize-help-describes-the-old-one-call-design.md`
- `sdlc/records/0128-phase-3a-build.md`
- `sdlc/records/0152-build-part-b.md`
- `sdlc/records/0163-answer-drift.md`
- `sdlc/records/0208-design-review.md`
- `sdlc/records/0211-tls-preflight.md`
- `sdlc/records/0212-rust-batching-preflight.md`
- `sdlc/records/0213-build.md`
- `sdlc/records/0214-code-review.md`
- `sdlc/records/0216-batching-build.md`
- `sdlc/records/0219-sqlite-build.md`
- `sdlc/records/0222-duckdb-batching-build.md`
- `sdlc/records/0224-duckdb-find-build.md`
- `sdlc/records/0224-macos-arm64-find-build.md`
- `sdlc/records/0224-sql-find-build.md`
- `sdlc/records/0225-usage-lock-preflight.md`
- `sdlc/records/0226-macos-arm64-package-proof.md`
- `sdlc/records/0227-macos-arm64-package-proof.md`
- `sdlc/records/0231-linux-arm64-build.md`
- `sdlc/records/0231-linux-arm64-preflight.md`
- `sdlc/records/0231-macos-arm64-build.md`
- `sdlc/records/0231-macos-intel-build.md`
- `sdlc/records/0232-build.md`
- `sdlc/records/0233-python-types-build.md`
- `sdlc/records/0235-build.md`
- `sdlc/records/0238-cache-model-build.md`
- `sdlc/records/2026-09-27-0172-shared-context-build.md`
- `sdlc/records/2026-09-28-cross-surface-remainder-preparation.md`
- `sdlc/records/2026-09-28-release-readiness-refresh.md`
- `sdlc/records/2026-09-28-run-accounting-remainder-preparation.md`
- `sdlc/records/qf-child-environment-isolation.md`
- `sdlc/records/qf-cli-diagnostic-corrections.md`
- `sdlc/records/qf-incremental-batch-guidance.md`
- `sdlc/records/qf-r-question-file-conformance.md`
- `sdlc/tickets/0211-private-tls-roots.md`
- `sdlc/tickets/0246-cache-binding-before-send.md`
- `sdlc/tickets/0249-merge-the-language-bindings.md`

The lane also owns its new Quick Fix build/review record and the existing private-name-recurrence issue’s status proposal. Root owns the plan, final issue closure and counts. Run the guard as its own cheap check, plus applicable prose/syntax/diff checks; no full gate or provider call is needed.
