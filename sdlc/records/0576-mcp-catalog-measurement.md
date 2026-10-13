# MCP discovery measurement

The compact-catalog branch at `e827f883e95815cf021dc32b8e24a06ed7b6e02a` compiled its MCP subprocess integration tests. The compiled command returned ten tools occupying 35,938 bytes as compact JSON, above the ticket's discovery budget.

The integration executable ran ten cases: nine passed and the catalog-budget assertion failed. The library test build separately failed because the framing-discovery test still passed an argument to the now-argument-free `tools::list`. The build stopped there; its backend and Clippy phases did not run.

These measurements came from the compiled command and test executable on that revision, rather than a source-schema estimate.

At `243002feb5fc0faf119b6f00b15084cb3a001bd5`, the full JSON-RPC tools/list response measured 24,753 bytes, or 24,754 including its newline. It contained ten tools. The coordinator and a fresh reviewer independently reproduced that measurement. The reviewer checked discovery fields and enum guidance against native admission and found that the advertised `thinkthen mcp --help` command still omitted the grammar guidance. Its argument metadata was overridden by the subcommand metadata.

Revision `8b7632282e6b4565f1e038b5a035f6a57a50f1f8` attached the existing help text to the subcommand and added a compiled-command regression that failed before the repair and passed afterward. The response size remained unchanged. Its exited checks passed: eleven stdio cases, thirty-six selected MCP library cases, nine client cases, twenty-one installed Python cases including a successful call to each tool, and the recorded example. Three release-only library cases remained ignored. Scoped Clippy, formatting and policy passed; these were targeted checks rather than a combined release qualification.
