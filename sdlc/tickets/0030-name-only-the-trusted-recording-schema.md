---
flow: build
priority: 46
opens: specification/recording.md crates/thinkthen-core/src/recording.rs crates/thinkthen/tests/backend/secrecy.rs sdlc/ratchet.json
---

# 0030: Name only the trusted recording schema

Status: landed

## Outcome

The recording contract says that a foreign-schema refusal may name the trusted fixed schema `thinkthen.recording/1` and never the untrusted schema read from the entry.

## Current Facts

Finding 3 in `sdlc/issues/closed/2026-09-19-hands-on-test-pass-one.md` records the mismatch. `specification/recording.md` says the refusal names no schema. The binary exits 5 and says that this version reads `thinkthen.recording/1`, without repeating the entry's schema. The core error comment also incorrectly says that the message names no schema.

The core unit proof is complete: it pins the exact sentence and proves a hostile schema reaches neither display nor debug output. The shared secrecy integration sweep is incomplete. Its `VERBS` matrix covers `decide`, `choose`, and `score`, while `filter` and `rank` are now built. Its comment claims every command participates, and `rust-standards.md` requires a secrecy claim to cover every command and failure path.

## Scope

- Apply ADR 0024's trusted-schema decision.
- Correct `specification/recording.md` to say that the refusal may name only the trusted fixed schema and never repeats the schema the entry named.
- Correct the `EntryError::Schema` comment to distinguish the trusted fixed schema in the diagnostic from the untrusted entry field.
- Extend the shared secrecy matrix so a hostile foreign-schema replay reaches every built command. Keep the existing `decide`, `choose`, and `score` coverage. Cover `filter` and `rank` under both `--lines` and `--jsonl`, in the bare and `--details` views. Each new case preserves the current refusal and sends no replay request.

Excluded: changing the diagnostic, exit code, schema value, entry parser, or replay behavior; broadening unrelated backend paths or usage-error matrices; accepting another schema; and every other finding in the first hands-on report.

## Acceptance

- The recording page and core error comment state the trusted and untrusted schema boundary accurately.
- The exact core hostile-schema test passes unchanged.
- The shared secrecy integration sweep covers the hostile foreign-schema route for all five built commands. `filter` and `rank` each run under `--lines` and `--jsonl`, with and without `--details`. Every case exits 5, includes the trusted fixed-schema sentence, excludes the key, hostile schema, and evidence marker from their forbidden channels, and proves replay added no request.
- The matrix remains the single owner of this command-wide secrecy claim. The ratchet equals the measured total, and the whole ladder passes with the key and base address unset.

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
- Reasons: ADR 0024 changes a Settled public contract around persistent recording files; the correction reaches the public page, core error contract, and shared integration matrix; proof uses hostile untrusted fields, exact output, every built command's applicable modes, request counts, and leak checks; wrong wording would misstate the privacy boundary to users but remains locally correctable.
- Selected model: `gpt-5.6-sol` with medium reasoning

## Review

- Design review: pending after the prior review's command-wide proof claim was disproved.
- Code review: accepted after a return to design. The reviewer found that the original secrecy matrix covered only `decide`, `choose`, and `score`, contrary to the ticket's every-command claim. The revised proof adds `filter` and `rank` under both framings and both views, with exact exit, trusted-schema, marker-absence, and request-count checks. The final review found no remaining defect.
