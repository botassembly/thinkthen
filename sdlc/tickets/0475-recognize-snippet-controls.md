# 0475: Let callers set recognition snippet width

Status: OPEN. Fresh ticket review accepted the design; implementation has not started.

Milestone: 0.2

Depends on: 0490
Depends on: 0461
Owner: builder.
Signed: queue owner, 2026-10-08.
Review: accept. Fresh read-only Sol review accepted the design after the signoff correction.

Reviews: revision b6970338e, accept

## Outcome

Let each recognize question choose the number of context pieces shown on either side of a token or name. Keep six when omitted. The PM's later ruling keeps this ticket to one snippet-width option and drops the optional batch-size control.

## Evidence

- Starts from: PM inbox `2026-10-08-pm-thinkthen-pm-scope-rulings-on-the-tcga-recognize-asks.md`, ask 10; `specification/recognize.md` and `core/recognize/questions.rs` fix six pieces and 40 step-1 questions. `shown` supplies both marked questions and request windows; `name_groups` also assumes 40. The demo measured a local improvement with wider context, not a general accuracy guarantee.
- Keeps: Existing default question bytes, results, thresholds, wording, cache and replay behavior, text/request-size admission, and one route per engine. Batch grouping is transport policy, not a new semantic answer rule.
- Changes: Admit `snippet_pieces` as an optional saved `recognize` member, native `RecognizeBuilder::snippet_pieces`, CLI `--snippet-pieces`, and equivalent typed declaration/readers already carrying recognize settings. It counts tokenizer pieces on each side, not words or Unicode characters; zero gives only the marked stretch and 6 is the omitted default. Use a checked nonnegative `u32` value on all surfaces; this is a cross-surface representation bound, not a product cap. Refuse negative, fractional, overflow, or platform-unrepresentable values before sending. Retain actual text, encoded request-byte, and profile admission. Apply it to step-1 and step-2 marked questions and their request evidence; keep caller wording intact. If small, admit positive checked `u32` `step1_batch_pieces` / `--step1-batch-pieces` / native builder with default 40; zero and overflow refuse. Values above 40 still pass existing encoded request-byte and profile admission, not an invented provider cap. Update grouping and step-2 group ownership together. Do not infer vendor capacity or select a batch automatically. Update `specification/recognize.md`, `specification/backends.md`, and `specification/settings.md` with spellings, representation bounds, actual byte admission, and errors before surface fanout.
- Proof: Saved fixture or loopback tests pin default request bytes, widened question and evidence windows, beginning/end clipping, zero and upper bounds, step-2 grouping, plan/live parity, invalid-input zero sends, and cache/replay identity. Changed snippets change generated question bytes and possibly shared evidence. Changing batch size may leave a generated question digest intact, but it changes step-1 or step-2 window evidence when group boundaries move; question cache keys include this shared state and must then change. Pin exact keys and request digests for both changed and unchanged windows. Check each existing declaration reader with its contract test, without a separate ticket per language.
- Defers: Stage-scoped context and boundary-only recognition belong to separate 0.2 tickets 0478 and 0479; 0.3 is the proxy. Make no accuracy or provider-capacity claim. If batch control materially broadens the planner, defer that optional part and record why.
