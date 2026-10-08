# 0464: Make input help and MCP schemas match admission

Status: OPEN. The after-sprint review found unsupported image help for find and contradictory file-reader combinations in MCP's advertised schema.

Milestone: 0.2

Owner: builder.
Severity: low public contract inconsistency.

## Outcome

CLI help and MCP tool schemas describe the native reader combinations callers can use. Invalid inputs still refuse locally before file reads or calls.

## Evidence

- Starts from: 0462 CLI/C/MCP review at 7ea661c1e; cli/args/find.rs advertises --image though cli/args/command.rs refuses it; find's command prose repeats a sentence. mcp/tools.rs::source_schema permits image media with line/window units and window units without a size, while public/input_files.rs and ReaderOptions refuse them.
- Keeps: The ten functions; images only on decide/choose/score; native validation and secrecy; existing file/window limits and error behavior.
- Changes: Hide find's unsupported image attachment flag or explicitly document its refusal, remove duplicate find prose, and constrain the MCP source schema to admitted unit/media/window combinations. Check defaults as well as explicit units against the actual native parser.
- Proof: Fresh ticket review and code review; existing help/schema tests and focused no-send invalid-input cases. Exercise omitted/default unit, explicit file images and valid text windows, plus refused contradictory combinations. Run applicable policy, tests and lint before landing; no hosted workflow.
- Defers: No image admission for the other seven functions, new function, alias, feature expansion or proof framework.
