---
flow: build
priority: 46
opens: crates/thinkthen-core/src/backend.rs crates/thinkthen/tests/backend/address.rs crates/thinkthen/tests/backend/recordings.rs specification/backends.md sdlc/ratchet.json
---

# 0031: Canonicalize DNS host case

Status: landed

## Outcome

Equivalent ASCII case spellings of one DNS host resolve to the same URL and recording digest, while every other accepted address byte keeps its current meaning.

## Current Facts

Finding 4 in `sdlc/issues/2026-09-19-hands-on-test-pass-one.md` records that scheme case is normalized but host case is not. `http://LOCALHOST` and `http://localhost` both pass the case-insensitive loopback rule, yet their resolved URLs and recording digests differ. `specification/backends.md` promises one digest across equivalent address case without stating the host exception.

Ticket 0019 explicitly chose to keep host case because normalization changes older mixed-case recording identities. ADR 0025 overturns that choice. A current inventory found 1,059 committed recording entries and only two URL values: 1,057 hosted entries at `https://api.typesafe.ai/v1/systemone` and two loopback fixtures at `http://127.0.0.1:8721/v1/systemone`. All are already canonical.

## Scope

- Apply ADR 0025 in the backend address parser. Lowercase ASCII letters in the unbracketed host after the existing validation succeeds.
- Preserve scheme normalization, path bytes and case, port spelling, bracketed IPv6 text, and all current accepted and refused address classes. Lowercase ASCII punycode labels. Preserve percent escapes in a host exactly and leave non-ASCII bytes unchanged.
- Update `specification/backends.md` to state the host normalization and its boundaries.
- Add focused parser cases that separate the host from the surrounding URL, plus one local record-and-replay case proving `LOCALHOST` and `localhost` share the same URL, entry filename, and answer without a replay request or key.

Excluded: DNS resolution, host validation, Unicode case folding, IDNA conversion, percent decoding or canonicalization, IPv6 canonicalization, port normalization, path normalization, a fallback for old mixed-case entries, changing any refusal, and changing committed recordings.

## Acceptance

- An exact parser case resolves `HTTPS://XN--BCHER-KVA.ExAmPlE:00443/MiXeD/%2F` to `https://xn--bcher-kva.example:00443/MiXeD/%2F/systemone`. It proves DNS and punycode host letters become lowercase while scheme normalization, leading-zero port, mixed-case path, and path percent escape keep their rules.
- Exact boundary cases preserve `[2001:DB8::A]`, resolve host `MiXeD%2EHoSt` to `mixed%2Ehost`, and resolve host `BÜCHER.ExAmPlE` to `bÜcher.example`. They prove bracketed IPv6 and host percent escapes stay byte-for-byte fixed, non-ASCII host text gets no case folding, and literal ASCII host letters become lowercase. Existing accepted and refused address tables remain green.
- One loopback integration flow records through one `LOCALHOST` spelling and replays through another case spelling with no key. It observes one canonical lowercase URL and digest-named file, one setup request, zero replay requests, and the recorded answer.
- The existing pinned built-in digest stays byte-for-byte unchanged. No committed recording file changes. The implementation record repeats the repository inventory and the spec rung replays the committed examples with no key or network.
- The ratchet equals the measured total, and the whole ladder passes with the key and base address unset.

## Dependencies

ADR 0025, decided with this ticket. Ticket 0019 is landed; this ticket overturns only its recorded host-case choice and preserves its address safety rules. Ticket 0030 is the landed sequencing predecessor in the plan and is not a technical dependency.

## Complexity

- Contract score: 2
- State and timing score: 1
- Reach score: 1
- Proof score: 2
- Cost of error score: 1
- Total: 7
- Minimum level floor: level 3, because recording identity is shared durable state
- Final level: 3
- Reasons: ADR 0025 overturns a prior choice and changes a Settled URL and recording identity contract; recording folders are persistent state; the change reaches one parser, its public page, and replay integration; proof must separate hostname bytes from scheme, port, path, IPv6, percent escapes, punycode, and 1,059 committed entries; a wrong boundary can cause replay misses or repeat paid work but is locally recoverable.
- Selected model: `gpt-5.6-sol` with medium reasoning

## Review

- Design review: accepted after the minimum level floor was raised to level 3 for shared durable recording identity. The reviewer accepted the byte boundaries, committed-recording inventory, explicit external compatibility cost, focused parser matrix, and local record-and-replay proof.
- Code review: accepted after one documentation correction. The reviewer confirmed the parser boundaries, recording identity, committed inventory, pinned digest, and helper locality. It found that an older sentence saying every HTTPS base was untouched contradicted host normalization; the page now says the clear-text restriction allows every HTTPS host.
