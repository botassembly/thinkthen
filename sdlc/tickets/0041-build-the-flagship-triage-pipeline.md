---
flow: build
priority: 48
opens: README.md demos transforms sdlc/issues sdlc/planning
---

# 0041: Build the flagship triage pipeline

Status: in progress

## Outcome

How-to 16 shows a believable support-ticket pipeline that fits on one slide. One `annotate` request per ticket produces three judgments, and one tested `jq` policy writes complete JSONL audit rows to `draft.jsonl`, `block.jsonl`, or `review.jsonl`.

## Current facts and decisions

Nineteen how-tos are green, and page 18 is the only page still coming. Page 16 now leads the README front window and is the page the talk and site consume. Pages 04 and 07 left after page 16 absorbed their review-queue, flat-row, null, table-input, and collision lessons.

Ian chose a tested `jq` file over a rules block in a question set for version 0.1. ADR 0013 records that ruling. The policy uses `action` for its automated route. `reviewed_action` means a person's recorded decision and uses the same `draft`, `block`, or `review` words. The later monitor must keep that field name and meaning.

## Scope

- Write six fictional TSV support tickets that read like real customer messages at a glance. They carry a pre-registered `reviewed_action` for testing, and no identifying or private data.
- Ask three questions about `/body` in one request: whether the message asks for a credential, which queue owns it, and its urgency on three written levels. The request must not include `reviewed_action`.
- Add one policy transform under `transforms/`. It preserves the full detailed row and adds `policy: {action, reason}`. Actions are `draft`, `block`, and `review`. Reasons are `unresolved`, `credential_request`, `out_of_scope`, `urgent`, and `routine`. Queue labels are `billing`, `shipping`, `account`, and `other`. These exact example words are public inputs to the later transforms. It checks rules in the same order as the reasons above.
- Route unresolved to `review`, credential requests to `block`, out-of-scope and urgent tickets to `review`, and routine tickets to `draft`.
- Add one short pipeline script that runs `annotate --tsv --details --jobs 4`, applies the policy once, and writes complete JSONL rows to the three named files inside a caller-named output directory. It atomically reserves a new destination with `mkdir`, builds the files in a hidden staging directory inside it, and moves complete files into place only after every step succeeds. Concurrent invocations lose at `mkdir`. A successful return marks publication; callers do not read the directory while the command runs. A handled failure or catchable interruption removes the directory this invocation created. Uncatchable termination may leave it for the next run to refuse. Its visible command flow must fit on one slide.
- Turn how-to 16 green from a committed recording, put it first in the README front window and ADR 0018, and update the active plans. The site and talk live outside this repository and consume this page; this ticket changes no unseen copy. Remove pages 04 and 07 only after page 16 carries their remaining lessons, including the answer-name collision caution.

The first reviewed draft is committed and pushed before the live recording. A cold marketing read checks whether the six tickets look real, the first result is immediate, and the pipeline is clear without project history. Any finding lands under `sdlc/issues/` and is resolved before the page turns green.

No Rust, command option, output format, question-set rule language, CSV output, or TSV output enters this ticket. CSV and TSV remain input only; every streamed result stays JSONL.

## Acceptance

- A fake `thinkthen` self-test proves the exact `annotate` shape, one policy pass, JSONL framing, and full-row preservation. It refuses an existing file, directory, or symlink. Two synchronized invocations prove that only the `mkdir` winner judges. Injected failure during judgment, validation, and each of the three final moves leaves no output. Cleanup is armed only after this invocation creates the destination, preserves the failing status, and covers EXIT and catchable termination. Success leaves exactly `draft.jsonl`, `block.jsonl`, and `review.jsonl`, with no staging directory.
- Policy fixtures cover all five rules, precedence overlaps, `null` for each answer kind, missing or malformed fields, the exact midpoint, and hostile strings. Unknown input fails rather than drafting.
- The six pre-registered cases match `reviewed_action` exactly and populate every output. The page names that field as a person's decision and uses no competing term.
- The page stays within the how-to limits. Its first result needs no visible setup, and the complete `thinkthen | jq` flow is short enough for one slide. The cold reader checks the draft before recording, and every finding is resolved before green. The README and ADR 0018 put page 16 first in the seven-page front window, and the pages check pins the order.
- After review, run the six cases once through `sdlc/scripts/live --max-tokens 5000` with `--max-retries 0` and one cache folder. Stop on any mismatch. Validate recording digests, model agreement, file modes, request count, absence of credentials, and replay with the key unset.
- Page 16 proves one request per ticket, all three JSONL audit files, the fail-closed unresolved path, the disclosure boundary, and the question-set change cost. Its caution names the answer-name collision before page 07 leaves. The four repository rungs and `git diff --check` pass.

