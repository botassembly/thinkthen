# 0490: Render caller-supplied tagged examples for recognition

Status: COMPLETE.

Milestone: 0.2

Depends on: 0489

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c477a9ac099cfcb43307e8e79c414215eb39bc97, accept

Reviews: revision 2e72414068757e9b3c21ae42b558693f0e44166a, accept

Landed: ffbab8d

## Outcome

Callers supply tagged examples without knowing recognition's piece boundaries or question wording. The PM cleared the evidence and scope conditions on 2026-10-08. Build in 0.2 after per-record context, rendering only answered step-1 questions before recognition controls and binding migration.

## Evidence

- Starts from: TCGA asks 3 and 4 and PM high-priority context ask 3 of 2026-10-08. Experiment 469 owns usefulness evidence; the builder implements software contracts and does not interpret clinical content.
- Keeps: Per-record context from 0489 as an independent input; default requests and keys when examples are absent; all languages and one endpoint. Retrieval remains caller-owned.
- Changes: Review an ADR for `--examples FILE` and `--examples-field POINTER`, shared and per-record precedence, inline bracket notation and JSON Lines containing text plus Unicode-scalar spans and declared kinds. Render only answered step-1 questions through recognition's own piecer. Show current-pass kinds and mark other entities OUT; add no step-2 examples. Refuse undeclared kinds and span edges inside a piece before calls. Show rendered examples and token accounting in `--plan`; include examples in stage cache identities and the shared Request schema. Claim `crates/thinkthen/src/core/recognize/**`, `crates/thinkthen/src/engine/facade/recognize/**`, `crates/thinkthen/src/public/**`, `crates/thinkthen/src/cli/recognize/**`, `specification/recognize.md`, `specification/request.schema.json` and `conformance/**`. Family tickets adopt the schema once. The PM cleared the experiment gate on 2026-10-08; build after 0489 and before the controls.
- Proof: Generic software examples cover both forms, non-ASCII spans, per-record isolation, invalid kind/edge refusal with zero sends, plan rendering, exact requests and offline cache/replay parity through typed interfaces. Do not promise model improvement from software fixtures.
- Defers: Retrieval, model selection, proxy business logic and step-2 examples. The approved incremental scope remains separate from general per-record context.

### Added public declarations

```text
RecognitionExample::Brackets(String)
RecognitionExample::Spans(RecognitionExampleText)
RecognitionExampleEntity::end: usize
RecognitionExampleEntity::kind: String
RecognitionExampleEntity::start: usize
RecognitionExampleText::entities: Vec<RecognitionExampleEntity>
RecognitionExampleText::kinds: Option<Vec<String>>
RecognitionExampleText::text: String
RecordInput::examples: Option<Vec<RecognitionExample>>
enum RecognitionExample
fn Recognize::examples(&self) -> &[RecognitionExample]
fn Recognize::with_examples(self, Vec<RecognitionExample>) -> Result<Recognize, Error>
fn RecordInput::map_original<U>(self, impl FnOnce(T) -> U) -> RecordInput<U>
fn RecordReading::with_examples_field(self, &str) -> Result<RecordReading, Error>
impl Deserialize<'de> for RecognitionExample
impl Deserialize<'de> for RecognitionExampleEntity
impl Deserialize<'de> for RecognitionExampleText
impl Serialize for RecognitionExample
impl Serialize for RecognitionExampleEntity
impl Serialize for RecognitionExampleText
struct RecognitionExampleEntity
struct RecognitionExampleText
```

The native example carrier lands independently before 0491 consumes it. Ticket 0491 owns its Request/schema projection; surface migration waits for both contracts.
