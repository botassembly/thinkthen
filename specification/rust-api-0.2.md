# Rust API changes for development 0.2

The public API inventory applies these reviewed declaration changes after the historical ticket blocks. These declarations record approved source changes; they do not infer approval from the current exports. The inventory retains its strict comparison and its four hostile changes.

## Authority

- [0550](../sdlc/tickets/0550-binding-defects-from-adversarial-review.md) adds `RequestReaderFailure::Producer`, `RequestDecodeCause` and `Error::request_decode_cause` for typed producer and admission failures. Its usage-counter change retires `Engine::finish_usage` and retains `finish_usage_status`.
- [0551](../sdlc/tickets/0551-one-named-request-constructor.md) adds the shared `Request` constructors, `RequestFunction` parsing and definition/input selection, and component JSON decoders. Its approved corrective constructor `Request::named_from_json` appears in [the input contract](types.md). The sixth [0558 amendment](../sdlc/tickets/0558-cut-rust-source-below-the-ceiling.md) keeps these component decoders through one private helper.
- [0552](../sdlc/tickets/0552-c-library-runs-the-shared-request.md) renames `DoorReply` to `CallReply`. Complete decision, choice, tag and score carriers serialize result/2 through `to_json`. Accepted revision `842ef95437345568f4fb3b344d8ba947612693fe` adds `SourceBudgetExceeded` and `Error::source_budget_exceeded` to project the retained C refusal sentences from typed native causes. Accepted revision `0333414aa3ac33dc1d617f24344a05dad91b1186` retains those projections.
- [0553](../sdlc/tickets/0553-python-r-host-request-residue.md) moves question construction, label rules and nullable columns behind Rust request APIs. Its accepted revision `aae01270d365676386c2b195dccbd51bb584d291` includes `RequestQuestion::inline`, `RequestQuestion::labels`, `RequestColumn`, `RequestColumnValues` and `RequestColumnView` with their declared methods and fields.
- [0554](../sdlc/tickets/0554-engine-methods-build-requests.md) admits native calls through `Request`. Accepted revision `018d71671b851391aa92a5fd29057a61d46fad81` includes `RequestRows`, `Engine::execute_request_rows`, and the original-preserving `CompleteRecord::map_original` and `CompleteFound::map_original` projections. [The record contract](records.md) documents that native row path. These are the existing six row variants referenced by 0558; this delta adds no rank, rank-set, find or relate row variant.
- [0556](../sdlc/tickets/0556-blocking-session-read.md) adds `RequestSession::wait`, `RequestSessionWaitInterest` and `RequestSessionReady` to replace binding polls with native readiness waits.
- [0558 and its six amendments](../sdlc/tickets/0558-cut-rust-source-below-the-ceiling.md) retire the unpublished complete, fallible-record and image wrappers below. The third and fourth amendments permit private production primitives with unchanged ownership tests. The fifth amendment retires `decide_many_complete_with` after its unchanged ownership test moves to existing `RequestRows::Decisions(batch).into_call()`.

[0565](../sdlc/tickets/0565-remove-compatibility-code.md) supersedes the shipped-wrapper keep rule and retires all twelve shipped bulk wrappers. Their behavioral tests use the existing Request row carriers. Exact packed Details and original-serialization assertions use the private primitive called by the Polars frame path, under the accepted private-production exception. The delta retires `try_annotate_with`, `try_rank_with`, `try_find_with` and `relate_complete_with` after their last external production callers retired. Released rank, find and annotation methods retain their production primitives; fallible input and complete relation tests use existing Request carriers.

[0581](../sdlc/tickets/0581-one-result-per-library-call.md) makes each Rust function return one typed `Call<T, R, P>` with `value()`, `probability()`, `results()` and `facts()`. Scalar decide uses `Option<bool>`; unresolved readings use `None`. Filter and annotate collect their existing streams. Rank returns original indexes and find returns an optional original unit. Filter and find require `Clone` to retain originals in both the answer and canonical records. The function-specific call aliases keep these signatures readable. All probability projections read the existing canonical results.