## Dependencies

None. `annotate`, TSV input, detailed rows, replay, and bounded jobs are on main. The live call waits for accepted design, implementation, code review, and the cold marketing read.

## Complexity

- Contract: 1
- State and timing: 2
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 7
- Minimum level floor: none
- Final level: 3
- Reasons: the public commands stay fixed, but this flagship page creates a public policy vocabulary, publishes a three-file audit set through one directory boundary, and needs hostile-input, partial-failure, disclosure, exact-routing, and live-recording proof.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if implementation exposes a command change, shared state, or a new public contract.

## Review

- Design review: accepted after two initial rejections and one implementation correction. The first revisions defined the page, vocabulary, paths, and proof. Implementation showed that ordinary `mv` can silently nest the run under a destination created after the precheck. The corrected portable design reserves the destination with `mkdir`, cleans handled failures, and defines successful process completion as the publication boundary. The reviewer accepted it without a new ADR.
- Code review: accepted after one rejection and remediation. The reviewer required exact answer names, found that GNU `mv` can nest the temporary directory if the destination appears after the precheck, separated the coordinator's live wrapper from the recording job, and found a stale README ordering sentence. The corrected reservation design and durable record passed re-review. Closure review remains before landing.

## Implementation

The local draft adds six fictional TSV tickets, a three-question set over `/body`, the fail-closed `triage` policy, and a pipeline that reserves a fresh destination and publishes three complete JSONL files on successful return. The policy test first failed because `triage.jq` did not exist, then passed eight routing cases and six safe refusals. The pipeline self-test covers the exact command, a single policy application, all three populated outputs, full rows, reviewed-action agreement, disclosure, destination ownership, and cleanup after handled failures.

The exact midpoint is urgency 1. It routes to `review` with reason `urgent`. The agent chose that inclusive boundary because the middle written level says a delay can cause a concrete problem. Changing it now requires a new question measurement and new page recordings.

Implementation disproved the first publication assumption. POSIX shell has no portable operation that atomically renames a directory while refusing any existing destination. The accepted correction creates the destination with `mkdir` before judgment, stages files inside it, removes it on handled failure, and defines successful return as publication. Callers must not read the directory while the process runs.

The coordinator alone invoked `sdlc/scripts/live --max-tokens 5000 demos/16-triage-pipeline/record.sh` once. The recording job used a temporary output directory, cleaned it on exit, and wrote only the shared cache. Direct keyless replay runs `triage` with `--replay recording/` and needs no new live reservation.

The one authorized command, `sdlc/scripts/live --max-tokens 5000 demos/16-triage-pipeline/record.sh`, completed once. Six mode-0600 entries use `thinkthen.recording/1`, `jev-1.13.0`, and the System One URL. They report 2,916 input and 468 output tokens. The durable reservation moved the shared ledger from 20,497,918 to 20,502,918 charged tokens. The recording job matched all six `reviewed_action` values, and a direct keyless replay printed two drafts, one block, and three reviews. No credential marker or live output remained.

Page 16 is green. Pages 19 and 13 hold page 04's band and coverage lessons, page 14 holds page 07's several judged columns, and page 16 holds the review pile, explicit null, complete flat input row, TSV-input/JSONL-output rule, disclosure boundary, and collision caution. Pages 04 and 07 therefore left. Ticket closure review and landing remain.
