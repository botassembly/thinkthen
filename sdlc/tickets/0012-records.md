---
flow: build
priority: 50
opens: crates spec specification/fixtures sdlc/ratchet.json sdlc/live-tokens demos/04-review-queue demos/README.md sdlc/planning/documentation-plan.md
---

# 0012: Records

Status: done

## Outcome

`decide`, `choose`, and `score` run over many records as `specification/records.md`, `result.md`, and `channels.md` describe: `--input FILE`, `--lines`, `--jsonl`, and `--field` with one pointer or several. One value prints per record in input order. The run stops at the first failure and says where. How-to 04 is green.

## Current Facts

The three verbs read one document from standard input. `records.md` is Settled except the default of `jobs`, and ADR 0010 now gives `--jobs N` with a default of 4. This ticket builds the sequential form, and ticket 0013 adds `--jobs`, `--cache DIR`, and `choose --options POINTER`. The rows under `recipes/rows/` already have the record-mode shape, with the record under `input`, so the recipes are the first consumer. `result.md` says `meta.tool` is always present, and the binary does not write it (`sdlc/issues/2026-09-19-review-leftovers-from-ticket-0008.md`). `sdlc/issues/2026-09-19-ideas-carried-from-the-design-captures.md` lists small input rules that `records.md` never states.

## Scope

- `--input FILE` reads a path. `--lines` makes each line a text record. `--jsonl` makes each line a JSON record. `--field POINTER` is RFC 6901, and it may be given several times: several pointers build an evidence object keyed by the last part of each pointer, and two members with one key are a usage error. `--field` without `--jsonl` reads the whole input as one JSON value. `--field` with `--lines` is a usage error. A pointer that finds nothing is exit 2 for that record before any request for it.
- The small input rules, each stated in `records.md` in one sentence and then built: duplicate keys in a JSON record, invalid UTF-8, and `NaN` or `Infinity` are refused; a carriage return before the line feed is stripped under `--lines`; `$.body`, `#/id`, wildcards, and negative indices are refused with a message that names RFC 6901.
- One request per record, sent in input order, one at a time. The bare value prints per record. Under `--details` each row carries `input`, the whole original record. `choose --raw` prints an empty line for an unresolved record. In record mode the exit code reports the run, and no record's answer sets it.
- A failed record stops the run. Rows already printed stay printed. One line on standard error names the record it stopped at, how many records finished, and how many of those came from a recording.
- `--dry-run` in record mode shows the first record's request and an `input` object naming the framing and the pointers, and reads no further.
- An empty input exits 0 with no output and no request.
- `meta.tool` holds the binary's name and version in every result. Pages and fixtures that print `meta` in full follow. No recording changes, because a recording holds no `meta`.
- How-to 04 (act only when the answer is sure, and send the rest to a person) turns green in the form of ADR 0011, recorded through `sdlc/scripts/live`. A block on the page feeds its rows to `recipes/counts` to prove that live record-mode rows have the shape the recipes read.

Excluded: `--jobs`, `--cache DIR`, `--options POINTER`, `filter`, `rank`, and `annotate`.

## Acceptance

- Integration tests against a local listener cover each framing, one and several pointers, the key clash, each refused input with zero requests for that record, order kept, the stop at the first failure with the standard-error line, the empty input, the record-mode plan, and `input` under `--details`.
- A property test holds that the number of output lines equals the number of records for any run that finishes.
- The pinned `decide` digest holds, and every committed recording and the rows under `recipes/rows/` still replay.
- The key and the evidence never appear in any error or Debug output, including the line that names the stopped record.
- The spec rung prints how-to 04 green with the key unset and touches no network. The recordings hold no key.
- The ratchet equals the measured total, and the commit that raises it says what grew, why it earns its lines, and where duplication was looked for first.
- The whole ladder is green, and a second agent reviews the public surface change.
