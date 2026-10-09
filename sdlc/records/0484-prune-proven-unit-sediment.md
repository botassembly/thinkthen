# 0484: Split release tests and remove test sediment

## Behavior

Large-input boundaries use ignored tests with the `release_only_` name prefix. The routine runner skips them, and the existing full-cases runner selects them after its routine pass. The selection excludes independent ignored load campaigns and subprocess helpers. Mixed cases retain their small invalid-input, secrecy and caller-limit checks in routine tests. The nested public consumer build and timing-history overflow also use the release selection. The routine runner requires nextest and applies the timeout from `.config/nextest.toml` in both workspaces.

The same release selection covers the CLI's bounded CSV and line readers, core record byte limits, MCP frame limits and aggregate source limits. Mixed MCP framing and image-reader cases retain their small malformed-input and caller-limit checks in routine tests. No distinct boundary claim is removed.

## What the build taught us

A size boundary can share a test with small negative cases. Moving the whole test hides those routine claims. Splitting only the large construction preserves them. A common ignored-test name selects release boundaries without selecting unrelated ignored subprocess and load tests. The ordinary cargo-test fallback cannot enforce nextest's routine timeout, so the runner now refuses it.

The release runner's direct cargo tests need their own isolated usage and configuration folders. The routine child runner's guard cannot protect its parent after the child exits. The parent now uses the existing scratch helpers, which clean up only the folders that invocation creates.

Large-input cases also live inside source modules. Selecting integration cases alone missed those boundaries; the release split must include their owning tests.

Unbounded nextest concurrency can starve small cache cases that pass alone. The routine profile limits test processes to two and keeps the five-second ceiling. Full shared-case replay and parity belong to release selection even when each individual corpus row is small; focused cache and other boundary regressions remain routine.

Moving tests between suites also changes the binding and Polars policy checks. Their independent scans retained the old ban on ignored cases after the release split. Exact reviewed file and function pairs admit the shared-case release tests while refusing arbitrary ignored names, including names with the release prefix. The actual lint gate exercises these scans together.

A table of small inputs can exceed the routine timeout through repeated command startups. Group rank cases by top limit and filter cases by threshold, preserving every input and assertion. Such tables remain routine behavior tests; their cost does not make them large-input release boundaries.

The 180-name relation matrix creates 32,220 output rows and 81 local requests, so it uses release selection and retains its exact row count, first row and request count. The existing routine CLI case `four_hundred_and_profile_limits_split_shared_rule_requests` checks 900 questions split into requests of 400, 400 and 100, shared relation rules and lower or higher profile limits. It supplies the small request-bound coverage without another case. The root source ceiling grows from 181607 to 181608 nonblank lines for the release annotation; explicit reviewer acceptance is required before landing. C source counts remain unchanged.
