# Rust API changes for development 0.2

The public API inventory applies these reviewed declaration changes after the historical ticket blocks. These declarations record approved source changes; they do not infer approval from the current exports. The inventory retains its strict comparison and its four hostile changes.

## Authority

- [0550](../sdlc/tickets/0550-binding-defects-from-adversarial-review.md) adds `RequestReaderFailure::Producer`, `RequestDecodeCause` and `Error::request_decode_cause` for typed producer and admission failures. Its usage-counter change retires `Engine::finish_usage` and retains `finish_usage_status`.
- [0551](../sdlc/tickets/0551-one-named-request-constructor.md) adds the shared `Request` constructors, `RequestFunction` parsing and definition/input selection, and component JSON decoders. Its approved corrective constructor `Request::named_from_json` appears in [the input contract](types.md). The sixth [0558 amendment](../sdlc/tickets/0558-cut-rust-source-below-the-ceiling.md) keeps these component decoders through one private helper.
- [0552](../sdlc/tickets/0552-c-library-runs-the-shared-request.md) renames `DoorReply` to `CallReply` and the five `legacy_details` methods to `compatibility_details`. It preserves frozen scalar details through `to_unbatched_scalar_json`. Accepted revision `842ef95437345568f4fb3b344d8ba947612693fe` adds `SourceBudgetExceeded` and `Error::source_budget_exceeded` to project the retained C refusal sentences from typed native causes. Accepted revision `0333414aa3ac33dc1d617f24344a05dad91b1186` retains those projections.
- [0553](../sdlc/tickets/0553-python-r-host-request-residue.md) moves question construction, label rules and nullable columns behind Rust request APIs. Its accepted revision `aae01270d365676386c2b195dccbd51bb584d291` includes `RequestQuestion::inline`, `RequestQuestion::labels`, `RequestColumn`, `RequestColumnValues` and `RequestColumnView` with their declared methods and fields.
- [0554](../sdlc/tickets/0554-engine-methods-build-requests.md) admits native calls through `Request`. Accepted revision `018d71671b851391aa92a5fd29057a61d46fad81` includes `RequestRows`, `Engine::execute_request_rows`, and the original-preserving `CompleteRecord::map_original` and `CompleteFound::map_original` projections. [The record contract](records.md) documents that native row path. These are the existing six row variants referenced by 0558; this delta adds no rank, rank-set, find or relate row variant.
- [0556](../sdlc/tickets/0556-blocking-session-read.md) adds `RequestSession::wait`, `RequestSessionWaitInterest` and `RequestSessionReady` to replace binding polls with native readiness waits.
- [0558 and its six amendments](../sdlc/tickets/0558-cut-rust-source-below-the-ceiling.md) retire the unpublished complete, fallible-record and image wrappers below. The third and fourth amendments permit private production primitives with unchanged ownership tests. The fifth amendment retires `decide_many_complete_with` after its unchanged ownership test moves to existing `RequestRows::Decisions(batch).into_call()`.

[0565](../sdlc/tickets/0565-remove-compatibility-code.md) supersedes the shipped-wrapper keep rule and retires all twelve shipped bulk wrappers. Their behavioral tests use the existing Request row carriers. Exact packed Details and original-serialization assertions use the private primitive called by the Polars frame path, under the accepted private-production exception. The delta retires `try_annotate_with`, `try_rank_with`, `try_find_with` and `relate_complete_with` after their last external production callers retired. Released rank, find and annotation methods retain their production primitives; fallible input and complete relation tests use existing Request carriers. The retained `details_many_recoverable_with` method stays hidden and outside the documented inventory under 0359. The sixth amendment keeps the source ceiling unchanged; the declarations below retain the approved source ceiling.

### Retired public declarations

