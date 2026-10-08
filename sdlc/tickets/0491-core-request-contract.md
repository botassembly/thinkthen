# 0491: Define one typed request contract for all functions

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

### Added public declarations

```text
Request::call: RequestCall
Request::schema: RequestVersion
RequestArguments::input: RequestInput
RequestArguments::options: RequestOptions
RequestArguments::question: RequestQuestion
RequestBatch::Count(usize)
RequestBatch::Named(String)
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
RequestDefinition::Annotate(QuestionSet)
RequestDefinition::Atomic(LoadedQuestion)
RequestDefinition::DecodedSet
RequestDefinition::DecodedSet::annotation: QuestionSet
RequestDefinition::DecodedSet::rank: Option<RankSet>
RequestDefinition::DynamicChoose(RecordChooseQuestion)
RequestDefinition::Find(FindQuestionFile)
RequestDefinition::Rank(Question)
RequestDefinition::RankSet(RankSet)
RequestDefinition::Recognition(Recognize)
RequestDefinition::Recognize(RecognizeQuestionFile)
RequestDefinition::Relate(Relate)
RequestEnvironment::controls: CallOptions<'a>
RequestEnvironment::feed: Option<RequestFeed<'a>>
RequestFraming::Csv
RequestFraming::Document
RequestFraming::Jsonl
RequestFraming::Lines
RequestFraming::Tsv
RequestFunction::Annotate
RequestFunction::Choose
RequestFunction::Decide
RequestFunction::Filter
RequestFunction::Find
RequestFunction::Rank
RequestFunction::Recognize
RequestFunction::Relate
RequestFunction::Score
RequestFunction::Tag
RequestImage::Bytes
RequestImage::Bytes::bytes: Vec<u8>
RequestImage::Bytes::media: ImageMedia
RequestImage::File
RequestImage::File::media: Option<ImageMedia>
RequestImage::File::path: PathBuf
RequestInput::Entities
RequestInput::Entities::items: Vec<RequestItem>
RequestInput::Feed
RequestInput::Feed::framing: RequestFraming
RequestInput::Feed::images: Vec<RequestImage>
RequestInput::Feed::name: String
RequestInput::Feed::reading: ReaderOptions
RequestInput::Json
RequestInput::Json::images: Vec<RequestImage>
RequestInput::Json::value: RawRecord
RequestInput::Records
RequestInput::Records::items: Vec<RequestItem>
RequestInput::Source
RequestInput::Source::source: RequestSource
RequestInput::Text
RequestInput::Text::images: Vec<RequestImage>
RequestInput::Text::text: String
RequestInput::Units
RequestInput::Units::items: Vec<RequestItem>
RequestItem::context: Option<RecordContext>
RequestItem::examples: Option<Vec<RecognitionExample>>
RequestItem::images: Vec<RequestImage>
RequestItem::options: Option<RecordOptions>
RequestItem::original: Option<RequestOriginal>
RequestOptions::attempts: bool
RequestOptions::batch: Option<RequestBatch>
RequestOptions::context: Option<String>
RequestOptions::context_field: Option<String>
RequestOptions::deadline_ms: Option<i64>
RequestOptions::details: bool
RequestOptions::examples: Option<Vec<RecognitionExample>>
RequestOptions::examples_field: Option<String>
RequestOptions::field: Option<Vec<String>>
RequestOptions::files_only: bool
RequestOptions::max_requests_total: Option<u64>
RequestOptions::model: Option<String>
RequestOptions::none: bool
RequestOptions::options_field: Option<String>
RequestOptions::threshold: Option<RequestThreshold>
RequestOptions::top: Option<usize>
RequestOriginal::Json
RequestOriginal::Json::value: RawRecord
RequestOriginal::Text
RequestOriginal::Text::text: String
RequestOutcome::Complete(Call<RequestValue>)
RequestOutcome::Failed
RequestOutcome::Failed::completed: RequestValue
RequestOutcome::Failed::error: Error
RequestQuestion::Definition
RequestQuestion::Definition::value: RequestDefinition
RequestQuestion::File
RequestQuestion::File::path: PathBuf
RequestQuestion::Name
RequestQuestion::Name::name: String
RequestQuestion::Reference
RequestQuestion::Reference::reference: String
RequestQuestion::Text
RequestQuestion::Text::text: String
RequestSource::media: ReaderMedia
RequestSource::paths: Vec<PathBuf>
RequestSource::reading: ReaderOptions
RequestThreshold::Cut(f64)
RequestThreshold::Rule(String)
RequestValue::Annotations(Vec<CompleteRecord<QuestionInput, CompleteAnnotated>>)
RequestValue::Choices(Vec<CompleteRecord<QuestionInput, CompleteChoice>>)
RequestValue::Decisions(Vec<CompleteRecord<QuestionInput, CompleteDecision>>)
RequestValue::Filtered(Vec<CompleteRecord<QuestionInput, CompleteFilter>>)
RequestValue::Found(CompleteFound<QuestionInput>)
RequestValue::Ranked(Vec<CompleteRecord<QuestionInput, CompleteRank>>)
RequestValue::Recognized(Vec<CompleteRecord<QuestionInput, CompleteRecognized>>)
RequestValue::Related(CompleteRecord<Vec<QuestionInput>, CompleteRelated>)
RequestValue::Scores(Vec<CompleteRecord<QuestionInput, CompleteScore>>)
RequestValue::SetRanked(Vec<CompleteRecord<QuestionInput, CompleteSetRank>>)
RequestValue::Tags(Vec<CompleteRecord<QuestionInput, CompleteTags>>)
RequestVersion::V1
const fn AdmittedRequest::request(&self) -> &Request
const fn RequestCall::arguments(&self) -> &RequestArguments
const fn RequestCall::function(&self) -> RequestFunction
enum RequestBatch
enum RequestCall
enum RequestDefinition
enum RequestFraming
enum RequestFunction
enum RequestImage
enum RequestInput
enum RequestOriginal
enum RequestOutcome
enum RequestQuestion
enum RequestThreshold
enum RequestValue
enum RequestVersion
fn AdmittedRequest::resolve_question(&self) -> Result<RequestDefinition, Error>
fn Engine::execute_request<'a>(&self, &'a AdmittedRequest, RequestEnvironment<'a>) -> Result<RequestOutcome, Error>
fn Request::admit(self) -> Result<AdmittedRequest, Error>
fn Request::from_json(&str) -> Result<Request, Error>
fn Request::new(RequestCall) -> Request
fn RequestBatch::native(&self) -> Result<BatchSetting, Error>
fn RequestFeed::new(impl Into<String>, impl Iterator<Item = Result<RequestItem, Error>> + 'a) -> RequestFeed<'a>
impl Default for RequestEnvironment
impl Default for RequestFraming
impl Default for RequestOptions
impl Deserialize<'de> for Request
impl Deserialize<'de> for RequestArguments
impl Deserialize<'de> for RequestBatch
impl Deserialize<'de> for RequestCall
impl Deserialize<'de> for RequestDefinition
impl Deserialize<'de> for RequestFraming
impl Deserialize<'de> for RequestFunction
impl Deserialize<'de> for RequestImage
impl Deserialize<'de> for RequestInput
impl Deserialize<'de> for RequestItem
impl Deserialize<'de> for RequestOptions
impl Deserialize<'de> for RequestOriginal
impl Deserialize<'de> for RequestQuestion
impl Deserialize<'de> for RequestSource
impl Deserialize<'de> for RequestThreshold
impl Deserialize<'de> for RequestVersion
impl From<FindQuestionFile> for RequestDefinition
impl From<LoadedQuestion> for RequestDefinition
impl From<Question> for RequestDefinition
impl From<QuestionSet> for RequestDefinition
impl From<RankSet> for RequestDefinition
impl From<Recognize> for RequestDefinition
impl From<RecognizeQuestionFile> for RequestDefinition
impl From<RecordChooseQuestion> for RequestDefinition
impl From<Relate> for RequestDefinition
impl Serialize for Request
impl Serialize for RequestArguments
impl Serialize for RequestBatch
impl Serialize for RequestCall
impl Serialize for RequestDefinition
impl Serialize for RequestFraming
impl Serialize for RequestFunction
impl Serialize for RequestImage
impl Serialize for RequestInput
impl Serialize for RequestItem
impl Serialize for RequestOptions
impl Serialize for RequestOriginal
impl Serialize for RequestQuestion
impl Serialize for RequestSource
impl Serialize for RequestThreshold
impl Serialize for RequestValue
impl Serialize for RequestVersion
struct AdmittedRequest
struct Request
struct RequestArguments
struct RequestEnvironment<'a>
struct RequestFeed<'a>
struct RequestItem
struct RequestOptions
struct RequestSource
```

