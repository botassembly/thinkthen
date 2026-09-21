# 0041: Build the flagship triage pipeline

Date: 2026-09-21

Status: recording complete; closure review and landing pending

## Built locally

The draft has six fictional support tickets in TSV, one question set with three questions on `/body`, one fail-closed `jq` policy, and one Bash pipeline. The pipeline reserves the caller's output directory before it asks once per ticket. It applies policy once and builds detailed JSONL audit rows in hidden staging inside that directory. After every row and split succeeds, three final file moves place `draft.jsonl`, `block.jsonl`, and `review.jsonl` in the reserved directory. Successful return marks publication.

The public policy vocabulary is fixed by the accepted ticket. Actions are `draft`, `block`, and `review`. Reasons are `unresolved`, `credential_request`, `out_of_scope`, `urgent`, and `routine`. Queues are `billing`, `shipping`, `account`, and `other`. `reviewed_action` is only a person's recorded decision.

The agent set urgency 1, the middle of the three-level scale, as the inclusive `urgent` boundary. The middle level says delay can cause a concrete customer problem. Changing this boundary now requires a new question measurement and new page recordings.

## Red and green evidence

`sh transforms/triage/test.sh` first failed because `transforms/triage/triage.jq` did not exist. After the transform was written, it passed eight routing cases and six malformed-input refusals. The cases cover every rule, precedence, a null for each answer kind, the exact midpoint, missing and extra answer names, an unknown queue, hostile strings, and exact preservation of the input row.

`demos/16-triage-pipeline/self-test` passes the exact command shape, one policy application, three populated JSONL outputs, full-row preservation, agreement with six pre-registered `reviewed_action` values, and a request disclosure check through the real binary's dry run. It also proves destination reservation, one judging winner, three final file moves, exact failure statuses, and cleanup after judgment, validation, move, and signal failures.

## Recording and closure

The cold marketing read filed [`2026-09-21-page-16-first-result-is-not-runnable.md`](../issues/2026-09-21-page-16-first-result-is-not-runnable.md). The page-local runner and reviewed recording resolved it. The code reviewer accepted the corrected behavior and tests after the durable prose matched them.

The first code review rejected four findings. Exact answer-name validation, the recording job boundary, and the README order sentence were corrected. The review disproved the first publication assumption: shell `mv` does not portably combine an atomic directory rename with no replacement. The accepted revision reserves the caller's name with `mkdir`, stages inside that owned directory, removes it after handled failures and catchable interruptions, and makes successful return the publication boundary. Two synchronized runs prove that only the reservation winner judges. Separate failures during judgment, validation, and each final file move preserve their status and remove the owned output. The recording and page-local runner resolved the cold-read issue.

The cold read found that the first result exposed temporary-directory setup and could not run before the recording existed. The small page-local `run` script now owns and cleans that temporary output, leaving one command in the first block.

The authorized coordinator ran `sdlc/scripts/live --max-tokens 5000 demos/16-triage-pipeline/record.sh` once. The six mode-0600 recordings have schema `thinkthen.recording/1`, model `jev-1.13.0`, the hosted System One URL, 2,916 input tokens, and 468 output tokens. The job matched all six recorded decisions. The shared ledger moved from 20,497,918 to 20,502,918 charged tokens.

A direct replay with the key and base-address variables unset printed `draft=2`, `block=1`, and `review=3`. Schema, digest, and request checks passed through replay. A credential-marker scan found none, and the recording job left no live output. Page 16 is green. Pages 04 and 07 left only after pages 19, 13, 14, and 16 demonstrably held every named lesson. Closure review and landing remain.
