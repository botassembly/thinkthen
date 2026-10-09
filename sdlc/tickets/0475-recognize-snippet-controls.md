# 0475: Let callers set recognition snippet width

Status: COMPLETE. Fresh ticket review accepted the design; implementation has not started.

Milestone: 0.2

Depends on: 0490
Depends on: 0461
Owner: builder.
Signed: queue owner, 2026-10-08.
Review: accept. Fresh read-only Sol review accepted the design after the signoff correction.

Reviews: revision b6970338e, accept

Reviews: revision 3edc055784c41e389c067c17bbd67a0e0d087145, accept

Landed: 71489a1

## Outcome

Let each recognize question choose the number of context pieces shown on either side of a token or name. Keep six when omitted. The PM's later ruling keeps this ticket to one snippet-width option and drops the optional batch-size control.

## Evidence

- Starts from: PM inbox `2026-10-08-pm-thinkthen-pm-scope-rulings-on-the-tcga-recognize-asks.md`, ask 10, narrowed by ask 12 of the priority-order message; `specification/recognize.md` and `core/recognize/questions.rs` fix six pieces and 40 step-1 questions. `shown` supplies both marked questions and request windows. The later TCGA measurement found no gain from wider context; this option supplies caller control without an accuracy claim.
- Keeps: Existing default question bytes, results, thresholds, wording, cache and replay behavior, text/request-size admission, and one route per engine. Batch grouping is transport policy, not a new semantic answer rule.
- Changes: Admit `snippet_pieces` as an optional saved `recognize` member, native `RecognizeBuilder::snippet_pieces`, CLI `--snippet-pieces`, and equivalent typed declaration/readers already carrying recognize settings. It counts tokenizer pieces on each side, not words or Unicode characters; zero gives only the marked stretch and 6 is the omitted default. Use a checked nonnegative `u32` value on all surfaces; this is a cross-surface representation bound, not a product cap. Refuse negative, fractional, overflow, or platform-unrepresentable values before sending. Retain actual text, encoded request-byte, and profile admission. Apply it to step-1 and step-2 marked questions and their request evidence; keep caller wording intact. Keep the existing batch grouping. Do not infer vendor capacity or select a batch automatically. Update `specification/recognize.md`, `specification/backends.md`, and `specification/settings.md` with spellings, representation bounds, actual byte admission, and errors before surface fanout.
- Proof: Saved fixture or loopback tests pin default request bytes, widened question and evidence windows, beginning/end clipping, zero and upper bounds, plan/live parity, invalid-input zero sends, and cache/replay identity. Changed snippets change generated question bytes and possibly shared evidence. Pin exact keys and request digests for both changed and unchanged windows. Check each existing declaration reader with its contract test, without a separate ticket per language.
- Defers: Batch-size controls are outside this ticket under the later PM ruling. Stage-scoped context and boundary-only recognition belong to separate 0.2 tickets 0478 and 0479; 0.3 is the proxy. Make no accuracy or provider-capacity claim.

### Added public declarations

```text
const fn RecognizeBuilder::snippet_pieces(self, u32) -> RecognizeBuilder
fn Recognize::with_snippet_pieces(self, u32) -> Result<Recognize, Error>
fn RecognitionReading::snippet_pieces(&self) -> u32
RequestOptions::snippet_pieces: Option<u32>
```
