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

Nineteen how-tos are green. Page 16 is the missing front-window page and the next item in the accepted plan. It is also the page the talk, site, and README lead with. Pages 04 and 07 remain only because page 16 has not absorbed their review-queue, flat-row, null, table-input, and collision lessons.

Ian chose a tested `jq` file over a rules block in a question set for version 0.1. ADR 0013 records that ruling. The policy uses `action` for its automated route. `reviewed_action` means a person's recorded decision and uses the same `draft`, `block`, or `review` words. The later monitor must keep that field name and meaning.

## Scope

- Write six fictional TSV support tickets that read like real customer messages at a glance. They carry a pre-registered `reviewed_action` for testing, and no identifying or private data.
- Ask three questions about `/body` in one request: whether the message asks for a credential, which queue owns it, and its urgency on three written levels. The request must not include `reviewed_action`.
- Add one policy transform under `transforms/`. It preserves the full detailed row and adds `policy: {action, reason}`. Actions are `draft`, `block`, and `review`. Reasons are `unresolved`, `credential_request`, `out_of_scope`, `urgent`, and `routine`. Queue labels are `billing`, `shipping`, `account`, and `other`. These exact example words are public inputs to the later transforms. It checks rules in the same order as the reasons above.
- Route unresolved to `review`, credential requests to `block`, out-of-scope and urgent tickets to `review`, and routine tickets to `draft`.
- Add one short pipeline script that runs `annotate --tsv --details --jobs 4`, applies the policy once, and writes complete JSONL rows to the three named files inside a caller-named output directory. The directory must not exist. The script builds one temporary sibling and renames that directory into place only after all three files are complete. Failure publishes no output directory. Its visible command flow must fit on one slide.
- Turn how-to 16 green from a committed recording, put it first in the README front window and ADR 0018, and update the active plans. The site and talk live outside this repository and consume this page; this ticket changes no unseen copy. Remove pages 04 and 07 only after page 16 carries their remaining lessons, including the answer-name collision caution.

The first reviewed draft is committed and pushed before the live recording. A cold marketing read checks whether the six tickets look real, the first result is immediate, and the pipeline is clear without project history. Any finding lands under `sdlc/issues/` and is resolved before the page turns green.

No Rust, command option, output format, question-set rule language, CSV output, or TSV output enters this ticket. CSV and TSV remain input only; every streamed result stays JSONL.

## Acceptance

- A fake `thinkthen` self-test proves the script's exact `annotate` shape, one policy pass, three output names, JSONL framing, full-row preservation, refusal of an existing destination, and one final directory rename. Injected failure before that rename leaves no destination or temporary directory.
- Policy fixtures cover all five rules, precedence overlaps, `null` for each answer kind, missing or malformed fields, the exact midpoint, and hostile strings. Unknown input fails rather than drafting.
- The six pre-registered cases match `reviewed_action` exactly and populate every output. The page names that field as a person's decision and uses no competing term.
- The page stays within the how-to limits. Its first result needs no setup, and the complete `thinkthen | jq` flow is short enough for one slide. A cold reader accepts the draft before recording. The README and ADR 0018 put page 16 first in the seven-page front window, and the pages check pins the order.
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

- Design review: accepted after two rejections. The first revision gives the three files one publication boundary, puts page 16 first in the repository's front window, retains the collision lesson, fixes the example vocabulary, and raises State and timing to 2. The second adds every opened path. The reviewer accepted the final design with no remaining finding.
- Code review: rejected once. The reviewer required exact answer names, found that GNU `mv` can nest the temporary directory if the destination appears after the precheck, separated the coordinator's live wrapper from the recording job, and found a stale README ordering sentence. The answer-name, recording, and README findings are remediated. Atomic no-replace directory publication needs a revised design before its implementation changes.

## Implementation

The local draft adds six fictional TSV tickets, a three-question set over `/body`, the fail-closed `triage` policy, and an atomic pipeline that publishes three complete JSONL files with one directory rename. The policy test first failed because `triage.jq` did not exist, then passed eight routing cases and five safe refusals. The pipeline self-test covers the exact command, a single policy application, all three populated outputs, full rows, reviewed-action agreement, disclosure, an existing destination, the final rename, and cleanup after an injected split failure.

The exact midpoint is urgency 1. It routes to `review` with reason `urgent`. The agent chose that inclusive boundary because the middle written level says a delay can cause a concrete problem. Ian can overturn it before the live recording.

The accepted publication assumption is disproved. POSIX shell has no portable operation that atomically renames a directory while refusing any existing destination. GNU `mv` can nest the temporary directory, and a direct rename can replace an empty destination. A symlink commit is portable but leaves a hidden backing directory and makes ordinary cleanup surprising. The agent recommends a small Linux/macOS no-replace rename helper if the actual-directory contract holds. A design reviewer must settle this before implementation continues.

The coordinator alone will invoke `sdlc/scripts/live --max-tokens 5000 demos/16-triage-pipeline/record.sh`. The recording job uses a temporary output directory, cleans it on every exit, and writes only the shared cache. A direct keyless replay runs `triage` with `--replay recording/` and needs no new live reservation.

Pages 04 and 07 remain until page 16 is green. The draft now carries their review pile, explicit null, complete flat input row, table-input/JSONL-output, atomic publication, disclosure, and collision lessons.
