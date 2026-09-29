Status: Open. Filed 2026-09-29 from experiment 413 (`~/workspace/experiments/413-liquid-d1-vs-jev/RESULTS.md`), which ran Liquid's d1 decision model through `thinkthen check` and Beatles Bench's hard subset.

# The systemone adapter sends null criteria that Liquid's d1 refuses

## The problem

A `decide` question with only one of `--true` and `--false` given serializes its missing side as a null criteria entry, for example `"criteria":{"true":"The text says the box was intact.","false":null}`. TypeSafe's hosted System One accepts that body. Liquid's d1 at `https://api.liquid.ai/decisions/v1/systemone` refuses it with status 422 and `param: questions.q1.criteria.false`. `thinkthen check` names this a critical finding, so a user who points `THINKTHEN_BASE_URL` at Liquid fails every one-sided `decide` question while two-sided and criteria-less questions pass.

Null criteria values inside `choice` and `score` questions pass on both backends; only the noul form differs. Liquid lists one model, `d1:free`, and speaks the same wire shape otherwise (experiment 413).

## The fix candidate

The adapter omits criteria entries whose value is null instead of serializing them. Omitting a null-false entry leaves the same information on the wire for TypeSafe, which ignores null descriptions, and satisfies Liquid's schema. Fixture goldens that carry a serialized null criteria entry change.

## Done when

`thinkthen check --url https://api.liquid.ai/decisions/v1 --model d1:free` reports no critical finding, and the adapter's fixtures cover a one-sided `decide` criteria question.
