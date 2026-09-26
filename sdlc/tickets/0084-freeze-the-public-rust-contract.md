---
flow: build
priority: 84
opens: sdlc/tickets/0084-freeze-the-public-rust-contract.md sdlc/planning/adr/0017-libraries-over-one-bound-core.md sdlc/planning/libraries/rust.md
---

# 0084: Freeze the public Rust contract

Status: landed; record `0084-build-contract.md`. Owner: Claude.

## Outcome and authority

Specify the candidate Rust source contract for ten user functions before 0085 and 0086 expose it.

0080 and 0081 own `recognize` and `relate` behavior but exclude public Rust. This ticket owns their Rust shape. ADR 0017 owns one blocking crate, shared options, and six error kinds. Its proposed amendment names the authorized ten functions. `decide_many`, `details`, and `usage` are supporting forms.

This design may be reviewed and recorded now. `Engine`, `EngineBuilder`, and `default_engine` remain provisional. Before 0085 or 0086, 0078 must land and those declarations must reconcile construction, `Clone + Send + Sync + Debug`, implicit initialization, omitted-width registration, and signal dependency placement. Ticket 0096 owns fork recovery and lands before 0086. A change outside that list reopens full review.

## Fixed boundaries

- One `thinkthen` package supplies library and default-`cli` binary. Modules stay private. No connector, planner, C symbol, async form, runtime, second crate, or proc macro is public. The one public callback is 0095's interrupt check.
- All calls block. Every fallible constructor or builder step returns `Result<_, Error>` at the step that receives the bad value. Builder terminal methods consume the builder.
- `ChooseQuestion<C>` and `TagQuestion<C>` retain their `Choice` type from builder through call. `Question` holds decide, score, rank, find, and parsed unbound values; `into_choose::<C>` and `into_tag::<C>` check a loaded value before binding it. `BandedQuestion` is separate. Only `decide` and `details` accept a band. `filter(&BandedQuestion, ..)` does not compile; a non-decision `Question` returns `Error::Usage` before an external effect.
- Streaming functions are `filter`, `decide_many`, and `annotate`. They accept `IntoIterator`, return `Batch`, bound retained work by effective width, and preserve input order. `rank`, `find`, and `relate` consume a finite iterator before they can answer and return one finite aggregate.
- Bulk records implement `Evidence`; `String` and `&str` work directly. JSON Pointer extraction stays command-only in 0.1.
- The first release keeps the typed `Description` and `QuestionSet` builders, `choices!`, and no derive. Structured descriptions retain insertion order. Duplicate named or extension fields fail. An implicit engine omits width; it never materializes the command default as an explicit width.

## Changes on reopening, 2026-09-24

Ticket 0095 records why five declarations changed and adds the binding members; 0086 checks both as one inventory. The free `filter`, `decide_many`, and `annotate` return a `default_engine` build failure as the batch's first item. `Batch` is neither `Send` nor `Sync`, so a host that runs work on another thread builds the batch there.

## Normative public inventory

Except for the provisional Engine subset named above, these declarations are normative after rustfmt. Names, bounds, arguments, returns, variants, and shown traits are fixed. Fields are private unless shown. 0086 checks fixtures and normalized `cargo public-api`; an absent item is not public.

