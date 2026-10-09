# 0484: Split release tests and remove test sediment

## Behavior

Large-input boundaries use ignored tests with the `release_only_` name prefix. The routine runner skips them, and the existing full-cases runner selects them after its routine pass. The selection excludes independent ignored load campaigns and subprocess helpers. Mixed cases retain their small invalid-input, secrecy and caller-limit checks in routine tests. The nested public consumer build and timing-history overflow also use the release selection. The routine runner requires nextest and applies the timeout from `.config/nextest.toml` in both workspaces.

Slice A preserves every claim and removes no test. Counting literal `#[test]` attributes in Rust files under `crates` and `conformance`, excluding `target` directories, gives 2,011 before and 2,019 after; eight mixed cases split into two tests. The in-source count under `crates/thinkthen/src` stays 526. Counting nonblank Rust lines over the source ratchet's directories gives 181,427 before and 181,549 after. The added markers and small-case splits account for the increase; the ceiling remains unchanged for explicit reviewer acceptance.

## What the build taught us

A size boundary can share a test with small negative cases. Moving the whole test hides those routine claims. Splitting only the large construction preserves them. A common ignored-test name selects release boundaries without selecting unrelated ignored subprocess and load tests. The ordinary cargo-test fallback cannot enforce nextest's routine timeout, so the runner now refuses it.

The release runner's direct cargo tests need their own isolated usage and configuration folders. The routine child runner's guard cannot protect its parent after the child exits. The parent now uses the existing scratch helpers, which clean up only the folders that invocation creates.

Splitting the large caption case left one invalid-caption case inside a table loop. A direct call retains the refusal and zero-request checks, satisfies Clippy, and reduces the measured Rust source ceiling from 181,549 to 181,547 nonblank lines.
