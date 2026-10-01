Status: closed 2026-10-01 by site tickets 0042 (`8c5aabc00`) and 0044 (`5c24d41ac`), as the site owner reported. Filed 2026-09-30 by ticket 0309 for marketing, which owns `site/` (`sdlc/planning/ownership.md`). Resolution: the reference page lists exit codes 0 to 7, 70, 130 and 143, and names 8 as the only reserved code. It states the local-server key rule, the named-backend key rule and the precedence tiers, and names `thinkthen.status/2` with one `status --json` example and a link to the specification. The three moved links now use `closed/`.

# The reference page's exit codes and key rule drift from the specification

`site/src/pages/reference.astro` disagrees with `specification/channels.md` and `specification/backends.md` in three places.

1. The exit-code table (`EXITS`, lines 54 to 62) lists 0 to 6 and 70. `channels.md` lines 63 and 65 also define 7 (a completed `annotate --jsonl --details --batch 1 --on-error continue` run with a missing-pointer error row) and 130 and 143 (SIGINT or SIGTERM stopped the command).
2. Line 192 says codes 7 and 8 are reserved and unused, and that a completed run exits 0 unless `annotate` exits 6. `channels.md` line 67 reserves only 8. Line 62 lets `relate` exit 6 too, and line 67 lets `annotate` exit 7.
3. Line 65 (`THINKTHEN_API_KEY`) and the exit 4 row say an absent or empty key is always exit 4. `backends.md` line 71 sends a request to `localhost`, `127.0.0.1`, or `[::1]` with no key and no `Authorization` header. Only another address exits 4. The code agrees (`crates/thinkthen/src/engine/facade.rs`, `fn key`).

4. Ticket 0334 (ADR 0114) added named backends. Line 217 names `thinkthen.status/1`; `status --json` is now `thinkthen.status/2`, with `backend.name` and `backend.key_variable` added and every version-1 path kept. The key rule now reads the selected backend's own variable (`TYPESAFE_API_KEY`, `LIQUIDAI_API_KEY`) when `--backend`, `THINKTHEN_BACKEND`, or the configuration's `backend` names one ([backends.md](../../../specification/backends.md#named-backends)).

## Done when

The reference page lists every exit code `channels.md` defines, names 8 as the only reserved code, states the local-server key rule, names `thinkthen.status/2`, and states the named-backend key rule.

## Site links that point at moved issues

Commit `cbe466b26` moved closed issues to `sdlc/issues/closed/`. Two site links still use the old path:

- `site/src/pages/reference.astro` links to architect review 07.
- `site/src/articles/code-that-understands.md` line 65 links to architect review 12.

Add `closed/` to both paths.

The issue triage of 2026-09-30 moved `2026-09-26-the-beatles-bench-section-keeps-its-own-copy.md` to `closed/`. `site/src/data/beatles.mjs` line 20 names its old path in a comment. Add `closed/` there too.
