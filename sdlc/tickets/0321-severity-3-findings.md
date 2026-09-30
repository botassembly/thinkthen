# 0321: Refuse an API key that holds a control character

Status: landed. Quick Fix. Plan: `sdlc/planning/cleanup-2026-09-30.md`, step 8. Issue: `sdlc/issues/closed/2026-09-26-architect-review-severity-3-findings.md`, report 12 finding 3.2.

## Outcome

A key holding any control character is a usage error before any send, with exit 2 and `the API key contains a control character`. The message repeats no part of the key. Before, only a carriage return or line feed was refused; a tab, an escape, DEL or a C1 character reached the HTTP layer and read as a network fault.

## Evidence

- Starts from: `Key::check_line_break` in `crates/thinkthen/src/engine/http.rs` refuses CR and LF. `http::post_marked_with_retry` and `request::first_use_key` call it, so the command and the library reach it. `tests/backend/backoff.rs` and `tests/backend/cache_identity.rs` pin it, and `specification/backends.md` states it.
- Keeps: both call sites, the usage kind and exit 2, the unbound-folder behavior of 0250, the zero-budget precedence, and every secrecy guarantee.
- Changes: the predicate becomes `Key::check_control` with `char::is_control` and the new sentence. The spec line and the changelog say so.
- Proof: the cache-identity table gains ESC and DEL rows beside LF and CR, each pinning exit 2, the exact standard error line, and zero sends; the second listener counts four sends. `sdlc/scripts/test`, workspace clippy, `policy.py`, `tickets` and `lint` pass.
- Defers: `sdlc/scripts/live` keeps its own line-break guard; it reads a real key and stays outside this fix.

## What the build taught us

The issue triage missed the existing check. A widened predicate in place was the whole fix.
