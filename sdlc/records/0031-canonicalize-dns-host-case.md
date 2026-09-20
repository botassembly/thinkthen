# 0031: Canonicalize DNS host case

Branch `ticket/0031-normalize-host-case`. Built 2026-09-20.

## What landed

Resolved backend URLs now lowercase literal ASCII letters in unbracketed DNS host text after the existing validation succeeds. Host-case variants therefore share one URL and recording digest.

The parser preserves scheme handling, percent escape triplets, raw non-ASCII bytes, bracketed IPv6 text, port spelling, and path bytes and case. It changes no accepted or refused address class. The backend page records these boundaries and no longer describes HTTPS base bytes as untouched.

## Red then green

The new parser case first showed that mixed-case DNS and punycode hosts survived unchanged. It now pins lowercased host text beside an unchanged leading-zero port, mixed-case path, and path escape. Further cases preserve bracketed IPv6, host percent escapes, and non-ASCII host bytes while lowercasing surrounding ASCII letters.

A local integration flow records through `LOCALHOST`, observes the canonical lowercase URL and digest filename, and replays through `localhost` with no key and no request. It makes one setup request and keeps one entry.

The first test location pushed `recordings.rs` over the 500-line limit. The integration proof moved to the address test owner and reuses two recording helpers. All 1,059 committed recordings remain unchanged: 1,057 use the canonical hosted URL and two use canonical numeric loopback. The pinned built-in digest and every committed replay pass.

## Review

The design reviewer required a level 3 floor because recording identity is shared durable state. It accepted ADR 0025's exact byte boundaries, compatibility inventory, parser matrix, and local record-and-replay proof.

The code reviewer found no parser or recording defect. It rejected one stale sentence that called every HTTPS base untouched. The corrected page limits that statement to admission under the clear-text rule, and the reviewer accepted the final diff.

## Choices made where the ticket was silent

Ian can overturn these choices.

- **Only literal ASCII DNS letters are normalized.** The parser performs no IDNA conversion, Unicode folding, percent decoding, IPv6 normalization, port normalization, or path normalization.
- **Old external mixed-case entries have no fallback.** They will miss until re-recorded or migrated with a consistent URL and filename. A fallback would preserve two identities for one request. No committed entry needs migration.

## Gates and size

The source ceiling is 17,036 measured Rust lines, up from 16,908. The increase is the host-only canonicalizer, boundary matrix, and local recording identity proof. Existing parser and recording helpers remain the single owners of validation, digesting, and fixture setup.

| Rung | Result |
| --- | --- |
| `sdlc/scripts/install` | exit 0 |
| `sdlc/scripts/lint` | exit 0, ratchet `17036/17036` |
| `sdlc/scripts/test` | exit 0 |
| `sdlc/scripts/spec` | exit 0 |

The coordinator ran the ladder with the key and base address unset. No live call ran.
