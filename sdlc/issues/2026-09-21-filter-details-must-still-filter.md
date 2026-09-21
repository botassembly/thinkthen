# `filter --details` must still filter

Status: Open. Ruled by Ian on 2026-09-21. For the build team.

## What happens today

`thinkthen filter 'Is this a complaint?' --lines` prints the two records that pass. The same command with `--details` prints all four records, including one at probability 0.04. A reader of the marketing deck saw this and took it for a bug.

The specification disagrees with itself. `specification/filter.md` says `--details` prints a result object "for every record, kept or not." `specification/result.md` says `--details` on `filter` and `rank` prints objects "for the same records in the same order that the bare values would have taken." The binary follows `filter.md`.

## The ruling

Ian's words: "Filter should still filter. I don't care what the details say."

`--details` changes what is printed about a record. It never changes which records print. `filter --details` prints one result object for each kept record, in input order, and nothing for a record that did not pass. `result.md` already says this, so `filter.md` and the binary move to match it.

A user who wants every record with its answer already has the tool: `decide --lines --details` prints one object per record, kept or not. `filter.md` should point there in place of the "kept or not" sentence. The three-pile example in `filter.md` already uses `decide`.

## What to check

- `rank --details` with `--top N` prints N objects. Confirm it and pin it.
- The same rule holds in every library and database surface: asking for details never widens the result.
- A shared conformance case pins `filter --details` to the kept records only.
- Any how-to or transform that leans on `filter --details` printing every record moves to `decide --details`. Search `demos/` and `transforms/`.

## What the deck did

The filter slide no longer shows `--details`. Its second example raises the threshold to 0.95 and one record passes.

Ian can overturn this ruling.

Two places already lean on the old behavior and need a look: `demos/03-grep-for-meaning/README.md` and `demos/43-lint-a-change/record.sh`.
