# 0484: Split release tests and remove test sediment

## Behavior

Large-input boundaries use ignored tests with the `release_only_` name prefix. The routine runner skips them, and the existing full-cases runner selects them after its routine pass. The selection excludes independent ignored load campaigns and subprocess helpers. Mixed cases retain their small invalid-input, secrecy and caller-limit checks in routine tests. The nested public consumer build and timing-history overflow also use the release selection. The routine runner requires nextest and applies the timeout from `.config/nextest.toml` in both workspaces.

The same release selection covers the CLI's bounded CSV and line readers, core record byte limits, MCP frame limits and aggregate source limits. Mixed MCP framing and image-reader cases retain their small malformed-input and caller-limit checks in routine tests. No distinct boundary claim is removed.

## What the build taught us

A size boundary can share a test with small negative cases. Moving the whole test hides those routine claims. Splitting only the large construction preserves them. A common ignored-test name selects release boundaries without selecting unrelated ignored subprocess and load tests. The ordinary cargo-test fallback cannot enforce nextest's routine timeout, so the runner now refuses it.

The release runner's direct cargo tests need their own isolated usage and configuration folders. The routine child runner's guard cannot protect its parent after the child exits. The parent now uses the existing scratch helpers, which clean up only the folders that invocation creates.

Large-input cases also live inside source modules. Selecting integration cases alone missed those boundaries; the release split must include their owning tests.

Unbounded nextest concurrency can starve small cache cases that pass alone. The routine profile limits test processes to two and keeps the five-second ceiling. Full shared-case replay and parity belong to release selection even when each individual corpus row is small; focused cache and other boundary regressions remain routine.
