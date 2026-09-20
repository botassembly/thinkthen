---
flow: build
priority: 46
opens: crates/thinkthen-core/src/backend.rs crates/thinkthen/tests/backend specification/backends.md sdlc/ratchet.json
---

# 0022: Name the refused address rule

Status: landed

## Outcome

A refused base address says which address rule failed without repeating the address, and every refusal happens before a request.

## Current Facts

The first hands-on test and the independent review of ticket 0019 found the same defect. `sdlc/issues/2026-09-19-hands-on-test-pass-one.md` and `sdlc/issues/2026-09-19-small-leftovers-from-the-security-ticket.md` hold the evidence.

On current main, an invalid scheme, user information, an empty port, a signed port, a port above 65535, a query, and a fragment all exit 2 with this sentence:

> a base address begins with `http://` or `https://` and carries no user information

The refusals are safe and occur before a request, but the sentence often names the wrong rule. `specification/backends.md` already separates the rules and requires the refusal to name the one that failed.

## Decisions

- An invalid scheme says: `a base address begins with http:// or https://`. The rendered diagnostic keeps the two schemes in backticks.
- User information says: `a base address carries no user information`.
- An empty, signed, or out-of-range port says: `a port is digits naming a number from 0 to 65535`.
- A query or fragment says: `a base address carries no query or fragment`.
- Every diagnostic omits the refused address and preserves exit 2. Parsing still stops before key access or a network request.
- `specification/backends.md` states the four exact rules and their safe refusal behavior.

Excluded: blank-base wording, clear-text host restrictions, host normalization, path handling, any accepted-address change, and every other finding in the two issue reports. Those findings remain there until a smallest useful ticket becomes active.

## Acceptance

- Exact integration cases pin the complete sentence and exit 2 for an invalid scheme, user information, empty port, signed port, port 65536, query, and fragment.
- Each case uses a counted local listener where applicable and proves zero requests. No test reaches an outside address.
- No standard output, standard error, plan, or debug text repeats the refused base. The shared refusal and secrecy coverage includes every new address-error path.
- Valid address cases and the clear-text loopback rule remain unchanged.
- `specification/backends.md` matches the four refusal categories.
- The ratchet equals the measured total, the whole ladder passes, and an independent reviewer checks error ownership, no-echo behavior, and the zero-request boundary.

## Dependencies

Ticket 0019 is landed. It introduced the address validation and the safe but overly broad refusal this ticket splits by rule.

## Complexity

- Contract score: 1
- State and timing score: 0
- Reach score: 1
- Proof score: 1
- Cost of error score: 1
- Total: 4
- Minimum level floor: none
- Final level: 2
- Reasons: one settled address rule needs four precise public diagnostics; the change stays in the backend parser, its integration coverage, and one specification page; exact no-request and no-echo cases provide direct proof; a wrong split misleads a user or risks echoing an address, but is local and reversible.
- Selected model: `gpt-5.6-luna` with high reasoning

## Review

- Design review: accepted after the original twelve-finding batch was split. The reviewer required one address-parser outcome, exact safe sentences, unchanged positive and loopback cases, and direct no-request proof. It confirmed complexity level 2 and the Luna High route.
- Code review: accepted after one remediation. The reviewer found that three integration fixtures appended a second port and therefore missed the literal empty, signed, and out-of-range forms. It also found that key-present cases did not prove address validation happened before key access. The corrected matrix uses the seven exact forms with and without a key, pins each complete sentence and exit code, and retains the counted zero-connection listener.
