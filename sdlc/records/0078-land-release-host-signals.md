# Land: ticket 0078, release host signals

Status: landed 2026-09-24. Owner: Claude.

## What landed

Branch `ticket/0078-fork-and-host-signals`. The build and its review fixes end at `fa5d836f`. The build record is `sdlc/records/0078-build-release-host-signals.md`. A fresh read-only Claude review returned four findings at `fafbe009` and accepted `fa5d836f` on re-review (`sdlc/records/0078-code-review.md`).

## Merges

The branch merged main at `c19771a8`, which brought in 0117 and the Debug Quick Fix. That merge conflicted only in `sdlc/ratchet.json`. `ratchet.mjs` measured 48801, and the ceiling was set to that. The full ladder ran at `fa5d836f`, the tree the reviewer accepted. The one-minute load was 1.40 at the start.

- `install`: exit 0.
- `lint`: exit 0.
- `test`: exit 0.
- `spec`: exit 0, `demos: 21 green, 0 red`.

The landing merge of main at `7d11ed01` brought in 0084. Main had gained only Markdown since `c19771a8`, so `lint` ran again at the merge commit `ffee41c1`. It exited 0 with `ratchet: crates + conformance 48801/48801`. The one-minute load was 7.36 at the start.

`sdlc/scripts/live` did not run, and no test used a paid backend.

## Follow-up

The 0084 and 0095 code review found that 0084's package-proof sentence and 0086 line 41 still treat `nix` as command-only. The issue `sdlc/issues/2026-09-24-reconcile-signal-dependencies-after-0078.md` carries the edit.
