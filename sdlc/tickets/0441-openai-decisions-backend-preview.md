# 0441: Prepare the Decisions backend for preview access

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

A complete backend implementation ticket is ready for admitted preview documentation and access. Implementation starts when the actual wire contract is available.

## Evidence

- Starts from: Existing OpenAI backend placeholder; official API overview and guide index contain no admitted Decisions wire schema. No paid calls were made. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 12.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Use the existing backend route, typed binding/SQL selection, facts, cache/replay and six errors. Require documented endpoint/authentication/question/result/probability/model/error behavior before choosing adapter details.
- Proof: After access supplies the admitted contract, saved exchanges prove successful finite-choice replies, rich descriptions, invalid input, missing probabilities, known errors, cache/replay, facts and zero-send refusals across the existing backend door. Never fabricate unsupported probabilities or substitute another API.
- Defers: Paid calls, access procurement, images and guessed wire behavior.

## Dependencies and ownership

External prerequisite: preview access and its admitted API schema. Ready means the task is specified, not that the backend is built. Write the missing-contract dependency explicitly in the plan; do not invent endpoints, model names, prices or dates.

0.2 ships without this backend. Implement it in a 0.2 point release when preview access and documentation arrive; ticket preparation remains part of the current plan.
