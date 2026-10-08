# 0481 — mcp-bound-images-and-question-files

Status: OPEN.

Milestone: 0.2

Reviews: revision b9027d08b, reject

Reviews: revision b9027d08b, accept

## Outcome

MCP refuses over-budget attachment calls before retaining unbounded file contents, and question selectors cannot block the worker on a nonregular file or consume its protocol input.

## Evidence

- Starts from: second-opinion PM message of 2026-10-08, ask 3; mcp/inputs.rs materializes attachments before complete native record admission; public/question_file.rs opens and reads a path without checking its file kind. These source gaps need owned reproductions before fixes.
- Keeps: Ordinary question files and named references, ordered duplicate images, existing native image/request/record limits, call IDs, cancellation and all ten tools. 0464 retains schema/help work; 0470 retains DuckDB staging.
- Changes: Reproduce repeated-image retention using small owned files under default settings, where record count is unlimited. Write and review an ADR before changing the settled MCP contract, as specification/README.md requires. The proposed ADR extends the existing MCP framing byte ceiling to aggregate retained attachment bytes across the complete inputs collection, explicitly explaining why this transport bound can be below the native per-image ceiling and documenting the support ruling. This is not a new record-count cap or a new SDK setting. Only after that ADR is accepted, update the MCP specification and thread one checked remaining-byte budget through explicit images, inline data and source/reading paths. Charge ordered duplicates for each retained copy; bound file reads before unbounded allocation and stop before later paths once the budget is exceeded. Preserve existing stricter native per-image/route admission and ordinary full-call preflight before sends. Reject nonregular question and plan paths through the shared capped reader before blocking open; preserve regular-file symlinks. Use the existing error mapping. Document any race limitation rather than adding an unnamed adversarial-file design.
  Claim `crates/thinkthen/src/mcp/**`, `crates/thinkthen/src/public/input/**`, `specification/mcp.md` and `sdlc/planning/adr/**`.
- Proof: Many individually valid rows repeating one image under default unlimited records hit the shared byte ceiling with bounded retained buffers, before later file reads and before sends. Cover explicit, inline and source-backed images; do not prove only a single descriptor or only a configured record cap. Owned FIFO and stdin selectors fail promptly; the next framed MCP request remains usable. Normal files, named references and duplicate ordering below the ceiling still work. Preserve cancel/shutdown and installed MCP cases. Fresh memory/liveness review.
- Defers: New MCP tools, endpoints, tracing and general filesystem architecture. No private files, report text or paid calls in reproduction.

## Progress

- 2026-10-08 started