```rust
pub struct Engine { /* private */ }
impl Clone for Engine {}
impl std::fmt::Debug for Engine {}
impl Engine {
    pub fn from_env() -> Result<Self, Error>;
    pub fn builder() -> EngineBuilder;
    pub fn decide<Q: DecisionQuestion + ?Sized>(&self, question: &Q, evidence: &str) -> Result<Answer, Error>;
    pub fn decide_with<Q: DecisionQuestion + ?Sized>(&self, question: &Q, evidence: &str, options: CallOptions<'_>) -> Result<Answer, Error>;
    pub fn choose<C: Choice>(&self, question: &ChooseQuestion<C>, evidence: &str) -> Result<Option<C>, Error>;
    pub fn choose_with<C: Choice>(&self, question: &ChooseQuestion<C>, evidence: &str, options: CallOptions<'_>) -> Result<Option<C>, Error>;
    pub fn score(&self, question: &Question, evidence: &str) -> Result<f64, Error>;
    pub fn score_with(&self, question: &Question, evidence: &str, options: CallOptions<'_>) -> Result<f64, Error>;
    pub fn tag<C: Choice>(&self, question: &TagQuestion<C>, evidence: &str) -> Result<Vec<C>, Error>;
    pub fn tag_with<C: Choice>(&self, question: &TagQuestion<C>, evidence: &str, options: CallOptions<'_>) -> Result<Vec<C>, Error>;
    pub fn filter<'a, I>(&'a self, question: &'a Question, records: I) -> Batch<'a, I::Item> where I: IntoIterator + 'a, I::Item: Evidence;
    pub fn filter_with<'a, I>(&'a self, question: &'a Question, records: I, options: CallOptions<'a>) -> Batch<'a, I::Item> where I: IntoIterator + 'a, I::Item: Evidence;
    pub fn rank<I>(&self, question: &Question, records: I) -> Result<Vec<Ranked<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence;
    pub fn rank_with<I>(&self, question: &Question, records: I, options: CallOptions<'_>) -> Result<Vec<Ranked<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence;
    pub fn find<I>(&self, question: &Question, units: I) -> Result<Found<I::Item>, Error> where I: IntoIterator, I::Item: Evidence;
    pub fn find_with<I>(&self, question: &Question, units: I, options: CallOptions<'_>) -> Result<Found<I::Item>, Error> where I: IntoIterator, I::Item: Evidence;
    pub fn annotate<'a, I>(&'a self, questions: &'a QuestionSet, records: I) -> Batch<'a, AnnotatedRecord<I::Item>> where I: IntoIterator + 'a, I::Item: Evidence;
    pub fn annotate_with<'a, I>(&'a self, questions: &'a QuestionSet, records: I, options: CallOptions<'a>) -> Batch<'a, AnnotatedRecord<I::Item>> where I: IntoIterator + 'a, I::Item: Evidence;
    pub fn recognize(&self, ask: &Recognize, evidence: &str) -> Result<Recognized, Error>;
    pub fn recognize_with(&self, ask: &Recognize, evidence: &str, options: CallOptions<'_>) -> Result<Recognized, Error>;
    pub fn relate<I>(&self, ask: &Relate, entities: I) -> Result<Vec<Edge>, Error> where I: IntoIterator<Item = Entity>;
    pub fn relate_with<I>(&self, ask: &Relate, entities: I, options: CallOptions<'_>) -> Result<Vec<Edge>, Error> where I: IntoIterator<Item = Entity>;
    pub fn decide_many<'a, I, Q: DecisionQuestion + ?Sized>(&'a self, question: &'a Q, records: I) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: Evidence;
    pub fn decide_many_with<'a, I, Q: DecisionQuestion + ?Sized>(&'a self, question: &'a Q, records: I, options: CallOptions<'a>) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: Evidence;
    pub fn details<Q: DetailQuestion + ?Sized>(&self, question: &Q, evidence: &str) -> Result<Details, Error>;
    pub fn details_with<Q: DetailQuestion + ?Sized>(&self, question: &Q, evidence: &str, options: CallOptions<'_>) -> Result<Details, Error>;
    pub fn usage(&self) -> Counters;
}
pub fn default_engine() -> Result<&'static Engine, Error>;
pub fn decide<Q: DecisionQuestion + ?Sized>(question: &Q, evidence: &str) -> Result<Answer, Error>;
pub fn decide_with<Q: DecisionQuestion + ?Sized>(question: &Q, evidence: &str, options: CallOptions<'_>) -> Result<Answer, Error>;
pub fn choose<C: Choice>(question: &ChooseQuestion<C>, evidence: &str) -> Result<Option<C>, Error>;
pub fn choose_with<C: Choice>(question: &ChooseQuestion<C>, evidence: &str, options: CallOptions<'_>) -> Result<Option<C>, Error>;
pub fn score(question: &Question, evidence: &str) -> Result<f64, Error>;
pub fn score_with(question: &Question, evidence: &str, options: CallOptions<'_>) -> Result<f64, Error>;
pub fn tag<C: Choice>(question: &TagQuestion<C>, evidence: &str) -> Result<Vec<C>, Error>;
pub fn tag_with<C: Choice>(question: &TagQuestion<C>, evidence: &str, options: CallOptions<'_>) -> Result<Vec<C>, Error>;
pub fn filter<'a, I>(question: &'a Question, records: I) -> Batch<'a, I::Item> where I: IntoIterator + 'a, I::Item: Evidence;
pub fn filter_with<'a, I>(question: &'a Question, records: I, options: CallOptions<'a>) -> Batch<'a, I::Item> where I: IntoIterator + 'a, I::Item: Evidence;
pub fn rank<I>(question: &Question, records: I) -> Result<Vec<Ranked<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence;
pub fn rank_with<I>(question: &Question, records: I, options: CallOptions<'_>) -> Result<Vec<Ranked<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence;
pub fn find<I>(question: &Question, units: I) -> Result<Found<I::Item>, Error> where I: IntoIterator, I::Item: Evidence;
pub fn find_with<I>(question: &Question, units: I, options: CallOptions<'_>) -> Result<Found<I::Item>, Error> where I: IntoIterator, I::Item: Evidence;
pub fn annotate<'a, I>(questions: &'a QuestionSet, records: I) -> Batch<'a, AnnotatedRecord<I::Item>> where I: IntoIterator + 'a, I::Item: Evidence;
pub fn annotate_with<'a, I>(questions: &'a QuestionSet, records: I, options: CallOptions<'a>) -> Batch<'a, AnnotatedRecord<I::Item>> where I: IntoIterator + 'a, I::Item: Evidence;
pub fn recognize(ask: &Recognize, evidence: &str) -> Result<Recognized, Error>;
pub fn recognize_with(ask: &Recognize, evidence: &str, options: CallOptions<'_>) -> Result<Recognized, Error>;
pub fn relate<I>(ask: &Relate, entities: I) -> Result<Vec<Edge>, Error> where I: IntoIterator<Item = Entity>;
pub fn relate_with<I>(ask: &Relate, entities: I, options: CallOptions<'_>) -> Result<Vec<Edge>, Error> where I: IntoIterator<Item = Entity>;
pub fn decide_many<'a, I, Q: DecisionQuestion + ?Sized>(question: &'a Q, records: I) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: Evidence;
pub fn decide_many_with<'a, I, Q: DecisionQuestion + ?Sized>(question: &'a Q, records: I, options: CallOptions<'a>) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: Evidence;
pub fn details<Q: DetailQuestion + ?Sized>(question: &Q, evidence: &str) -> Result<Details, Error>;
pub fn details_with<Q: DetailQuestion + ?Sized>(question: &Q, evidence: &str, options: CallOptions<'_>) -> Result<Details, Error>;
pub fn usage() -> Result<Counters, Error>;

pub struct EngineBuilder { /* private */ }
impl std::fmt::Debug for EngineBuilder {}
impl EngineBuilder {
    pub fn from_env() -> Result<Self, Error>;
    pub fn base_url(self, value: &str) -> Result<Self, Error>;
    pub fn api_key(self, value: &str) -> Result<Self, Error>;
    pub fn model(self, value: &str) -> Result<Self, Error>;
    pub fn width(self, value: u8) -> Result<Self, Error>;
    pub fn max_requests(self, value: Option<usize>) -> Result<Self, Error>;
    pub fn default_cache(self) -> Self;
    pub fn cache_at(self, value: impl AsRef<std::path::Path>) -> Result<Self, Error>;
    pub fn no_cache(self) -> Self;
    pub fn cache_bytes(self, value: u64) -> Result<Self, Error>;
    pub fn build(self) -> Result<Engine, Error>;
}

pub trait Evidence { fn evidence(&self) -> &str; }
impl Evidence for String {}
impl Evidence for &str {}
pub trait DecisionQuestion: private::Sealed {}
impl DecisionQuestion for Question {}
impl DecisionQuestion for BandedQuestion {}
pub trait DetailQuestion: private::Sealed {}
impl DetailQuestion for Question {}
impl DetailQuestion for BandedQuestion {}
impl<C: Choice> DetailQuestion for ChooseQuestion<C> {}
impl<C: Choice> DetailQuestion for TagQuestion<C> {}

pub struct Description { /* private */ }
impl Clone for Description {}
impl std::fmt::Debug for Description {}
impl Eq for Description {}
impl PartialEq for Description {}
impl Description {
    pub fn text(value: &str) -> Result<Self, Error>;
    pub fn builder() -> DescriptionBuilder;
    pub fn as_json(&self) -> &str;
}
pub struct DescriptionBuilder { /* private */ }
impl std::fmt::Debug for DescriptionBuilder {}
impl DescriptionBuilder {
    pub fn what(self, value: &str) -> Result<Self, Error>;
    pub fn not_for(self, value: &str) -> Result<Self, Error>;
    pub fn example(self, value: &str) -> Result<Self, Error>;
    pub fn field_json(self, name: &str, compact_json_value: &str) -> Result<Self, Error>;
    pub fn build(self) -> Result<Description, Error>;
}

pub struct Question { /* private */ }
pub struct BandedQuestion { /* private */ }
impl Clone for Question {}
impl Clone for BandedQuestion {}
impl std::fmt::Debug for Question {}
impl std::fmt::Debug for BandedQuestion {}
impl Question {
    pub fn decide(text: &str) -> Result<DecideBuilder, Error>;
    pub fn choose<C: Choice>(text: &str) -> Result<ChooseBuilder<C>, Error>;
    pub fn tag<C: Choice>(text: &str) -> Result<TagBuilder<C>, Error>;
    pub fn score(text: &str) -> Result<ScoreBuilder, Error>;
    pub fn rank(text: &str) -> Result<Self, Error>;
    pub fn find(text: &str) -> Result<Self, Error>;
    pub fn from_json(value: &str) -> Result<LoadedQuestion, Error>;
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<LoadedQuestion, Error>;
    pub fn into_choose<C: Choice>(self) -> Result<ChooseQuestion<C>, Error>;
    pub fn into_tag<C: Choice>(self) -> Result<TagQuestion<C>, Error>;
}
pub struct ChooseQuestion<C: Choice> { /* private, retains C */ }
pub struct TagQuestion<C: Choice> { /* private, retains C */ }
pub enum LoadedQuestion { Question(Question), Banded(BandedQuestion) }
impl Clone for LoadedQuestion {}
impl std::fmt::Debug for LoadedQuestion {}
pub struct DecideBuilder { /* private */ }
impl DecideBuilder {
    pub fn yes(self, value: Description) -> Result<Self, Error>;
    pub fn no(self, value: Description) -> Result<Self, Error>;
    pub fn model(self, value: &str) -> Result<Self, Error>;
    pub fn cut(self) -> Question;
    pub fn cut_at(self, value: f64) -> Result<Question, Error>;
    pub fn band(self, low: f64, high: f64) -> Result<BandedQuestion, Error>;
}
pub struct ChooseBuilder<C: Choice> { /* private */ }
impl<C: Choice> ChooseBuilder<C> {
    pub fn option(self, value: C, description: Option<Description>) -> Result<Self, Error>;
    pub fn model(self, value: &str) -> Result<Self, Error>;
    pub fn build(self) -> Result<ChooseQuestion<C>, Error>;
    pub fn cut_at(self, value: f64) -> Result<ChooseQuestion<C>, Error>;
}
pub struct TagBuilder<C: Choice> { /* private */ }
impl<C: Choice> TagBuilder<C> {
    pub fn label(self, value: C, description: Option<Description>) -> Result<Self, Error>;
    pub fn model(self, value: &str) -> Result<Self, Error>;
    pub fn cut(self) -> Result<TagQuestion<C>, Error>;
    pub fn cut_at(self, value: f64) -> Result<TagQuestion<C>, Error>;
}
pub struct ScoreBuilder { /* private */ }
impl ScoreBuilder {
    pub fn level(self, name: &str, description: Option<Description>) -> Result<Self, Error>;
    pub fn model(self, value: &str) -> Result<Self, Error>;
    pub fn build(self) -> Result<Question, Error>;
}
pub trait Choice: Clone + Eq + Send + Sync + 'static {
    fn label(&self) -> &'static str;
    fn labels() -> &'static [&'static str];
    fn from_label(value: &str) -> Option<Self>;
}
// choices! accepts attributes, visibility, an enum name, and Variant => "label" entries.
// It emits Clone, Copy, Debug, Eq, PartialEq, the Choice implementation, and inherent label/labels/from_label.

pub struct QuestionSet { /* private */ }
impl Clone for QuestionSet {}
impl std::fmt::Debug for QuestionSet {}
impl QuestionSet {
    pub fn from_json(value: &str) -> Result<Self, Error>;
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, Error>;
    pub fn builder() -> QuestionSetBuilder;
}
pub struct QuestionSetBuilder { /* private */ }
impl QuestionSetBuilder {
    pub fn question(self, name: &str, value: Question) -> Result<Self, Error>;
    pub fn banded(self, name: &str, value: BandedQuestion) -> Result<Self, Error>;
    pub fn choose<C: Choice>(self, name: &str, value: ChooseQuestion<C>) -> Result<Self, Error>;
    pub fn tag<C: Choice>(self, name: &str, value: TagQuestion<C>) -> Result<Self, Error>;
    pub fn build(self) -> Result<QuestionSet, Error>;
}

pub struct CallOptions<'a> { /* private */ }
impl<'a> Clone for CallOptions<'a> {}
impl<'a> Copy for CallOptions<'a> {}
impl<'a> std::fmt::Debug for CallOptions<'a> {}
impl<'a> Default for CallOptions<'a> {}
impl<'a> CallOptions<'a> {
    pub const fn new() -> Self;
    pub const fn cancel(self, value: &'a CancelToken) -> Self;
    pub fn deadline_at(self, value: std::time::Instant) -> Self;
    pub fn deadline_after(self, value: std::time::Duration) -> Result<Self, Error>;
}
pub struct CancelToken { /* private */ }
impl Clone for CancelToken {}
impl std::fmt::Debug for CancelToken {}
impl Default for CancelToken {}
impl CancelToken { pub fn new() -> Self; pub fn cancel(&self); pub fn is_cancelled(&self) -> bool; }
pub struct Batch<'a, T> { /* private */ }
impl<T> std::fmt::Debug for Batch<'_, T> {}
impl<T> Iterator for Batch<'_, T> { type Item = Result<T, Error>; }

pub enum Answer { Yes, No, Unsure }
pub enum Judgment { Decision(Answer), Choice(Option<String>), Score(f64), Tags(Vec<String>) }
pub struct NamedProbability { /* private */ }
impl NamedProbability { pub fn name(&self) -> &str; pub fn probability(&self) -> f64; }
pub enum Probabilities { YesNo { yes: f64 }, Named(Vec<NamedProbability>) }
pub struct Usage { /* provider-reported counts */ }
impl Usage { pub fn input_tokens(&self) -> u64; pub fn output_tokens(&self) -> u64; }
pub struct Counters { /* process counts */ }
impl Counters { pub fn requests_sent(&self) -> u64; pub fn cache_answers(&self) -> u64; pub fn input_tokens(&self) -> u64; pub fn output_tokens(&self) -> u64; }
pub struct Details { /* private */ }
impl Details {
    pub fn value(&self) -> &Judgment;
    pub fn probabilities(&self) -> &Probabilities;
    pub fn nearest(&self) -> Option<&str>;
    pub fn model(&self) -> &str;
    pub fn question_sha256(&self) -> &str;
    pub fn requests(&self) -> &[String];
    pub fn requests_sent(&self) -> u64;
    pub fn cached(&self) -> bool;
    pub fn usage(&self) -> Option<&Usage>;
    pub fn confidence(&self) -> Option<f64>;
    pub fn url(&self) -> &str;
    pub fn failed_questions(&self) -> usize;
}
pub struct Row<T, V> { /* private */ }
impl<T, V> Row<T, V> { pub fn input(&self) -> &T; pub fn value(&self) -> &V; pub fn into_parts(self) -> (T, V); }
pub struct Ranked<T> { /* private */ }
impl<T> Ranked<T> { pub fn input(&self) -> &T; pub fn probability(&self) -> f64; pub fn into_input(self) -> T; }
pub struct Candidate<T> { /* private */ }
impl<T> Candidate<T> { pub fn input(&self) -> Option<&T>; pub fn probability(&self) -> f64; pub fn is_none(&self) -> bool; }
pub struct Found<T> { /* private */ }
impl<T> Found<T> { pub fn selected(&self) -> Option<&T>; pub fn candidates(&self) -> &[Candidate<T>]; pub fn into_selected(self) -> Option<T>; }
pub enum Annotated { Decision(Answer), Choice(Option<String>), Score(f64), Tags(Vec<String>), Failed(Failed) }
pub struct NamedAnnotation { /* private */ }
impl NamedAnnotation { pub fn name(&self) -> &str; pub fn value(&self) -> &Annotated; }
pub struct AnnotatedRecord<T> { /* private */ }
impl<T> AnnotatedRecord<T> { pub fn input(&self) -> &T; pub fn values(&self) -> &[NamedAnnotation]; pub fn into_input(self) -> T; }
pub struct Failed { /* private */ }
impl Failed { pub fn kind(&self) -> ErrorKind; pub fn cause(&self) -> FailureCause; }
pub enum FailureCause { MissingAnswer, WrongKind, MissingProbability, InvalidProbability, InvalidDistribution, UnexpectedProbability }

pub struct Kind { /* private */ }
impl Kind { pub fn new(name: &str, description: Option<Description>) -> Result<Self, Error>; pub fn name(&self) -> &str; pub fn description(&self) -> Option<&Description>; }
pub struct RelationRule { /* private */ }
impl RelationRule {
    pub fn one_way(name: &str, source: &str, target: &str) -> Result<Self, Error>;
    pub fn both_ways(name: &str, source: &str, target: &str) -> Result<Self, Error>;
    pub fn reads(self, value: &str) -> Result<Self, Error>;
    pub fn name(&self) -> &str;
}
pub struct Recognize { /* private */ }
impl Recognize { pub fn builder() -> RecognizeBuilder; }
pub struct RecognizeBuilder { /* private */ }
impl RecognizeBuilder {
    pub fn kind(self, value: Kind) -> Result<Self, Error>;
    pub fn relation(self, value: RelationRule) -> Result<Self, Error>;
    pub fn threshold(self, value: f64) -> Result<Self, Error>;
    pub fn relation_threshold(self, value: f64) -> Result<Self, Error>;
    pub fn model(self, value: &str) -> Result<Self, Error>;
    pub fn build(self) -> Result<Recognize, Error>;
}
pub struct Relate { /* private */ }
impl Relate { pub fn builder() -> RelateBuilder; }
pub struct RelateBuilder { /* private */ }
impl RelateBuilder {
    pub fn relation(self, value: RelationRule) -> Result<Self, Error>;
    pub fn threshold(self, value: f64) -> Result<Self, Error>;
    pub fn model(self, value: &str) -> Result<Self, Error>;
    pub fn build(self) -> Result<Relate, Error>;
}
pub struct Entity { /* private */ }
impl Entity { pub fn new(name: &str, kind: &str) -> Result<Self, Error>; pub fn name(&self) -> &str; pub fn kind(&self) -> &str; }
pub struct RecognizedEntity { /* private */ }
impl RecognizedEntity { pub fn name(&self) -> &str; pub fn kind(&self) -> &str; pub fn start(&self) -> usize; pub fn end(&self) -> usize; pub fn strength(&self) -> f64; pub fn byte_range(&self, text: &str) -> Option<std::ops::Range<usize>>; pub fn name_in<'a>(&self, text: &'a str) -> Option<&'a str>; }
pub struct Relation { /* private */ }
impl Relation { pub fn relation(&self) -> &str; pub fn source(&self) -> &RecognizedEntity; pub fn target(&self) -> &RecognizedEntity; pub fn probability(&self) -> f64; }
pub struct Recognized { /* private */ }
impl Recognized { pub fn entities(&self) -> &[RecognizedEntity]; pub fn relations(&self) -> Option<&[Relation]>; }
pub struct Edge { /* private */ }
impl Edge { pub fn relation(&self) -> &str; pub fn source(&self) -> &Entity; pub fn target(&self) -> &Entity; pub fn probability(&self) -> f64; }

pub enum Error { Usage(ErrorDetail), Backend(ErrorDetail), Local(ErrorDetail), Cancelled(ErrorDetail), Deadline(ErrorDetail), Defect(ErrorDetail) }
pub enum ErrorKind { Usage, Backend, Local, Cancelled, Deadline, Defect }
pub struct ErrorDetail { /* private */ }
impl ErrorDetail { pub fn message(&self) -> &str; }
impl Error { pub const fn kind(&self) -> ErrorKind; pub const fn retryable(&self) -> bool; pub const fn detail(&self) -> &ErrorDetail; }
impl std::fmt::Display for Error {}
impl std::fmt::Debug for Error {}
impl std::error::Error for Error {}
```

