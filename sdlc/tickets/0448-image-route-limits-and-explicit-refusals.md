# 0448: Enforce documented image limits and refuse dropped images

Status: in progress. Native/CLI implementation in ticket/0447-native-images awaits coordinator dependency/code review and landing. Unknown local physical-batch/projector profiles remain refused; ticket stays open for that genuine prerequisite and host coverage.

Milestone: 0.2
Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Every image request is admitted against its exact route’s documented capability and limits. Unsupported or image-dropping routes refuse locally before any send.

## Evidence

- Starts from: Main 399c6c7c7 and the existing SDK plan. PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, ask 2; experiment 0034 design, recorded spike at 8ec4fbffc0dbf8ceaa0fea5607749ff2f683a2a0 and subsequent OpenRouter controls. 0036 reports are pending.
- Keeps: Existing text behavior, typed SDK parity, six errors, cancellation, secrecy, spend limits and zero-send strict replay. Core remains free of I/O; the one Rust engine and native file reader remain shared.
- Changes: Record primary documentation/version, formats, image count, encoded body bytes, dimensions/patch/aspect budgets and runtime/projector requirements per enabled route. Validate final packed request including repeated images. Choose refusal for 0.2 instead of SDK resizing; never inherit the spike’s 32 KB/64 KB bounds as vendor limits.
- Proof: Independent boundary fixtures cover valid multi-image inputs, format/dimension/count/body excess, malformed/truncated data, packing overflow and a valid input above the spike cap where documented limits permit it. Count zero connections for refusals. OpenRouter controls require an image question to refuse before a connection, while ordinary text works. Debug/errors do not expose bytes or keys.
- Defers: Automatic resizing, causal/accuracy claims, unknown formats/limits, extra paid calls and routing fallback.

## Dependencies and ownership

0447 invokes admission before every send/retry/split. 0432 runs support/refusal cases everywhere. Read 0036 findings when delivered; record disposition without new paid experiments or expanding a passed check.

## Design notes

Historical 0034 reports Liquid eight images, JSON below 4.5 MB, 10,000 patches and ≤100:1 ratio; llama.cpp has eight images and compatible projector/physical-batch requirements. These are documentation leads, not newly verified shipping limits. Recheck admitted primary sources at implementation. Undocumented/unsupported route capability cannot be enabled by accepting transport alone. OpenRouter’s tested Clef/Clef-flash route showed no observable image use and stays disabled for images. Keep SDK memory/safety caps distinct from vendor limits. Validate allowed media under bounded decoding; a new decoder dependency follows normal dependency review. No silent resizing, OCR, external-URL fetching or fallback model. A future explicitly requested resize must be a reviewed transformation recorded in the result, not a hidden way to pass a cap.

Start from vendors’ documented limits now. Experiment 0036 is later feedback, not an implementation or release dependency.
