# 0418: Rank question sets through typed language and frame APIs

Status: COMPLETE. Reviewed implementation and adoption pass the full 29-consumer installed campaign at 60f0dcb9a. See [0432 qualification](../records/0432-shared-parity-cases.md#final-installed-qualification-2026-10-08). Final platform, release QA and publication remain under 0425.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

C, every SDK and supported dataframe variant exposes saved decide question-set rank with the landed shared turns merge and full member/final facts.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Define additive typed input/output; retain original indexes/member names and single-question behavior. Remove the old foreign-later deferral.
- Proof: Same independent turns cases as 0417 through each named public method; stable ties, duplicates, one-member equivalence, invalid inputs and zero-send replay.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0401D is landed; 0426–0431 own host carriers; 0410 owns frames; 0432 enforces full coverage.


## Native complete set rank, 2026-10-06

The additive native path admits all typed originals, every member declaration and explicit per-record context before lookup/send. It uses one ordinary record pipeline, shared controls and existing per-question cache/coalescing, then the retained stable member sorts and depth-by-depth turns merge. Duplicate visits still consume their turn. Saved score ranking remains the single-question rank route; saved rank sets remain decide-only.

`CompleteSetRank` carries the final numeric position, selecting member key, complete winning judgment and all ordered typed member judgments, each with its own stable member position and qualified answer identity. Parent identity includes every actual ordered observation and child answer ID, final position and member ownership; per-member and final metadata retain partial reported token shares. Presentation names, native declarations, batch sizes and transient calls remain outside request/cache identity. Result/2 serialization uses the existing rank shape plus `question_name`; original-containing results retain arbitrary caller-owned content. Released rank_set/SetRanked APIs remain unchanged, and their observer input placeholder is corrected to actual evidence.

### Added public declarations

```text
struct CompleteRankMember
struct CompleteSetRank
fn CompleteRankMember::name(&self) -> &str
const fn CompleteRankMember::result(&self) -> &CompleteRank
fn CompleteSetRank::members(&self) -> &[CompleteRankMember]
fn CompleteSetRank::question_name(&self) -> &str
const fn CompleteSetRank::result(&self) -> &CompleteRank
fn CompleteSetRank::to_json(&self) -> Result<String, Error>
const fn CompleteSetRank::value(&self) -> usize
impl Serialize for CompleteSetRank
fn Engine::rank_set_complete_with<I, T>(&self, &RankSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::rank_set_records_complete_with<I, T>(&self, &RankSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::try_rank_set_records_complete_with<I, T>(&self, &RankSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
```

Five public cases pin an independently expected six-question exchange and turns order, actual members/partial usage/owned non-Send originals, duplicate coalescing and zero-send replay across batch sizes, finite admission/reader refusal and empty work, and started member failures with actual owned observations/facts. Existing released rank-set cases are retained. Host adoption, CLI/schema adoption, fresh whole High review and full landing gates remain open.

Source grows 994 nonblank Rust lines (143694 → 144688), for complete typed set-rank carriers, the reused one-call record pipeline, pure member/final identity and checked metadata aggregation, and independent public behavior tests. Existing record admission, rank observers, stable sorting and turns merging were checked and reused instead of another implementation. Focused Clippy, five native cases and eight released set-rank compatibility cases pass; all files remain below the cap.

The Rust/Python/JavaScript/TypeScript/Ruby/R family checklist is complete under [0431](../records/0431-complete-existing-typed-sdks.md), including its seven installed/archive qualifications. This ticket remains open for the other SDK, SQL and dataframe surfaces; 0431 does not close their adoption.
