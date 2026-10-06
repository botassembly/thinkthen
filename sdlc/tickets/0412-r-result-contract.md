# 0412: Document and test R index conventions

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

## Outcome

R’s one-based result positions and canonical zero-based mapping are explicit and tested without changing existing R results.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Update binding-author guidance and R consumer examples, distinguishing result indexes from physical one-based source lines and host span offsets.
- Proof: Rank duplicate/tie and find-none fixtures prove mapping; located spans retain physical coordinates.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0431 owns typed R source carriers; 0439 owns Linux install instructions.

## Successful find acceptance

Include a saved successful find selecting a non-first candidate. Assert its R one-based selected position and canonical zero-based input index independently. Retain the none result, duplicate/tie and physical-coordinate checks.