Every public type is `Debug`. `Engine` is provisionally `Clone + Send + Sync`; `CancelToken` is `Clone + Default + Send + Sync`; `CallOptions` is `Copy + Clone + Default + Send + Sync`. Questions and values are `Clone + PartialEq` when generic members are; `Description`, `Kind`, and `Entity` also are `Eq`. `Answer`, `FailureCause`, `ErrorKind`, `Usage`, and `Counters` are `Copy + Clone + Eq + PartialEq`. Builders, `Error`, and `ErrorDetail` are neither `Clone` nor `Copy`. Formatting is log-safe. Shown enums, including `Error`, are exhaustive and stable for matching.

The crate root exports `choices!` and no module. The macro accepts attributes, visibility, an enum name, and at least one `Variant => "label"` entry, with an optional trailing comma. Duplicate labels fail compilation. The emitted enum and methods are exactly those stated in the inventory.

## Result meanings

`Usage` is only the optional provider token report attached to one `Details`. `Counters` is the broader process total returned by `Engine::usage`; it includes sends from failed calls and retries and has no reset. `requests_sent` in `Details` counts successful-result attempts only. These three counts never share a type.

`DescriptionBuilder::example` is the sole examples method. Each call appends its string to one JSON array in call order. The `examples` member occupies the object's position of the first `example` call, appears once, and is omitted when no example was added. Thus `what("x")?.example("a")?.not_for("y")?.example("b")?.build()?` emits `{"what":"x","examples":["a","b"],"not_for":"y"}`. `field_json("examples", ..)` conflicts with `example` in either order and fails on the call that introduces the conflict. Strings are preserved except required JSON escaping; no call sorts or normalizes them.

