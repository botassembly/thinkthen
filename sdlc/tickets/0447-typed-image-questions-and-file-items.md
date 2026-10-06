# 0447: Execute typed image questions and read image files

Status: ready. Planning only; implementation follows accepted ticket review.

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
