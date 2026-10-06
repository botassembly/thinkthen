# 0408: Expose complete probabilities and details everywhere

Status: in progress. Native implementation in lane0 on ticket/0443-native-complete-results; host adoption and final landing checks remain open.

Native carrier WIP: all ten functions now have concrete complete types.
Recognition's existing typed entities and stage distributions moved from the
facade into core and retain their legacy serialization. New recognition
accessors expose complete piece, name and pair tables; relation member
carriers retain the complete answer distribution alongside accepted/rejected
and failed member readings. Eight pure result serializer cases and the
27 existing recognition CLI/loopback cases pass. Complete execution routes,
identity propagation, CLI result/2 adoption and native public exchanges are
still open; SQL and host adoption remain separate work.

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
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0426–0431 expose typed fields; 0435 owns invocation facts; 0432 enforces them.

When this change is pushed to main, notify the experiments team through pm with the commit, changed behavior and affected experiment 0035 steps. They rerun only affected steps without waiting for release. Note the notification in this ticket’s single landing record.

Native execution WIP now covers all ten functions through the ordinary engine.
Borrowed resolved question/readings expose actual authored content and ordered
options, cuts/bands, raw choose/score/find picks and aggregate rules. Named
annotation/relation members retain their own actual sources, observations,
partial usage and admitted readings on success or failure. Three public
saved-response cases pin arbitrary authored maps/null, raw pick before cut,
member usage shares and safe Debug output. Owned observer snapshots, complete
call/error projections, input composition and CLI/schema adoption remain open.

### Added public declarations

```text
struct QuestionContent<'a>
impl Serialize for QuestionContent
fn QuestionContent::text(&self) -> Option<&str>
const fn QuestionContent::is_null(&self) -> bool
fn QuestionContent::to_json(&self) -> Result<String, Error>
enum ResolvedThreshold
ResolvedThreshold::Cut(f64)
ResolvedThreshold::Band
ResolvedThreshold::Band::low: f64
ResolvedThreshold::Band::high: f64
struct ResolvedOption<'a>
const fn ResolvedOption::name(&self) -> &str
const fn ResolvedOption::description(&self) -> Option<QuestionContent<'_>>
struct ResolvedQuestion<'a>
fn ResolvedQuestion::kind(&self) -> QuestionKind
fn ResolvedQuestion::text(&self) -> QuestionContent<'_>
fn ResolvedQuestion::options(&self) -> impl Iterator<Item = ResolvedOption<'_>>
fn ResolvedQuestion::yes(&self) -> Option<QuestionContent<'_>>
fn ResolvedQuestion::no(&self) -> Option<QuestionContent<'_>>
struct FindReading<'a>
fn FindReading::text(&self) -> QuestionContent<'_>
const fn FindReading::offers_none(&self) -> bool
struct ResolvedRelationRule<'a>
fn ResolvedRelationRule::name(&self) -> &str
fn ResolvedRelationRule::source(&self) -> &str
fn ResolvedRelationRule::target(&self) -> &str
fn ResolvedRelationRule::reads(&self) -> &str
const fn ResolvedRelationRule::either(&self) -> bool
const fn ResolvedRelationRule::single(&self) -> bool
struct RecognitionReading<'a>
fn RecognitionReading::kinds(&self) -> impl ExactSizeIterator<Item = ResolvedOption<'_>>
fn RecognitionReading::relations(&self) -> impl ExactSizeIterator<Item = ResolvedRelationRule<'_>>
const fn RecognitionReading::threshold(&self) -> ResolvedThreshold
const fn RecognitionReading::relation_threshold(&self) -> ResolvedThreshold
fn RecognitionReading::profile(&self) -> Option<&str>
struct RelationReading<'a>
fn RelationReading::relations(&self) -> impl ExactSizeIterator<Item = ResolvedRelationRule<'_>>
const fn RelationReading::threshold(&self) -> ResolvedThreshold
fn RelationReading::fields(&self) -> Option<(&str, &str)>
fn RelationReading::profile(&self) -> Option<&str>
fn CompleteChoice::raw_pick(&self) -> Option<&str>
fn CompleteScore::raw_level(&self) -> Option<&str>
fn CompleteFound::question(&self) -> FindReading<'_>
fn CompleteFound::raw_pick(&self) -> &str
fn CompleteRecognized::question(&self) -> RecognitionReading<'_>
fn CompleteRelated::question(&self) -> RelationReading<'_>
fn CompleteAnnotationMember::question(&self) -> ResolvedQuestion<'_>
fn CompleteAnnotationMember::threshold(&self) -> Option<ResolvedThreshold>
fn CompleteAnnotationMember::question_sources(&self) -> &[QuestionSource]
fn CompleteAnnotationMember::observations(&self) -> &[Observation]
fn CompleteAnnotationMember::reported_usage(&self) -> Option<ReportedUsage>
fn CompleteRelationMember::question(&self) -> ResolvedQuestion<'_>
fn CompleteRelationMember::threshold(&self) -> Option<ResolvedThreshold>
fn CompleteRelationMember::question_sources(&self) -> &[QuestionSource]
fn CompleteRelationMember::observations(&self) -> &[Observation]
fn CompleteRelationMember::reported_usage(&self) -> Option<ReportedUsage>
fn CompleteDecision::question(&self) -> ResolvedQuestion<'_>
fn CompleteDecision::threshold(&self) -> Option<ResolvedThreshold>
fn CompleteChoice::question(&self) -> ResolvedQuestion<'_>
fn CompleteChoice::threshold(&self) -> Option<ResolvedThreshold>
fn CompleteTags::question(&self) -> ResolvedQuestion<'_>
fn CompleteTags::threshold(&self) -> Option<ResolvedThreshold>
fn CompleteScore::question(&self) -> ResolvedQuestion<'_>
fn CompleteScore::threshold(&self) -> Option<ResolvedThreshold>
fn CompleteFilter::question(&self) -> ResolvedQuestion<'_>
fn CompleteFilter::threshold(&self) -> Option<ResolvedThreshold>
fn CompleteRank::question(&self) -> ResolvedQuestion<'_>
fn CompleteRank::threshold(&self) -> Option<ResolvedThreshold>
```
