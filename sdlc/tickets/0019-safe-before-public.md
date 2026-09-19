---
flow: build
priority: 35
opens: crates spec specification README.md Cargo.toml .github sdlc/scripts sdlc/planning/rust-standards.md sdlc/ratchet.json
---

# 0019: Safe before public

Status: in progress

## Outcome

The findings of the security and command-line review of 2026-09-19 are closed before the repository goes public. The key never crosses a network in clear text, a hostile or huge record cannot exhaust memory, a user at a terminal is told what the tool waits for, and the build checks its own dependencies.

## Current Facts

A second agent read the code and ran the binary with no key. It found the design sound in most places: no redirect is followed, TLS is never weakened, the key appears in no plan, recording, error, or Debug value, every malformed answer from a server is exit 4, a response is capped at 1 MiB, recordings are written to a temporary file with mode 0600 and renamed, every message goes to standard error with the program's name first, and the argument conventions hold. It found three things that must change. Ticket 0013 carries two of them: the run now stops when the reader closes the pipe, and a label or a level that holds a control character is refused. This ticket carries the third and the smaller findings.

## Scope

- **Plain `http://` is refused unless the host is loopback.** `localhost`, `127.0.0.1`, and `[::1]` stay allowed, because tests and a local adapter use them. Any other host under `http://` is a usage error before any request, and the message says that the key would cross the network in clear text. No option overrides it. A user with a server on another machine uses `https://` or a tunnel to loopback. `backends.md` loses the sentence that allows it.
- **A record has a size limit.** A record over 16 MiB is refused as exit 2 for that record before any request. The number is fixed and no option sets it, because the vendor's token budget refuses evidence far smaller than that. `records.md` states it in one sentence.
- **A terminal on standard input gets one line.** When no `--input` is given and standard input is a terminal, the tool prints one line on standard error that says it is reading evidence from the terminal and how to end it. The line never appears in a pipe.
- **The build checks its dependencies.** `sdlc/scripts/lint` runs `cargo deny check advisories bans licenses` with a `deny.toml` that allows the licenses in the tree today, and `sdlc/scripts/install` checks that `cargo-deny` is present. The workflow installs it at a pinned version. `rust-standards.md` loses its note about the gap.
- **The release profile is stated.** `[profile.release]` sets `overflow-checks = true` and `panic = "abort"`. The build in `sdlc/scripts/` uses `--locked` everywhere.
- **The workflow pins what it runs.** Each action is pinned to a commit SHA with its tag in a comment, and the downloaded `jq` is checked against a recorded SHA-256.
- **Three sentences.** `README.md` says beside its recording bullet that a recording holds the evidence, so committing one publishes it. `backends.md` names the proxy variables the HTTP client reads. `recording.md` says that an address is stored in every recording, path included, so a token must never sit in the path.

Excluded: any new option, a trust store for a private certificate authority, and shell completions or a manual page, which wait for the release pass.

## Acceptance

- Unit tests in the core cover the address rule: each loopback spelling accepted, a named host and a private address refused under `http://`, and every `https://` base untouched.
- Integration tests cover the refused record with zero requests for it, and the terminal line through a pseudo-terminal or the smallest honest substitute, with no line in a pipe.
- The key and the evidence never appear in any new message.
- The pinned `decide` digest holds, and every committed recording still replays.
- The push check is green with `cargo deny` running in it.
- The ratchet equals the measured total, and the commit that raises it says what grew and why.
- The whole ladder is green, and a second agent reviews the public surface change.
