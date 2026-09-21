# 0041: Build the flagship triage pipeline

Date: 2026-09-21

Status: draft complete; cold read, code review, recording, and landing pending

## Built locally

The draft has six fictional support tickets in TSV, one question set with three questions on `/body`, one fail-closed `jq` policy, and one Bash pipeline. The pipeline asks once per ticket, keeps detailed JSONL audit rows, applies policy once, fills `draft.jsonl`, `block.jsonl`, and `review.jsonl` in one temporary sibling, then publishes that directory with one rename.

The public policy vocabulary is fixed by the accepted ticket. Actions are `draft`, `block`, and `review`. Reasons are `unresolved`, `credential_request`, `out_of_scope`, `urgent`, and `routine`. Queues are `billing`, `shipping`, `account`, and `other`. `reviewed_action` is only a person's recorded decision.

The agent set urgency 1, the middle of the three-level scale, as the inclusive `urgent` boundary. The middle level says delay can cause a concrete customer problem. Ian can overturn this before recording.

## Red and green evidence

`sh transforms/triage/test.sh` first failed because `transforms/triage/triage.jq` did not exist. After the transform was written, it passed eight routing cases and five malformed-input refusals. The cases cover every rule, precedence, a null for each answer kind, the exact midpoint, an unknown queue, hostile strings, and exact preservation of the input row.

`demos/16-triage-pipeline/self-test` passes the exact command shape, one policy application, three populated JSONL outputs, full-row preservation, agreement with six pre-registered `reviewed_action` values, a request disclosure check through the real binary's dry run, an existing destination, one final rename, and cleanup after an injected failure.

## Still pending

Page 16 remains red. A cold marketing reader and an independent code reviewer inspect this pushed draft next. After both accept it, one authorized run uses the ticket's 5,000-token cap to create the recording. The result then gets checked for request count, digests, model agreement, modes, credentials, and keyless replay before the page turns green. Pages 04 and 07 remain until then.
