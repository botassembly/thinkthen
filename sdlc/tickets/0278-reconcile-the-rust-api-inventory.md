# 0278 — Reconcile the frozen Rust API inventory

Status: COMPLETE.

Opened as: 2026-10-11. Fresh High code review accepted `05dc98da`; the reviewed checker and exact proof are integrated. The [issue](../issues/closed/2026-09-29-frozen-rust-api-inventory-lags-the-release-line.md) is closed.

## Outcome and scope

Make `sdlc/scripts/inventory` check the accepted current **no-default-features** public Rust declaration set, without accepting incidental exports. Keep historical 0084, 0095 and 0147 blocks and ADR 0017's builder rename intact. The independently reviewed normative delta below retires the 36 pre-0212 eager signatures and adds their 36 `Call<T>` replacements plus 67 approved additions. The [preparation record](../records/0278-rust-api-inventory-preparation.md) maps every family to accepted authority and current source. This ticket creates no public API, SQL/DataFrame contract, dependency or provider call. The issue closes only after an implementation and its exact checker proof land.

The frozen blocks currently declare 375 items. The delta removes 36 and adds 103, for 442 expected declarations after review. `built()` reports 611 raw items in the warm listing; 169 extra implicit trait impls are allowed by the existing contract prose, leaving exactly 442 checked items. These arithmetic facts are cross-checks, not a rule to derive expected data from the built listing.

## Normative API delta

These two blocks are independent expected declarations. They are canonical checker spellings rather than compilable Rust: the syntax preserves type/variant fields, `const`, generic bounds and return types. Their membership is subject to independent review of the cited accepted decisions. Do not regenerate them from a new extraction when the checker runs. The first block must be wholly present in the pre-delta contract; the second wholly absent from it. A missing/duplicate overlap is a contract-file error, not a reason to silently discard an entry.

### Retired declarations

```text
fn Engine::choose<C: Choice>(&self, &ChooseQuestion<C>, &str) -> Result<Option<C>, Error>
fn Engine::choose_with<C: Choice>(&self, &ChooseQuestion<C>, &str, CallOptions<'_>) -> Result<Option<C>, Error>
fn Engine::decide<Q: DecisionQuestion + ?Sized>(&self, &Q, &str) -> Result<Answer, Error>
fn Engine::decide_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Answer, Error>
fn Engine::details<Q: DetailQuestion + ?Sized>(&self, &Q, &str) -> Result<Details, Error>
fn Engine::details_with<Q: DetailQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Details, Error>
fn Engine::find<I>(&self, &Question, I) -> Result<Found<I::Item>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::find_with<I>(&self, &Question, I, CallOptions<'_>) -> Result<Found<I::Item>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::rank<I>(&self, &Question, I) -> Result<Vec<Ranked<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::rank_with<I>(&self, &Question, I, CallOptions<'_>) -> Result<Vec<Ranked<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::recognize(&self, &Recognize, &str) -> Result<Recognized, Error>
fn Engine::recognize_with(&self, &Recognize, &str, CallOptions<'_>) -> Result<Recognized, Error>
fn Engine::relate<I>(&self, &Relate, I) -> Result<Vec<Edge>, Error> where I: IntoIterator<Item = Entity>
fn Engine::relate_with<I>(&self, &Relate, I, CallOptions<'_>) -> Result<Vec<Edge>, Error> where I: IntoIterator<Item = Entity>
fn Engine::score(&self, &Question, &str) -> Result<f64, Error>
fn Engine::score_with(&self, &Question, &str, CallOptions<'_>) -> Result<f64, Error>
fn Engine::tag<C: Choice>(&self, &TagQuestion<C>, &str) -> Result<Vec<C>, Error>
fn Engine::tag_with<C: Choice>(&self, &TagQuestion<C>, &str, CallOptions<'_>) -> Result<Vec<C>, Error>
fn choose<C: Choice>(&ChooseQuestion<C>, &str) -> Result<Option<C>, Error>
fn choose_with<C: Choice>(&ChooseQuestion<C>, &str, CallOptions<'_>) -> Result<Option<C>, Error>
fn decide<Q: DecisionQuestion + ?Sized>(&Q, &str) -> Result<Answer, Error>
fn decide_with<Q: DecisionQuestion + ?Sized>(&Q, &str, CallOptions<'_>) -> Result<Answer, Error>
fn details<Q: DetailQuestion + ?Sized>(&Q, &str) -> Result<Details, Error>
fn details_with<Q: DetailQuestion + ?Sized>(&Q, &str, CallOptions<'_>) -> Result<Details, Error>
fn find<I>(&Question, I) -> Result<Found<I::Item>, Error> where I: IntoIterator, I::Item: Evidence
fn find_with<I>(&Question, I, CallOptions<'_>) -> Result<Found<I::Item>, Error> where I: IntoIterator, I::Item: Evidence
fn rank<I>(&Question, I) -> Result<Vec<Ranked<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn rank_with<I>(&Question, I, CallOptions<'_>) -> Result<Vec<Ranked<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn recognize(&Recognize, &str) -> Result<Recognized, Error>
fn recognize_with(&Recognize, &str, CallOptions<'_>) -> Result<Recognized, Error>
fn relate<I>(&Relate, I) -> Result<Vec<Edge>, Error> where I: IntoIterator<Item = Entity>
fn relate_with<I>(&Relate, I, CallOptions<'_>) -> Result<Vec<Edge>, Error> where I: IntoIterator<Item = Entity>
fn score(&Question, &str) -> Result<f64, Error>
fn score_with(&Question, &str, CallOptions<'_>) -> Result<f64, Error>
fn tag<C: Choice>(&TagQuestion<C>, &str) -> Result<Vec<C>, Error>
fn tag_with<C: Choice>(&TagQuestion<C>, &str, CallOptions<'_>) -> Result<Vec<C>, Error>
```

