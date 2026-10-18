# 0697 — Retire the "decision model" wording

Status: COMPLETE.

Milestone: 0.2.1

Reviews: revision 07f89ac, accept

## Outcome

- ThinkThen's public text calls Jev and models like it "System One models", never "decision models": the root README, the crate and library READMEs, the skill, the site pages, site data and the site's generated Markdown.
- Sentences read naturally after the change. Where "decision" described the model, the text says the model returns a probability and the caller's code decides.
- Saved recordings stay unchanged, because they hold real provider responses. Test fixture strings that stand in for provider error text stay unchanged.

### Proof

- `git grep -il 'decision model'` lists only `site/recordings/` and test fixtures.
- The site builds, and the documentation examples whose text changed replay.

## Evidence

Ian ruled on 2026-10-18 that "decision model" is a bad name for System One models: the model returns a probability, and the decision happens in the caller's code. About 41 public files outside `sdlc/` use the term, mostly on the site and in the library READMEs. ThinkThen's own vocabulary already prefers "System One model".

This is a text-only change with no code or contract effect, so it can run as its own lane in any phase.

## Progress

- 2026-10-18 started
