# 0029: Document lazy replay

Branch `ticket/0029-document-lazy-replay`. Built 2026-09-20.

## What landed

ADR 0023 and the recording page now say that replay derives one digest filename and reads only that requested entry. It does not scan the folder. An unrelated file is ignored, while a missing or damaged requested entry keeps its existing local failure.

No production behavior changed. The existing implementation already followed this rule.

## Red then green

The settled recording page claimed every non-entry file in the folder caused exit 5. The corrected page states the digest-directed behavior.

One integration flow records an entry, plants fixed malformed private text in an unrelated `notes.txt`, and replays successfully without a key or request. It then writes the same malformed bytes to the requested digest path and pins exit 5, the exact entry name, line 3 column 0, empty standard output, and absence of the planted text. The counted setup request is drained before replay, and both replay attempts leave the request count at zero.

## Review

The design reviewer required ADR 0023 because this change corrects a Settled specification. It also raised the ticket to level 3 for the compatibility decision, persistent recording state, and hostile-file proof. It accepted digest-directed access because one exchange stays independent of unrelated files and replay reads less private material.

The code reviewer accepted the final diff with no findings. It confirmed the test's no-key environment, request-count semantics, exact safe diagnostic, documentation, and exact ratchet.

## Choices made where the ticket was silent

Ian can overturn this choice.

- **Replay is not a folder audit.** A bad unrelated file is discovered only if its digest becomes the requested entry. This keeps replay bounded to the exchange the caller asked for.

## Gates and size

The source ceiling is 16,875 measured Rust lines, up from 16,836. The increase is one binary integration flow for the two sides of the contract. It extends the existing recording test owner and adds no production path.

| Rung | Result |
| --- | --- |
| `sdlc/scripts/install` | exit 0 |
| `sdlc/scripts/lint` | exit 0, ratchet `16875/16875` |
| `sdlc/scripts/test` | exit 0 |
| `sdlc/scripts/spec` | exit 0 |

The coordinator ran the ladder with the key and base address unset. No live call ran.
