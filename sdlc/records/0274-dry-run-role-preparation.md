# 0274 dry-run role guidance preparation

Date: 2026-09-29. Source: `origin/main` `60aec53e`. Design only; no source, build, test, network, or provider action in this pass. [Ticket](../tickets/0274-dry-run-role-guidance.md) is pending independent design review.

## Evidence and decision

Register [row 4](../issues/2026-09-20-new-user-stumble-register.md) observed a newcomer swapping question and evidence and asked for plain words above the dry-run JSON. Current `specification/channels.md` makes stdout results-only and a dry run one compact JSON document; stderr carries person-facing diagnostics. `PlanDocument` in `crates/thinkthen/src/core/plan_document.rs` serializes the exact request, with `request.state` holding one-document evidence and `request.questions.q1.instructions` holding the question. The one-document `decide` path in `crates/thinkthen/src/cli/asking/plan.rs` writes that JSON with no role explanation. Existing help and reference prose say which input is which, but they are absent from the observed command output. Field names alone do not explain the roles in plain words.

Recommend the ticket's fixed one-line **stderr** hint when **stdout is a terminal**, immediately before ordinary one-document plan JSON. This meets the visual intent of “above” without breaking scripts that parse stdout. A pipe or file keeps today's captured stdout and stderr exactly; no new flag or plan field. The hint contains only fixed field names and role words, so it cannot echo judged text or a key. The coordinator should explicitly accept this adjustment to the old remedy; it does not require a new Ian decision.

The scope limit matters. `spec/decide.md`'s record-batch example at source pin `60aec53e` has `request.state` equal to `Each question quotes the text it asks about.` and quotes record evidence in `request.questions` instructions. Repeating the one-document sentence there would be wrong. `find`, `annotate`, `recognize`, and `relate` use other plan paths or schemas. Leave them alone until an observed, specific wording need exists. This is one criterion, not a dry-run redesign.

## Build handoff

The proposed source seam is `cli/asking/plan.rs::print_plan`, after validation/serialization and before `edge::write_line`; the ordinary one-document condition is `!reading.streams()`. `edge::waiting_on_terminal` already demonstrates a production TTY boolean with stderr output, but `cli/edge.rs` is close to its 500-nonblank-line cap, so keep the new helper local if one is needed. `decide_edge.rs` is also near its cap. Any Rust growth changes the measured `sdlc/ratchet.json` total. The exact files and conditional proof files are in the ticket.

Start with a failing terminal hint check; require exact terminal and nonterminal bytes at the real output boundary, not a mock that emits the sentence itself. Existing outside-in checks already pin exact piped plan bytes and empty stderr (`crates/thinkthen/tests/decide_edge.rs`) and count zero loopback requests on `--dry-run` (`crates/thinkthen/tests/backend/exchange.rs::a_dry_run_prints_the_plan_and_opens_no_connection`). Reuse those checks. A channel sentence in `specification/channels.md` must distinguish terminal one-document guidance from the unchanged record and aggregate plans. No timing, stress, paid call, or broad rung is necessary for this design.

## What preparation taught us

The literal “above JSON” wording collides with the settled stdout contract. The useful part is a visible role explanation during the novice's interactive run. A generic hint for all plans would mislabel batch evidence, so the first implementation should stay at the one-document asking boundary. Confirm that scope and the exact channel behavior in fresh design review before code changes.
