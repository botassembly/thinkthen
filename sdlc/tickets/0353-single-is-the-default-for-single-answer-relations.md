# 0353: relate recommends the menu for single-answer relations

Status: ready. Serves issue `../issues/2026-09-30-relate-pair-planner-loses-precision-on-the-beatles-bench.md`, whose decision run passed its bar on 2026-09-30. Plan: `sdlc/planning/cleanup-2026-09-30.md`. The coordinator sets this default; Ian can overturn it.

## Outcome

`"single": true` is relate's documented default for a relation where each source has at most one target. The relate page, its examples, and every demo, fixture README and site page that shows such a relation write `single`. The relate page's Precision section states the menu's measured figures beside the pair planner's and still points to `recognize --relation` for relations a text states. A rule without `single` asks yes/no pairs as today. The precision issue closes.

## Evidence

- Starts from: ticket 0342, which added the opt-in menu, and the issue's "Result, 2026-09-30: the menu passes". On the default backend at the 0.5 cut the menu scored edge F1 0.635 on the 182 songs against 0.523, and precision 0.541 on the 16 missing-album sets against 0.296. It answered none for 16 of 31 songs whose album was missing. Recall on the songs fell from 0.693 to 0.551. At the tuned cut of 0.47 it won on the missing-album sets and tied on solo sets. ADR 0057 as amended by 0342 keeps yes/no pairs for every rule without `single`.
- Keeps: the wire form, question keys, cached answers and output of every rule without `single`, and of every `single` rule. `single` stays opt-in in the question file, so an absent `single` still means yes/no pairs; a many-answer relation asked as a menu would lose true edges, the fault ADR 0057 names. Inline rules keep their form. The `single` with `either` refusal stays.
- Changes: pages and examples only.
  - `specification/relate.md`: the Precision section replaces "unmeasured" with the menu's measured figures, the none-of-these rate and the recall cost, and the rule paragraph says to mark a single-answer relation `single`.
  - Relate examples on the relate page, in `demos/`, in `specification/fixtures/relate/README.md` and on `site/` pages that show a single-answer relation, such as song to album, add `"single": true`, with their recordings made again where a question changes. Site pages follow `sdlc/planning/ownership.md`.
- Proof: `sdlc/scripts/spec` runs every changed page and green how-to against its recording, and `lint` passes its page and link checks. A changed example replays with no key and no network. No paid call is needed; a new recording, if one is needed, runs through `sdlc/scripts/live` under a token cap.
- Defers: an inline `single` form, a `single` setter on the library builders, and the Liquid d1 cell of the bench's third condition, which the decision run left out. The Liquid cell goes on the relate page when a capped run measures it.