[0582](../sdlc/tickets/0582-one-sql-details-route-per-function.md) adds a separate recovering Request path. `RequestRecoveredRow` retains its original ordinal, canonical successful records or safe error, and the typed logical failure cause when present. Invalid eager input refuses before sending; runtime record failures can remain alongside completed records. Rank, find and relate retain one whole-set invocation. Ordinary Request values and packet shapes retain their existing contracts.

[0612](../sdlc/tickets/0612-answers-in-the-packet.md) adds Rust-owned settled answers to calls and final session packets. `SettledAnswer` fixes each function's value and probability types. `AnswerShape` preserves scalar and collection inputs, and the resolution types retain unresolved readings, failed occurrences and failed annotation members. The opaque original, decision, recognition and relation carriers expose their admitted native values without reading caller evidence again.

[0628](../sdlc/tickets/0628-one-session-queue.md) makes recovery a mode of the shared session queue. `RequestRecoveringSession` delegates ownership to its inner `RequestSession`, whose destructor cancels without joining the worker. The worker retains the shared queue until it settles. The recovering wrapper no longer implements `Drop` itself.

The former `Ranked`, `SetRanked` and `AnnotatedRecord` value carriers are retired. Canonical complete records retain their inputs and fields inside the call. The generated rank schema describes the current complete records and session packets.

### Retired public declarations

