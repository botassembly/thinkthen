# 0730 — zig-generated-primitive-union-allocator

Status: OPEN.

Milestone: 0.2.1

Reviews: revision 8397d3f, accept

## Outcome

Generated Zig primitive-union readers compile with the pinned Zig 0.15.2 compiler. Readers discard the allocator only when every decoding arm leaves it unused. Public signatures, callers, decoding assertions and the compiler pin stay unchanged.

## Changes

- `build-tools/results/templates/zig.py`: detect allocator references in primitive-union decoding arms and emit the existing allocator discard convention when no arm uses it. Match the allocator identifier independently of `v.data` member access.
- `libraries/zig/src/results_generated.zig`: regenerate numeric/null and integer/null readers through `python3 build-tools/results/generate.py --target zig`.

## Proof

Given the generated Zig library, when its installed smoke builds and decodes the recorded answer, it compiles without unused allocator errors and preserves the existing decoding assertions.

- Check generation with `python3 build-tools/results/generate.py --target zig --check`.
- Run `sh sdlc/scripts/smoke libraries/zig` through `sdlc/scripts/job` on the admitted M5 job at the pushed revision.

## Evidence

- Starting revision: `88cbe872bce804d7e1be3bfa12d7d3913668c869`.
- Yellow job `1792458135783-426943` at `b7befb48b` reported unused allocator parameters in `readAnswerShape8AnyOf1Items` and `readAnswerShape4AnyOf1Items`. The same generated readers exist at the starting revision.
- Source regeneration completed successfully and changed only the two affected generated readers. Each gains `_ = a;`. Allocation-bearing primitive-union readers retain their allocator references.

## Progress

- 2026-10-19 started
