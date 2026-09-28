---
flow: design
priority: 251
opens: sdlc/tickets/0251-safe-replay-miss-context.md sdlc/records/0251-replay-context-preflight.md
---

# 0251: Reconcile safe replay miss context with register 102

Status: prepared for fresh independent design review. No implementation or closure approved. [Preflight](../records/0251-replay-context-preflight.md) uses current source `3918bba2` and the original experiment 284/102 criterion; the coordinator owns any source claim and count change.

## Outcome and proposed design

Give a strict-replay reader the failed record or request range, useful question-set context **when the file provides a safe name**, and the exact entry digest. Keep `Local`/exit 5, empty answer output, zero sends, read-only replay, and the existing safe fixed wording. Current 0221 already gives the digest, position/range and annotate group ordinal/member count. The parsed question file has no set-name field. Propose **no new runtime carrier** for this ticket: use that proven context and ask the reviewer/coordinator to rule whether the original conditional name requirement is satisfied by a file without a declared set name. A filesystem basename or member label is caller-controlled, may contain the configured key or other private text, and must not be silently printed. If literal basename output is required, defer implementation for a reviewed secrecy and output decision; do not claim this design meets it.

## Evidence

- **Starts from:** original register 102 in `$HOME/workspace/experiments/284-issue-register/102-replay-miss-does-not-say-why.md` asks for record N, question-set name when a file named it, and exact digest. The current plan row retracts the later invented component-by-component digest criterion.
- **Keeps:** accepted 0221 at reviewed `4a740fdc`, `Failure::ReplayMiss` and its `Stopped`/`BatchFailed` wrappers, command-owned closed source labels, exact digest, Local/exit 5, no send, replay-folder immutability and credential/path/evidence secrecy.
- **Changes:** this design corrects the issue-to-proof mapping only. `QuestionSet::parse` has named question members but no top-level set name; annotate group ordinal/member count is the safe existing context. No source, schema, public API or page change is authorized by this ticket yet.
- **Proof:** retained `tests/backend/recordings/replay_context.rs` already runs seven request shapes and an exact existing-folder miss. If the reviewer wants a narrower record-N witness, extend that file with one record-2 annotate miss in an unchanged existing folder, exact digest, zero listener sends and sensitive basename/member withholding. It must execute the compiled CLI; a string-format unit test alone is insufficient. No provider or broad suite.
- **Defers:** a literal filesystem-name diagnostic, new set-name grammar, component-level digest diagnosis, recognize-stage or relate-chunk provenance, and any exposure of caller text. Those need their own justified accepted outcomes. No closure until the conditional-name interpretation is reviewed.

## Routing and retained contract

The coordinator can classify register 102 as already fixed by 0221 **only if** “when a file named it” means a declared set name, which the current grammar lacks, and group ordinal/count is accepted as the available question context. If it means basename, keep the row open and request a separate safe-name rule before source work. Do not reopen 0221's closed fixed labels or infer a unique question from a grouped request. Prospective source, if that decision changes: `cli/annotate.rs`, `cli/annotate_schedule.rs`, `cli/annotate/batching.rs`, `cli/failure/recording.rs`, and the existing `tests/backend/recordings/replay_context.rs`; root must claim them. No build is proposed on the present interpretation.

## What the build taught us

No build has begun. Preparation found that the accepted 0221 implementation already carries position and digest, while its ticket's remaining component-diagnosis sentence is not an original criterion. The file schema has no declared set name, and an arbitrary filename is not safe diagnostic text. Record the fresh review ruling and any later outside-in result here before landing a behavioral change.