### Added declarations

```text
BatchSetting::Max
BatchSetting::Records(NonZeroUsize)
ObservedRow::Annotated(&'a [NamedAnnotation])
ObservedRow::Find(Option<usize>)
ObservedRow::Judgment(&'a Judgment)
ObservedRow::Recognized(&'a Recognized)
ObservedRow::Relations(&'a [Edge])
RecordObservation::Question
RecordObservation::Question::detail: QuestionDetail<'a>
RecordObservation::Question::index: usize
RecordObservation::Question::member: Option<&'a str>
RecordObservation::Question::position: usize
RecordObservation::Question::stage: Option<&'static str>
RecordObservation::Row
RecordObservation::Row::index: usize
RecordObservation::Row::value: ObservedRow<'a>
const fn Call::facts(&self) -> &Facts
const fn Call::value(&self) -> &T
const fn CallOptions::batch(self, BatchSetting) -> CallOptions<'a>
const fn CallOptions::context(self, &'a str) -> CallOptions<'a>
const fn CallOptions::observe(self, &'a (dyn for<'r> Fn(RecordObservation<'r>) + Send + Sync)) -> CallOptions<'a>
const fn Facts::cache_answers(&self) -> u64
const fn Facts::input_tokens(&self) -> Option<u64>
const fn Facts::output_tokens(&self) -> Option<u64>
const fn Facts::records(&self) -> u64
const fn Facts::requests_sent(&self) -> u64
const fn Facts::seconds(&self) -> f64
const fn QuestionDetail::cached(&self) -> bool
const fn QuestionDetail::confidence(&self) -> Option<f64>
const fn QuestionDetail::failed_questions(&self) -> usize
const fn QuestionDetail::failure(&self) -> Option<FailureCause>
const fn QuestionDetail::requests_sent(&self) -> u64
const fn QuestionDetail::usage(&self) -> Option<Usage>
enum BatchSetting
enum ObservedRow<'a>
enum RecordObservation<'a>
fn Batch::facts(&self) -> Option<&Facts>
fn Call::into_value(self) -> T
fn Choice::description(&self) -> Result<Option<Description>, Error>
fn Details::profile_warning(&self) -> Option<(&str, &str)>
fn Engine::choose<C: Choice>(&self, &ChooseQuestion<C>, &str) -> Result<Call<Option<C>>, Error>
fn Engine::choose_many<'a, I, C: Choice>(&'a self, &'a ChooseQuestion<C>, I) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::choose_many_with<'a, I, C: Choice>(&'a self, &'a ChooseQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::choose_with<C: Choice>(&self, &ChooseQuestion<C>, &str, CallOptions<'_>) -> Result<Call<Option<C>>, Error>
fn Engine::decide<Q: DecisionQuestion + ?Sized>(&self, &Q, &str) -> Result<Call<Answer>, Error>
fn Engine::decide_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<Answer>, Error>
fn Engine::details<Q: DetailQuestion + ?Sized>(&self, &Q, &str) -> Result<Call<Details>, Error>
fn Engine::details_many<'a, I, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I) -> Batch<'a, Row<I::Item, Details>> where I: IntoIterator + 'a, I::Item: Evidence + Serialize
fn Engine::details_many_with<'a, I, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Details>> where I: IntoIterator + 'a, I::Item: Evidence + Serialize
fn Engine::details_with<Q: DetailQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<Details>, Error>
fn Engine::find<I>(&self, &Question, I) -> Result<Call<Found<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::find_with<I>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Found<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::rank<I>(&self, &Question, I) -> Result<Call<Vec<Ranked<I::Item>>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::rank_with<I>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<Ranked<I::Item>>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::recognize(&self, &Recognize, &str) -> Result<Call<Recognized>, Error>
fn Engine::recognize_with(&self, &Recognize, &str, CallOptions<'_>) -> Result<Call<Recognized>, Error>
fn Engine::relate<I>(&self, &Relate, I) -> Result<Call<Vec<Edge>>, Error> where I: IntoIterator<Item = Entity>
fn Engine::relate_with<I>(&self, &Relate, I, CallOptions<'_>) -> Result<Call<Vec<Edge>>, Error> where I: IntoIterator<Item = Entity>
fn Engine::score(&self, &Question, &str) -> Result<Call<f64>, Error>
fn Engine::score_many<'a, I>(&'a self, &'a Question, I) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::score_many_with<'a, I>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::score_with(&self, &Question, &str, CallOptions<'_>) -> Result<Call<f64>, Error>
fn Engine::tag<C: Choice>(&self, &TagQuestion<C>, &str) -> Result<Call<Vec<C>>, Error>
fn Engine::tag_many<'a, I, C: Choice>(&'a self, &'a TagQuestion<C>, I) -> Batch<'a, Row<I::Item, Vec<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::tag_many_with<'a, I, C: Choice>(&'a self, &'a TagQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Vec<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn Engine::tag_with<C: Choice>(&self, &TagQuestion<C>, &str, CallOptions<'_>) -> Result<Call<Vec<C>>, Error>
fn EngineBuilder::batch(self, BatchSetting) -> EngineBuilder
fn EngineBuilder::ca_bundle(self, impl AsRef<Path>) -> Result<EngineBuilder, Error>
fn Error::facts(&self) -> Option<&Facts>
fn Facts::model(&self) -> Option<&str>
fn QuestionDetail::model(&self) -> &str
fn QuestionDetail::probabilities(&self) -> Option<&Probabilities>
fn QuestionDetail::question_sha256(&self) -> &str
fn QuestionDetail::requests(&self) -> &[String]
fn QuestionDetail::url(&self) -> &str
fn QuestionDetail::value(&self) -> Option<&Judgment>
fn choose<C: Choice>(&ChooseQuestion<C>, &str) -> Result<Call<Option<C>>, Error>
fn choose_many<'a, I, C: Choice>(&'a ChooseQuestion<C>, I) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn choose_many_with<'a, I, C: Choice>(&'a ChooseQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Option<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn choose_with<C: Choice>(&ChooseQuestion<C>, &str, CallOptions<'_>) -> Result<Call<Option<C>>, Error>
fn decide<Q: DecisionQuestion + ?Sized>(&Q, &str) -> Result<Call<Answer>, Error>
fn decide_with<Q: DecisionQuestion + ?Sized>(&Q, &str, CallOptions<'_>) -> Result<Call<Answer>, Error>
fn details<Q: DetailQuestion + ?Sized>(&Q, &str) -> Result<Call<Details>, Error>
fn details_with<Q: DetailQuestion + ?Sized>(&Q, &str, CallOptions<'_>) -> Result<Call<Details>, Error>
fn find<I>(&Question, I) -> Result<Call<Found<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn find_with<I>(&Question, I, CallOptions<'_>) -> Result<Call<Found<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn rank<I>(&Question, I) -> Result<Call<Vec<Ranked<I::Item>>>, Error> where I: IntoIterator, I::Item: Evidence
fn rank_with<I>(&Question, I, CallOptions<'_>) -> Result<Call<Vec<Ranked<I::Item>>>, Error> where I: IntoIterator, I::Item: Evidence
fn recognize(&Recognize, &str) -> Result<Call<Recognized>, Error>
fn recognize_with(&Recognize, &str, CallOptions<'_>) -> Result<Call<Recognized>, Error>
fn relate<I>(&Relate, I) -> Result<Call<Vec<Edge>>, Error> where I: IntoIterator<Item = Entity>
fn relate_with<I>(&Relate, I, CallOptions<'_>) -> Result<Call<Vec<Edge>>, Error> where I: IntoIterator<Item = Entity>
fn score(&Question, &str) -> Result<Call<f64>, Error>
fn score_many<'a, I>(&'a Question, I) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: Evidence
fn score_many_with<'a, I>(&'a Question, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, f64>> where I: IntoIterator + 'a, I::Item: Evidence
fn score_with(&Question, &str, CallOptions<'_>) -> Result<Call<f64>, Error>
fn tag<C: Choice>(&TagQuestion<C>, &str) -> Result<Call<Vec<C>>, Error>
fn tag_many<'a, I, C: Choice>(&'a TagQuestion<C>, I) -> Batch<'a, Row<I::Item, Vec<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn tag_many_with<'a, I, C: Choice>(&'a TagQuestion<C>, I, CallOptions<'a>) -> Batch<'a, Row<I::Item, Vec<C>>> where I: IntoIterator + 'a, I::Item: Evidence
fn tag_with<C: Choice>(&TagQuestion<C>, &str, CallOptions<'_>) -> Result<Call<Vec<C>>, Error>
struct Call<T>
struct Facts
struct QuestionDetail<'a>
```

