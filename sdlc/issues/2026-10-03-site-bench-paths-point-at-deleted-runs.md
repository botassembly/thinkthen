# Two site paths point at bench runs that left the bench's main branch

Status: open. Filed 2026-10-03 from the Beatles Bench message "The bench's old runs leave main; two site paths need a look" of 2026-10-02. Owner: the queue owner.
Milestone: later

Beatles Bench ticket 0026 deleted every old run under `results/runs/` and `results/archive/` from the bench's main branch. The bench's history keeps every file at commit a6a6be71. Main keeps stub `README.md` files at `results/runs/2026-09-26-thinkthen-jev` and `results/runs/2026-09-26-thinkthen-jev-open-book`, each linking its run at a6a6be71.

The site's `pull-bench` reads only the bench commit `site/examples/beatles/bench-pin` names, 97090765, so nothing breaks today.

1. `site/examples/beatles/folders.json` runs the `blind-spots` example in `results/runs/2026-09-26-thinkthen-jev` and reads its recording. It needs its own source before `bench-pin` moves past a6a6be71. The site already keeps a converted copy of that recording, which may serve.
2. `site/src/articles/rad.md` links `results/runs/2026-09-26-thinkthen-jev-open-book` on the bench's main branch. The stub keeps the link working. Repointing it to the run at a6a6be71 makes it stable.

Both are due before `site/examples/beatles/bench-pin` moves past bench commit a6a6be71. Leaving them breaks a site example on that move.
