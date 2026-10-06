# 0440: Move audit and diff under runs

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

## Outcome

thinkthen runs audit and thinkthen runs diff are the visible commands. Existing audit/diff names remain working hidden aliases.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 11.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Add parent dispatch/help and amend reserved-noun command guidance while preserving offline behavior and exact old inputs/output/exit codes.
- Proof: Compare old and new dispatch over saved run fixtures; pin errors, stdout/stderr and exit codes. Help advertises runs forms and hides old aliases. Both routes send zero requests.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

Update reserved-runs ADR/CLI specification and agent command examples; no new semantic function or proxy service.
