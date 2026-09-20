---
flow: build
priority: 46
opens: specification/recording.md crates/thinkthen-core/src/recording.rs
---

# 0030: Name only the trusted recording schema

Status: ready

## Outcome

The recording contract says that a foreign-schema refusal may name the trusted fixed schema `thinkthen.recording/1` and never the untrusted schema read from the entry.

## Current Facts

Finding 3 in `sdlc/issues/2026-09-19-hands-on-test-pass-one.md` records the mismatch. `specification/recording.md` says the refusal names no schema. The binary exits 5 and says that this version reads `thinkthen.recording/1`, without repeating the entry's schema. The core error comment also incorrectly says that the message names no schema.

The current proof already covers the full boundary. A core unit test pins the exact sentence and proves a hostile schema reaches neither display nor debug output. The shared secrecy integration test puts hostile text in every entry field and drives the refusal through every command, both output views, and document and record modes. It pins exit 5, the trusted schema sentence, no added replay request, and no evidence marker in the diagnostic.

## Scope

- Apply ADR 0024's trusted-schema decision.
- Correct `specification/recording.md` to say that the refusal may name only the trusted fixed schema and never repeats the schema the entry named.
- Correct the `EntryError::Schema` comment to distinguish the trusted fixed schema in the diagnostic from the untrusted entry field.

Excluded: changing the diagnostic, exit code, schema value, entry parser, replay behavior, or tests; accepting another schema; and every other finding in the first hands-on report.

## Acceptance

- The recording page and core error comment state the trusted and untrusted schema boundary accurately.
- The exact core hostile-schema test and shared secrecy integration sweep pass unchanged. Their existing coverage is direct and complete, so this ticket adds no redundant test.
- The ratchet is unchanged, and the whole ladder passes with the key and base address unset.

## Dependencies

ADR 0024, decided with this ticket. Ticket 0004 is landed; it introduced the recording schema and refusal. Ticket 0029 is the landed sequencing predecessor in the plan and is not a technical dependency.

## Complexity

- Contract score: 2
- State and timing score: 1
- Reach score: 1
- Proof score: 2
- Cost of error score: 1
- Total: 7
- Minimum level floor: none
- Final level: 3
- Reasons: ADR 0024 changes a Settled public contract around persistent recording files; the correction reaches the public page and the core error contract; existing proof uses hostile untrusted fields, exact output, all commands and modes, request counts, and leak checks; wrong wording would misstate the privacy boundary to users but remains locally correctable.
- Selected model: `gpt-5.6-sol` with medium reasoning

## Review

- Design review: accepted. The reviewer confirmed ADR 0024's trusted-versus-untrusted boundary, the comment-and-page-only scope, the existing exact unit and command-wide secrecy proof, and the level 3 Sol Medium route.
- Code review: pending.
