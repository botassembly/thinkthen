# Refresh site and bench relate examples for pair questions

Status: open. Filed from ticket 0167 for the marketing lead, who owns `site/`. No external message has been sent.

Current main still uses standalone `relate`'s choice planner for different kinds. Ticket 0167 changes that request shape to one yes/no question for every allowed pair, one shared entity state, and at most 400 questions per request. It keeps the edge shape. This issue takes effect when 0167 lands. It supersedes the earlier statement in `2026-09-27-site-relate-samples-after-the-three-steps.md` that relate recordings still replay.

The site examples under `site/examples/beatles/relate/` and `site/examples/functions/relate/` replay request digests from the choice planner. Their recordings will miss after 0167. Rebuild those examples from the landed command and its new recordings. Explain that an edge comes from the model's knowledge of the names, while `recognize --relation` asks what the text states. Remove descriptions of a choice, asking side, option fallback, or per-rule state.

The bench's relate F1 of 0.72 measures the old choice planner. Keep it labeled as historical evidence or rerun its fixed corpus against the pair planner before publishing a current score. Do not relabel the old number as a pair result.

Done when the marketing owner verifies the site examples replay and the bench score names the method and revision it measured. This issue records the handoff; it authorizes no marketing edit, external message, or paid run.