The 36 retired lines cover Engine and root `choose`, `decide`, `details`, `find`, `rank`, `recognize`, `relate`, `score` and `tag`, each with ordinary and `_with` forms. The corresponding 36 additions retain their exact arguments and bounds. Of the remaining 67, 62 are ticket 0212 and ADR 0089 batch/facts/observer exports; 2 are 0230's dynamic details bridge; 0203, 0211 and 0243 supply one each. The reviewed implementation claim changes the inventory checker only, not public API source.

## Build route after review

1. Claim `sdlc/scripts/inventory`, this ticket and a new build record. Extend `contract()` to read the two static 0278 blocks after its existing 0084/0095/0147 and ADR 0017 steps. For these canonical text blocks, read one nonblank declaration per line, reject duplicate entries within or across blocks, assert every retired entry belongs to the old set and no added entry does, then apply set subtraction/union. Do not modify `built()`'s source extraction or compare expected output to itself. Keep its `--no-default-features` invocation.
2. Preserve the existing four plants: added export, removed item, changed signature, extra trait. Add one small parser-edge proof if the text-block reader or canonicalization would otherwise drop a `RecordObservation::Question::detail` field, `BatchSetting::Records(NonZeroUsize)`, `const fn Facts::records`, or nested `Result<Call<Vec<Ranked<_>>>, Error>` return. This proof must exercise the checker/parser against an independently pinned string, not a copied current listing. No new public test hook.
3. Run the real inventory script using pinned nightly and warm output, require its 442 expected items and four refused plants. Check focused policy, pages, tickets and diff. No full lint, default-feature claim, package or all-port run is implied.

