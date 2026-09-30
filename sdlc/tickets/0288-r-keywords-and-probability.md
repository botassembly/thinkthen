# 0288 — R keywords and probability (T6)

Status: Accepted; building on `ticket/0288-r-keywords-and-probability`. Imported from reviewed preparation commit `8bf14799`; accepted ADRs 0105 and 0107 settle the outcome.

## Outcome

`$probability` beside `$value` (NULL on score and tag, pinned); uniform
`threshold =`, `true =`, `false =`, `context =`, `batch =`, `deadline_ms =`;
`evidence` → `input`; the `...` rule (every caught name `usage`; `deadline =`
fails with "deadline was renamed deadline_ms, in milliseconds").

Correction: tt_plan with a judge belongs to F6, not this ticket.

## Prerequisites and proposed files

Prerequisite: T1 and T7; before F6. Proposed file families: `libraries/r/thinkthen/R/{thinkthen.R,extendr-wrappers.R}; libraries/r/thinkthen/src/rust/src/{lib.rs,calls.rs,ffi.rs}; R check/README/ratchets`. Refresh exact nested helpers, package member inventories, nonblank source headroom and current Lanes claims before implementation. No source file is claimed by this preparation draft.

## Smallest meaningful proof

Selected R vector replay pins one reply value/probability, NULL/refusal, rename message and real cap. Count exact accepted loopback request bodies and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only the affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0105, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: R uniform keywords, Call probability, input name and active engine cap.
- Proof: Selected R vector replay pins one reply value/probability, NULL/refusal, rename message and real cap. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

The R question constructor already checked raw file JSON with `tt_question_check` before `jsonlite::fromJSON`; this build kept that order. The native call still gives raw question text to `Question::from_json`. Per-call `batch` and `context` were already implemented, so the existing same-session Max and batch-one facts witness was extended instead of adding an API. The engine remains immutable after selection.

R partially matches `deadline` to a `deadline_ms` formal unless `...` precedes that formal. The first installed test caught this and the final interface now raises the exact migration sentence with `thinkthen_usage`. `I(5000)` also reached the shared JSON settings parser as an array; the R boundary unwraps only a plain numeric `AsIs` scalar before the shared check. The native crossing still checks its numeric type and range. The Unicode recognition test requires an explicit UTF-8 locale in a minimal environment; it passed with `LANG=C.UTF-8` and `LC_ALL=C.UTF-8`.

The Call carrier now exposes a yes probability for decide and the chosen label's probability for choose. NA input and an unsure choice keep NA probability; score and tag keep NULL. The existing native decide payload already held the yes probability. The choose path reads the selected label from the same owned details row. No companion send or pricing arithmetic was added. Shared-core pricing landed at `9159e7c0c` during the build. The R engine constructor explicitly refuses its new engine-only price keys as Usage until a later claim; default unpriced facts are retained.

The retained `facts.R` loopback case now pins values and probabilities, copied facts, original positions, receipt, exact packed request bytes, literal context, Max versus batch one, the active process cap across calls, and zero listener arrivals for invalid keywords, deadline, and a raw question-file duplicate before R map conversion. The old deadline fixtures were converted from seconds to milliseconds. `verbs.R` still checks the six error kinds and typed refusals; distinct hook, interrupt, text, cache-identity, engine secrecy, and recognition ownership cases remain. No scaffold test was deleted. This slice does not close the original SQL/data-frame issue: the Python frame work, Judge-based R `tt_plan` in 0297, other bindings, shared examples, and release qualification remain open.

The R Rust ratchet rose from 2163 to 2215 nonblank lines for core settings validation at the FFI boundary, selected-choice probability, millisecond conversion, and the process cap. The R source/test ratchet rose from 1869 to 1997 for uniform arguments, Call carrier, migration handling, and measured host assertions. Existing `calls`, `ffi`, `.tt_settled`, and `.tt_result` paths absorbed the change; no parallel parser or new dependency was added. A fresh reviewer must check both increases and the public Call shape.

The source after pricing main `9159e7c0c` passed offline Clippy and format. `policy.py` checked 189 resolved packages. The later documentation-only main `94f09d89d` was merged before the final install. The source-matched scratch install completed in `/tmp/tt-r-0288-lib` at package version 0.0.1: `thinkthen.so` SHA-256 `d9f2c615ca7d873882ab66e3d6e5f60fabab8777f0d88942525cf94d95669320`, installed R database SHA-256 `446486b5dace68092afe5c011292459931277372300965897fd66ee7ee1d0096`. That is an installed local artifact, not a tarball or release pass. Focused installed loopback tests passed 144 checks across `facts`, `verbs`, `engine`, `recognize`, `hook`, `text`, `portable_batch_identity`, and `interrupt`, with 106 counted requests. The first installed attempt lacked the user R library holding jsonlite 2.0.0; the final install explicitly supplied it. No provider call, load, churn, all-surface gate, or package qualification ran.
