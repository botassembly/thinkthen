# MCP shared admission

The MCP slice builds from `0ac45ca65cda6e240b8dabb32ed1c098991cafc5`. MCP maps carrier aliases into the public Request and executes the admitted request through the native engine. CLI conversion belongs to the next slice of ticket 0512.

`mcp/admission.rs` decodes semantic controls through RequestOptions. The edge retains cancellation, reserved proxy activation, duplicate-key refusal and the single-string field alias. RequestOptions owns unknown semantic fields and native control grammar. MCP excludes `details` because every tool returns a complete result.

`mcp/request.rs` decodes descriptor context, examples and seed spans through RequestItem. The existing public RecordOptions projection retains MCP's authored-map shortlist grammar and authored order. Context decoding retains the established safe MCP refusal sentence. Every supplied proxy value, including null, reaches native reserved-activation refusal. Selector exclusivity, source aliases and explicit image paths retain MCP framing and host authority. The native descriptor feed owns applicability, conflicts, source admission, attachment accounting and execution validation.

`mcp/tools.rs` derives recognition controls and item examples and seeds from the canonical schema. The obsolete recognition exclusion in `specification/mcp.md` is removed. The existing MCP conformance entry rejects production imports of private core or engine modules; a planted private-engine import fails before any call.

Two execution regressions protect actual boundary-only output, stage context, snippet width and empty item overrides, plus malformed per-item examples and seeds with zero sends and withheld payloads. The boundary-only regression fails against the starting code because its descriptor decoder rejects the native examples field. Existing MCP cases remain unchanged.

Targeted default-profile Rust MCP tests pass: 39 tests. Focused library Clippy passes with warnings denied. Offline policy passes with its existing warnings on unchanged files. Formatting and diff checks pass. The source measurement is 180580 nonblank Rust lines: production decreases by five lines and the two behavior regressions add 97. The `sdlc/ratchet.json` ceiling remains unchanged for the required fresh review of the 92-line increase. Duplicate option carriers, the field iterator and private context decoding were removed before proposing growth.

The rebuilt installed command passes all 258 required MCP parity cells, the nine client fixtures and the no-key installed replay check. Its installed transport suite passes all 20 tests after replacing the obsolete rank-cutoff refusal oracle with an accepted numeric cutoff and the native wrong-function `none` refusal. The large installed test file retains distinct protocol, file-authority, cancellation, secrecy and image-bound behaviors; only that rank contract case changes.

The command artifact is `target/debug/thinkthen`, SHA-256 `9803089b4573a2103912d03e20002d94c729dde57b134fe6f8f1a568943dec8d`. The owned fixture backend is `target/debug/conformance-backend`, SHA-256 `56a1f1242c8187966c82593aeb2ebd59128ca1e0ca9cb9361ff5d82fbeeae853`. Builds and installed verification use the lane memory and swap caps and the default supported Cargo profile. No release or stress gate is claimed.
