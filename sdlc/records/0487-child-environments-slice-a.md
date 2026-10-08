# 0487 Slice A: Isolate test child homes

The full test runner now calls the existing `config_home` helper after `usage_guard`, so later test commands and demos receive an owned configuration home. The Python child helper supplies an owned home while preserving explicit overrides; the MCP conformance and installed runners use it. The Rust child helper supplies one isolated home method to the named-question, cache-policy, and public-controls callers. The existing children check rejects direct XDG or APPDATA writes in those six ordinary callers. Explicit configuration, default-path, secrecy, Windows, and container fixtures retain their intentional environments.

Before the entry fix, an owned malformed ambient configuration caused the triage demo to exit 5. After it, the demo passed under valid and malformed ambient configurations. Targeted Rust tests passed 37 cases; native named lookup and cache/error children passed under malformed ambient configuration. MCP installed smoke and 16 behavior cases passed under both configurations and an installed path containing spaces. The normal MCP check passed 252 shared parity cases and all 16 installed behavior tests. An earlier 180-second trial timed out; the normal check used the repository's 1,800-second limit and exited 0.

Fresh read-only review found one script-table formatting error, fixed it, then accepted `f1a49f8bc`. After main's recognition changes were merged at `878ba6198`, the measured Rust ratchet was 165,084. On that combined source, full test, lint, and spec exited 0: 1,778 workspace tests, 340 library-only tests, 23 external-consumer tests, all binding smokes, and 24 green demos. The children self-test passed 44 cases, including planted direct-write refusals.

Other language runners and Rust environment-behavior families still construct their own child maps. Further family migration and matching lint coverage remain outside this slice.

## Slice B: Python child homes

The Python test wrapper now forwards an owned home to `conformance/children/children.py`. Its product-child helper passes the test folder while preserving the explicit fake key, loopback backend, cache folder and caller overrides. Named-backend tests use the same helper instead of repeating its folder map. Selected-setup tests no longer overwrite those folders afterward. Examples pass their existing temporary directory as the home, so their Windows product children use owned application folders. Tool children retain their startup environment, and Windows configuration fixtures retain their existing paths and access controls. The existing children check covers exactly these four Python files in addition to its earlier callers.

On the existing built extension and loopback backend, the named-backend, selected-setup and secrecy tests plus the engine's explicit-cache, replay, URL-precedence and environment-setting cases passed. All examples passed. The same selection and examples passed with an owned malformed ambient configuration and planted dummy parent keys, tokens and passwords. Linux ran these checks; Windows access controls were preserved in source but were not exercised. No native fixtures, product code or intentional environment-behavior cases changed.

The first full routine Python run failed because the existing extension lacked current collection functions, the current complete-call signature and test probes. Planning and producer-release failures also reproduced with the original helper loaded from the starting revision. Rebuilding only the Python extension with the normal offline probe configuration resolved every failure: the full routine suite, including its Arrow, release and stopping checks, passed. The selected malformed-parent cases and both example environments then passed on that build. Stress cases remain outside the routine profile. Checks and individual exits are under the lane's `target/0487-*` paths.

## What the build taught us

Isolating the entry runner fixes the inherited-configuration failure for its descendants. Shared child helpers then remove duplicate setup from ordinary callers while leaving tests of environment behavior explicit.

The Python helper's owned `config`, `cache`, `state` and `local` paths match the fixture locations already used by callers. Passing the home removes repeated maps without relocating fixtures. Explicit environment overrides still win, including the engine test that places usage totals in its chosen state folder. Supplying folder names does not create them, so the explicit-cache test still detects unwanted default-cache writes.

A passing focused environment selection cannot establish that a cached extension matches the whole Python source package. Running the routine suite exposed the mismatch, and the normal probe build settled it without changing expectations or broadening the product change.
