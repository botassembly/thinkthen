---
flow: build
priority: 88
opens: crates/thinkthen/src/recorder.rs crates/thinkthen-core/src/recording.rs crates/thinkthen/tests/backend specification/recording.md specification/records.md sdlc/ratchet.json
---

# 0026: Keep one response per recording entry

Status: landed

## Outcome

A recording folder never silently changes the response for an existing request digest. Repeating the same exchange succeeds, a different response fails safely, and concurrent writers cannot replace the winner.

## Current Facts

The recorder writes a private temporary file and renames it over the digest path. Two identical requests that receive false then true print both answers during the recorded run, leave only the true response, and replay as true then true. The current page promises both same-answer replay and newest-answer replacement. ADR 0020 resolves the conflict in favor of immutable content-addressed evidence. Existing entries, names, schema, and replay lookup stay valid.

## Scope

- Install a complete private entry without replacing an existing digest path. Use a standard-library filesystem operation and add no dependency.
- When the final entry already exists, validate it against the request. Compare its stored JSON response with the response extracted from the newly serialized entry. Whitespace outside the backend's JSON value and formatting around the v1 envelope do not distinguish responses. Return a new local conflict failure when the stored responses differ.
- Apply the same rule when another worker or process wins the path between the first check and installation.
- Attempt to remove the temporary file on every returned result. Preserve the current folder and entry modes and refusal of damaged entries. A process crash may still leave its complete private dot-prefixed temporary file.
- Rewrite the replacement rule in `recording.md`, clarify successful-run replay and cache behavior in `recording.md` and `records.md`, and state that new writes require hard-link support in the recording folder.

Excluded: a transcript schema, occurrence numbers, changing digests or entry JSON, locking a whole folder, deduplicating live requests before they are sent, and automatic deletion or refresh.

## Acceptance

- A red integration test records two identical requests that receive different valid answers under `--jobs 1`. The run stops at the second record with exit 5, prints only the first answer, leaves the first response intact, and names no response content in the error.
- Recording the same request and response again, including a valid response padded with outer whitespace and an existing entry with a reformatted envelope, succeeds and leaves byte-identical entry contents. A meaningfully different padded response conflicts.
- A deterministic test starts two separate command processes against the same folder, URL, and request. A barrier holds both backend responses until both processes have missed the entry; the two responses are valid and different. Exactly one process installs and exits successfully. The other exits 5 with the fixed non-secret conflict. The final bytes match the winner, the entry mode is `0600`, and no temporary name remains.
- A damaged existing entry keeps its current safe error and is never replaced. A directory forced at the final entry path exercises the failed-install path. Success, idempotence, conflict, damaged-entry, and forced-error cases each leave no temporary name after returning.
- Every committed recording replays under the unchanged digest and schema. The source ceiling equals the measured total, and the full ladder passes.

## Dependencies

ADR 0020, decided with this ticket. Ticket 0025 is landed.

## Complexity

- Contract score: 2
- State and timing score: 2
- Reach score: 1
- Proof score: 2
- Cost of error score: 2
- Total: 9
- Minimum level floor: level 4
- Final level: 4
- Reasons: this changes a settled durable-format rule; correctness depends on atomic races across workers and processes, failure cleanup, and old-entry compatibility; a wrong write can replace private evidence that cannot be reconstructed.
- Selected model: `gpt-5.6-sol` with medium reasoning

The change is irreducible at this level. Splitting installation from conflict semantics would either permit silent replacement or introduce a format state no accepted contract explains.

## Review

- Design review: accepted after correction. The reviewer required an actual two-process race, explicit hard-link filesystem support, honest crash cleanup, and deterministic cleanup checks on every returned path. It confirmed the immutable first-complete rule, compatibility claim, and irreducible level 4 route.
- Code review: accepted after one correction. The first pass found that outer whitespace around a valid backend JSON value made an identical second response conflict. The corrected implementation compares the stored response values extracted from both valid entries, and a new test proves that envelope formatting does not affect identity or rewrite the winner. The reviewer then accepted the atomic install, process race, cleanup, privacy, compatibility, and error behavior with no remaining finding.
