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

Fresh code review found that a double-bracket body accepted a nested single-bracket tag. The body parser now consults the existing single-bracket parser and refuses mixed nesting while retaining ordinary bracket literals. Both nesting directions and literal brackets pass the four parser/renderer tests. The source ceiling includes this small regression.

The fix review also found that a literal closing bracket adjoining a double-tag delimiter fell outside the entity span. The body parser now counts unescaped literal bracket pairs before recognizing its outer delimiter. An exact text and scalar-span assertion passes alongside the mixed-nesting refusal. Binding ceilings that include the shared R complete sources were updated to count the required initializer migration.

The final parser review found inconsistent escaped literal bracket pairing in the convenience double-bracket alias. The coordinator authorized its removal. The final input grammar uses only the requested `[TEXT | KIND]` notation and structured examples. One table compares escaped and unescaped literal bracket variants against the same exact text and scalar-span carrier; nesting still refuses. Fixtures and documentation use this grammar. The four focused parser tests pass.

Fresh correction review accepted `2e72414068757e9b3c21ae42b558693f0e44166a`. That revision removes the optional alias, retains the caller's text-first bracket notation and tests literal bracket variants against exact structured offsets. The review findings and intermediate fixes above explain why one grammar replaced the alias.