`Details::nearest` is `Some` only for score. `Probabilities::YesNo` carries the probability of yes. `Probabilities::Named` preserves declared option, label, or level order. A typed single call maps a failed logical answer to `Error::Backend`; it never maps failure to `None`, `Unsure`, or an empty collection. `Failed::kind()` always returns `ErrorKind::Backend`; `FailureCause` is the exact closed mapping of the six backend-answer failures already in main.

`details` accepts `Question`, `BandedQuestion`, `ChooseQuestion<C>`, or `TagQuestion<C>` and covers `decide`, `choose`, `tag`, and `score`. The other functions add no second details call in 0.1; command `--details` documents remain serialization contracts.

Each choose option or tag label must be the next member of `C::labels()`; a duplicate or wrong order fails on that call, and closing fails when members remain. Loaded binding applies the same exact list check. A question typed with one enum cannot compile at a call expecting another.

`Found::candidates` contains every input in stable order and the synthetic `none` candidate last when enabled. `selected` borrows the chosen original input. `Ranked` owns the original input. `Recognized::relations()` returns `None` when no rule was supplied and `Some(&[])` when rules produced no edge. Recognition offsets count Unicode scalar values; the helpers return `None` for an invalid engine range and never panic. `Relation` repeats complete recognized endpoints. `Edge` repeats complete standalone `Entity` endpoints.