```text
fn BandedQuestion::with_wording_version(self, WordingVersion) -> Result<BandedQuestion, Error>
fn ChooseBuilder::with_wording_version(self, WordingVersion) -> Result<ChooseBuilder<C>, Error>
fn ChooseQuestion::with_wording_version(self, WordingVersion) -> Result<ChooseQuestion<C>, Error>
fn DecideBuilder::with_wording_version(self, WordingVersion) -> Result<DecideBuilder, Error>
fn LabelBuilder::with_wording_version(self, WordingVersion) -> Result<LabelBuilder, Error>
fn Question::with_wording_version(self, WordingVersion) -> Result<Question, Error>
fn Recognize::with_wording_version(self, WordingVersion) -> Result<Recognize, Error>
fn RecordChooseQuestion::with_wording_version(self, WordingVersion) -> Result<RecordChooseQuestion, Error>
fn Relate::with_wording_version(self, WordingVersion) -> Result<Relate, Error>
fn ScoreBuilder::with_wording_version(self, WordingVersion) -> Result<ScoreBuilder, Error>
fn TagBuilder::with_wording_version(self, WordingVersion) -> Result<TagBuilder<C>, Error>
fn TagQuestion::with_wording_version(self, WordingVersion) -> Result<TagQuestion<C>, Error>
fn Engine::relate_complete_with<I>(&self, &Relate, I, CallOptions<'_>) -> Result<Call<CompleteRelated>, Error> where I: IntoIterator<Item = Entity>
fn Engine::try_annotate_with<'a, I, T>(&'a self, &'a QuestionSet, I, CallOptions<'a>) -> Batch<'a, AnnotatedRecord<T>> where I: IntoIterator<Item = Result<T, Error>> + 'a, T: Evidence + 'a
fn Engine::try_find_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Found<T>>, Error> where I: IntoIterator<Item = Result<T, Error>>, T: Evidence
fn Engine::try_rank_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<Ranked<T>>>, Error> where I: IntoIterator<Item = Result<T, Error>>, T: Evidence
fn Engine::decide_many<'a, I, Q: DecisionQuestion + ?Sized>(&'a self, &'a Q, I) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::decide_many_with<'a, I, Q: DecisionQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::details_many<'a, I, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I) -> Batch<'a, Row<I::Item, Details>> where I: IntoIterator + 'a, I::Item: Evidence + Serialize
fn Engine::details_many_with<'a, I, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Details>> where I: IntoIterator + 'a, I::Item: Evidence + Serialize
fn Engine::filter<'a, I>(&'a self, &'a Question, I) -> Batch<'a, I::Item> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::filter_with<'a, I>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, I::Item> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::choose_many<'a, I, C: Choice>(&'a self, &'a ChooseQuestion<C>, I) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::choose_many_with<'a, I, C: Choice>(&'a self, &'a ChooseQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::score_many<'a, I>(&'a self, &'a Question, I) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::score_many_with<'a, I>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::tag_many<'a, I, C: Choice>(&'a self, &'a TagQuestion<C>, I) -> Batch<'a, Row<I::Item, Vec<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::tag_many_with<'a, I, C: Choice>(&'a self, &'a TagQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Vec<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn CompleteChoice::legacy_details(&self) -> Result<Details, Error>
fn CompleteDecision::legacy_details(&self) -> Result<Details, Error>
fn CompleteRecord::legacy_details(&self) -> Result<Details, Error>
fn CompleteScore::legacy_details(&self) -> Result<Details, Error>
fn CompleteTags::legacy_details(&self) -> Result<Details, Error>
fn DoorReply::new(String, Facts, Option<Vec<AttemptObservation>>) -> Result<DoorReply, Error>
fn Engine::annotate_complete_with<I, T>(&self, &QuestionSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteAnnotated>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::annotate_records_complete_with<I, T>(&self, &QuestionSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteAnnotated>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::choose_complete_with<Q: DetailQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<CompleteChoice>, Error>
fn Engine::choose_dynamic_records_complete_with<I, T>(&self, &RecordChooseQuestion, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteChoice>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::choose_input<C: Choice>(&self, &ChooseQuestion<C>, &QuestionInput) -> Result<Call<Option<C>>, Error>
fn Engine::choose_input_complete_with<Q: DetailQuestion + ?Sized>(&self, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<CompleteChoice>, Error>
fn Engine::choose_input_many<'a, I, C: Choice>(&'a self, &'a ChooseQuestion<C>, I) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::choose_input_many_with<'a, I, C: Choice>(&'a self, &'a ChooseQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::choose_input_with<C: Choice>(&self, &ChooseQuestion<C>, &QuestionInput, CallOptions<'_>) -> Result<Call<Option<C>>, Error>
fn Engine::choose_many_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteChoice>>>, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::choose_records_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteChoice>>>, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::decide_complete_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<CompleteDecision>, Error>
fn Engine::decide_input<Q: DecisionQuestion + ?Sized>(&self, &Q, &QuestionInput) -> Result<Call<Answer>, Error>
fn Engine::decide_input_complete_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<CompleteDecision>, Error>
fn Engine::decide_input_many<'a, I, Q: DecisionQuestion + ?Sized>(&'a self, &'a Q, I) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::decide_input_many_with<'a, I, Q: DecisionQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::decide_input_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<Answer>, Error>
fn Engine::decide_many_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteDecision>>>, Error> where Q: DecisionQuestion + ?Sized, I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::decide_records_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteDecision>>>, Error> where Q: DecisionQuestion + ?Sized, I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::details_input<Q: DetailQuestion + ?Sized>(&self, &Q, &QuestionInput) -> Result<Call<Details>, Error>
fn Engine::details_input_many<'a, I, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I) -> Batch<'a, Row<I::Item, Details>> where I: IntoIterator + 'a, I::Item: InputEvidence + Serialize
fn Engine::details_input_many_with<'a, I, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Details>> where I: IntoIterator + 'a, I::Item: InputEvidence + Serialize
fn Engine::details_input_with<Q: DetailQuestion + ?Sized>(&self, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<Details>, Error>
fn Engine::filter_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteFilter>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::filter_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteFilter>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::find_complete_with<I>(&self, &Question, I, CallOptions<'_>) -> Result<Call<CompleteFound<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::find_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<CompleteFound<T>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::finish_usage(&self) -> UsagePersistence
fn Engine::input_details<Q: DetailQuestion + ?Sized>(&self, InputFunction, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<Details>, Error>
fn Engine::rank_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::rank_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::rank_set_complete_with<I, T>(&self, &RankSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::rank_set_records_complete_with<I, T>(&self, &RankSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::recognize_complete_with(&self, &Recognize, &str, CallOptions<'_>) -> Result<Call<CompleteRecognized>, Error>
fn Engine::recognize_records_complete_with<I, T>(&self, &Recognize, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRecognized>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::relate_records_complete_with<I, T>(&self, &Relate, I, CallOptions<'_>) -> Result<Call<CompleteRecord<Vec<T>, CompleteRelated>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::score_complete_with(&self, &Question, &str, CallOptions<'_>) -> Result<Call<CompleteScore>, Error>
fn Engine::score_input(&self, &Question, &QuestionInput) -> Result<Call<f64>, Error>
fn Engine::score_input_complete_with(&self, &Question, &QuestionInput, CallOptions<'_>) -> Result<Call<CompleteScore>, Error>
fn Engine::score_input_many<'a, I>(&'a self, &'a Question, I) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::score_input_many_with<'a, I>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::score_input_with(&self, &Question, &QuestionInput, CallOptions<'_>) -> Result<Call<f64>, Error>
fn Engine::score_many_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteScore>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::score_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteScore>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::tag_complete_with<Q: DetailQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<CompleteTags>, Error>
fn Engine::tag_many_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteTags>>>, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::tag_records_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteTags>>>, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::try_annotate_records_complete_with<'a, I, T>(&'a self, &'a QuestionSet, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteAnnotated>> where I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_choose_dynamic_records_complete_with<'a, I, T>(&'a self, &'a RecordChooseQuestion, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteChoice>> where I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_choose_input_many_with<'a, I, R: InputEvidence + 'a, C: Choice>(&'a self, &'a ChooseQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<R, Option<C>>> where I: IntoIterator<Item = Result<R, Error>> + 'a
fn Engine::try_choose_records_complete_with<'a, Q, I, T>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteChoice>> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_decide_input_many_with<'a, I, R: InputEvidence + 'a, Q: DecisionQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<R, Answer>> where I: IntoIterator<Item = Result<R, Error>> + 'a
fn Engine::try_decide_records_complete_with<'a, Q, I, T>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteDecision>> where Q: DecisionQuestion + ?Sized, I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_details_input_many_with<'a, I, T, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<T, Details>> where I: IntoIterator<Item = Result<T, Error>> + 'a, T: InputEvidence + Serialize + 'a
fn Engine::try_details_many_with<'a, I, T, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<T, Details>> where I: IntoIterator<Item = Result<T, Error>> + 'a, T: Evidence + Serialize + 'a
fn Engine::try_filter_records_complete_with<'a, I, T>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteFilter>> where I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_filter_with<'a, I, T>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, T> where I: IntoIterator<Item = Result<T, Error>> + 'a, T: Evidence + 'a
fn Engine::try_find_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<CompleteFound<T>>, Error> where I: IntoIterator<Item = Result<T, Error>>, T: Evidence
fn Engine::try_find_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<CompleteFound<T>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Engine::try_rank_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Engine::try_rank_set_records_complete_with<I, T>(&self, &RankSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Engine::try_recognize_records_complete_with<I, T>(&self, &Recognize, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRecognized>>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Engine::try_relate_records_complete_with<I, T>(&self, &Relate, I, CallOptions<'_>) -> Result<Call<CompleteRecord<Vec<T>, CompleteRelated>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Engine::try_score_input_many_with<'a, I, R: InputEvidence + 'a>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, Row<R, f64>> where I: IntoIterator<Item = Result<R, Error>> + 'a
fn Engine::try_score_records_complete_with<'a, I, T>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteScore>> where I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_tag_records_complete_with<'a, Q, I, T>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteTags>> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
impl Serialize for DoorReply
struct DoorReply
```

