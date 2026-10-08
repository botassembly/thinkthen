# 0464: Make input help and MCP schemas match admission

Status: OPEN. The after-sprint review found unsupported image help for find and contradictory file-reader combinations in MCP's advertised schema. A caller also found that decide details can return an authored meaning where the help promises a boolean.

Milestone: 0.2

Owner: builder.
Severity: low public contract inconsistency.

Reviews: revision e71fa0b01, accept

Reviews: revision 16eb57be4beea685e5d1b3f7f0f392b13d27de6a, accept

## Outcome

CLI help, decide output documentation and MCP tool schemas describe the behavior callers can use. The MCP command prints the established configuration permission warning on standard error. Invalid inputs still refuse locally before file reads or calls.

## Evidence

- Starts from: 0462 CLI/C/MCP review at 7ea661c1e; cli/args/find.rs advertises --image though cli/args/command.rs refuses it; find's command prose repeats a sentence. mcp/tools.rs::source_schema permits image media with line/window units and window units without a size, while public/input_files.rs and ReaderOptions refuse them. The 2026-10-08 tcga-demo decide details report observed an authored --true sentence in `value` although cli/args/command.rs and specification/decide.md promise only true, false or null. specification/result.md permits an authored meaning in result/2. The second-opinion PM mail assigns the missing MCP configuration warning to 0464: cli/mod.rs returns through mcp_entry before its ordinary warn_configuration call, although specification/settings.md requires a command warning for a configuration file writable by another user.
- Keeps: The ten functions; images only on decide/choose/score; native validation and secrecy; existing file/window limits and error behavior. Preserve decide's authored meaning result, typed yes/no answer, bare output and exit-code behavior. Preserve the existing warning text and permission rule, the configuration refusal, and clean MCP protocol standard output.
- Changes: Hide find's unsupported image attachment flag or explicitly document its refusal, remove duplicate find prose, and constrain the MCP source schema to admitted unit/media/window combinations. Check defaults as well as explicit units against the actual native parser. Correct decide help and specification/decide.md to distinguish bare output from details `value` when --true or --false supplies an authored meaning; explain that `answer.probability` and `threshold` determine the yes/no reading. Print the same configuration permission warning once on MCP standard error when its actual configuration is shared; do not duplicate configuration reads or send the warning on protocol standard output.
- Proof: Fresh ticket review and code review; existing help/schema tests and focused no-send invalid-input cases. Exercise omitted/default unit, explicit file images and valid text windows, plus refused contradictory combinations. Add a focused local decide case with authored true and false meanings that checks bare output, details `value`, the answer kind/probability, threshold and exit code against the corrected text. A focused MCP startup case checks one warning on standard error for another-writable configuration, clean protocol standard output, and no warning for an owner-private configuration; retain the existing refusal for unsafe backend definitions. Run applicable policy, tests and lint before landing; no hosted workflow.
- Defers: No image admission for the other seven functions, new function, alias, feature expansion or proof framework.
