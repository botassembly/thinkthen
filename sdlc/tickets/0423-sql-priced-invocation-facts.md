# 0423: State SQL pricing and timing limits accurately

Status: COMPLETE.

Opened as: 2026-10-11. Source review accepted; public site build passed. Full tests and lint run on the landing commit.
Milestone: 0.2

## Outcome

Public SQL documentation explains what plan and usage report, how callers obtain external pricing and elapsed time, and where those values are unknown. It makes no claim that SQL returns an isolated dollar cost or engine-call duration. This adopts the external-pricing alternative expressly authorized by the experiments team's message. An isolated priced facts envelope remains later work.

## Evidence

- Starts from: `inbox/thinkthen/2026-10-05-experiments-0033-0-2-gap-sql-plan-and-usage-omit-cost-and-time.md`, which explicitly permits supported external pricing and a narrower slide 8 claim. Ticket 0300 already supplies exact caller-priced facts in Rust, CLI and C; SQL adoption was deferred. Design sizing found native price settings, engine identity changes and a new completed invocation envelope would be needed across all three SQL hosts.
- Keeps: all existing SQL function signatures, result shapes, keyless no-send plans, process token totals, precise unknown-value behavior, secrecy and spending controls. Existing core fixed-point cost arithmetic and its distinct correctness tests remain intact.
- Changes: explain plan's estimated input-token band, unknown output and future duration; distinguish process usage totals from a single call; document client-measured statement wall time and external provider/caller pricing. State that provider invoices and complete attempt usage govern actual charges. Warn against summing repeated row metadata from a packed request or subtracting shared counters during concurrent work. Update the three database READMEs, public facts guidance and affected install text. Answer the experiments message so slide 8 uses the narrower supported claim.
- Proof: inspect the existing plan/usage/details contract against the current host implementation; check changed public prose and links through the normal site build. Reuse existing tests for token totals and shared-request metadata. Add no prose-only product tests or pricing runner. One fresh review; full tests and lint at landing.
- Defers: new SQL price settings; thinkthen_details_many; invocation-specific facts across all three hosts; cost for every semantic function, failed call or whole SQL statement; predictions of output cost and duration; provider billing integrations.

## Supported external facts

A client can measure wall time around a SQL statement. That time includes database and client work and is not an engine-only measurement. A caller can apply a known tariff to complete provider-reported input/output usage outside SQL. Use decimal arithmetic and label the result as an estimate; the provider's invoice determines actual charges. Missing usage or a missing tariff means unknown cost. SQL's cumulative usage does not prove completeness of every failed attempt. Never represent a missing value as zero or an input-only estimate as a total price.
