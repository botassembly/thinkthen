# The private-name lint fails on main outside site/

Filed on 2026-09-28 by the marketing lead during a site landing.

`THINKTHEN_PRIVATE_NAMES=<the list> sdlc/scripts/lint` fails on main at 730c9145. It flags 61 lines in 36 files. None of them sit in `site/`. The site landing added no hit. The same run on its branch prints the same list.

`2026-09-25-sdlc-history-names-private-repositories.md` says every file outside `site/` was cleared. Files written since then have brought names back. This issue names no private project. The files are:

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
- `sdlc/records/qf-child-environment-isolation.md`
- `sdlc/records/qf-cli-diagnostic-corrections.md`
- `sdlc/records/qf-incremental-batch-guidance.md`
- `sdlc/records/qf-r-question-file-conformance.md`
- `sdlc/tickets/0211-private-tls-roots.md`

The fix belongs to the main builder. Describe each consumer generically, then rerun the lint with the list.
