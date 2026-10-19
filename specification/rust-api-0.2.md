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

[0629](../sdlc/tickets/0629-rust-run-call-driver.md) adds `Engine::call` as the synchronous owner of a named call. It consumes `RequestArguments` and `RequestEnvironment`, borrows the engine and stop controls, owns the optional named pull producer, and returns the settled `Call<RequestValue>` or the native error with final facts when execution started. The producer advances on the caller; no separate reader escapes. Admission rejects a missing named producer or a producer supplied for inline or source input before advancing it. Producer failures retain their original typed `Error` as the standard error source. The three question conversions consume their prepared question without parsing JSON: banded decisions retain `LoadedQuestion::Banded`; typed choose and tag retain their underlying question and authored labels in `LoadedQuestion::Question`. The returned request value carries labels rather than the original choice enum. The [decide](../libraries/rust/examples/decide.rs), [choose](../libraries/rust/examples/choose.rs) and [tag](../libraries/rust/examples/tag.rs) examples use this owner and the shared argument builder.

[0643](../sdlc/tickets/0643-database-settings-in-rust.md), accepted at revision `2478857b6`, adds `EngineBuilder::setting` to apply a setting name and raw JSON value through the shared settings parser. The method retains the reviewed settings validation and SQL profile handling.

[0639](../sdlc/tickets/0639-split-cli-and-mcp-crates.md) keeps MCP execution in the existing `Engine::call` family. `CallOptions::retained_attachment_limit` bounds aggregate retained compressed attachment bytes; inline preflight precedes admission clones. `RequestDescriptor` and `RequestFeed::from_descriptors` retain deferred whole-file originals, whole-feed structural admission and eager composition under one attachment budget. Descriptor admission precedes question resolution, source reads and sends; every repeated image occurrence charges the budget. `Error::completed` exposes only the actual completed prefix of a failed call. This explicit access can expose sensitive originals; Debug, Display, complete error serialization and standard error sources omit it. These supported transport operations avoid exposing private engine or domain modules. Ian can overturn these API details.

The same ticket exposes the existing explicit cache maintenance operations for `cache convert`, `cache unused`, `cache prune` and offline status counts. The caller owns the named directory and parses command options. The library keeps store locking, ownership checks, current-time capture, selection rules and failure classification. `CacheCounts`, `CacheConversion`, `CachePruneOptions`, `CachePruneSelection` and `CachePruned` are reexports of the existing store values; they add no second implementation or maintenance capability.

`current_usage_month` and `read_usage` expose the existing count-only store for offline status. The caller owns the explicit directory and selected month. `UsageTotals` projects the existing public `Counters` for that month and all recorded months. Reads retain shared locking, permissions and safe path-specific failure messages and never change persisted state.

The command's final facts use the existing count-only `Facts` request-size getters and `Engine::http_time`. HTTP time uses the native monotonic union of actual HTTP and body-read intervals, so overlapping sends count once. `Engine::usage_failure` projects only a generated filename, or the fact that the directory failed, and a closed category. It exposes no caller path or operating-system error text. The command owns its one-time persistence warning and retains the existing writer-disable behavior.

The annotation command uses `Engine::plan_request` through the same native grouped preview packer. `PlanEstimate::question_requests` exposes the uninterrupted wire count and per-group counts in authored order. `drops_detail` retains the selected adapter's description warning. These observations and the existing size getters let command rendering keep its plan and count lines without exposing the native backend or summary. Added detail storage stays out of generic plan JSON and Debug output.

Recognition and relation command dry runs use the same `Engine::plan_request`. `prepared_requests` borrows native digests and encoded bodies without rebuilding them. Recognition retains the first record's request disclosure and tokenizer, name and relation bounds; aggregate counts still cover every input. Relations retain native entity admission, authored rule order, methods, logical counts and per-rule request counts, including empty input. These explicit body and rule views can expose original evidence and authored text. Their Debug output withholds that content. They add no fields to generic plan serialization. Existing adapter accounting and staged upper-bound caveats still apply. `token_estimate_note` retains the actual adapter caveat for every preview, including empty input, while staying out of generic plan serialization. `Engine::is_built_in` delegates the native resolved-endpoint comparison for the command's existing built-in batching warning, including the default service and alternate-path distinction. `Engine::provider_name`, `model_is_mutable_alias` and `profile_name` borrow the actual built adapter, model and profile observations needed by backend checks, recording warnings and threshold mismatch presentation. They read nothing and send nothing; the command owns warning text, conditions and once-only timing.

