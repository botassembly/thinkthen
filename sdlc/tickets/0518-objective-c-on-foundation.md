# 0518: Build Objective-C on Apple Foundation

Status: OPEN.

Milestone: 0.2

Depends on: 0516

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

The review amendment below names the Apple-only implementation and its native qualification obligation.

Objective-C uses Foundation: `NSError`, ARC, blocks for async and cancellation, nullability annotations and Foundation collections.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The binding is GNU Objective-C without Foundation, so callers free objects by hand and get no `NSError`.
- Keeps: The shared suite's behavior.
- Changes: Rebuild `libraries/objective-c` on the 0516 pattern. Remove `TTJSON.c` and the native view copies. Claim `libraries/objective-c/**`.
- Proof: The shared suite passes on macOS through the installed package.
- Defers: GNU Objective-C support, which ends here.

## Review amendment

Initial claims are `libraries/objective-c/Sources/ThinkThen.h`, `libraries/objective-c/Sources/ThinkThen.m`, `libraries/objective-c/check.sh`, `libraries/objective-c/source-package.json`, `libraries/objective-c/README.md` and `libraries/objective-c/checks/installed.py`. Select and name the existing installed consumer source before coding. Declare Apple-only package support, retaining shared results and errors. Prove callback ownership, cancellation and ARC cleanup through an installed macOS package. Remove native JSON/view copies only after their callers migrate. Linux checks cannot replace this native proof, and this ticket grants no machine or workflow permission.

## Surface assessment amendment

Own the Objective-C target template and generated host objects from 0513's common graph; 0504 does not build this surface again. Settle 0517's Apple package and loader design before implementation. Coordinate final artifact collection and installed-consumer routing with 0517, including removal of the Linux-only Objective-C routing assumption. A local Apple package and actual Foundation consumer must satisfy the guide; declaration generation on Linux is not native qualification.
