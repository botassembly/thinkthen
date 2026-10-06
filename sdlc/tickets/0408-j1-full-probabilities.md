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
const fn FindReading::profile(&self) -> Option<&str>
fn Question::find_from_json(&str) -> Result<Question, Error>
fn Question::load_find(impl AsRef<Path>) -> Result<Question, Error>
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
struct CompleteDecision
impl Serialize for CompleteDecision
fn CompleteDecision::meta(&self) -> ResultMetadata<'_>
const fn CompleteDecision::answer_id(&self) -> &AnswerId
const fn CompleteDecision::identity(&self) -> &ResultIdentity
fn CompleteDecision::probabilities(&self) -> Probabilities
fn CompleteDecision::confidence(&self) -> Option<f64>
fn CompleteDecision::usage(&self) -> Option<Usage>
fn CompleteDecision::to_json(&self) -> Result<String, Error>
struct CompleteChoice
impl Serialize for CompleteChoice
fn CompleteChoice::meta(&self) -> ResultMetadata<'_>
const fn CompleteChoice::answer_id(&self) -> &AnswerId
const fn CompleteChoice::identity(&self) -> &ResultIdentity
fn CompleteChoice::probabilities(&self) -> Probabilities
fn CompleteChoice::confidence(&self) -> Option<f64>
fn CompleteChoice::usage(&self) -> Option<Usage>
fn CompleteChoice::to_json(&self) -> Result<String, Error>
struct CompleteTags
impl Serialize for CompleteTags
fn CompleteTags::meta(&self) -> ResultMetadata<'_>
const fn CompleteTags::answer_id(&self) -> &AnswerId
const fn CompleteTags::identity(&self) -> &ResultIdentity
fn CompleteTags::probabilities(&self) -> Probabilities
fn CompleteTags::confidence(&self) -> Option<f64>
fn CompleteTags::usage(&self) -> Option<Usage>
fn CompleteTags::to_json(&self) -> Result<String, Error>
struct CompleteScore
impl Serialize for CompleteScore
fn CompleteScore::meta(&self) -> ResultMetadata<'_>
const fn CompleteScore::answer_id(&self) -> &AnswerId
const fn CompleteScore::identity(&self) -> &ResultIdentity
fn CompleteScore::probabilities(&self) -> Probabilities
fn CompleteScore::confidence(&self) -> Option<f64>
fn CompleteScore::usage(&self) -> Option<Usage>
fn CompleteScore::to_json(&self) -> Result<String, Error>
struct CompleteFilter
impl Serialize for CompleteFilter
fn CompleteFilter::meta(&self) -> ResultMetadata<'_>
const fn CompleteFilter::answer_id(&self) -> &AnswerId
const fn CompleteFilter::identity(&self) -> &ResultIdentity
fn CompleteFilter::probabilities(&self) -> Probabilities
fn CompleteFilter::confidence(&self) -> Option<f64>
fn CompleteFilter::usage(&self) -> Option<Usage>
fn CompleteFilter::to_json(&self) -> Result<String, Error>
struct CompleteRank
impl Serialize for CompleteRank
fn CompleteRank::meta(&self) -> ResultMetadata<'_>
const fn CompleteRank::answer_id(&self) -> &AnswerId
const fn CompleteRank::identity(&self) -> &ResultIdentity
fn CompleteRank::probabilities(&self) -> Probabilities
fn CompleteRank::confidence(&self) -> Option<f64>
fn CompleteRank::usage(&self) -> Option<Usage>
fn CompleteRank::to_json(&self) -> Result<String, Error>
const fn CompleteDecision::value(&self) -> Answer
fn CompleteChoice::value(&self) -> Option<&str>
fn CompleteTags::value(&self) -> &[String]
const fn CompleteScore::value(&self) -> f64
const fn CompleteFilter::value(&self) -> bool
const fn CompleteRank::value(&self) -> usize
struct CompleteRecord<T, R>
const fn CompleteRecord::original(&self) -> &T
const fn CompleteRecord::ordinal(&self) -> usize
const fn CompleteRecord::result(&self) -> &R
fn CompleteRecord::into_parts(self) -> (T, R)
struct CompleteFound<T>
impl Serialize for CompleteFound
fn CompleteFound::meta(&self) -> ResultMetadata<'_>
const fn CompleteFound::answer_id(&self) -> &AnswerId
const fn CompleteFound::identity(&self) -> &ResultIdentity
fn CompleteFound::to_json(&self) -> Result<String, Error> where T: Serialize
fn CompleteFound::usage(&self) -> Option<Usage>
struct CompleteAnnotated
impl Serialize for CompleteAnnotated
fn CompleteAnnotated::meta(&self) -> ResultMetadata<'_>
const fn CompleteAnnotated::answer_id(&self) -> &AnswerId
const fn CompleteAnnotated::identity(&self) -> &ResultIdentity
fn CompleteAnnotated::to_json(&self) -> Result<String, Error>
struct CompleteRecognized
impl Serialize for CompleteRecognized
fn CompleteRecognized::meta(&self) -> ResultMetadata<'_>
const fn CompleteRecognized::answer_id(&self) -> &AnswerId
const fn CompleteRecognized::identity(&self) -> &ResultIdentity
fn CompleteRecognized::to_json(&self) -> Result<String, Error>
fn CompleteRecognized::usage(&self) -> Option<Usage>
struct CompleteRelated
impl Serialize for CompleteRelated
fn CompleteRelated::meta(&self) -> ResultMetadata<'_>
const fn CompleteRelated::answer_id(&self) -> &AnswerId
const fn CompleteRelated::identity(&self) -> &ResultIdentity
fn CompleteRelated::to_json(&self) -> Result<String, Error>
fn CompleteRelated::usage(&self) -> Option<Usage>
fn CompleteFound::selected(&self) -> Option<&T>
fn CompleteFound::candidates(&self) -> &[Candidate<T>]
fn CompleteFound::confidence(&self) -> Option<f64>
fn CompleteFound::into_selected(self) -> Option<T>
fn CompleteAnnotated::members(&self) -> impl ExactSizeIterator<Item = CompleteAnnotationMember<'_>>
struct CompleteAnnotationMember<'a>
fn CompleteAnnotationMember::name(&self) -> &str
fn CompleteAnnotationMember::answer_id(&self) -> Option<&AnswerId>
fn CompleteAnnotationMember::failure_id(&self) -> Option<&FailureId>
fn CompleteAnnotationMember::value(&self) -> Option<Judgment>
fn CompleteAnnotationMember::probabilities(&self) -> Option<Probabilities>
fn CompleteAnnotationMember::confidence(&self) -> Option<f64>
fn CompleteAnnotationMember::failure(&self) -> Option<FailureCause>
fn CompleteAnnotationMember::request(&self) -> &str
const fn CompleteRecognized::value(&self) -> &Recognized
fn CompleteRecognized::probabilities(&self) -> RecognitionProbabilities<'_>
struct RecognitionProbabilities<'a>
fn RecognitionProbabilities::pieces(&self) -> impl ExactSizeIterator<Item = PieceProbabilities<'_>>
fn RecognitionProbabilities::names(&self) -> impl ExactSizeIterator<Item = NameProbabilities<'_>>
fn RecognitionProbabilities::pairs(&self) -> impl ExactSizeIterator<Item = PairProbability<'_>>
struct PieceProbabilities<'a>
fn PieceProbabilities::range(&self) -> Range<usize>
fn PieceProbabilities::tags(&self) -> Vec<NamedProbability>
struct NameProbabilities<'a>
fn NameProbabilities::range(&self) -> Range<usize>
fn NameProbabilities::kinds(&self) -> Option<Vec<NamedProbability>>
fn NameProbabilities::edges(&self) -> Option<Vec<NamedProbability>>
struct PairProbability<'a>
fn PairProbability::relation(&self) -> &str
fn PairProbability::source(&self) -> Range<usize>
fn PairProbability::target(&self) -> Range<usize>
fn PairProbability::probability(&self) -> f64
fn CompleteRelated::value(&self) -> &[Edge]
fn CompleteRelated::members(&self) -> impl ExactSizeIterator<Item = CompleteRelationMember<'_>>
struct CompleteRelationMember<'a>
fn CompleteRelationMember::relation(&self) -> &str
fn CompleteRelationMember::reads(&self) -> &str
fn CompleteRelationMember::source(&self) -> Entity
fn CompleteRelationMember::target(&self) -> Option<Entity>
fn CompleteRelationMember::answer_id(&self) -> Option<&AnswerId>
fn CompleteRelationMember::failure_id(&self) -> Option<&FailureId>
fn CompleteRelationMember::probabilities(&self) -> Option<Probabilities>
fn CompleteRelationMember::failure(&self) -> Option<FailureCause>
fn CompleteRelationMember::request(&self) -> &str
const fn CompleteRelationMember::method(&self) -> RelationMethod
const fn CompleteRelationMember::direction(&self) -> RelationDirection
const fn CompleteRelationMember::probability(&self) -> Option<f64>
const fn CompleteRelationMember::accepted(&self) -> Option<bool>
enum RelationMethod
RelationMethod::YesNo
RelationMethod::Choice
impl Serialize for RelationMethod
enum RelationDirection
RelationDirection::SourceToTarget
RelationDirection::Either
impl Serialize for RelationDirection
```

Saved native find preparation now uses the common ordered grammar and capped reader, retains model/profile and rejects authored cuts/candidates/batch. Actual execution retains original units and partial reported usage; CLI file selection shares the same parser and fields. Complete CLI result/2/schema adoption remains in progress.
