# 0398 slice C: Enforce the rehearsed release commit

Date: 2026-10-04
Status: implementation accepted; landing pending; hosted proof remains open
Starting revision: `b780323909505450743af30d8bea2ee07a51e65b`
Accepted design: `ebfa557cae241ee134b52a25a24af40854e511c8`
Ticket: `sdlc/tickets/0398-release-safety.md`

## Behavior

Release resolve keeps its existing mode, ref, checkout and version guards. It then requires a successful completed workflow dispatch on the exact checked-out SHA from main or numeric release/X.Y. Rehearse resolve makes no metadata query. The helper selects release.yml through the workflow API and checks every eligibility field again, including workflow path, repository prefix and branch suffix. It parses the complete paginated JSON stream before accepting evidence. Command errors, timeout and malformed or incomplete responses refuse. Failed resolve emits no outputs.

Only resolve gains actions read permission and its source step receives GH_TOKEN. The validator pins resolve's exact permission map, checkout, source step and three output mappings. It refuses skipped or ignored resolve, command bypasses and token expansion into another build job. The eighteen jobs, five command targets, four Unix binding families and install-check dispatch remain.

## Proof

The focused release archive proof passes. The resolve edge table pins exact gh GET arguments, query counts, output lines and refusal sentences. It covers eligible branches and path forms, wrong SHA, a successful tag run, wrong event/status, missing fields, conflicting workflow/repository/ref paths, empty results, second-page success, malformed or truncated streams, command failure after partial success output, timeout and a missing executable. Original ref and checkout refusal sentences remain pinned. Invalid version and mode refuse before a query.

The aggregate workflow self-test passes 110/110 cases. All prior cases remain, including slice B's independent 38-case table. The existing actions-write plant now expects both the new resolve refusal and its original dispatch permission refusal. Real-file workflow validation and ticket validation pass. Required scoped lint passed with exit 0 on the implementation committed as `0289c5a65fbb81f1f6a7b3191ad06f1024e65f32`. The run used a 12 GiB memory and 1 GiB swap scope, two offline build jobs and the coordinator-verified wrapper for the owned local file-source fixture. The ambient installed tool PATH remained available. Policy, format, Clippy, documentation and inventory checks passed. The private-name check counted 35 names with no matches. Local receipts are `target/0398c-lint.log` and `target/0398c-checks/workflows-final.log`. No full checkpoint was run for this slice.

The coordinator's manual primary API queries found rehearsal 37126990511 on `abac3ce61bf1188b40cbc3c2ef0a0589eb247c86`, from release/0.1 with the bare release workflow path. For `08328c9c04574719b9e93900dd8fa46ad3b645b4`, successful completed dispatch 37130570517 has head_branch v0.1.2. It is a release tag run and supplies no eligible branch rehearsal. Offline fixtures preserve both records. The implementation performed no authenticated remote query or dispatch.

## Remaining proof

Slice A/B hosted proof and the exact approved rehearsal remain open. This guard does not prove trusted publishers or checkpoint coverage. The coordinator will request an exact reviewed run after the candidate lands. The doc-test issue retains checkpoint enforcement. Service failure, deleted history or the API search limit can require another rehearsal. Ian's release approvals remain.

## Review correction

Fresh High code review found that a workflow-level environment could pass the validator and make GH_TOKEN available to build jobs. The validator now pins the accepted absence of workflow-level env. Two plants modify the actual release workflow object with inherited GH_TOKEN and BASH_ENV and require the exact resolve refusal. Existing job-specific token checks remain. Focused workflow proof passes 112/112 cases, including both new plants and all prior cases. Required scoped lint passed with exit 0 on correction `cde3c5f0913762f3b9ba98025147d49ea174cdcd` under the same bounded offline two-job wrapper. Policy, format, Clippy, documentation and inventory checks passed. The private-name check found no matches. Local receipts are `target/0398c-lint-env-fix.log` and `target/0398c-checks/workflows-env-fix.log`. Fresh High code review accepted the whole corrected candidate at `78c7fe748bc33dd2b102dbd175f751cdc40b37c7`. Independent offline replay passed all 112 cases and real-file workflow, ticket and diff checks. No actionable findings remain.
