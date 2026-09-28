# Whoever can write a named cache or recording folder decides the answers

Status: closed by ticket 0242, code accepted at `2e3e32c6`. Ticket 0163 supplied the writer-authority pages; 0242 implements the selected named-folder warning and documents its limits. The check is advisory and does not authenticate entries.

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

## Closure evidence and limits

Fresh independent code review accepted `2e3e32c6263f27b35e2dbce33b2e7a232eaf842d` and reran the three compiled warning boundary cases. The command warns before it trusts an existing named Unix folder with another owner or group/other directory write permission. Private folders and keyless replay retain their behavior. The recording and security pages state the warning and its limits.

The original done criterion above is met. A later status sentence had accidentally promoted the optional keyed check into required entry-integrity work. The original issue says only to consider it later, and experiment 283 finding 40 calls it optional hardening. The independent reviewer confirmed this distinction. Signing, library advisories, ACL and ancestor checks, non-Unix permission classification and concurrent permission changes remain explicit limits or future ideas; closing this issue claims no authentication guarantee.
