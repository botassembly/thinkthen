# MCP discovery measurement

The compact-catalog branch at `e827f883e95815cf021dc32b8e24a06ed7b6e02a` compiled its MCP subprocess integration tests. The compiled command returned ten tools occupying 35,938 bytes as compact JSON, above the ticket's discovery budget.

The integration executable ran ten cases: nine passed and the catalog-budget assertion failed. The library test build separately failed because the framing-discovery test still passed an argument to the now-argument-free `tools::list`. The build stopped there; its backend and Clippy phases did not run.

These measurements came from the compiled command and test executable on that revision, rather than a source-schema estimate.
