# 0022: Name the refused address rule

Branch `ticket/0022-hands-on-repairs`. Built 2026-09-20.

## What landed

The backend address parser now returns a separate safe error for an invalid scheme, user information, an invalid port, and a query or fragment. Each error names the rule that failed, repeats no part of the refused address, exits 2, and happens before key access or a request.

Valid addresses and the clear-text loopback restriction are unchanged. The backend specification and executable decide page use the new exact sentences.

## Red then green

The first exact integration test expected separate sentences and received the old combined scheme-and-user-information sentence.

The final matrix covers an invalid scheme, user information, an empty port, a signed port, port 65536, a query, and a fragment. Every case runs with a key and without one. Both forms return the same complete exit-2 diagnostic, which proves address validation precedes missing-key handling. Listener-backed cases also count zero connections and zero requests.

The common refusal sweep includes all four error classes and continues to check every command. Existing valid-address and loopback cases protect accepted behavior.

## Review

The design reviewer rejected the original ticket because it bundled twelve independent findings. Ticket 0022 was rewritten around one parser outcome. The reviewer accepted that smaller level 2 design and the Luna High route. The remaining first-pass findings stay in their issue reports until each becomes active.

The code reviewer accepted the implementation after one remediation. The first port fixtures added a second port to the listener authority, so all three exercised the same malformed-colon path. They also always supplied a key. The corrected cases use the literal empty, signed, and 65536 forms and repeat the full matrix without a key.

## Choices made where the ticket was silent

Ian can overturn this choice.

- **A missing host keeps its existing safe host error.** Ticket 0022 splits only the four settled address rules from `backends.md`. It does not broaden accepted addresses or change blank-base, missing-host, path, or clear-text behavior.

## Gates and size

The source ceiling is 16,836 measured Rust lines, up from 16,715. The growth is the exact binary matrix, shared refusal coverage, and distinct core error variants. The implementation reuses the existing parser and one error per rule; no duplicate parsing path was added.

| Rung | Result |
| --- | --- |
| `sdlc/scripts/install` | exit 0 |
| `sdlc/scripts/lint` | exit 0, ratchet `16836/16836` |
| `sdlc/scripts/test` | exit 0 |
| `sdlc/scripts/spec` | exit 0 |

The coordinator ran the ladder with the key and base address unset. No live call ran.