```text
fn CompleteDecision::to_scalar_json(&self) -> Result<String, Error>
fn CompleteChoice::to_scalar_json(&self) -> Result<String, Error>
fn CompleteTags::to_scalar_json(&self) -> Result<String, Error>
fn CompleteScore::to_scalar_json(&self) -> Result<String, Error>
RequestCall::Annotate(RequestArguments)
RequestCall::Choose(RequestArguments)
RequestCall::Decide(RequestArguments)
RequestCall::Filter(RequestArguments)
RequestCall::Find(RequestArguments)
RequestCall::Rank(RequestArguments)
RequestCall::Recognize(RequestArguments)
RequestCall::Relate(RequestArguments)
RequestCall::Score(RequestArguments)
RequestCall::Tag(RequestArguments)
RequestOptions::details: bool
const fn CompleteCall::facts(&self) -> &CompleteFacts<'_>
const fn CompleteCall::value(&self) -> &T
enum RequestCall
fn AnnotatedRecord::input(&self) -> &T
fn AnnotatedRecord::into_input(self) -> T
fn AnnotatedRecord::value_json(&self) -> String
fn AnnotatedRecord::values(&self) -> &[NamedAnnotation]
fn BandedQuestion::with_wording_version(self, WordingVersion) -> Result<BandedQuestion, Error>
fn Call::complete(&self) -> Option<CompleteCall<'_, T>>
fn ChooseBuilder::with_wording_version(self, WordingVersion) -> Result<ChooseBuilder<C>, Error>
fn ChooseQuestion::with_wording_version(self, WordingVersion) -> Result<ChooseQuestion<C>, Error>
fn CompleteChoice::legacy_details(&self) -> Result<Details, Error>
fn CompleteDecision::legacy_details(&self) -> Result<Details, Error>
fn CompleteRecord::legacy_details(&self) -> Result<Details, Error>
fn CompleteScore::legacy_details(&self) -> Result<Details, Error>
fn CompleteTags::legacy_details(&self) -> Result<Details, Error>
fn DecideBuilder::with_wording_version(self, WordingVersion) -> Result<DecideBuilder, Error>
fn DoorReply::new(String, Facts, Option<Vec<AttemptObservation>>) -> Result<DoorReply, Error>
fn Engine::annotate<'a, I>(&'a self, &'a QuestionSet, I) -> Batch<'a, AnnotatedRecord<I::Item>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::annotate_complete_with<I, T>(&self, &QuestionSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteAnnotated>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::annotate_records_complete_with<I, T>(&self, &QuestionSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteAnnotated>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::annotate_with<'a, I>(&'a self, &'a QuestionSet, I, CallOptions<'a>) -> Batch<'a, AnnotatedRecord<I::Item>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::choose<C: Choice>(&self, &ChooseQuestion<C>, &str) -> Result<Call<Option<C>>, Error>
fn Engine::choose_complete_with<Q: DetailQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<CompleteChoice>, Error>
fn Engine::choose_dynamic_records_complete_with<I, T>(&self, &RecordChooseQuestion, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteChoice>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::choose_input<C: Choice>(&self, &ChooseQuestion<C>, &QuestionInput) -> Result<Call<Option<C>>, Error>
fn Engine::choose_input_complete_with<Q: DetailQuestion + ?Sized>(&self, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<CompleteChoice>, Error>
fn Engine::choose_input_many<'a, I, C: Choice>(&'a self, &'a ChooseQuestion<C>, I) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::choose_input_many_with<'a, I, C: Choice>(&'a self, &'a ChooseQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::choose_input_with<C: Choice>(&self, &ChooseQuestion<C>, &QuestionInput, CallOptions<'_>) -> Result<Call<Option<C>>, Error>
fn Engine::choose_many<'a, I, C: Choice>(&'a self, &'a ChooseQuestion<C>, I) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::choose_many_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteChoice>>>, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::choose_many_with<'a, I, C: Choice>(&'a self, &'a ChooseQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::choose_records_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteChoice>>>, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::choose_with<C: Choice>(&self, &ChooseQuestion<C>, &str, CallOptions<'_>) -> Result<Call<Option<C>>, Error>
fn Engine::decide<Q: DecisionQuestion + ?Sized>(&self, &Q, &str) -> Result<Call<Answer>, Error>
fn Engine::decide_complete_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<CompleteDecision>, Error>
fn Engine::decide_input<Q: DecisionQuestion + ?Sized>(&self, &Q, &QuestionInput) -> Result<Call<Answer>, Error>
fn Engine::decide_input_complete_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<CompleteDecision>, Error>
fn Engine::decide_input_many<'a, I, Q: DecisionQuestion + ?Sized>(&'a self, &'a Q, I) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::decide_input_many_with<'a, I, Q: DecisionQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::decide_input_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<Answer>, Error>
fn Engine::decide_many<'a, I, Q: DecisionQuestion + ?Sized>(&'a self, &'a Q, I) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::decide_many_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteDecision>>>, Error> where Q: DecisionQuestion + ?Sized, I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::decide_many_with<'a, I, Q: DecisionQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::decide_records_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteDecision>>>, Error> where Q: DecisionQuestion + ?Sized, I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::decide_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<Answer>, Error>
fn Engine::details<Q: DetailQuestion + ?Sized>(&self, &Q, &str) -> Result<Call<Details>, Error>
fn Engine::details_input<Q: DetailQuestion + ?Sized>(&self, &Q, &QuestionInput) -> Result<Call<Details>, Error>
fn Engine::details_input_many<'a, I, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I) -> Batch<'a, Row<I::Item, Details>> where I: IntoIterator + 'a, I::Item: InputEvidence + Serialize
fn Engine::details_input_many_with<'a, I, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Details>> where I: IntoIterator + 'a, I::Item: InputEvidence + Serialize
fn Engine::details_input_with<Q: DetailQuestion + ?Sized>(&self, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<Details>, Error>
fn Engine::details_many<'a, I, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I) -> Batch<'a, Row<I::Item, Details>> where I: IntoIterator + 'a, I::Item: Evidence + Serialize
fn Engine::details_many_with<'a, I, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Details>> where I: IntoIterator + 'a, I::Item: Evidence + Serialize
fn Engine::details_with<Q: DetailQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<Details>, Error>
fn Engine::filter<'a, I>(&'a self, &'a Question, I) -> Batch<'a, I::Item> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::filter_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteFilter>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::filter_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteFilter>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::filter_with<'a, I>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, I::Item> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::find<I>(&self, &Question, I) -> Result<Call<Found<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::find_complete_with<I>(&self, &Question, I, CallOptions<'_>) -> Result<Call<CompleteFound<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::find_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<CompleteFound<T>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::find_with<I>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Found<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::finish_usage(&self) -> UsagePersistence
fn Engine::input_details<Q: DetailQuestion + ?Sized>(&self, InputFunction, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<Details>, Error>
fn Engine::rank<I>(&self, &Question, I) -> Result<Call<Vec<Ranked<I::Item>>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::rank_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::rank_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::rank_set<I>(&self, &RankSet, I) -> Result<Call<Vec<SetRanked<I::Item>>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::rank_set_complete_with<I, T>(&self, &RankSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::rank_set_records_complete_with<I, T>(&self, &RankSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::rank_set_with<I>(&self, &RankSet, I, CallOptions<'_>) -> Result<Call<Vec<SetRanked<I::Item>>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::rank_with<I>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<Ranked<I::Item>>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::recognize(&self, &Recognize, &str) -> Result<Call<Recognized>, Error>
fn Engine::recognize_complete_with(&self, &Recognize, &str, CallOptions<'_>) -> Result<Call<CompleteRecognized>, Error>
fn Engine::recognize_records_complete_with<I, T>(&self, &Recognize, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRecognized>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::recognize_with(&self, &Recognize, &str, CallOptions<'_>) -> Result<Call<Recognized>, Error>
fn Engine::relate<I>(&self, &Relate, I) -> Result<Call<Vec<Edge>>, Error> where I: IntoIterator<Item = Entity>
fn Engine::relate_complete_with<I>(&self, &Relate, I, CallOptions<'_>) -> Result<Call<CompleteRelated>, Error> where I: IntoIterator<Item = Entity>
fn Engine::relate_records_complete_with<I, T>(&self, &Relate, I, CallOptions<'_>) -> Result<Call<CompleteRecord<Vec<T>, CompleteRelated>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::relate_with<I>(&self, &Relate, I, CallOptions<'_>) -> Result<Call<Vec<Edge>>, Error> where I: IntoIterator<Item = Entity>
fn Engine::score(&self, &Question, &str) -> Result<Call<f64>, Error>
fn Engine::score_complete_with(&self, &Question, &str, CallOptions<'_>) -> Result<Call<CompleteScore>, Error>
fn Engine::score_input(&self, &Question, &QuestionInput) -> Result<Call<f64>, Error>
fn Engine::score_input_complete_with(&self, &Question, &QuestionInput, CallOptions<'_>) -> Result<Call<CompleteScore>, Error>
fn Engine::score_input_many<'a, I>(&'a self, &'a Question, I) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::score_input_many_with<'a, I>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::score_input_with(&self, &Question, &QuestionInput, CallOptions<'_>) -> Result<Call<f64>, Error>
fn Engine::score_many<'a, I>(&'a self, &'a Question, I) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::score_many_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteScore>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::score_many_with<'a, I>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::score_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteScore>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::score_with(&self, &Question, &str, CallOptions<'_>) -> Result<Call<f64>, Error>
fn Engine::tag<C: Choice>(&self, &TagQuestion<C>, &str) -> Result<Call<Vec<C>>, Error>
fn Engine::tag_complete_with<Q: DetailQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<CompleteTags>, Error>
fn Engine::tag_many<'a, I, C: Choice>(&'a self, &'a TagQuestion<C>, I) -> Batch<'a, Row<I::Item, Vec<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::tag_many_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteTags>>>, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::tag_many_with<'a, I, C: Choice>(&'a self, &'a TagQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Vec<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::tag_records_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteTags>>>, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::tag_with<C: Choice>(&self, &TagQuestion<C>, &str, CallOptions<'_>) -> Result<Call<Vec<C>>, Error>
fn Engine::try_annotate_records_complete_with<'a, I, T>(&'a self, &'a QuestionSet, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteAnnotated>> where I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_annotate_with<'a, I, T>(&'a self, &'a QuestionSet, I, CallOptions<'a>) -> Batch<'a, AnnotatedRecord<T>> where I: IntoIterator<Item = Result<T, Error>> + 'a, T: Evidence + 'a
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
fn Engine::try_find_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Found<T>>, Error> where I: IntoIterator<Item = Result<T, Error>>, T: Evidence
fn Engine::try_rank_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Engine::try_rank_set_records_complete_with<I, T>(&self, &RankSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Engine::try_rank_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<Ranked<T>>>, Error> where I: IntoIterator<Item = Result<T, Error>>, T: Evidence
fn Engine::try_recognize_records_complete_with<I, T>(&self, &Recognize, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRecognized>>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Engine::try_relate_records_complete_with<I, T>(&self, &Relate, I, CallOptions<'_>) -> Result<Call<CompleteRecord<Vec<T>, CompleteRelated>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Engine::try_score_input_many_with<'a, I, R: InputEvidence + 'a>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, Row<R, f64>> where I: IntoIterator<Item = Result<R, Error>> + 'a
fn Engine::try_score_records_complete_with<'a, I, T>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteScore>> where I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_tag_records_complete_with<'a, Q, I, T>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteTags>> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn LabelBuilder::with_wording_version(self, WordingVersion) -> Result<LabelBuilder, Error>
fn Question::with_wording_version(self, WordingVersion) -> Result<Question, Error>
fn Ranked::index(&self) -> usize
fn Ranked::input(&self) -> &T
fn Ranked::into_input(self) -> T
fn Ranked::probability(&self) -> f64
fn Ranked::row(&self) -> RankedRow<'_>
fn Recognize::with_wording_version(self, WordingVersion) -> Result<Recognize, Error>
fn RecordChooseQuestion::with_wording_version(self, WordingVersion) -> Result<RecordChooseQuestion, Error>
fn Relate::with_wording_version(self, WordingVersion) -> Result<Relate, Error>
fn ScoreBuilder::with_wording_version(self, WordingVersion) -> Result<ScoreBuilder, Error>
fn SetRanked::index(&self) -> usize
fn SetRanked::input(&self) -> &T
fn SetRanked::into_input(self) -> T
fn SetRanked::probability(&self) -> f64
fn SetRanked::question_name(&self) -> &str
fn TagBuilder::with_wording_version(self, WordingVersion) -> Result<TagBuilder<C>, Error>
fn TagQuestion::with_wording_version(self, WordingVersion) -> Result<TagQuestion<C>, Error>
fn Tally::run<T>(&self, impl FnOnce() -> Result<Call<T>, Error>) -> Result<Call<T>, Error>
impl Serialize for CompleteCall
impl Serialize for DoorReply
impl Serialize for RankedRow
struct AnnotatedRecord<T>
struct Call<T>
struct CompleteCall<'a, T>
struct DoorReply
struct Ranked<T>
struct RankedRow<'a>
struct SetRanked<T>
```

