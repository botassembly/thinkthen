# 0081: Build the shared relation foundation

Status: redesigned 0081 accepted for implementation; accepted 0088 waits for landed 0081.

## Trial result

The original 0081 combined the shared relation migration, complete public `relate` command, Option A result, partial failure, documentation, and the full secrecy matrix. Sol rejected design round 1 for missing shared ownership, false wildcard and H claims, an unresolved detailed shape, and stale budgets. Luna remediation pass 1 assigned shared ownership and incorporated Ian's Option A ruling.

Sol rejected design round 2 because budgets omitted near-limit splits and full proof; Option A lacked exact outer, directional, H, and failure unions; profile fallback and field mapping were ambiguous; H state was not byte-exact; and acceptance omitted the complete gate and secrecy matrix. Sol also identified partial-failure exit behavior as an unresolved public decision. Luna remediation pass 2 incorporated the resulting combined-design corrections after Ian ruled partial output at exit 6.

Ian then authorized splitting the oversized ticket. The Luna trial stopped at its two-remediation limit. No implementation started. Redesigned 0081 owns only the independently testable shared relation foundation and recognition compatibility. New ticket 0088 owns the public `relate` command and all outward contracts. Sol Medium now drives both tickets, with separate independent Sol design and code review.

Carver independently rejected commit `5828fece` because 0081 omitted `specification/recognize.md` ownership and exact per-concrete fallback proof, while 0088 contradicted settled question-file profile identity, file exit codes, empty-input behavior, and dry-run provenance. The remediation adds those owners and proofs, keeps saved calibration `profile` distinct from runtime `--profile FILE`, includes both identity sources in their proper schemas, and changes no Ian ruling, budget, dependency, product code, or surface.

Carver independently accepted both corrected designs at commit `113642b2`. The review confirmed 0081 at level 2 with a 1,500-line gross ceiling and 0088 at level 3 with a 2,300-line gross ceiling. Only 0081 may begin implementation. Ticket 0088 remains blocked on landed 0081.

No product code, public surface, live call, paid call, focused gate, complete gate, targeted repair, reopened implementation defect, or trustworthy elapsed start-to-accept time exists. No separate 0088 record exists because this record preserves the split review and remediation; the 0088 ticket is its current authority.
