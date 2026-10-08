# 0490: Tagged recognition examples

The implementation follows the accepted design in ADR 0127. Its initial carrier checkpoint is `c3833a478e46571d53641b42e728ddbe9efdbba8`, based on source main `aaf7e598f`. The final source adds the original `[TEXT | KIND]` spelling, indexed safe diagnostics, optional pointer selection, native coverage and public documentation. Ticket and lane status remain in pm.

The pure carrier, bracket parser and renderer live in `core/recognize/examples/`. Structured offsets count Unicode scalar values and must align with recognition pieces. Optional example vocabulary validates broader kinds while current recognition kinds determine which entities answer OUT. Rendering uses the actual boundary questions. It never supplies examples to kind, edge or relation requests. Retrieval and pass selection remain caller responsibilities.

The existing State and Asks carry rendered examples beside evidence and derive their ordinary keys from that state. Context retains its separate rendering and digest. Native `Recognize::with_examples`, `RecordInput.examples` and `RecordReading::with_examples_field` use that carrier. CLI recognition reads shared bracket or JSON Lines files, selects per-record arrays and admits all initial bodies before sends. Missing selections retain shared examples; an empty array clears them. Null and invalid traversal refuse. RecordInput mapping preserves all controls. Frozen C carriers are unchanged; canonical Request transport belongs to ticket 0491.

Nonempty examples enforce the configured ceiling against the complete encoded boundary body in preflight, plan and live preparation. The baseline `Bound::WHOLE` does not enforce that ceiling; the example path uses a sized boundary bound plus a final encoded-body check. Omission and empty examples retain baseline bytes and keys. Model-window admission remains separate work under 0461B.

The focused checks passed: three parser/renderer tests, five CLI backend cases, fourteen native tests selected by recognition names, package all-target Clippy, policy, settings and six executable recognition examples. Backend cases count actual loopback sends for early refusal, preserve earlier request and attempt facts on later failure, compare equivalent bracket and scalar examples, verify exact ceiling and one byte over, and exercise cache changes and replay. Native cases reuse `conformance/recognition-examples.json` with non-ASCII and combining characters. No paid backend or publication ran.

The source ceiling grows for the typed carrier, single-bracket compatibility, safe selection and native behavior coverage. Mapping RecordInput centrally replaces repeated field copies rather than adding another carrier. Existing cohesive CLI arguments and public recognition files retain their source-size warnings. Full gate results will be recorded here after completion.

## What the build taught us

The default whole-body recognition bound intentionally preserves existing request sizing and does not enforce the configured byte ceiling. Examples need a sized boundary bound and a final check of the complete encoded body; measuring only the annotated text would miss context, questions and JSON overhead.

Pointer resolution previously collapsed missing members and invalid traversal into the same absent result. Example fallback needs that distinction, so the existing decoded pointer traversal now exposes optional selection with a typed error. Existing ordinary resolution keeps its established optional behavior.
