# 0447: Execute typed image questions and read image files

Status: in progress. Native/CLI hosted slice A reviewed and landing. Local runtime profiles and required host adoption remain open.

Milestone: 0.2
Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Decide, choose and score accept one or more ordered images per question on every supported surface, up to the admitted route limits. An explicit image file is one located item.

## Evidence

- Starts from: Main 399c6c7c7 and the existing SDK plan. PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, ask 1; experiment 0034 design, recorded spike at 8ec4fbffc0dbf8ceaa0fea5607749ff2f683a2a0 and subsequent OpenRouter controls. 0036 reports are pending.
- Keeps: Existing text behavior, typed SDK parity, six errors, cancellation, secrecy, spend limits and zero-send strict replay. Core remains free of I/O; the one Rust engine and native file reader remain shared.
- Changes: Add a typed immutable image/multi-image input and native execution, scalar CLI repeatable --image inputs and explicit whole-file image reader mode. Preserve order and duplicates. Folder files remain separate items; explicit attachment lists form a comparison question. Images carry filename provenance outside evidence/cache identity and no invented text-line ranges.
- Proof: Reuse saved 0034 exchanges and one shared multi-image fixture. Independently assert complete two-image bodies/order/context, duplicate retention, result details/facts, zero-send replay and image-free compatibility. Files tests pin source ordering, absent line positions, relocation equality, malformed media/framing zero sends and retained later-read failure behavior. No accuracy claims.
- Defers: Proxy business logic and screens, other modalities and unmeasured function combinations.

## Dependencies and ownership

0448 supplies route admission before sends. 0426 owns additive C image handles/counts/lifetimes; 0427–0431 own typed public adapters. 0410/0296 own frames/accessor adaptation; 0452 owns SQL images/readers. 0442/0444 own identity/store; 0432 owns complete executed support/refusal cases. Settle shared input/source shape before parallel host edits.

## Design notes

Image input is explicit; ordinary text, paths or BLOBs never become images implicitly. Core receives bytes but opens no file. Refuse image line/window framing and do not bundle a folder into one question. A question’s attachments travel together; splitting to fit a request may partition questions, never discard or separate attachments. Required 0.2 image functions are decide/choose/score. Tag/filter/rank/annotate/find are text-only until admitted function-specific evidence and a reviewed amendment; recognize/relate remain text-only. Every such refusal executes through every public surface with zero sends. Ordinary raw-JSON compatibility doors cannot satisfy typed support. Existing text SourceRecord stays valid; use an additive image location carrier instead of fabricating first_line/last_line.

Start from vendors’ documented limits now. Experiment 0036 is later feedback, not an implementation or release dependency.

### Added public declarations

The additive image input and file-reader APIs keep the existing text methods unchanged.

