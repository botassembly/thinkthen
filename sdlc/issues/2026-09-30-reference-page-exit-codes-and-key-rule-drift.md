Status: Open. Filed 2026-09-30 by ticket 0309 for marketing, which owns `site/` (`sdlc/planning/ownership.md`).

# The reference page's exit codes and key rule drift from the specification

`site/src/pages/reference.astro` disagrees with `specification/channels.md` and `specification/backends.md` in three places.

1. The exit-code table (`EXITS`, lines 54 to 62) lists 0 to 6 and 70. `channels.md` lines 63 and 65 also define 7 (a completed `annotate --jsonl --details --batch 1 --on-error continue` run with a missing-pointer error row) and 130 and 143 (SIGINT or SIGTERM stopped the command).
2. Line 192 says codes 7 and 8 are reserved and unused, and that a completed run exits 0 unless `annotate` exits 6. `channels.md` line 67 reserves only 8. Line 62 lets `relate` exit 6 too, and line 67 lets `annotate` exit 7.
3. Line 65 (`THINKTHEN_API_KEY`) and the exit 4 row say an absent or empty key is always exit 4. `backends.md` line 71 sends a request to `localhost`, `127.0.0.1`, or `[::1]` with no key and no `Authorization` header. Only another address exits 4. The code agrees (`crates/thinkthen/src/engine/facade.rs`, `fn key`).

## Done when

The reference page lists every exit code `channels.md` defines, names 8 as the only reserved code, and states the local-server key rule.