`RequestOptions::max_text_bytes` carries the recognition command's explicit positive original-text ceiling into the existing recognition admission. Omission keeps the native ceiling; other verbs refuse the option. Ordinary, streaming and recovering recognition share the same private control and byte comparison. The command keeps its argument parsing and presentation. `instructions` and `entity_definition` replace saved recognition task wording independently; absence retains each saved value. Blank text and use with another verb refuse during header admission. Ordinary execution and preview share the native recognition overlay. Atomic `yes`, `no`, `labels` and `levels` use the existing native question resolver. Supplied meanings replace saved meanings, and supplied lists replace the entire saved list with native order, duplicate and cardinality checks. Absent controls retain saved content; explicit null meanings and descriptions remain authored content. The command owns option spelling and source labels.

`CallOptions::shared_state_groups` selects native shared-state preparation for admitted annotation groups within the existing call family. The CLI backend check uses it to retain the four fixed probes' original evidence state, question order and authored descriptions. Native preparation remains private. The same selection reaches preview and execution, including packing, profile admission, adapter descriptions, decoding and accounting. Other functions refuse before question or input source access. Shared-state groups retain the existing native partial-reply behavior, with usable and failed members in authored order. Every-member-failed replies retain the existing native refusal. HTTP, transport, malformed response, admission, observer and cancellation behavior, observation ordering, worker joins and call facts stay unchanged. Ordinary annotation retains quoted preparation by default. The CLI fixes its probe batch at the existing `BatchSetting::Max` to prepare each small root group as one request; full-group native profile admission refuses instead of splitting an invalid probe. `CompleteAnnotationMember::leader` borrows the retained native choice pick or score level before threshold reading, including tied/below-threshold choices, and returns none for decide, tag or failure; it never infers from probabilities. No C setting, raw request submission or result packet is added.

`EngineBuilder::cache_settings` exposes the captured cache path, enablement, platform-default flag, pruning target and existing source tiers without resolving a route. The CLI environment uses this for cache maintenance and its shared-configuration warning, so unknown backends and irrelevant invalid addresses do not refuse explicit-path maintenance. `resolved_settings` reuses the same cache derivation. Environment cache capture, selection, warning timing and output remain command responsibilities. This operation reads no credentials or answers and creates no directory.

`EngineBuilder::from_config` captures only platform configuration and default cache/state paths. The command owns capture and relevance of request environment settings, applies the effective tier with existing builder setters, and records its own displayed tier labels. `ResolvedSettings::key_variables` exposes only the selected route's existing ordered credential-variable names after backend/address host guards. The command then captures that route's keys once and applies the final credential-in-address check. `from_env` retains its existing eager admission behavior. No private configuration or backend type crosses this boundary.

The command applies its captured request rate with `EngineBuilder::requests_per_minute` and its existing debug retry-delay input with `retry_wait`. These setters retain the shared posting-address pacer, native retry policy and default one-second wait. They introduce no extra environment reads or execution owner.

`EngineBuilder::configuration_shared` retains the existing early permission warning without resolving a route. `EngineBuilder::resolved_settings` projects the already captured configuration, route, key presence, cache selection and usage path for offline status and request-plan presentation. It sends nothing and opens nothing. The view exposes a credential variable name and presence only; address-secret guards run before the view returns, and its Debug representation withholds addresses, model names and local paths. Command rendering stays outside the library. The captured metadata and the existing selected backend retain one source of tier provenance.

`Error::with_source` retains an adapter-owned typed cause through standard error-source access. This removes the library's need to name a command reader's error type. The caller owns that source's disclosure policy; an existing cause is preserved. Ordinary error Debug, Display and complete serialization omit the source; explicit standard source access retains it. The named-call producer wrapper still preserves the original native `Error` as its direct source.

