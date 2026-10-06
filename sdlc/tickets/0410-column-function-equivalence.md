# 0410: Complete all ten dataframe functions and located files

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

## Outcome

pandas, Python Polars and Rust Polars expose all ten functions, equivalent identity/results/errors/storage, and typed located-file routes.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4 and 7.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Remove container-only refusals; add Rust Polars filter/rank/find/recognize/relate through the existing engine. Preserve nulls, duplicate index/name, whole candidate/entity sets, spans and relation endpoints. Lazy whole-set helpers explicitly materialize the logical collection where required; do not claim per-morsel streaming equivalence. Fix the deadline test’s invalid demand for two sends before expiry under load.
- Proof: Compare frame and ordinary saved inputs with nulls/duplicates/empty rows and all ten outputs. Reuse native file fixture. Already-expired deadline sends zero; in-flight timing accepts the valid completed prefix and preserves cancellation/facts without new clocks or timeout inflation.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0296 owns pandas Series accessor; 0431 owns ordinary Python/Rust file carriers; 0300 owns checked aggregate pricing. Source helpers reuse the native reader, never another parser.
