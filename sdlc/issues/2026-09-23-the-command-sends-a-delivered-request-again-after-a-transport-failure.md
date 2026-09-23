# The command sends a delivered request again after a transport failure

Status: Open

One `thinkthen decide` call sends the same paid request three times when the backend reads the whole request and then resets the connection or never answers. The backend received every copy, and the vendor bills each delivery. The user sees one failure and pays for three sends.

## Reproduction

A Python stub on `127.0.0.1` read each request's headers and its full body, counted the POST, and then either reset the connection (`SO_LINGER` zero, then close) or slept 30 seconds. The command ran from origin/main at 1b894d9, built with `cargo build --offline --locked`, on 2026-09-23. The key was a made-up dummy value, and each mode had its own scratch cache folder:

    $ THINKTHEN_API_KEY=dummy THINKTHEN_BASE_URL=http://127.0.0.1:8341 thinkthen decide 'Is this fine?' --timeout 2 --input msg.txt
    thinkthen: the backend closed the connection before a reply; try again or change --max-retries
    exit 4, wall 3.55s; the stub counted 3 full POSTs of 120 bytes

    $ THINKTHEN_API_KEY=dummy THINKTHEN_BASE_URL=http://127.0.0.1:8342 thinkthen decide 'Is this fine?' --timeout 2 --input msg.txt
    thinkthen: the backend timed out; increase --timeout or try again
    exit 4, wall 9.22s; the stub counted 3 full POSTs of 120 bytes

`--max-retries` was left at its default of 2. Each retry resent the whole body after the first copy had arrived.

## Where it lives

`crates/thinkthen/src/engine/http.rs:268-275`, `is_retried`:

    Error::Transport(TransportKind::Refused) => false,
    Error::Transport(_) => true,

`post_observed` (line 124) retries every failure `is_retried` accepts. A reset, an early close, and a read timeout all reach it as `Error::Transport`. `specification/backends.md:59` states the same rule: "A retry happens after a transport failure other than a refused connection". The code follows the page, so the page changes with the code.

## What the surfaces branch did

The surfaces branch measured the same three sends on its stand-in and stopped them in dd8a383 on `surfaces-wave6` ("Never send a request again after a transport failure"). Its issue is `2026-09-23-a-delivered-request-is-sent-again-after-a-transport-failure.md` on that branch, and `standin/tests/no_resend.rs` pins the rule. The stand-in sends no transport failure again. A refusal, a failed connect, and a name that did not resolve already fail at once. A 429 or a listed 5xx status keeps its retries, because the backend answered and said it did not take the request. The error message still says whether the caller's own second try could help. Until main changes, the command and the surfaces bill differently for the same failure.

## Smallest fix

Make `is_retried` return false for every `Error::Transport`. Keep the status retries as they are. Amend `specification/backends.md` line 59 and the transport-failure table to say a transport failure is never sent again. Change the "try again or change `--max-retries`" phrases, because `--max-retries` no longer governs a transport failure. Add a loopback test beside the existing ones in `http.rs` that reads the full body, resets, and asserts one POST. Add a second test that stalls past the timeout and asserts one POST.

A connect that never finished could still be retried safely. The command's `TransportKind` does not separate a failed connect from a failed read today, although ureq 3.4.2 has `Error::ConnectionFailed`. The smallest fix retries no transport failure. A later ticket can split out the failed connect if a user needs that retry back.

## Overlap

Ticket 0064 (`sdlc/tickets/0064-bound-retry-waits-and-report-sends.md`) bounded retry waits and added `requests_sent`. Its peer-close test "follows the existing retry rule", so it kept the resend. It does not cover this finding. `2026-09-21-a-closed-connection-still-costs-the-whole-timeout.md` is closed and covered only the wait. `2026-09-21-a-retried-send-is-invisible-to-the-user.md` is closed and made the sends visible. This issue asks for fewer sends.

## How bad it is for a user

Major. A flaky network or a backend that drops replies turns one paid call into three, and the default settings do it.

Found by the wave-7 probe of main after the surfaces branch's fifth review.
