# 0606 — hosted-builds-once-and-cached

Status: OPEN.

Milestone: 0.3

## Outcome

The Windows release job builds each release flag set once and reuses the wheel, and the Swift and Objective-C workflows cache their cargo output.

## Evidence

- Source: the read-only build-efficiency audit of 2026-10-16, at Ian's request. Timings come from the 0543 release campaign's phase records, 22 revisions from 2026-10-14 to 2026-10-16. Re-measure on main before acting.
- The Windows job takes about 2,013 seconds. About 735 seconds is release compiling: the wheel (264 s), then pack builds for the command (179 s), the C archive (152 s) and the wheel again (140 s).
- The Swift workflow's release build takes 190 seconds of its 419.5-second step with no cache. Objective-C compiles for 78 seconds of its 80-second step with no cache.
- Hosted runs spent 903 runner minutes over 20 revisions: Windows 659, Swift 190, Objective-C 54. Only `gate.yml` and `windows.yml` cache cargo output.
- Check first whether the separate per-part Windows builds are deliberate.
- Cost before: about 33.5 minutes per Windows job and about 100 uncached Apple runner minutes per 20 revisions. Expected after: the Windows job near 26 minutes, and Apple builds that hit the cache on a second run.

## Scope

- Changes: `sdlc/scripts/release-pack`, `.github/workflows/windows.yml`, `.github/workflows/objective-c.yml`, `.github/workflows/swift-lane.yml`, `libraries/python/check-windows.py`.
- Keeps: every check's coverage.

## Proof

The Windows log shows one wheel build and one build per flag set, and a second Apple run shows a cache hit. After landing, measure again and record the result here.
