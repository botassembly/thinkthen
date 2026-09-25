# The site needs a Beatles Bench section

Status: Open

Filed by the marketing session on 2026-09-25, for Ian's ask of that day.

## What is missing

thinkthen.dev has no page for Beatles Bench. Ian wants one section that walks through the ten functions on one worked example each.

Each page shows:

- the example's picture
- the command
- its output
- how context fills that function's gap, or why no context helps

## Where the content comes from

Beatles Bench ticket 0006 (github.com/botassembly/beatles-bench, `sdlc/tickets/0006-one-worked-example-per-function.md`) adds `examples/<NN>-<fn>/` for each function. Each folder holds:

- a recorded run that replays with no key
- the committed outputs
- `slide.png`
- `README.md`, the article

## Asks

1. Add a Beatles Bench section that links to the repository.
2. Pull each page from the bench's `examples/*/README.md` and `slide.png`, the way `scripts/pull-examples.mjs` pulls the deck. Copy no text by hand.
3. Fail the build when a pulled page is missing or its example has no committed output.

The bench ticket lands first. Ian can overturn all three.