`RecognizeBuilder` accepts zero to twenty uniquely named kinds in insertion order. Zero kinds means `person`, `organization`, and `place`, in that order. Name and relation cuts default to inclusive `0.5`. Rules are uniquely named and preserve insertion order. `RelateBuilder` requires at least one uniquely named rule and defaults its inclusive cut to `0.5`. `relate` accepts at most 255 unique `name` plus `kind` entities, preserves iterator order, and rejects an exact duplicate before an external effect. `RelationRule::both_ways(name, "*", "*")` is the library spelling of bare `--either NAME`; two kinds match a relate-file rule with `"either":true`. Neither builder exposes planner selection, one/many, packing, recognition policy, or a second input set.

## Runtime ownership after acceptance

This ticket proves no runtime behavior. Ticket 0085 owns private-facade tests for process width registration, optional width, retries, retry classification, deadline and cancellation precedence, finite aggregation, bounded streaming, ordering, worker joining on success/error/drop, process counters, and sanitization before values reach the public wrapper.

After the post-0078 reconciliation, 0086 owns public delegation, lazy `default_engine`, failed-init retry, omitted width, environment capture, formatting secrecy, exports, matching, compile fixtures, and package proof. Neither implementation ticket may then change the inventory; contrary evidence returns 0084 to design.

