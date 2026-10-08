# 0466: Refuse contradictory R complete inputs

Status: OPEN. The review traced mixed record/file inputs to a branch that silently discards the other source.

Milestone: 0.2

Owner: builder.

Reviews: revision e71fa0b01, accept

Reviews: revision ce732929a, accept

## Outcome

R complete calls refuse contradictory fields before reading files or sending requests; accepted input still selects exactly the caller’s evidence.

## Evidence

- Starts from: 0462 SDK review; libraries/r/thinkthen/src/rust/src/complete/inputs.rs::Input allows records, paths, options and jsonl for either kind, while prepare selects only one branch.
- Keeps: Public typed result/batch behavior, native validation, original-record retention, per-record context/options, secrecy and exact request counts.
- Changes: Validate cross-kind input fields at admission, including distinctions between omitted fields and explicitly supplied fields. Verify public R constructors and JSON conversion; reject contradictory record/file sources with the normal safe usage error.
- Proof: Fresh ticket/code review; public R tests cover both conflicting kinds, zero request counts and no file reads, plus valid records/files and complete batch behavior. Run applicable tests/lint/policy. No hosted workflow or paid call.
- Defers: No new input format, R API redesign, per-language proof records or release management.
