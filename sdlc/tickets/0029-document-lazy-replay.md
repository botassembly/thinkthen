---
flow: build
priority: 46
opens: specification/recording.md crates/thinkthen/tests/backend/recordings.rs sdlc/ratchet.json
---

# 0029: Document lazy replay

Status: ready

## Outcome

The recording page says that replay reads only the digest-named entry it needs. An unrelated file in the folder is ignored, while a damaged requested entry remains a safe local failure at exit 5.

## Current Facts

Finding 2 in `sdlc/issues/2026-09-19-hands-on-test-pass-one.md` records the mismatch. `specification/recording.md` says any file under the replay folder that is not an entry causes exit 5. The implementation instead derives one entry name from the exchange digest and reads only that path. The hands-on test confirmed that a stray `notes.txt` beside a valid requested entry is ignored and replay succeeds.

When that requested entry is damaged, replay already exits 5. Its diagnostic names the digest-derived entry and the JSON line and column without repeating the damaged text. Lazy reading is the intended behavior; the page is wrong.

## Scope

- Apply ADR 0023's digest-directed replay decision.
- Change `specification/recording.md` to say replay reads only the digest-named entry for the requested exchange and ignores unrelated files in the folder.
- Keep the existing rule for a requested entry that cannot be parsed: exit 5, with a diagnostic that names the entry and safe JSON line and column but repeats none of its contents.
- Extend the existing replay integration coverage with one fixed flow: a valid requested entry still replays with an unrelated non-entry beside it, then fixed malformed bytes at the requested path produce the exact safe exit-5 diagnostic. The test uses no key and counts zero requests.

Excluded: scanning or validating the folder, changing entry names or digests, changing replay misses, changing recording writes, and every other finding in the first hands-on report.

## Acceptance

- `specification/recording.md` describes lazy replay and the requested-entry failure accurately.
- One deterministic integration flow proves that an unrelated non-entry is ignored and that fixed malformed bytes at the requested digest path exit 5 with the exact entry name, line, and column. It proves zero requests and no echo of the malformed contents.
- Existing replay, miss, recording, and secrecy checks remain green. The ratchet equals the measured total, and the whole ladder passes with the key and base address unset.

## Dependencies

ADR 0023, decided with this ticket. Ticket 0004 is landed; it introduced digest-named recording entries and lazy replay. Ticket 0022 is only the landed sequencing predecessor in the plan and is not a technical dependency.

## Complexity

- Contract score: 2
- State and timing score: 1
- Reach score: 1
- Proof score: 2
- Cost of error score: 1
- Total: 7
- Minimum level floor: none
- Final level: 3
- Reasons: ADR 0023 makes a public compatibility decision between digest-directed access and whole-folder validation; recording folders are persistent user state; the decision reaches the replay contract, its integration owner, and the public page; proof combines an unrelated hostile file, exact requested-entry failure bytes, zero requests, and no content echo; a wrong rule misleads users about private folder contents but remains locally correctable.
- Selected model: `gpt-5.6-sol` with medium reasoning

## Review

- Design review: accepted after ADR 0023 was added. The reviewer required a recorded compatibility decision because the page is Settled, raised the ticket to level 3 for persistent recording state and hostile-file proof, and accepted the combined success-and-failure integration flow.
- Code review: pending.
