# The site's recordings need a re-key for the default model pin

Status: Open. Filed 2026-09-26 by the queue owner for the marketing lead, from ticket 0159. Owner: the marketing lead, who owns `site/` (`sdlc/planning/ownership.md`).

## What happened

Ticket 0159 changed the default model from the alias `jev-latest` to the pinned version `jev-1.13.0`. A recording's digest covers the request body, and the body names the model. So every default request now asks for a new digest, and a replay of a recording made under the alias misses.

The ticket re-keyed the queue-owned recordings under `crates/thinkthen/tests`, `demos/` and `transforms/rows`. It left `site/` alone. `site/recordings` holds 193 entries that asked for `jev-latest`, and `site/examples` holds 54 more. Every one of their replies names `jev-1.13.0`. The site's hand-run build replays against the old key until the re-key lands. The Pages workflow runs only by hand, so the gap breaks no deploy that nobody starts.

## What to run

From the repository root, on a clean tree, with the key unset:

```sh
env -u THINKTHEN_API_KEY sdlc/scripts/rekey-model jev-latest jev-1.13.0 \
  site/recordings \
  site/examples/beatles/bench/examples/annotate/recording \
  site/examples/beatles/bench/examples/choose/recording \
  site/examples/beatles/bench/examples/decide/recording \
  site/examples/beatles/bench/examples/filter/recording \
  site/examples/beatles/bench/examples/find/recording \
  site/examples/beatles/bench/examples/rank/recording \
  site/examples/beatles/bench/examples/recognize/recording \
  site/examples/beatles/bench/examples/relate/recording \
  site/examples/beatles/bench/examples/score/recording \
  site/examples/beatles/bench/examples/tag/recording \
  site/examples/beatles/bench/results/runs/2026-09-26-thinkthen-jev/recording
```

The script prints each old digest beside its new one. Replace each old digest that a site page or saved output prints with its new one in the same commit. `site/examples/beatles/backends/1-check.out` and `site/examples/install/backends/1-dry-run.out` show `jev-latest` as the model sent or planned, and they change to `jev-1.13.0` when they are captured again. `sdlc/scripts/README.md` lists the script's refusals.

## When it closes

It closes when the site commit lands. A Quick Fix then removes `sdlc/scripts/rekey-model`, its fixture, and its `lint` line.
