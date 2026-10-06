# 0435: Return isolated SQL call facts and caller-priced cost

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Each SQL invocation returns final facts and optional exact caller-priced cost from the same judgment. DuckDB, SQLite and PostgreSQL agree.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4, 6 and 9.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Add facts to the existing details result from its owning Rust Call, including call_id. Preserve ordinary scalar/table outputs and cumulative usage APIs. Adopt the engine-scoped price pair and shared exact arithmetic from 0300.
- Proof: A details call sends/evaluates once; concurrent invocations retain isolated counts. Packed rows repeat one invocation’s facts and call_id, so deduplicating that ID counts its sends once. Never subtract cumulative usage snapshots or sum historical per-row usage shares. Test exact rounding, incomplete usage, overflow, cache/replay zero spend and started-failure facts through the reviewed native SQL error route.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0300 supplies shared price settings/arithmetic; 0442 supplies facts metadata. This ticket owns SQL adoption and updates 0423 guidance to supersede its earlier per-call-facts deferral. A statement containing several calls has several invocation facts; statement-wide accounting is outside this contract.

## Design notes

Specify an accessible started-failure facts route for each SQL host before code. If a native error cannot carry an object, use a bounded reviewed additive failure-result route rather than a cumulative counter. No second evaluation. Caller prices are estimates, never provider bills.

## Full-result answer identity

0450 IDs belong on SQL full details/observation results from the same native evaluation. Ordinary scalar values retain compatibility. Expose per-logical-result IDs independently from invocation call_id; repeated packed invocation facts do not collapse distinct logical answers. Coordinate image overloads in 0452 without a second judgment.
