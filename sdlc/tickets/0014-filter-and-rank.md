---
flow: build
priority: 50
opens: crates spec specification/filter.md specification/rank.md specification/fixtures sdlc/ratchet.json sdlc/live-tokens demos/03-grep-for-meaning demos/06-top-search-hits demos/09-what-leaves-the-machine demos/43-lint-a-change demos/README.md sdlc/planning/documentation-plan.md
---

# 0014: `filter` and `rank`

Status: ready

## Outcome

`filter` keeps the records that reach the mark and prints them unchanged. `rank` prints the records with the most likely yes first. Both run over the record mode of tickets 0012 and 0013 and add no second way to read, send, or record. How-tos 03, 06, and 43 are green.

## Current Facts

`filter.md` and `rank.md` are Settled by ADR 0007. Both ask one yes/no question of each record, so both reuse the `decide` request and the record loop. `rank` holds every record until the input ends. The live probe of ticket 0011 emulated `rank --top 1` with one `decide` per line and a local sort, and its rows give a first fixture.

## Scope

- `filter QUESTION (--lines|--jsonl)` prints each kept record byte for byte as it arrived, in input order. `--threshold T` is a single cut and defaults to 0.5. The band form is a usage error with a message that names `decide --details` and `jq` for three piles. An unresolved record cannot occur under a single cut. `--details` prints one result object per record, kept or not.
- `rank QUESTION (--lines|--jsonl)` prints each record byte for byte, highest probability of yes first, and exact ties keep input order. `--top N` prints the first `N` and saves no request. `--top 0` and a negative `N` are usage errors. `--details` prints the result objects of the printed records in the printed order.
- Both take the question as text or as `@FILE` holding a `decide` question, and both take `--true TEXT` and `--false TEXT`, exactly as ticket 0017 built them for `decide`. A file that holds a band is refused by both, because neither takes one.
- A missing framing is a usage error for both. `--quiet` and `--raw` are refused by both, and `rank` refuses `--threshold`. Each refusal names the command to use instead.
- Both honor `--field`, `--input`, `--jobs`, `--record`, `--replay`, `--cache`, and `--dry-run` as record mode built them. A failed record stops the run. `filter` has then printed a prefix of its output. `rank` has printed nothing, and its line on standard error says so.
- "Byte for byte" covers a JSON record with odd spacing and a text line with trailing spaces. The record's line ending is written as a line feed.
- `meta.tool`, the result object, and the recording entry are those of `decide`. A recording made by `decide` over the same records replays under `filter` and `rank` with the same question, and a test proves it.
- The cut, the sort by probability of yes, the tiebreak by input order, and the `--top N` slice are pure functions in `thinkthen-core`, so a library in another language can be held to the same cases. ADR 0017 gives the reason.
- The short help of `rank` states the method in one sentence: one yes/no question per record, a local sort, no comparison of two records.
- How-tos 03 (keep only the records that match a meaning), 06 (put the best matches first), and 43 (lint a change by meaning and fail the build) turn green in the form of ADR 0011 and to the limits of ADR 0016, recorded through `sdlc/scripts/live`. Page 03 absorbs 09: its `--dry-run` block proves what leaves the machine, and the red folder 09 is deleted. How-to 06 uses the several-pointer form to rank passages against a query. ADR 0018 gives the design of page 43: one record per changed hunk from a committed fixture, `filter` with the convention in a question file, the kept hunks printed as the report, and a build that exits 1 when any hunk is kept. Page 43 is in the README's front window, so a newcomer with thirty seconds must follow it.

Excluded: `annotate`, `find`, any rubric for `rank`, any pairwise comparison, and any streaming form of `rank`.

## Acceptance

- Integration tests against a local listener cover: kept records unchanged byte for byte, order kept under `--jobs 4`, the band refused, each refused option with its message, `--top` bounds, ties in input order, the stop at a failed record for each command, the empty input, and the plan under `--dry-run`.
- A property test holds that `filter` prints a subsequence of its input and that `rank` prints a permutation of its input, or the first `N` of one.
- A test replays one `decide` recording under `filter` and under `rank` with no request sent.
- The key and the evidence never appear in any error or Debug output.
- The pinned `decide` digest holds, and every committed recording still replays.
- The spec rung prints how-tos 03, 06, and 43 green with the key unset and touches no network. The recordings hold no key.
- The ratchet equals the measured total, and the commit that raises it says what grew, why it earns its lines, and where duplication was looked for first.
- The whole ladder is green, and a second agent reviews the public surface change.
