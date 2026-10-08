# 0487 Slice A: Isolate test child homes

The full test runner now calls the existing `config_home` helper after `usage_guard`, so later test commands and demos receive an owned configuration home. The Python child helper supplies an owned home while preserving explicit overrides; the MCP conformance and installed runners use it. The Rust child helper supplies one isolated home method to the named-question, cache-policy, and public-controls callers. The existing children check rejects direct XDG or APPDATA writes in those six ordinary callers. Explicit configuration, default-path, secrecy, Windows, and container fixtures retain their intentional environments.

Before the entry fix, an owned malformed ambient configuration caused the triage demo to exit 5. After it, the demo passed under valid and malformed ambient configurations. Targeted Rust tests passed 37 cases; native named lookup and cache/error children passed under malformed ambient configuration. MCP installed smoke and 16 behavior cases passed under both configurations and an installed path containing spaces. The normal MCP check passed 252 shared parity cases and all 16 installed behavior tests. An earlier 180-second trial timed out; the normal check used the repository's 1,800-second limit and exited 0.

Fresh read-only review found one script-table formatting error, fixed it, then accepted `f1a49f8bc`. After main's recognition changes were merged at `878ba6198`, the measured Rust ratchet was 165,084. On that combined source, full test, lint, and spec exited 0: 1,778 workspace tests, 340 library-only tests, 23 external-consumer tests, all binding smokes, and 24 green demos. The children self-test passed 44 cases, including planted direct-write refusals.

Other language runners and Rust environment-behavior families still construct their own child maps. Further family migration and matching lint coverage remain outside this slice.

## What the build taught us

Isolating the entry runner fixes the inherited-configuration failure for its descendants. Shared child helpers then remove duplicate setup from ordinary callers while leaving tests of environment behavior explicit.