## Compile and package acceptance for 0084

Ticket 0086 extracts the inventory into signature fixtures and records normalized `cargo public-api` output. Compile-pass fixtures cover every declaration, all methods and free functions, a `ChooseQuestion<Team>` returning `Option<Team>`, a `TagQuestion<Topic>` returning `Vec<Topic>`, checked loaded-question binding, both description forms including repeated `example`, question-set insertion, every builder terminal, exhaustive error matching, `Batch`, `Send + Sync`, recognition slicing, and relation endpoints.

Compile-fail fixtures prove a band cannot reach `filter`; unfinished builders, typed choice mismatches, and unbound loaded choices fail; and no derive, async method, reset, public module, connector, planner, callback beyond the interrupt check, mutable result field, public `ErrorDetail` constructor/conversion, or string-based `Error` constructor/conversion exists. Public tuple variants remain constructors.

Package proof compares Cargo metadata with and without default features and rejects every package activated only by `cli`, including `clap`, `csv-core`, and `signal-hook`, but allows `nix`.

0084 acceptance is design-only: the proposed ADR amendment and this ticket agree; rustfmt parses the extracted blocks of 0084 and 0095; every public name has one owner and one exact shape; `wc -m` stays under 30,000; and `git diff --check` passes. No compile check is credited as proof of runtime behavior.

