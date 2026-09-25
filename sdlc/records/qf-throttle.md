# Quick Fix qf-throttle: call the width the throttle

Status: landed. It records Ian's ruling of 2026-09-24 that the width setting is named the throttle. A fresh read-only Opus review is in `sdlc/records/qf-throttle-review.md`.

## Result

- `sdlc/planning/adr/0017-libraries-over-one-bound-core.md` gains the amendment "the width is called the throttle". It sets the scope: `EngineBuilder::throttle(n)` in Rust, the host's spelling on each surface, the throttle on every page and ADR, `--jobs` kept on the command line, and private code names left as `width`. Ticket 0086 builds the public name and rewords the two user-facing width messages. Ian can overturn the scope.
- `sdlc/planning/adr/0032-explicit-backend-profiles-carry-limits-and-calibration.md` gains a one-line amendment for its width list.
- Landed tickets 0084 and 0095 each gain a dated line. 0084 dropped one paragraph that repeated its compile-fail rules, so it stays under its 30,000-character cap at 29,998. 0095 is at 11,979 of 16,000.
- `sdlc/planning/surfaces-port-guide.md` names the throttle in its shared rules, the builder note, the mapping row, row R1-8, and G9. Historical index rows keep their words.
- `specification/records.md`, `specification/roadmap.md`, `site/src/pages/reference.astro`, and `demos/12-keep-going/README.md` say `--jobs` sets the throttle. Every number on them is unchanged.
- No code changed.

## Checks

With `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset:

- `lint`: exit 0.
- `spec`: exit 0, `demos: 21 green, 0 red`. It ran because a demo page changed. No page under `spec/` changed.
- `sdlc/scripts/live` did not run.
