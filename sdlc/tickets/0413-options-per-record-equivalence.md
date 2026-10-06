# 0413: Supply per-record candidate options everywhere

Status: in progress. Native implementation in lane0 on ticket/0443-native-complete-results; host adoption and final landing checks remain open.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

Choose batches across every SDK, keyed SQL and frames carry independent ordered candidates and descriptions for each original record.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Define identity/order/description/refusal semantics before code; reuse CLI options projection and preserve fixed-question calls. Dynamic tag/annotate options remain their separately scoped issue.
- Proof: Two different shortlists, descriptions, duplicate/missing labels, missing pointer, none and excluded candidates pin wire bytes, originals and zero-send invalid/replay cases.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0407 owns per-record context; family tickets adopt shared carriers.

## Native work in progress

Lane0 adds `RecordInput<T>`, validated ordered `RecordOptions` and `RecordOption`, reusing LabelBuilder and Record::choices. `Description::from_json` admits the existing string/object/array/null grammar through the shared ordered parser. Projected options now retain object/array/null descriptions as authored. Null option descriptions mean absent; shortlists replace rather than merge. Public admission tests pin order, descriptions, invalid candidates and withheld Debug. Execution routes and complete-result integration remain open. The unpublished C draft will receive optional choices and text-only optional context once, preserving released layouts without an extra draft record_v2 API.

### Added public declarations

```text
struct RecordOption
RecordOption::name: String
RecordOption::description: Option<Description>
struct RecordOptions
fn RecordOptions::new(Vec<RecordOption>) -> Result<RecordOptions, Error>
fn RecordOptions::project(&str, &str) -> Result<RecordOptions, Error>
fn RecordOptions::options(&self) -> &[RecordOption]
fn Description::from_json(&str) -> Result<Description, Error>
struct RecordChooseQuestion
fn Question::choose_records(&str) -> Result<RecordChooseQuestion, Error>
fn RecordChooseQuestion::from_json(&str) -> Result<RecordChooseQuestion, Error>
fn RecordChooseQuestion::load(impl AsRef<Path>) -> Result<RecordChooseQuestion, Error>
fn RecordChooseQuestion::cut_at(self, f64) -> Result<RecordChooseQuestion, Error>
fn RecordChooseQuestion::model(self, &str) -> Result<RecordChooseQuestion, Error>
fn RecordChooseQuestion::text(&self) -> QuestionContent<'_>
fn RecordChooseQuestion::threshold(&self) -> Option<ResolvedThreshold>
fn Engine::choose_dynamic_records_complete_with<I, T>(&self, &RecordChooseQuestion, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteChoice>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
```

Native execution now admits a required whole shortlist on every eager original, including later-record zero-send refusal and an empty zero-observation call. Saved choose controls use the existing resolver and batch precedence. Host adoption remains open.
