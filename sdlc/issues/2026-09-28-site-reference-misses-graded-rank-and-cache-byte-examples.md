# The site reference omits graded rank and concrete cache identity examples

Status: open. Owner: marketing under `sdlc/planning/ownership.md`. Filed against main `f1b0acac`. This is a copied website guidance gap, separate from the accepted graded-rank implementation and the original register52 cache/recording reader guides. The build team does not claim site source.

## What a reader sees

`site/src/pages/reference.astro` line81 says rank returns every record with the most likely yes first. Ticket0223 also accepts a saved score question through `rank @FILE`, then orders by its weighted score. The blanket reference sentence omits that shipped command form. Ordinary yes/no rank and library rank retain their existing behavior.

The same page's cache section at lines202–206 gives the exact encoded request digest formula but no concrete line-ending or JSON-spelling examples. The root README cache guide and `specification/recording.md` now carry those examples in the separately reviewed command-pages correction. Keep the copied reference aligned with that guidance, or link directly to it rather than restating an incomplete promise.

## Completion criteria

- Correct the blanket rank reference row to cover both ordinary yes/no rank and the command's saved-score route. Preserve the short page style and existing yes/no tutorial examples.
- Give the cache reader the concrete encoded-evidence examples, either briefly inline or through a clear link to the existing recording guide. Scope them correctly: one-document text retains LF versus CRLF and literal JSON spellings, while record framing may strip terminators or re-encode parsed JSON before request identity is formed. Do not promise that every raw input byte affects the key.
- Run the affected generated-site checks from current source. No runtime change, provider call, broad test campaign or publication is required.

## Evidence and boundaries

`0223-graded-rank-build.md` and `0223-code-review.md` record the implemented command route and retained semantics. `qf-current-command-contract-pages.md` records the corrected normative pages and exact encoded-evidence probe. The original register52 asks for cache and recording reader guidance; the root cache guide and recording specification satisfy it. This copied website follow-up must not keep that original criterion open indefinitely. Optional custom level weights and reciprocal-rank fusion remain with the separate graded-ranking issue.
