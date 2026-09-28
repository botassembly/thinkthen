# The site's relate samples after the three-step recognize

Status: closed. Confirmed by ticket 0241 code review at `db7e2418`. See [the acceptance record](../../records/0241-code-review.md).

Original report status: Open. Filed 2026-09-27 by the queue owner for the marketing lead, from ticket 0147. Owner: the marketing lead, who owns `site/` (`sdlc/planning/ownership.md`).

## What changed

Ticket 0147 changes how rules read on `recognize` and `relate`. A bare rule such as `--relation knows` means any kind to any kind on both commands. `ANY` on either side means `*`. A relate file's rule may leave out `source` or `target`, which means `*`. `relate`'s requests, planner and output do not change, so its recordings still replay.

`recognize`'s relations change. Step 3 asks one yes/no question per ordered pair a rule allows: "Does the text itself state that …?". Each edge repeats both names in the new name shape, with `text` in place of `name`.

## What the site holds that no longer holds

- `site/examples/beatles/relate/` and `site/examples/functions/relate/` still replay. They could show the bare rule or `ANY` where a sample names `*`.
- Any site sample or catalog text that shows a `recognize` edge with `source.name` or `target.name`, or describes recognize's relations as the relate planner's choice questions. `site/src/data/catalog.mjs` says "With `--relation`, more calls follow. They ask how the names connect."; the recognize page now says what those calls ask.

## What to do

Check the relate samples still replay, and update any recognize edge the site shows from `specification/recognize.md`.