## Dependencies and exclusions

0080/0081 supply behavior; 0076/0077 deadline and width. 0095 is reviewed beside it. 0078 blocks the `Engine` portion. 0086 follows 0085.

Excluded: implementation, dependencies, features, ratchets, publication, C ABI, bindings, live calls, and paid calls. `surfaces` is evidence only.

## Complexity

Contract 2; state and timing 0; reach 2; proof 2; cost of error 1; total 7. Final level: 3.

## Amended 2026-09-24

ADR 0017 section 5 has surfaces configure the engine. `EngineBuilder::from_env` reads the environment like `Engine::from_env`. Setters override. `Engine::from_env()` equals `EngineBuilder::from_env()?.build()`. 0086 tests it. Ian can overturn it.

Package proof runs `cargo check`, `test`, and `package` with `--locked -p thinkthen --no-default-features`.

0078 made `nix` a Unix library dependency.

Amended 2026-09-24: the ADR 0017 amendment of that date renames the width to the throttle. `EngineBuilder::width` above is `EngineBuilder::throttle`, and an omitted width is an omitted throttle. 0086 builds that name.

## Amended 2026-09-25 (ticket 0130)

The optional `polars` feature adds two root names, `PolarsEngine` and the re-exported `polars` crate, only when it is on. The frozen inventory is built with `--no-default-features`, and it does not change. Ian can overturn this.

## Review

- Design review: `sdlc/records/2026-09-24-spine-review-contract.md` rejected, then `2026-09-24-rereview-contract.md` accepted after fixes.
- Code review: `sdlc/records/0084-0095-code-review.md`.
