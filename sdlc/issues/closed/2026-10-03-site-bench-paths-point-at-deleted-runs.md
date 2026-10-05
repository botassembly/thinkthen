# Two site paths point at bench runs that left the bench's main branch

Status: closed.
Resolution: b7e5dd3e464117d38701617ca18c5fdb68f5fb2a
Milestone: later

Beatles Bench ticket 0026 deleted every old run under `results/runs/` and `results/archive/` from the bench's main branch. The bench's history keeps every file at commit a6a6be71. Main keeps stub `README.md` files at `results/runs/2026-09-26-thinkthen-jev` and `results/runs/2026-09-26-thinkthen-jev-open-book`, each linking its run at a6a6be71.

The site's `pull-bench` reads only the bench commit `site/examples/beatles/bench-pin` names, 97090765, so nothing breaks today.

1. `site/examples/beatles/folders.json` runs the `blind-spots` example in `results/runs/2026-09-26-thinkthen-jev` and reads its recording. It needs its own source before `bench-pin` moves past a6a6be71. The site already keeps a converted copy of that recording, which may serve.
2. `site/src/articles/rad.md` links `results/runs/2026-09-26-thinkthen-jev-open-book` on the bench's main branch. The stub keeps the link working. Repointing it to the run at a6a6be71 makes it stable.

Both are due before `site/examples/beatles/bench-pin` moves past bench commit a6a6be71. Leaving them breaks a site example on that move.

Ticket 0402 slice A's candidate moves the retained three-row recording byte for byte into blind-spots files/recording, removes only its bench mapping and pins the open-book link to `a6a6be71`. Focused blind-spots smoke passes both outputs: `"john"` at exit 0 and `null` at exit 3. The lander can close this issue when the reviewed slice lands.

Slice A landed at `b7e5dd3e4` with exact accepted source `53de60f6b`. Its full actual documentation replay, strict zero-stale proof and final site build pass. C repeats that documentation proof. The outcome was already landed; this update closes its leftover issue status.
