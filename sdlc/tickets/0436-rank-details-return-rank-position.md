# 0436: Return rank position in detailed rank values

Status: in progress. Native implementation in lane0 on ticket/0443-native-complete-results; host adoption and final landing checks remain open.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

Detailed rank records carry a useful typed value instead of null while preserving probability/member detail and input identity.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 8.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Review and amend the current deliberate null contract. Proposed value is the one-based final rank position, matching the SQL rank-position convention, not an invented score across question sets.
- Proof: Pin sorted positions, stable ties, duplicate input identities, cut/top composition, question-set turns and single-question weighted ranking. Replayed answers rerank under changed reading rules with zero sends. Preserve existing ordinary output, facts and complete member probabilities.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

Settle exact public contract before code; coordinate 0406/0417/0418 and typed host schemas. Proposed positions are assigned after reading-rule selection and before top truncation, so top retains the same position prefix.

## Schema compatibility

The current result specification requires thinkthen.result/2 and a changelog entry when a detailed member’s type or meaning changes. 0442 owns one coordinated detailed-result schema transition, including this numeric rank value, the strict host readers and public examples. This ticket supplies the rank contract/implementation; 0442 supplies the versioned envelope. Acceptance pins thinkthen.result/2 together with numeric positions through affected public consumers. Preserve old bare convenience results and raw provider recordings; do not emit changed value semantics under thinkthen.result/1.

## Native work in progress

Separate canonical result/2 atomic serialization accepts a positive integer rank position while retaining the legacy result/1 projection. CompleteRank exposes the position and complete probabilities. Pure serializer cases pin integer semantics and unchanged legacy null. Position finalization, runtime routing, CLI/schema/corpus adoption and observation routes remain open.
