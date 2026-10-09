# 0505: Give C and Zig complete typed session views

Status: OPEN.

Milestone: 0.2

Depends on: 0503
Depends on: 0513
Depends on: 0517

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

## Outcome

C callers read every known result field through additive complete session views generated from Rust, with explicit presence and retained failure facts. Zig reads the same views with optionals and error unions. The frozen 0.1 C ABI is unchanged. Ada (0528) and COBOL (0529) build on these views.

## Evidence

- Starts from: The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) and finding 2 of the [0521 assessment](../records/0521-surface-contract-assessment.md). `FactsV1` in `libraries/c/src/ffi/carriers/metadata.rs` lacks current request-size and persistence observations. `libraries/c/src/complete/metadata.rs` deliberately drops partial token usage. Regenerating those declarations would keep the losses. [ADR 0129](../planning/adr/0129-owned-json-sessions.md) leaves the complete C view contract open.
- Keeps: Every frozen 0.1 C symbol, signature, layout, error code and lifetime rule. Compatibility layouts stay intact. Zig's execution and owned-result code stays unless replaced with parity.
- Changes: Meet the caller acceptance and the C and Zig sections of `../../libraries/BINDING-AUTHOR.md`. Slices in order:
  - Complete C session views: settle accessor names, presence tags, lifetimes and unknown-member retention. Add their Rust storage and conversion from 0513's semantic graph, and extend `sdlc/scripts/generate-c-header.py`. Every known field gets typed access, including independently present token dimensions and persistence. Unknown members may survive through an owned opaque extension. Claim the new view and conversion paths, the installed C driver and the generator inputs, named before coding.
  - C package and docs: ship the C archive with the header and prebuilt library under 0517's design, and teach the new views in `libraries/c/README.md` beside the frozen compatibility exports that 0515 documents.
  - Zig: read the generated header, expose optionals and error unions, package the library under 0517's design, update `libraries/zig/README.md` and remove old public names after installed parity. Claim `libraries/zig/**` files named per slice.
- Proof: An installed C consumer reads every known field and failure facts through shared cases. One installed typed case keeps these fields and nested views after session destruction and before result destruction. An installed Zig consumer passes the same cases. Raw JSON availability alone does not satisfy parity. Record handwritten code removed and added, counting generator changes, in the landing record.
- Defers: Final distribution assembly to 0530. Ada to 0528. COBOL to 0529. Dead `libraries/zig/src/complete.zig` removal and frozen-export documentation to 0515.
