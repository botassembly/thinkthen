# 0698 — release-pack reports its real exit

Status: COMPLETE.

Milestone: 0.2.1

Reviews: revision 8d9ac12, accept

## Outcome

- Changes: `sdlc/scripts/release-pack`, `sdlc/scripts/release-archive-self-test.py`.
- `release-pack` runs under `set -u` without reading an unset `CFLAGS`, and its cleanup trap preserves the script's exit status. A pack that writes no archive exits nonzero and names the missing archive.

### Proof

- A focused check runs `release-pack` with `CFLAGS` unset and with a forced failure before the archive is written. It fails before the change (unbound variable, then exit 0 with no archive) and passes after: the first succeeds, the second exits nonzero.

## Evidence

The 0643 installed proof on the M5 hit `release-pack` reading an unbound `CFLAGS` near line 724; its cleanup trap then reported exit 0 although no archive existed, and the installed session failed later with no clear cause. This is a false success in the release machinery, the class 0.2.1 rule 3 forbids. It is a small prerequisite that lands alone, ahead of any installed proof that packs the C library.

## Progress

- 2026-10-18 started
- 2026-10-18 next: Fresh review accepted 8d9ac1252 and the focused archive check passed with unset CFLAGS and exact forced exit 42; run its separate main closing gate after the current core landing, then land it alone as ruled.