```text
ImageMedia::Jpeg
ImageMedia::Png
ImageSourceRecord::file: String
ImageSourceRecord::record: ImageInput
InputFunction::Annotate
InputFunction::Choose
InputFunction::Decide
InputFunction::Filter
InputFunction::Find
InputFunction::Rank
InputFunction::Recognize
InputFunction::Relate
InputFunction::Score
InputFunction::Tag
InputReaderOptions::media: ReaderMedia
InputReaderOptions::reading: ReaderOptions
QuestionInput::Images(ImageEvidence)
QuestionInput::Text(String)
ReaderMedia::Image
ReaderMedia::Text
SourceItem::Image(ImageSourceRecord)
SourceItem::Text(SourceRecord<String>)
const MAX_IMAGES: usize
const MAX_IMAGE_BYTES: usize
const fn ImageInput::height(&self) -> u32
const fn ImageInput::media(&self) -> ImageMedia
const fn ImageInput::width(&self) -> u32
const fn ImageMedia::mime(self) -> &'static str
const fn InputFunction::name(self) -> &'static str
enum ImageMedia
enum InputFunction
enum QuestionInput
enum ReaderMedia
enum SourceItem
fn Engine::choose_input<C: Choice>(&self, &ChooseQuestion<C>, &QuestionInput) -> Result<Call<Option<C>>, Error>
fn Engine::choose_input_many<'a, I, C: Choice>(&'a self, &'a ChooseQuestion<C>, I) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::choose_input_many_with<'a, I, C: Choice>(&'a self, &'a ChooseQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::choose_input_with<C: Choice>(&self, &ChooseQuestion<C>, &QuestionInput, CallOptions<'_>) -> Result<Call<Option<C>>, Error>
fn Engine::decide_input<Q: DecisionQuestion + ?Sized>(&self, &Q, &QuestionInput) -> Result<Call<Answer>, Error>
fn Engine::decide_input_many<'a, I, Q: DecisionQuestion + ?Sized>(&'a self, &'a Q, I) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::decide_input_many_with<'a, I, Q: DecisionQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Answer>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::decide_input_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<Answer>, Error>
fn Engine::details_input<Q: DetailQuestion + ?Sized>(&self, &Q, &QuestionInput) -> Result<Call<Details>, Error>
fn Engine::details_input_many<'a, I, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I) -> Batch<'a, Row<I::Item, Details>> where I: IntoIterator + 'a, I::Item: InputEvidence + Serialize
fn Engine::details_input_many_with<'a, I, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Details>> where I: IntoIterator + 'a, I::Item: InputEvidence + Serialize
fn Engine::details_input_with<Q: DetailQuestion + ?Sized>(&self, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<Details>, Error>
fn Engine::input_details<Q: DetailQuestion + ?Sized>(&self, InputFunction, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<Details>, Error>
fn Engine::score_input(&self, &Question, &QuestionInput) -> Result<Call<f64>, Error>
fn Engine::score_input_many<'a, I>(&'a self, &'a Question, I) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::score_input_many_with<'a, I>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: InputEvidence
fn Engine::score_input_with(&self, &Question, &QuestionInput, CallOptions<'_>) -> Result<Call<f64>, Error>
fn Engine::try_choose_input_many_with<'a, I, R: InputEvidence + 'a, C: Choice>(&'a self, &'a ChooseQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<R, Option<C>>> where I: IntoIterator<Item = Result<R, Error>> + 'a
fn Engine::try_decide_input_many_with<'a, I, R: InputEvidence + 'a, Q: DecisionQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<R, Answer>> where I: IntoIterator<Item = Result<R, Error>> + 'a
fn Engine::try_details_input_many_with<'a, I, T, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<T, Details>> where I: IntoIterator<Item = Result<T, Error>> + 'a, T: InputEvidence + Serialize + 'a
fn Engine::try_score_input_many_with<'a, I, R: InputEvidence + 'a>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, Row<R, f64>> where I: IntoIterator<Item = Result<R, Error>> + 'a
fn ImageEvidence::images(&self) -> &[ImageInput]
fn ImageEvidence::new(Option<String>, Vec<ImageInput>) -> Result<ImageEvidence, Error>
fn ImageEvidence::text(&self) -> Option<&str>
fn ImageInput::bytes(&self) -> &[u8]
fn ImageInput::new(ImageMedia, impl Into<Arc<[u8]>>) -> Result<ImageInput, Error>
fn InputEvidence::question_input(&self) -> QuestionInput
fn InputFileReader::new(impl Into<String>, R, InputReaderOptions) -> Result<InputFileReader<R>, Error>
fn InputReaderOptions::validate(self) -> Result<InputReaderOptions, Error>
fn choose_input<C: Choice>(&ChooseQuestion<C>, &QuestionInput) -> Result<Call<Option<C>>, Error>
fn choose_input_with<C: Choice>(&ChooseQuestion<C>, &QuestionInput, CallOptions<'_>) -> Result<Call<Option<C>>, Error>
fn decide_input<Q: DecisionQuestion + ?Sized>(&Q, &QuestionInput) -> Result<Call<Answer>, Error>
fn decide_input_with<Q: DecisionQuestion + ?Sized>(&Q, &QuestionInput, CallOptions<'_>) -> Result<Call<Answer>, Error>
fn details_input<Q: DetailQuestion + ?Sized>(&Q, &QuestionInput) -> Result<Call<Details>, Error>
fn details_input_with<Q: DetailQuestion + ?Sized>(&Q, &QuestionInput, CallOptions<'_>) -> Result<Call<Details>, Error>
fn read_inputs(impl IntoIterator<Item = impl AsRef<Path>>, InputReaderOptions) -> Result<SourceItems, Error>
fn score_input(&Question, &QuestionInput) -> Result<Call<f64>, Error>
fn score_input_with(&Question, &QuestionInput, CallOptions<'_>) -> Result<Call<f64>, Error>
impl Default for InputReaderOptions
impl Default for ReaderMedia
impl Deserialize<'de> for ImageMedia
impl Deserialize<'de> for InputReaderOptions
impl Deserialize<'de> for ReaderMedia
impl InputEvidence for ImageSourceRecord
impl InputEvidence for QuestionInput
impl InputEvidence for SourceItem
impl InputEvidence for T
impl Iterator for InputFileReader
impl Iterator for SourceItems
impl Serialize for ImageEvidence
impl Serialize for ImageInput
impl Serialize for ImageMedia
impl Serialize for ImageSourceRecord
impl Serialize for InputReaderOptions
impl Serialize for QuestionInput
impl Serialize for ReaderMedia
impl Serialize for SourceItem
struct ImageEvidence
struct ImageInput
struct ImageSourceRecord
struct InputFileReader<R: BufRead>
struct InputReaderOptions
struct SourceItems
trait InputEvidence
type InputFileReader::Item = Result<SourceItem, Error>
type SourceItems::Item = Result<SourceItem, Error>
```
