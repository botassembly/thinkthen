# Debt issues lack the three fields from Ian's ruling

Status: open. Filed 2026-09-30 by the pm team after verifying 6c9a1384d.

Ian ruled on 2026-09-30 that debt keeps both severity and trigger. The convention landed in this repository's issue README carries `Kind: debt` and `Pay when:` but is missing the other three ruled fields, and so are all nine debt issues.

The missing fields, per the model in botassembly/sdlc ADR 0006 (amended 2026-09-30) and pm ticket 0012:

1. `Severity: high | medium | low` — the cost of keeping the debt.
2. `Debt: NNN` — a sequential number assigned at capture, so items can be named and counted.
3. `Paid: YYYY-MM-DD` — the off-date, written when an item leaves the list. The filename date stays as the on-date.

The affected records: the README's debt section, the four tagged issues (Zig linker workaround, static-library check, old batching files, site replay folders), and the five filed issues (DuckDB failure under load, PostgreSQL cancel limit under load, lazy Polars streaming, audit-and-diff batch setting, duplicate how-to list).

Once added, `pm debt` can group by severity with stable numbering and answer when debt was paid. Until then the issues are nonconforming against the ruled model.
