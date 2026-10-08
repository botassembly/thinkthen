# 0464: Make input help and MCP schemas match admission

Status: OPEN. The after-sprint review found unsupported image help for find and contradictory file-reader combinations in MCP's advertised schema. A caller also found that decide details can return an authored meaning where the help promises a boolean.

Milestone: 0.2

Owner: builder.
Severity: low public contract inconsistency.

Reviews: revision e71fa0b01, accept

## Outcome

CLI help, decide output documentation and MCP tool schemas describe the behavior callers can use. Invalid inputs still refuse locally before file reads or calls.

## Evidence

- Starts from: 0462 CLI/C/MCP review at 7ea661c1e; cli/args/find.rs advertises --image though cli/args/command.rs refuses it; find's command prose repeats a sentence. mcp/tools.rs::source_schema permits image media with line/window units and window units without a size, while public/input_files.rs and ReaderOptions refuse them. The 2026-10-08 tcga-demo decide details report observed an authored --true sentence in `value` although cli/args/command.rs and specification/decide.md promise only true, false or null. specification/result.md permits an authored meaning in result/2.
- Keeps: The ten functions; images only on decide/choose/score; native validation and secrecy; existing file/window limits and error behavior. Preserve decide's authored meaning result, typed yes/no answer, bare output and exit-code behavior.
- Changes: Hide find's unsupported image attachment flag or explicitly document its refusal, remove duplicate find prose, and constrain the MCP source schema to admitted unit/media/window combinations. Check defaults as well as explicit units against the actual native parser. Correct decide help and specification/decide.md to distinguish bare output from details `value` when --true or --false supplies an authored meaning; explain that `answer.probability` and `threshold` determine the yes/no reading.
- Proof: Fresh ticket review and code review; existing help/schema tests and focused no-send invalid-input cases. Exercise omitted/default unit, explicit file images and valid text windows, plus refused contradictory combinations. Add a focused local decide case with authored true and false meanings that checks bare output, details `value`, the answer kind/probability, threshold and exit code against the corrected text. Run applicable policy, tests and lint before landing; no hosted workflow.
- Defers: No image admission for the other seven functions, new function, alias, feature expansion or proof framework.