`RequestDefinition::question_sha256` exposes the existing canonical identity used by complete results for atomic, ordered-set, find and recognition questions. Atomic identities retain the resolved threshold and profile; recognition uses the native stage-context normalization. `relation_sha256` requires explicit record or line interpretation, so offline audit accepts both existing relation identities without rewriting saved JSON. A dynamic choose template has no complete question identity before per-record options arrive and returns Usage. These operations read no file and send nothing. The command retains digest mismatch precedence and owns all file mutation.

`ParsedJson::from_json` admits any native ordered JSON shape for read-only consumers without the execution record byte ceiling. `ParsedJson::content` borrows the original content; clones share that parsed value and Debug withholds it. The type has no execution conversion, implicit serialization or owned-content API. Offline commands retain their own object-only admission and selector behavior. `RawRecord` and request construction retain their existing byte admission.

`Error::diagnostic` supplies typed retained causes needed by command diagnostics. `ErrorDiagnostic` projects existing transport, status, storage, bounded context, request range and safe parser facts. `TransportFailure` reexports the existing closed connection subtype. `ProviderFailure` contains only provider identifiers already admitted by the current key/request echo filters and excludes provider bodies and free-text messages. The command keeps its own sentences and option advice. Native errors never become standard error sources through this projection. JSON, profile, threshold and configuration refusals expose closed classifications and numeric parser locations. These classifications omit parser prose, duplicate paths, authored values and credential bytes; configuration sentences retain only the existing safe producer wording.

MCP discovery now reads `request_schema`, `RequestFunction::ALL`, `allows_option` and `images` through the public library. These expose the existing committed schema, canonical function order and discovery predicates without copying schemas or function tables into the command package. Full native admission remains authoritative for option validation.

Offline command analysis shares the existing validated `Probability` and `Threshold` types. Their canonical constructors, decimal/band parser, default, display and serialization retain the native rules. `Threshold::apply` projects the existing classifier to `Option<bool>`; the private outcome enum stays private. `validate_pointer` applies the existing closed pointer grammar before a host opens files. These operations do not parse or copy original JSON through another representation.

Offline command reports inspect `RawRecord::content` through additional borrowed `QuestionContent` operations: member and array access, ordered field and element iteration, exact number and boolean access, and the existing native pointer grammar. The library keeps duplicate, nonfinite and depth validation; command reporting does not receive or copy the private JSON tree or implement another parser. These views retain false, null, numeric distinctions and authored order without reparsing originals.

