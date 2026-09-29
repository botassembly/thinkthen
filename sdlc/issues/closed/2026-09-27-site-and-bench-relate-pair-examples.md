# Refresh site and bench relate examples for pair questions

Status: closed 2026-09-29 by the marketing lead. The site's relate examples already replay from pair-planner recordings made after 0167; `npm run build` smoke matched all 97 examples at ce04682c. The site now says that relate answers from knowledge of the names and that `recognize --relation` links names a text states. Beatles Bench 7240b86c labels the relate F1 of 0.719 and its example as historical, names the old choice planner and thinkthen 02dc0b96, and changes no number. A pair-planner rerun of the bench corpus waits for its own ticket. The site change lands with this closure after fresh read-only review.

Claimed by the marketing lead on 2026-09-29 for 0.1. Marketing fixes, replays and lands it, and closes this issue with the commit. SQL examples wait for ADR 0105 (workspace experiment 2038) so each page is rewritten once.

Current main still uses standalone `relate`'s choice planner for different kinds. Ticket 0167 changes that request shape to one yes/no question for every allowed pair, one shared entity state, and at most 400 questions per request. It keeps the edge shape. This issue takes effect when 0167 lands. It supersedes the earlier statement in `2026-09-27-site-relate-samples-after-the-three-steps.md` that relate recordings still replay.

The site examples under `site/examples/beatles/relate/` and `site/examples/functions/relate/` replay request digests from the choice planner. Their recordings will miss after 0167. Rebuild those examples from the landed command and its new recordings. Explain that an edge comes from the model's knowledge of the names, while `recognize --relation` asks what the text states. Remove descriptions of a choice, asking side, option fallback, or per-rule state.

The bench's relate F1 of 0.72 measures the old choice planner. Keep it labeled as historical evidence or rerun its fixed corpus against the pair planner before publishing a current score. Do not relabel the old number as a pair result.

Done when the marketing owner verifies the site examples replay and the bench score names the method and revision it measured. This issue records the handoff; it authorizes no marketing edit, external message, or paid run.
