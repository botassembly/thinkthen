# Whoever can write a named cache or recording folder decides the answers

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, report 12, finding 2.2. The documentation half blocks 0.1. The warning on writable folders does not. Owner for the documentation half: ticket 0163 on `ticket/0163-the-cache-folder-and-its-pages`, ready for review.

## What happens

Named `--cache`, `--record` and `--replay` folders at mode `0777` are accepted. Only the default folder is checked for privacy. Entries carry no integrity check. The report changed `"noul": 0.93` to `0.01` in one entry, and the next run flipped from `true` to `false` with `requests_sent: 0` and `cached: true`. `--replay` gave the same result.

`specification/recording.md` line 62 treats every file field as untrusted text for printing. It says nothing about a folder deciding answers. A shared `THINKTHEN_CACHE` on a team volume, a CI cache restored across branches, or a recording edited in a pull request can change gate decisions without a sign.

## Checked on main

Verified by reading the code: `crates/thinkthen/src/engine/recorder.rs:137` checks `require_private` only when `private_default` is set. The edited-entry run comes from the report.

## What would fix it

1. Say in `recording.md` and `SECURITY.md` that a folder's writers control its answers.
2. Warn on, or refuse, a group- or world-writable folder the tool did not create.
3. Consider an optional keyed check on entries later.

Report 12, finding 3.3, raises the same point for the configuration file. The severity 3 roll-up lists it.

## Done when

Both pages state the trust rule, and the queue owner has decided the writable-folder check.
