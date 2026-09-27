# `check` and `score` get 422 on a null level description

Status: Closed on 2026-09-26. Done by ticket 0160 on 2026-09-26: a `null` score level is sent as an empty object, and the paid `check` run against the reference backend exited 0 with every probe passing (`sdlc/records/0160-build-the-answer-contract-holds.md`). Filed 2026-09-26 by the queue owner from local experiment 273, report 06, finding I-3.

## What happens

The report ran these live.

- `thinkthen check --url https://api.typesafe.ai/v1` exits 4 with `critical score: ... status 422`.
- A `score` question file whose level has a `null` description gets 422. The same file with an object description passes.

`specification/backends.md` allows a `null` description. `check`'s score probe sends one on purpose: `specification/check.md` line 30 says it sends "`criteria` as an array holding a `null`, a string, and an object". The refusal phrase the user sees blames size or format.

## Checked on main

Partly verified. `check.md:30` does send `null` in score criteria. The 422 is a live finding from the report. It was not rerun here, because a live call is paid.

## What would fix it

Record the live finding in the repository. Then choose one:

- Stop sending `null` in score criteria. Send the level name, for example.
- Or refuse a `null` score description locally, with a clear message, and change the spec.

Then make the `check` probe send only what the reference backend accepts. The fix needs one paid confirmation run, which waits for Ian's authorization.

## Done when

`check` passes against the reference backend, and a `score` file that the spec allows is either sent in a form the backend accepts or refused locally with a sentence that names the field.