`RequestSessionFeedOptions::retain_success_on_cancel` defaults to false. Explicit opt-in retains only an otherwise successful native result when cancellation arrives during final settlement; active work still stops. Both ordinary and recovering sessions apply the same policy to inline, source and feed requests. The command opts in to preserve its existing earned whole-set result under SIGINT, drains final output and facts with Settlement interest, and retains its existing signal exit. Rust callers, C adapters and MCP keep dominant cancellation unless they explicitly request this policy. The operation reuses the existing private settlement rule and exposes no scheduler, sink or readiness internals.

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
struct PreparedRequest<'a>
PreparedRequest::digest: &'a str
PreparedRequest::body: &'a [u8]
impl Clone for PreparedRequest<'_>
impl Copy for PreparedRequest<'_>
impl Debug for PreparedRequest<'_>
struct RecognitionPlan
RecognitionPlan::pieces: usize
RecognitionPlan::name_requests_upper_bound: usize
RecognitionPlan::relation_pairs_upper_bound: Option<usize>
impl Clone for RecognitionPlan
impl Copy for RecognitionPlan
impl Debug for RecognitionPlan
impl Eq for RecognitionPlan
impl PartialEq for RecognitionPlan
struct PlannedRelation<'a>
PlannedRelation::name: &'a str
PlannedRelation::source: &'a str
PlannedRelation::target: &'a str
PlannedRelation::reads: &'a str
PlannedRelation::either: bool
PlannedRelation::method: RelationMethod
PlannedRelation::logical_questions: usize
PlannedRelation::request_count: usize
impl Clone for PlannedRelation<'_>
impl Copy for PlannedRelation<'_>
impl Debug for PlannedRelation<'_>
struct RelationPlan<'a>
impl Clone for RelationPlan<'_>
impl Copy for RelationPlan<'_>
impl Debug for RelationPlan<'_>
const fn RelationPlan::entity_count(&self) -> usize
fn RelationPlan::rules(&self) -> impl ExactSizeIterator<Item = PlannedRelation<'_>>
fn PlanEstimate::prepared_requests(&self) -> impl ExactSizeIterator<Item = PreparedRequest<'_>>
const fn PlanEstimate::recognition(&self) -> Option<&RecognitionPlan>
fn PlanEstimate::relations(&self) -> Option<RelationPlan<'_>>
RequestOptions::max_text_bytes: Option<usize>
RequestOptions::yes: Option<Description>
RequestOptions::no: Option<Description>
RequestOptions::labels: Option<Vec<RecordOption>>
RequestOptions::levels: Option<Vec<RecordOption>>
RequestOptions::instructions: Option<String>
RequestOptions::entity_definition: Option<String>
fn EngineBuilder::requests_per_minute(self, u32) -> Result<EngineBuilder, Error>
fn EngineBuilder::retry_wait(self, Duration) -> EngineBuilder
const fn PlanEstimate::largest_request_bytes(&self) -> usize
const fn PlanEstimate::largest_request_estimated_input_tokens(&self) -> u64
const fn PlanEstimate::token_estimate_method(&self) -> &'static str
const fn PlanEstimate::token_estimate_note(&self) -> &'static str
fn EngineBuilder::preload_ca_bundle(self) -> Result<EngineBuilder, Error>
ResolvedSettings::model_is_mutable_alias: bool
ResolvedSettings::is_built_in: bool
fn Engine::is_built_in(&self) -> bool
fn Engine::provider_name(&self) -> &'static str
fn Engine::model_is_mutable_alias(&self) -> bool
fn Engine::profile_name(&self) -> Option<&str>
const fn PlanEstimate::drops_detail(&self) -> bool
fn PlanEstimate::question_requests(&self) -> Option<(usize, &[usize])>
const fn Facts::largest_request_bytes(&self) -> usize
const fn Facts::largest_request_estimated_input_tokens(&self) -> Option<u64>
const fn Facts::token_estimate_method(&self) -> &'static str
fn Engine::http_time(&self) -> Option<Duration>
fn Engine::usage_failure(&self) -> Option<UsageFailure>
struct UsageFailure
UsageFailure::file_name: Option<String>
UsageFailure::category: UsageFailureCategory
impl Clone for UsageFailure
impl Debug for UsageFailure
impl Eq for UsageFailure
impl PartialEq for UsageFailure
enum UsageFailureCategory
UsageFailureCategory::InvalidContents
UsageFailureCategory::UnsafeOrUnreadable
impl Clone for UsageFailureCategory
impl Copy for UsageFailureCategory
impl Debug for UsageFailureCategory
impl Eq for UsageFailureCategory
impl PartialEq for UsageFailureCategory
struct Probability
enum ProbabilityError
ProbabilityError::NotFinite
ProbabilityError::OutOfRange
fn Probability::new(f64) -> Result<Probability, ProbabilityError>
const fn Probability::as_f64(self) -> f64
impl Clone for Probability
impl Copy for Probability
impl Debug for Probability
impl PartialEq for Probability
impl Serialize for Probability
impl From<Probability> for f64
impl Clone for ProbabilityError
impl Copy for ProbabilityError
impl Debug for ProbabilityError
impl PartialEq for ProbabilityError
impl Eq for ProbabilityError
impl Display for ProbabilityError
impl Error for ProbabilityError
struct Threshold
enum ThresholdError
ThresholdError::NotANumber
ThresholdError::NotFinite
ThresholdError::CutOutOfRange
ThresholdError::BandOutOfRange
ThresholdError::BandNotRising
fn Threshold::cut(f64) -> Result<Threshold, ThresholdError>
fn Threshold::band(f64, f64) -> Result<Threshold, ThresholdError>
const fn Threshold::cut_value(self) -> Option<f64>
fn Threshold::apply(self, Probability) -> Option<bool>
impl Clone for Threshold
impl Copy for Threshold
impl Debug for Threshold
impl PartialEq for Threshold
impl Default for Threshold
impl FromStr for Threshold
impl Display for Threshold
impl Serialize for Threshold
type Threshold::Err = ThresholdError
impl Clone for ThresholdError
impl Copy for ThresholdError
impl Debug for ThresholdError
impl PartialEq for ThresholdError
impl Eq for ThresholdError
impl Display for ThresholdError
impl Error for ThresholdError
fn validate_pointer(&str) -> Result<(), Error>
fn EngineBuilder::from_config() -> Result<EngineBuilder, Error>
ResolvedSettings::key_variables: Vec<String>
RequestSessionFeedOptions::retain_success_on_cancel: bool
fn QuestionContent::member(self, &str) -> Option<QuestionContent<'a>>
fn QuestionContent::element(self, usize) -> Option<QuestionContent<'a>>
fn QuestionContent::fields(self) -> Option<impl ExactSizeIterator<Item = (&'a str, QuestionContent<'a>)> + 'a>
fn QuestionContent::elements(self) -> Option<impl ExactSizeIterator<Item = QuestionContent<'a>> + 'a>
fn QuestionContent::number(self) -> Option<&'a Number>
const fn QuestionContent::boolean(self) -> Option<bool>
fn QuestionContent::pointer(self, &str) -> Result<Option<QuestionContent<'a>>, Error>
const fn request_schema() -> &'static str
const RequestFunction::ALL: [RequestFunction; 10]
fn RequestFunction::allows_option(self, &str) -> bool
const fn RequestFunction::images(self) -> bool
fn EngineBuilder::configuration_shared(&self) -> bool
enum RelationIdentityMode
impl Clone for RelationIdentityMode
impl Copy for RelationIdentityMode
impl Debug for RelationIdentityMode
impl Eq for RelationIdentityMode
impl PartialEq for RelationIdentityMode
RelationIdentityMode::Records
RelationIdentityMode::Lines
fn RequestDefinition::question_sha256(&self) -> Result<String, Error>
fn RequestDefinition::relation_sha256(&self, mode: RelationIdentityMode) -> Result<String, Error>
struct ParsedJson
impl Clone for ParsedJson
impl Debug for ParsedJson
fn ParsedJson::from_json(text: &str) -> Result<Self, Error>
fn ParsedJson::content(&self) -> QuestionContent<'_>
enum JsonFailure
impl Clone for JsonFailure
impl Copy for JsonFailure
impl Debug for JsonFailure
impl Eq for JsonFailure
impl PartialEq for JsonFailure
JsonFailure::DuplicateName
JsonFailure::NotFinite
JsonFailure::TooDeep
JsonFailure::Syntax
JsonFailure::Syntax::line: usize
JsonFailure::Syntax::column: usize
enum ConfigurationFailure
impl Clone for ConfigurationFailure
impl Copy for ConfigurationFailure
impl Debug for ConfigurationFailure
impl Eq for ConfigurationFailure
impl PartialEq for ConfigurationFailure
ConfigurationFailure::Unreadable
ConfigurationFailure::Prices
ConfigurationFailure::Grammar
enum ProfileFailure
impl Clone for ProfileFailure
impl Copy for ProfileFailure
impl Debug for ProfileFailure
impl Eq for ProfileFailure
impl PartialEq for ProfileFailure
ProfileFailure::Json(JsonFailure)
ProfileFailure::Object
ProfileFailure::Schema
ProfileFailure::Name
ProfileFailure::UnknownKey
ProfileFailure::Limit(&'static str)
ProfileFailure::NoLimit
ProfileFailure::ImageProfile
enum ErrorDiagnostic<'a>
impl Debug for ErrorDiagnostic
fn Error::diagnostic(&self) -> Option<ErrorDiagnostic<'_>>
ErrorDiagnostic::Json(JsonFailure)
ErrorDiagnostic::RecordJson(JsonFailure)
ErrorDiagnostic::QuestionJson
ErrorDiagnostic::QuestionJson::failure: JsonFailure
ErrorDiagnostic::QuestionJson::from_file: bool
ErrorDiagnostic::Threshold(ThresholdError)
ErrorDiagnostic::Profile
ErrorDiagnostic::Profile::failure: ProfileFailure
ErrorDiagnostic::Profile::from_file: bool
ErrorDiagnostic::Configuration
ErrorDiagnostic::Configuration::failure: ConfigurationFailure
ErrorDiagnostic::Configuration::message: &'a str
ErrorDiagnostic::Transport(TransportFailure)
ErrorDiagnostic::Status(u16)
ErrorDiagnostic::Provider
ErrorDiagnostic::Provider::cause: Box<ErrorDiagnostic<'a>>
ErrorDiagnostic::Provider::provider: ProviderFailure<'a>
ErrorDiagnostic::SendBudget(SendBudgetDenial)
ErrorDiagnostic::EstimatedInput
ErrorDiagnostic::EstimatedInput::reason: &'a EstimatedInputDenial
ErrorDiagnostic::EstimatedInput::images: bool
ErrorDiagnostic::TokenLimit
ErrorDiagnostic::ReplyTooLarge(u64)
ErrorDiagnostic::Reply(String)
ErrorDiagnostic::ReplayMiss(&'a str)
ErrorDiagnostic::StoreAmbiguous
ErrorDiagnostic::StoreHotJournal
ErrorDiagnostic::StoreVersion(&'a str)
ErrorDiagnostic::Entry
ErrorDiagnostic::Entry::name: &'a str
ErrorDiagnostic::Entry::reason: &'a str
ErrorDiagnostic::RecordingStorage
ErrorDiagnostic::RecordingForbidden
ErrorDiagnostic::RecordingPathIsFile
ErrorDiagnostic::DefaultCachePrivate
ErrorDiagnostic::UsageUnreadable(&'a str)
ErrorDiagnostic::CacheEntry
ErrorDiagnostic::Defect(&'static str)
ErrorDiagnostic::Usage(&'static str)
ErrorDiagnostic::ProfileLimit
ErrorDiagnostic::ProfileLimit::profile: &'a str
ErrorDiagnostic::ProfileLimit::dimension: &'static str
ErrorDiagnostic::ProfileLimit::limit: usize
ErrorDiagnostic::ProfileLimit::actual: usize
ErrorDiagnostic::Cancelled
ErrorDiagnostic::Deadline(String)
ErrorDiagnostic::NoKey(&'a str)
ErrorDiagnostic::WidthActive(String)
ErrorDiagnostic::ModelsDiffer(Option<(&'a str, &'a str)>)
ErrorDiagnostic::UsageOverflow
ErrorDiagnostic::RecognitionKinds
ErrorDiagnostic::RecognitionExamples(String)
ErrorDiagnostic::RecognitionLogical
ErrorDiagnostic::TextTooLong
ErrorDiagnostic::TextTooLong::bytes: usize
ErrorDiagnostic::TextTooLong::limit: usize
ErrorDiagnostic::RecognitionRelationLimit(String)
ErrorDiagnostic::Range
ErrorDiagnostic::Range::first: usize
ErrorDiagnostic::Range::last: usize
ErrorDiagnostic::Range::cause: Box<ErrorDiagnostic<'a>>
ErrorDiagnostic::PartialReply
ErrorDiagnostic::PartialReply::first: usize
ErrorDiagnostic::PartialReply::last: usize
ErrorDiagnostic::PartialReply::cause: Option<String>
ErrorDiagnostic::Context
ErrorDiagnostic::Context::initial: bool
ErrorDiagnostic::Context::dimension: &'static str
ErrorDiagnostic::Context::limit: usize
ErrorDiagnostic::Context::actual: usize
ErrorDiagnostic::Context::profile: Option<&'a str>
ErrorDiagnostic::RelationEntityCount(usize)
ErrorDiagnostic::ModelControl
ErrorDiagnostic::ModelBlank
ErrorDiagnostic::Pointer
ErrorDiagnostic::Pointer::option: &'static str
ErrorDiagnostic::Pointer::selector: &'a str
ErrorDiagnostic::Pointer::reason: String
ErrorDiagnostic::Batch
ErrorDiagnostic::AnnotationThreshold
ErrorDiagnostic::QuestionUnreadable(String)
ErrorDiagnostic::QuestionTooLarge
ErrorDiagnostic::QuestionNotUtf8
ErrorDiagnostic::Question
ErrorDiagnostic::Question::message: String
ErrorDiagnostic::Question::from_file: bool
ErrorDiagnostic::QuestionSet(String)
ErrorDiagnostic::Reading(String)
ErrorDiagnostic::Record(String)
ErrorDiagnostic::InvalidUtf8
ErrorDiagnostic::Rendering(String)
ErrorDiagnostic::RecognitionConfiguration
ErrorDiagnostic::RecognitionConfiguration::message: String
ErrorDiagnostic::RecognitionConfiguration::from_file: bool
ErrorDiagnostic::RelationConfiguration
ErrorDiagnostic::RelationConfiguration::message: String
ErrorDiagnostic::RelationConfiguration::from_file: bool
ErrorDiagnostic::EntitySet(String)
struct ProviderFailure<'a>
impl Debug for ProviderFailure
impl Clone for ProviderFailure
impl Copy for ProviderFailure
ProviderFailure::backend: &'static str
ProviderFailure::quota: bool
ProviderFailure::alternate_backend_hint: bool
ProviderFailure::error_type: Option<&'a str>
ProviderFailure::code: Option<&'a str>
enum TransportFailure
TransportFailure::Timeout
TransportFailure::NameLookup
TransportFailure::Refused
TransportFailure::PrematureClose
TransportFailure::Tls
TransportFailure::Other
impl Debug for TransportFailure
impl Clone for TransportFailure
impl Copy for TransportFailure
impl Eq for TransportFailure
impl PartialEq for TransportFailure
fn Error::with_source(self, impl Error + Send + Sync + 'static) -> Error
struct CacheSettings
impl Clone for CacheSettings
impl Debug for CacheSettings
fn CompleteAnnotationMember::leader(&self) -> Option<&str>
fn CallOptions::shared_state_groups(self) -> Self
fn EngineBuilder::cache_settings(&self) -> CacheSettings
CacheSettings::configuration_shared: bool
CacheSettings::cache_enabled: bool
CacheSettings::cache_enabled_source: &'static str
CacheSettings::cache_path: Option<PathBuf>
CacheSettings::cache_path_source: &'static str
CacheSettings::cache_is_platform_default: bool
CacheSettings::cache_target_bytes: u64
CacheSettings::cache_target_source: &'static str
struct ResolvedSettings
impl Debug for ResolvedSettings
impl Clone for ResolvedSettings
fn EngineBuilder::resolved_settings(&self) -> Result<ResolvedSettings, Error>
ResolvedSettings::configuration_path: Option<PathBuf>
ResolvedSettings::configuration_present: bool
ResolvedSettings::configuration_shared: bool
ResolvedSettings::backend_name: Option<String>
ResolvedSettings::url: String
ResolvedSettings::url_source: &'static str
ResolvedSettings::model: String
ResolvedSettings::model_source: &'static str
ResolvedSettings::key_variable: String
ResolvedSettings::api_key_set: bool
ResolvedSettings::cache_enabled: bool
ResolvedSettings::cache_enabled_source: &'static str
ResolvedSettings::cache_path: Option<PathBuf>
ResolvedSettings::cache_path_source: &'static str
ResolvedSettings::cache_is_platform_default: bool
ResolvedSettings::cache_target_bytes: u64
ResolvedSettings::cache_target_source: &'static str
ResolvedSettings::usage_path: Option<PathBuf>
struct UsageTotals
UsageTotals::month: Counters
UsageTotals::total: Counters
impl Debug for UsageTotals
impl Clone for UsageTotals
impl Copy for UsageTotals
impl Eq for UsageTotals
impl PartialEq for UsageTotals
fn current_usage_month() -> String
fn read_usage(&Path, &str) -> Result<UsageTotals, Error>
struct CacheCounts
CacheCounts::answers: u64
CacheCounts::bytes: u64
CacheCounts::old_entries: u64
impl Debug for CacheCounts
impl Default for CacheCounts
struct CacheConversion
CacheConversion::answers: usize
impl Debug for CacheConversion
impl Default for CacheConversion
struct CachePruneOptions
CachePruneOptions::max_size: u64
CachePruneOptions::older_than: Option<Duration>
CachePruneOptions::answered_by_other_than: Option<String>
impl Debug for CachePruneOptions
struct CachePruneSelection
CachePruneSelection::keys: Vec<Vec<u8>>
CachePruneSelection::bytes: u64
CachePruneSelection::kept: u64
CachePruneSelection::kept_bytes: u64
impl Debug for CachePruneSelection
impl Default for CachePruneSelection
struct CachePruned
CachePruned::removed: u64
CachePruned::removed_bytes: u64
CachePruned::remaining: u64
CachePruned::remaining_bytes: u64
impl Debug for CachePruned
impl Default for CachePruned
fn cache_counts(&Path, bool) -> Result<CacheCounts, Error>
fn cache_unused(&Path, &HashSet<String>) -> Result<Vec<String>, Error>
fn convert_cache(&Path) -> Result<CacheConversion, Error>
fn preview_cache_prune(&Path, &CachePruneOptions) -> Result<CachePruneSelection, Error>
fn prune_cache(&Path, &CachePruneOptions) -> Result<CachePruned, Error>
struct RequestDescriptor
RequestDescriptor::item: RequestItem
RequestDescriptor::source: Option<RequestSource>
impl Debug for RequestDescriptor
fn CallOptions::retained_attachment_limit(self, usize) -> Result<CallOptions<'a>, Error>
fn RequestFeed::from_descriptors(impl Into<String>, Vec<RequestDescriptor>) -> RequestFeed<'a>
fn Error::completed(&self) -> Option<&RequestValue>
const fn RequestFunction::name(self) -> &'static str
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
fn Engine::call(&self, &str, RequestArguments, RequestEnvironment<'_>) -> Result<Call<RequestValue>, Error>
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
fn EngineBuilder::setting(self, &str, &str) -> Result<EngineBuilder, Error>
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
impl From<BandedQuestion> for RequestDefinition
impl From<ChooseQuestion<C>> for RequestDefinition
impl From<TagQuestion<C>> for RequestDefinition
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

The command admits the selected certificate bundle with `EngineBuilder::preload_ca_bundle`
before named-cache and alias warnings. The explicit operation reuses the native bundle
loader and privately retains its parsed roots for build; selecting a bundle invalidates
that retained state. Ordinary builders remain lazy. `ResolvedSettings::model_is_mutable_alias`
uses the actual resolved adapter and model so the command can warn before storage opens.
Neither operation exposes certificate bytes, provider bodies, or credentials.

`ResolvedSettings::is_built_in` reuses that same native resolved-backend predicate
without building storage or transport. The command reads it to retain its request-size
warning before profile and certificate admission; no URL rule is copied at the edge.

`EngineBuilder::select_throttle(self, u8) -> Result<EngineBuilder, Error>` selects
the existing fork-aware process throttle immediately, retaining it for build. Selection
persists after a builder drop or a later failure. The same selection succeeds and a
different one retains the typed conflict. Ordinary `throttle` remains lazy. Commands
call this only at their original explicit-jobs admission point; omitted jobs select nothing.

`storage_writable_by_another(&Path) -> bool` is the existing best-effort named-folder
warning signal, reusing native directory metadata ownership/world-write rules. It returns
false for missing paths, files, metadata failures and non-Unix platforms. It is no security
or storage-validity guarantee. The command retains its private-default exemption, record
before replay path selection, and warning order before any storage build.

`ErrorDiagnostic::InputJson { line: usize, column: usize }` preserves a retained native
whole-document syntax cause separately from `RecordJson(JsonFailure)`. Only numeric
location facts are projected; ordinary error shape, kind, facts and source stay unchanged.

`RecordReading::read_document(&self, &[u8]) -> Result<RecordInput<RecordEvidence>, Error>`
uses the native whole-document reader with its already admitted fields, then the existing
composition. Without fields it retains literal text; fields select JSON. Native size,
UTF-8, syntax, context and options admission retain their precedence. It opens nothing
and introduces no execution conversion. Annotation-specific document reading and JSONL
retain their separate contracts.

`CompleteRank::score(&self) -> Option<f64>` borrows the retained native weighted score
for score ranks, separate from final position and raw leader. Decision ranks return None.
Set ranks use their existing `result()` projection. No probability weighting is copied.
