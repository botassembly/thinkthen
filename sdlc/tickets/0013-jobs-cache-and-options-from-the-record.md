---
flow: build
priority: 50
opens: crates spec specification/records.md specification/choose.md specification/channels.md specification/fixtures sdlc/ratchet.json sdlc/live-tokens demos/12-keep-going demos/21-options-from-the-record demos/README.md sdlc/planning/documentation-plan.md
---

# 0013: Jobs, the cache folder, and options from the record

Status: held by Ian on 2026-09-19

## Outcome

A run over many records sends several requests at once and prints the same bytes as a run that sends one at a time. `--cache DIR` resumes a run that stopped. `choose --options POINTER` takes each record's candidate list from the record. How-tos 12 and 21 are green.

## Current Facts

Ticket 0012 built record mode with one request in flight. `records.md` gives `jobs` a default of 4 and leaves it Draft with no home on the command line, and ADR 0010 gives it one: `--jobs N`, an advanced option. `records.md` settles `--cache DIR` as `--record DIR --replay DIR`. `choose.md` settles `--options POINTER`, and `backends.md` says a description travels as the value under the option's key in `criteria`. `backends.md` already gives the retry rule for a status of 429. The HTTP client is blocking `ureq`.

## Scope

- `--jobs N` takes a whole number from 1 to 32 and defaults to 4. It appears in the long help alone. `--jobs` outside record mode is a usage error. `records.md` loses the Draft mark on `jobs`, and `channels.md` lists `--jobs N` and `--cache DIR` among the advanced options.
- Standard threads and a bounded buffer carry the work. No asynchronous runtime and no new dependency enter for this. The buffer holds at most `N` finished rows ahead of the next row to print, so memory stays flat on a long run.
- Output never depends on `--jobs`. A run with any `N` prints the bytes that `--jobs 1` prints, on standard output and on standard error.
- A failed record stops the run as ticket 0012 built it. The rows before the failed record print, and no row after it prints. The line on standard error names the earliest failed record. A request that finished after the failed record is still written under `--record`, because it was billed and a resume should not pay twice. No new request starts after the first failure is seen.
- When the program downstream closes the pipe, the tool stops reading and stops scheduling, and it exits as `channels.md` says.
- `--cache DIR` means `--record DIR --replay DIR`. `--cache` beside `--record` or `--replay` is a usage error. It appears in the long help alone.
- `choose --options POINTER` reads a list of labels, or a map from label to description, from each record. A map's descriptions travel in `criteria`. The rules of positional options hold for each record: 2 to 255 labels, no duplicate, no empty label, and every label a string. A record that breaks one is exit 2 for that record before any request for it. `--options` beside positional options is a usage error, and `--options` outside `--jsonl` is a usage error. `choose.md` says `--lines` today, and a line of text holds no pointer, so the page is corrected. The evidence still follows `--field`, and `--options` never changes it.
- Under `--details` each row carries the options it was asked with, in the record's order. `--dry-run` shows the first record's options in the plan.
- How-to 12 (resume a long run that stopped) and how-to 21 (choose from a list that differs for every record) turn green in the form of ADR 0011, recorded through `sdlc/scripts/live`. How-to 12 stops a run with a listener that fails on one record, reruns it, and shows that the second run pays for the rest alone.

Excluded: `filter`, `rank`, `annotate`, any rate limiter beyond the retry rule, and any progress display.

## Acceptance

- A property test holds that any `--jobs` from 1 to 32 prints the bytes of `--jobs 1` for any run, finished or stopped.
- Integration tests against a local listener that answers out of order cover: order kept, no more than `N` requests in flight, the stop at the earliest failure with later finished records still recorded, the closed pipe, and `--jobs` refused outside record mode.
- An integration test stops a `--cache` run at one record, reruns it, and counts the listener's requests: the second run sends only the unfinished records.
- Integration tests cover `--options` as a list and as a map, each refused candidate list with zero requests for that record, the clash with positional options, and the options under `--details` and in the plan.
- The key and the evidence never appear in any error or Debug output from a worker thread.
- The pinned `decide` digest holds, and every committed recording still replays.
- The spec rung prints how-tos 12 and 21 green with the key unset and touches no network. The recordings hold no key.
- The ratchet equals the measured total, and the commit that raises it says what grew, why it earns its lines, and where duplication was looked for first.
- The whole ladder is green, and a second agent reviews the public surface change.
