---
flow: build
priority: 221
opens: sdlc/tickets/0221-replay-miss-context.md sdlc/records/0221-replay-miss-preflight.md sdlc/records/0221-design-review.md
---

# 0221: Explain which request missed strict replay

Status: design accepted at `9458b445` by a fresh independent reviewer; the coordinator approved the safe ordinal/count/digest choice. Owner: Codex. This is register 102's diagnostic improvement, not a change to replay identity, answer behavior, or exit status. The [preflight](../records/0221-replay-miss-preflight.md) distinguishes what current main `0685cfaa` already reports from missing context. No runtime implementation or register closure is claimed.

## Outcome

On a strict replay miss, a reader can locate the command source and request that needs a fixture without displaying the question, evidence, API key, backend address or a caller's arbitrary file path. The diagnostic does not identify which digest component changed; repair still compares the named entry with the current request. Keep the existing entry filename digest and its explanation. Keep exit 5, empty answer output, zero network sends, read-only replay folders, and the exact request digest. A cache miss in a write-capable cache is not a replay refusal and stays unchanged.

## Proposed boundary

1. Keep `Failure::ReplayMiss(name)` and `cli/failure/recording.rs` as the common safe cause. Add context only where the command knows it, immediately above that cause; do not make the recorder infer a row or question from a body. Use a closed, literal command/source label (`decide`, `filter`, `rank`, `choose`, `tag`, `score`, `annotate`, `find`, `recognize`, `relate`) and the source form (`one document`, `record N`, `records N to M`, `complete find set of N units`, `recognize stage`, `relate request chunk`) when the producer can prove it. The entry digest remains the repair key. Do not repeat the current `Stopped`/`BatchFailed` record or range when those wrappers already give it. A whole request over several records must never be described as one failed record merely because its range starts there.
2. A one-document ask reports its command and `one document`. A many-record ask uses current `Stopped` position or `BatchFailed` range, including the half that missed after a split. For `choose --options`, say only that options came from the record; do not echo a pointer or choice text. `find` sends one aggregate set, so report `complete find set of N units` at its call boundary and never `record 1` for its request. A `recognize` miss may report a fixed stage only when the producer supplies it: `boundary`, combined `kind and edge`, or `relation`. Its second stage can send kind and edge questions together, so it must not label a request as only one without exact provenance. A chunk ordinal for recognize or relate likewise requires a private producer carrier; never infer it from CLI input or a digest. `relate` must not name a unique entity or relation when one chunk covers several. If a stage/chunk cannot be proved at the stopped boundary, use the command and digest alone rather than guess.
3. `annotate` groups several named questions by `on`; a missed request can belong to more than one member. Use a group ordinal and its number of members, not a fabricated single question name. The original register asks for the question set's name, but the file has named members and no separate set name; its path is caller-controlled. The grammar limits member names to lowercase ASCII letters, digits and underscores, which prevents terminal control bytes but does not prove that a member name is not a credential. The accepted safe context does not print the path or member names. Raw names remain outside this ticket unless a separate secrecy rule is reviewed.
4. This is diagnostic text, so no public Rust error enum, recording schema, request-body, result JSON, retry, cancellation, or transport change follows. Preserve the current stopped-run order and `--facts` placement. Keep fixed labels before the existing digest/cause and avoid a second line that can drift from the stop. The current source has both ordinary one-record paths and grouped batch/recognize/relate paths; a general string attached below the recorder would lose that distinction.

## Small outside-in proof

Reuse the loopback/recording helpers in `crates/thinkthen/tests/backend/{recordings,batching,cache_identity}.rs` and the relevant recognize/relate/annotate existing helpers. Record one known entry, change only its question or batch shape under strict `--replay`, then assert exit 5, no new listener request, empty stdout, the digest-bearing safe diagnostic, and no key/question/evidence in stderr. The small distinct table should cover a one-document ask, an ordinary streaming row, a multi-record request range, one named-set group with more than one member, `find`'s complete set, and one multi-chunk recognize or relate path where the proposed fixed stage/chunk can actually be observed. For a stopped run, assert one stop line with its current record/range before the final `--facts` line; a usage warning, when present, stays between them. Snapshot the replay folder before and after a miss and require identical entries and metadata. Retain the existing exact legacy-entry and read-only-folder tests. A test need not recreate every verb's identical cause text. Do not add a test-only export, transport hook, network run, or full replay campaign.

## Prospective files and stop condition

The smallest likely mutation is `cli/failure/{recording,stopped}.rs` plus an internal context carrier in `cli/failure.rs`, `cli/asking{,/batched}.rs`, `cli/annotate_schedule.rs`, `cli/find.rs`, `cli/recognize.rs`, `cli/relate.rs`, and only the engine facade/chunk path needed for a proved stage. The preflight measures tight parents and identifies existing tests. Exact source claims precede implementation. The safe group-ordinal outcome is settled; do not replace it with raw member or file text.

## Evidence

- Starts from: local register `experiments/284-issue-register/102-replay-miss-does-not-say-why.md`, its experiment 273 report 09, and current main `0685cfaa`.
- Keeps: `ReplayMiss` entry digest, exit 5, zero sends under strict replay, read-only folder handling, current `Stopped`/`BatchFailed` placement, and safe fixed diagnostics.
- Changes: only proven command/source context for a missing entry, with no raw request contents and no false unique-row claim.
- Proof: existing replay listener and exact diagnostic helpers, with one small outside-in table over genuinely different request shapes.
- Defers: raw named annotate members and identifying which digest component changed; cache misses, new replay repair tooling, and broader diagnostic redesign. Register 102's source-location improvement alone does not prove full fixture repair from one diagnostic line.

## What preparation taught us

`Stopped` and `BatchFailed` already solve much of register 102's row/range complaint. The actual gap is source identity where one request has several logical questions or no row number. The old phrase “question set's name” does not match the parsed file's structure: it has named members and no top-level set name. Its member alphabet is safe for a terminal but not a guarantee that names contain no secret. Fresh review accepted group ordinal/member count instead. The build should record which context values were actually needed to locate a fixture and which were redundant with the digest.
