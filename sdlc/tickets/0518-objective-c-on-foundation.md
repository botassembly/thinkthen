# 0518: Build Objective-C on Apple Foundation

Status: OPEN.

Milestone: 0.2

Depends on: 0516

## Outcome

Objective-C uses Foundation: `NSError`, ARC, blocks for async and cancellation, nullability annotations and Foundation collections.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The binding is GNU Objective-C without Foundation, so callers free objects by hand and get no `NSError`.
- Keeps: The shared suite's behavior.
- Changes: Rebuild `libraries/objective-c` on the 0516 pattern. Remove `TTJSON.c` and the native view copies. Claim `libraries/objective-c/**`.
- Proof: The shared suite passes on macOS through the installed package.
- Defers: GNU Objective-C support, which ends here.