## Outcome

Define one versioned Rust Request contract for all ten functions and native execution; generate specification/request.schema.json and parse thinkthen_call through a compatibility translation.

## Evidence

- Reviewed design: [ADR 0125](../planning/adr/0125-one-request-contract-and-native-admission.md) defines the versioned Request, legacy C translation, shared CLI/native admission and offline schema generation. Fresh design review corrected the legacy repeated-key and recognize-record refusal boundary before accepting it.

- Starts from: PM binding architecture asks 2 and 5; libraries/c/src/call.rs and MCP independently parse envelopes.
- Keeps: Frozen 0.1 C symbols and accepted thinkthen_call grammar, named typed methods, current result contract, pure core and one endpoint.
- Changes: Review an ADR for tagged inputs, closed request objects, optional controls and absent/null semantics. Serde/schemars derive the schema from one edge-owned type; native callers use that type without JSON round trips. Claim `crates/thinkthen/src/public/**`, shared admission, `libraries/c/src/call.rs`, schema generation and `specification/request.schema.json`. Depends on0489; coordinate0468 facts before readers settle.
  CLI commands construct the same typed Request and use shared admission before file reads or model calls; no CLI JSON round trip or second validation implementation. Claim `crates/thinkthen/src/cli/**` alongside native/C admission. Preserve valid command behavior and legacy flag spellings. The PM's corrected order starts this ticket alongside 0489's final checks and the binding experiment; integrate 0489 before the final contract landing.
- Proof: Legacy/canonical requests and valid CLI commands agree; CLI and native invalid selectors, unknown input members and invalid media refuse before reads/sends. Count file-reader access and loopback sends on invalid CLI cases. Shared typed cases and committed schema regeneration detect drift.
- Defers: Proxy policy, model routing and replacing public named methods with raw JSON. Size: large shared execution change.