### Added public declarations

```text
RequestCall::arguments: RequestArguments
RequestCall::function: RequestFunction
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
RequestRecoveringRead::End
RequestRecoveringRead::Pending
RequestRecoveringRead::Result(RequestRecoveringResult)
RequestRecoveringResult::Observation
RequestRecoveringResult::Observation::function: RequestFunction
RequestRecoveringResult::Observation::value: OwnedRecordObservation
RequestRecoveringResult::Row(RequestRecoveredRow)
RequestRecoveringResult::Terminal(RequestSessionTerminal)
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
RequestSessionWaitInterest::InterruptibleSettlement
RequestSessionWaitInterest::Result
RequestSessionWaitInterest::Settlement
RequestSessionWaitInterest::SettlementArm
SourceBudgetExceeded::Find
SourceBudgetExceeded::Plan
SourceBudgetExceeded::Rank
SourceBudgetExceeded::Relate
const fn Call::probability(&self) -> &P
const fn Call::results(&self) -> &R
const fn Picked::probability(&self) -> f64
const fn RequestCall::new(RequestFunction, RequestArguments) -> RequestCall
const fn RequestColumn::is_empty(&self) -> bool
const fn RequestColumn::len(&self) -> usize
const fn RequestRecoveredRow::cause(&self) -> Option<FailureCause>
const fn RequestRecoveredRow::ordinal(&self) -> usize
enum RequestColumnValues
enum RequestDecodeCause
enum RequestRecoveringRead
enum RequestRecoveringResult
enum RequestRows<'a, T>
enum RequestSessionReady
enum RequestSessionWaitInterest
enum SourceBudgetExceeded
fn CallReply::new(String, Facts, Option<Vec<AttemptObservation>>) -> Result<CallReply, Error>
fn CompleteChoice::probability(&self) -> Option<f64>
fn CompleteDecision::probability(&self) -> Option<f64>
fn CompleteFilter::probability(&self) -> Option<f64>
fn CompleteFound::map_original<U>(self, impl FnMut(T) -> U) -> CompleteFound<U>
fn CompleteFound::probability(&self) -> Option<f64>
fn CompleteRank::probability(&self) -> Option<f64>
fn CompleteRecognized::probability(&self) -> Vec<Option<f64>>
fn CompleteRecord::map_original<U>(self, impl FnOnce(T) -> U) -> CompleteRecord<U, R>
fn CompleteRelated::probability(&self) -> Vec<Option<f64>>
fn CompleteSetRank::probability(&self) -> Option<f64>
fn Engine::annotate<'a, I>(&'a self, &'a QuestionSet, I) -> Result<AnnotationCall<I::Item>, Error> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::annotate_with<'a, I>(&'a self, &'a QuestionSet, I, CallOptions<'a>) -> Result<AnnotationCall<I::Item>, Error> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::choose<C: Choice>(&self, &ChooseQuestion<C>, &str) -> Result<ChoiceCall<C>, Error>
fn Engine::choose_with<C: Choice>(&self, &ChooseQuestion<C>, &str, CallOptions<'_>) -> Result<ChoiceCall<C>, Error>
fn Engine::decide<Q: DecisionQuestion + ?Sized>(&self, &Q, &str) -> Result<DecisionCall, Error>
fn Engine::decide_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<DecisionCall, Error>
fn Engine::execute_request_recovering<'a>(&self, &'a AdmittedRequest, RequestEnvironment<'a>) -> Result<Call<Vec<RequestRecoveredRow>>, Error>
fn Engine::execute_request_recovering_sink<'a>(&self, &'a AdmittedRequest, RequestEnvironment<'a>, &dyn Fn(RequestRecoveredRow)) -> Result<Call<()>, Error>
fn Engine::execute_request_rows<'a, I, T>(&self, &AdmittedRequest, I, CallOptions<'a>) -> Result<RequestRows<'a, T>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::filter<'a, I, T>(&self, &Question, I) -> Result<FilterCall<T>, Error> where I: IntoIterator<Item = T> + 'a, T: InputEvidence + Clone + 'a
fn Engine::filter_with<'a, I, T>(&self, &Question, I, CallOptions<'a>) -> Result<FilterCall<T>, Error> where I: IntoIterator<Item = T> + 'a, T: InputEvidence + Clone + 'a
fn Engine::find<I>(&self, &Question, I) -> Result<FindCall<I::Item>, Error> where I: IntoIterator, I::Item: Evidence + Clone
fn Engine::find_with<I>(&self, &Question, I, CallOptions<'_>) -> Result<FindCall<I::Item>, Error> where I: IntoIterator, I::Item: Evidence + Clone
fn Engine::rank<I>(&self, &Question, I) -> Result<RankCall<I::Item>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::rank_set<I>(&self, &RankSet, I) -> Result<SetRankCall<I::Item>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::rank_set_with<I>(&self, &RankSet, I, CallOptions<'_>) -> Result<SetRankCall<I::Item>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::rank_with<I>(&self, &Question, I, CallOptions<'_>) -> Result<RankCall<I::Item>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::recognize(&self, &Recognize, &str) -> Result<RecognitionCall, Error>
fn Engine::recognize_with(&self, &Recognize, &str, CallOptions<'_>) -> Result<RecognitionCall, Error>
fn Engine::relate<I>(&self, &Relate, I) -> Result<RelationCall, Error> where I: IntoIterator<Item = Entity>
fn Engine::relate_with<I>(&self, &Relate, I, CallOptions<'_>) -> Result<RelationCall, Error> where I: IntoIterator<Item = Entity>
fn Engine::request_recovering_session_with_feed_options(&self, Request, Surface, RequestSessionFeedOptions) -> Result<RequestRecoveringSession, Error>
fn Engine::score(&self, &Question, &str) -> Result<ScoreCall, Error>
fn Engine::score_with(&self, &Question, &str, CallOptions<'_>) -> Result<ScoreCall, Error>
fn Engine::start_request_recovering_session(&self, Request, Surface) -> Result<RequestRecoveringSession, Error>
fn Engine::tag<C: Choice>(&self, &TagQuestion<C>, &str) -> Result<TagCall<C>, Error>
fn Engine::tag_with<C: Choice>(&self, &TagQuestion<C>, &str, CallOptions<'_>) -> Result<TagCall<C>, Error>
fn Error::request_decode_cause(&self) -> Option<RequestDecodeCause>
fn Error::source_budget_exceeded(&self) -> Option<SourceBudgetExceeded>
fn Request::filter(RequestArguments) -> Request
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
fn RequestRecoveredRow::outcome(&self) -> Result<&RequestValue, &Error>
fn RequestRecoveringSession::cancel(&self)
fn RequestRecoveringSession::finish(&self, Option<RequestReaderFailure>) -> Result<(), Error>
fn RequestRecoveringSession::finish_native_reader_error(&self, Error) -> Result<(), Error>
fn RequestRecoveringSession::next_packet(&self, RequestSessionWaitInterest) -> Result<RequestRecoveringRead, Error>
fn RequestRecoveringSession::push_with<E: From<Error>>(&self, impl FnOnce() -> Result<RequestSessionDescriptor, E>) -> Result<RequestSessionPushStatus, E>
fn RequestRecoveringSession::try_push(&self, RequestSessionDescriptor) -> Result<RequestSessionPush, Error>
fn RequestRecoveringSession::try_push_json(&self, &str) -> Result<RequestSessionPushStatus, Error>
fn RequestRecoveringSession::try_push_with<E: From<Error>>(&self, impl FnOnce() -> Result<RequestSessionDescriptor, E>) -> Result<RequestSessionPushStatus, E>
fn RequestRecoveringSession::try_read(&self) -> RequestRecoveringRead
fn RequestRecoveringSession::wait(&self, RequestSessionWaitInterest, Duration) -> RequestSessionReady
fn RequestRows::facts(&self) -> Option<&Facts>
fn RequestSession::next_packet(&self, RequestSessionWaitInterest) -> Result<RequestSessionRead, Error>
fn RequestSession::push_with<E: From<Error>>(&self, impl FnOnce() -> Result<RequestSessionDescriptor, E>) -> Result<RequestSessionPushStatus, E>
fn RequestSession::try_push_with<E: From<Error>>(&self, impl FnOnce() -> Result<RequestSessionDescriptor, E>) -> Result<RequestSessionPushStatus, E>
fn RequestSession::wait(&self, RequestSessionWaitInterest, Duration) -> RequestSessionReady
fn Tally::run<T, R, P>(&self, impl FnOnce() -> Result<Call<T, R, P>, Error>) -> Result<Call<T, R, P>, Error>
impl Display for SourceBudgetExceeded
impl Error for SourceBudgetExceeded
impl From<(Question, RecordReading)> for RequestDefinition
impl From<(Recognize, RecordReading)> for RequestDefinition
impl FromStr for RequestFunction
impl Serialize for Call
impl Serialize for CallReply
impl Serialize for FailureCause
impl Serialize for RequestRecoveredRow
struct Call<T, R, P>
struct CallReply
struct RequestCall
struct RequestColumn
struct RequestColumnView
struct RequestRecoveredRow
struct RequestRecoveringSession
type AnnotationCall<T> = Call<Vec<Vec<NamedAnnotation>>, Vec<CompleteRecord<T, CompleteAnnotated>>>
type ChoiceCall<C> = Call<Option<C>, Vec<CompleteChoice>>
type DecisionCall = Call<Option<bool>, Vec<CompleteDecision>>
type FilterCall<T> = Call<Vec<T>, Vec<CompleteRecord<T, CompleteFilter>>, Vec<Option<f64>>>
type FindCall<T> = Call<Option<T>, CompleteFound<T>>
type RankCall<T> = Call<Vec<usize>, Vec<CompleteRecord<T, CompleteRank>>, Vec<Option<f64>>>
type RecognitionCall = Call<Recognized, Vec<CompleteRecognized>, Vec<Option<f64>>>
type RelationCall = Call<Vec<Edge>, Vec<CompleteRelated>, Vec<Option<f64>>>
type RequestFunction::Err = Error
type ScoreCall = Call<f64, Vec<CompleteScore>>
type SetRankCall<T> = Call<Vec<usize>, Vec<CompleteRecord<T, CompleteSetRank>>, Vec<Option<f64>>>
type TagCall<C> = Call<Vec<C>, Vec<CompleteTags>>
AnswerResolution::Failed
AnswerResolution::Resolved
AnswerResolution::Unresolved
AnswerResolutions::Members(BTreeMap<String, AnswerResolution>)
AnswerResolutions::Occurrences(BTreeMap<usize, AnswerResolution>)
AnswerResolutions::State(AnswerResolution)
AnswerShape::Collection(Vec<T>)
AnswerShape::Scalar(T)
RequestSessionTerminal::answer: Option<SettledAnswer>
RequestSessionTerminal::resolution: AnswerResolution
Settled::index: AnswerShape<Option<usize>>
Settled::pick: AnswerShape<Option<String>>
Settled::probability: AnswerShape<Option<P>>
Settled::resolution: AnswerShape<AnswerResolutions>
Settled::value: AnswerShape<Option<T>>
SettledAnswer::Annotate(Settled<BTreeMap<String, SettledJudgment>, Option<f64>>)
SettledAnswer::Choose(Settled<Option<String>, Option<f64>>)
SettledAnswer::Decide(Settled<SettledDecision, Option<f64>>)
SettledAnswer::Filter(Settled<Vec<SettledOriginal>, Vec<Option<f64>>>)
SettledAnswer::Find(Settled<Option<SettledOriginal>, Option<f64>>)
SettledAnswer::Rank(Settled<Vec<usize>, Vec<Option<f64>>>)
SettledAnswer::Recognize(Settled<SettledRecognition, Vec<Option<f64>>>)
SettledAnswer::Relate(Settled<SettledRelations, Vec<Option<f64>>>)
SettledAnswer::Score(Settled<f64, Option<f64>>)
SettledAnswer::Tag(Settled<Vec<String>, Option<f64>>)
SettledJudgment::Choice(Option<String>)
SettledJudgment::Decision(Option<bool>)
SettledJudgment::Score(f64)
SettledJudgment::Tags(Vec<String>)
const fn CompleteError::resolution(&self) -> &AnswerResolution
const fn SettledDecision::primitive(&self) -> Option<bool>
const fn SettledRecognition::source_value(&self) -> Option<&SourceRecognition>
const fn SettledRecognition::value(&self) -> &Recognized
enum AnswerResolution
enum AnswerResolutions
enum AnswerShape<T>
enum SettledAnswer
enum SettledJudgment
fn Call::answer(&self) -> Option<&SettledAnswer>
fn SettledDecision::authored(&self) -> Option<QuestionContent<'_>>
fn SettledOriginal::original(&self) -> &QuestionInput
fn SettledRelations::source_edges(&self) -> Option<&[SourceRelationEdge]>
fn SettledRelations::value(&self) -> &[Edge]
impl Serialize for AnswerResolution
impl Serialize for AnswerResolutions
impl Serialize for AnswerShape
impl Serialize for Settled
impl Serialize for SettledAnswer
impl Serialize for SettledDecision
impl Serialize for SettledJudgment
impl Serialize for SettledOriginal
impl Serialize for SettledRecognition
impl Serialize for SettledRelations
struct Settled<T, P>
struct SettledDecision
struct SettledOriginal
struct SettledRecognition
struct SettledRelations
```