### Added public declarations

```text
RequestColumnValues::Choices(Vec<Option<String>>)
RequestColumnValues::Decisions(Vec<Option<bool>>)
RequestColumnValues::Scores(Vec<Option<f64>>)
RequestColumnValues::Tags(Vec<Vec<String>>)
RequestColumnView::probability: Option<Vec<Option<f64>>>
RequestColumnView::values: RequestColumnValues
RequestDecodeCause::Malformed
RequestDecodeCause::PerItemContext
RequestDecodeCause::UnsupportedDeclaration
RequestReaderFailure::Producer
RequestRows::Annotations(Batch<'a, CompleteRecord<T, CompleteAnnotated>>)
RequestRows::Choices(Batch<'a, CompleteRecord<T, CompleteChoice>>)
RequestRows::Decisions(Batch<'a, CompleteRecord<T, CompleteDecision>>)
RequestRows::Filtered(Batch<'a, CompleteRecord<T, CompleteFilter>>)
RequestRows::Scores(Batch<'a, CompleteRecord<T, CompleteScore>>)
RequestRows::Tags(Batch<'a, CompleteRecord<T, CompleteTags>>)
RequestSessionReady::Cancelled
RequestSessionReady::Closed
RequestSessionReady::End
RequestSessionReady::Input
RequestSessionReady::Result
RequestSessionReady::Timeout
RequestSessionWaitInterest::Input
RequestSessionWaitInterest::Result
RequestSessionWaitInterest::Settlement
SourceBudgetExceeded::Find
SourceBudgetExceeded::Plan
SourceBudgetExceeded::Rank
SourceBudgetExceeded::Relate
const fn RequestColumn::is_empty(&self) -> bool
const fn RequestColumn::len(&self) -> usize
enum RequestColumnValues
enum RequestDecodeCause
enum RequestRows<'a, T>
enum RequestSessionReady
enum RequestSessionWaitInterest
enum SourceBudgetExceeded
fn CallReply::new(String, Facts, Option<Vec<AttemptObservation>>) -> Result<CallReply, Error>
fn CompleteChoice::compatibility_details(&self) -> Result<Details, Error>
fn CompleteChoice::to_unbatched_scalar_json(&self) -> Result<String, Error>
fn CompleteDecision::compatibility_details(&self) -> Result<Details, Error>
fn CompleteDecision::to_unbatched_scalar_json(&self) -> Result<String, Error>
fn CompleteFound::map_original<U>(self, impl FnMut(T) -> U) -> CompleteFound<U>
fn CompleteRecord::compatibility_details(&self) -> Result<Details, Error>
fn CompleteRecord::map_original<U>(self, impl FnOnce(T) -> U) -> CompleteRecord<U, R>
fn CompleteScore::compatibility_details(&self) -> Result<Details, Error>
fn CompleteScore::to_unbatched_scalar_json(&self) -> Result<String, Error>
fn CompleteTags::compatibility_details(&self) -> Result<Details, Error>
fn CompleteTags::to_unbatched_scalar_json(&self) -> Result<String, Error>
fn Engine::execute_request_rows<'a, I, T>(&self, &AdmittedRequest, I, CallOptions<'a>) -> Result<RequestRows<'a, T>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Error::request_decode_cause(&self) -> Option<RequestDecodeCause>
fn Error::source_budget_exceeded(&self) -> Option<SourceBudgetExceeded>
fn Request::for_definition(RequestDefinition, RequestInput, RequestOptions) -> Request
fn Request::for_function(RequestFunction, RequestArguments) -> Request
fn Request::named(&str, RequestArguments) -> Result<Request, Error>
fn Request::named_from_json(&str, &str, &str, &str) -> Result<Request, Error>
fn RequestColumn::new(RequestFunction, Vec<Option<RequestItem>>) -> Result<(RequestColumn, Vec<RequestItem>), Error>
fn RequestColumn::positions(&self) -> &[usize]
fn RequestColumn::view(&self, &RequestValue) -> Result<Option<RequestColumnView>, Error>
fn RequestFunction::for_definition(&RequestDefinition) -> RequestFunction
fn RequestFunction::host_input(self, Vec<RequestItem>, Option<RequestItem>) -> RequestInput
fn RequestInput::from_json(&str) -> Result<RequestInput, Error>
fn RequestItem::from_json(&str) -> Result<RequestItem, Error>
fn RequestQuestion::from_json(&str) -> Result<RequestQuestion, Error>
fn RequestQuestion::inline(RequestFunction, Option<String>, Vec<(String, Description)>) -> Result<RequestQuestion, Error>
fn RequestQuestion::labels(Vec<String>, Vec<(String, Description)>, bool) -> Result<Description, Error>
fn RequestRows::facts(&self) -> Option<&Facts>
fn RequestSession::wait(&self, RequestSessionWaitInterest, Duration) -> RequestSessionReady
impl Display for SourceBudgetExceeded
impl Error for SourceBudgetExceeded
impl FromStr for RequestFunction
impl Serialize for CallReply
struct CallReply
struct RequestColumn
struct RequestColumnView
type RequestFunction::Err = Error
```