Retain `EXTRA_TRAITS` only as the accepted implicit Debug/Clone/Copy/PartialEq/Eq/StructuralPartialEq allowance; retain `NEVER_CLONE` and the every-public-type-Debug check. If the source differs from any line below after review, investigate the accepted API first. An authorized API change needs a new reviewed delta; an accidental export needs a separately claimed source correction.

## Evidence

- Starts from: Main `0b06da8c6`; the [issue](../issues/closed/2026-09-29-frozen-rust-api-inventory-lags-the-release-line.md), saved selected checkpoint, bounded no-default-features listing, and the accepted tickets/ADRs mapped in the [preparation record](../records/0278-rust-api-inventory-preparation.md).
- Keeps: 0084/0095/0147 inventory history, ADR 0017 rename, implicit-trait rules, mandatory Debug, prohibited Clone/Copy, four mutation plants and the existing public API behavior.
- Changes: One additive reviewed normative delta and a small checker reader that composes it with prior independent declaration sources.
- Proof: 36 retired + 103 added declared items, exact 442-item contract versus actual no-default-features extraction, all four plants refused, parser-edge witness and focused repository checks.
- Defers: Feature-enabled public inventories and any future API additions to their own reviewed scope; this ticket makes no SQL/DataFrame or wrapper runtime claim.

## What preparation taught us

A raw set difference overcounts the defect because the checker intentionally permits 169 implicit trait impls. The accepted 0212 transition accounts for every missing signature and most additions, but later 0230, 0203, 0211 and 0243 decisions account for five otherwise easy-to-miss exports. The checker invocation is `--no-default-features`; an earlier intake called it default features. The exact source and decisions, not the current extraction alone, determined the delta.

## What the build taught us

The historical contract composes with the reviewed text delta without changing the existing Rust declaration parser or built-listing canonicalization. Its 375 earlier entries become 442, and the pinned no-default-features extraction now reports zero differences while refusing all four existing mutations. A short literal witness protects tuple variant, observer field, `const fn` and nested `Call` spellings; separate malformed-delta cases prove duplicate, overlap and historical membership refusals. No test was deleted or consolidated because the old four plants cover distinct export, removal, signature and trait regressions. The preparation correctly found the 169 policy-allowed implicit trait impls; no additional source-authority mismatch appeared. The [build record](../records/0278-rust-api-inventory-build.md) gives commands and limits. Fresh High code review accepted `05dc98da`; coordinator integration closes the checker issue.
