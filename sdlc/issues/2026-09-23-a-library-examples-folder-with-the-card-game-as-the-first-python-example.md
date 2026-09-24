# A library examples folder, with the card game as the first Python example

Status: Open

Ian ruled on 2026-09-23: ThinkThen should pull together an examples folder, and the card game built by experiment 250 (`experiments/250-thinkthen-plays-a-card-game` in the workspace, not in this repo) becomes one of the Python examples.

## What is asked

1. An `examples/` folder in this repo for programs that call the library surfaces — distinct from `demos/`, which ADR 0018 fixes at twenty shell how-tos and which `sdlc/scripts/pages` checks. An example is a working program a developer can read end to end and adapt; a how-to is one shell job with a green page. The folder needs its own README drawing that line, and the `pages` check needs to stay green by not counting `examples/`.
2. The first Python example is River Run, the card game from experiment 250: a program whose every decision is a thinkthen judgment, one engine held for the whole run, a saved question set for the batched screen, and a strategy guide that lives in a versioned question file. The experiment's bash driver stays in the experiment as the shell-loop seat; the Python example is the third seat (a program that keeps the result).
3. The example runs against a recording (`--replay`), so a reader runs it with no key and no spend, the same discipline the demos carry.

## Sequencing

Blocked on the library surfaces merging to main and passing their own cases, the same gate experiment 250's conversion waits on. When it lands, port `game.py` and the question files, hold one engine per run, and measure the before and after against the experiment's recorded runs (32 requests one-decision-per-request versus 13 one-per-screen on the CLI; the library run adds the warm-connection number).

Found by experiment 250. Ian's words: "we probably should start pulling together an examples folder in ThinkThen, and this can be one of the Python examples."
