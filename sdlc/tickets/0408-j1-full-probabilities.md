# 0408: Expose complete probabilities and details everywhere

Status: in progress. Native implementation in lane0 on ticket/0443-native-complete-results; host adoption and final landing checks remain open.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

Every surface offers complete ordered question probabilities, identities, confidence when supplied and failure details, including questions that contribute no selected output.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Extend shared details carriers and SQL details without changing bare outputs. Cover dropped filter rows, find none/all candidates, failed annotation members, recognition stages and rejected relation edges.
- Proof: Compare each distribution and member state to independent saved expectations; check started failure, replay zero sends and secrecy.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0426–0431 expose typed fields; 0435 owns invocation facts; 0432 enforces them.

When this change is pushed to main, notify the experiments team through pm with the commit, changed behavior and affected experiment 0035 steps. They rerun only affected steps without waiting for release. Note the notification in this ticket’s single landing record.
