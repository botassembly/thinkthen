---
flow: build
priority: 50
opens: crates spec specification/annotate.md specification/result.md specification/fixtures sdlc/ratchet.json sdlc/live-tokens demos/07-judged-columns demos/08-release-checklist demos/README.md sdlc/planning/documentation-plan.md
---

# 0015: `annotate`

Status: waiting on ticket 0014

## Outcome

`annotate FILE` asks a saved file of questions about one document or about each record, and it adds one field per question. One record makes one request for each distinct `on`. How-tos 07 and 08 are green. Ticket 0016 then writes the flagship triage how-to and the first eval how-tos over this command with no further code.

## Current Facts

`annotate.md` is Settled for the file grammar, the output, and several pointers on `on`. ADR 0010 struck structured question values. The three single-question verbs already build each question type, read each answer, and apply each threshold, so `annotate` adds a file parser, a grouping of questions by `on`, and a merge of answers into the record. ADR 0013 proposes an optional `rules` block for this file and is not accepted. This ticket builds no `rules` block, and the parser refuses the key as it refuses any unknown key.

## Scope

- The question file as `annotate.md` gives it: `version` 1, an optional top-level `threshold`, and `questions`. A question has exactly one of `decide`, `choose`, or `score`. `options` is a list or a map from label to description. `levels` is a list, lowest first. A `choose` question takes a single cut alone. A name uses lowercase letters, digits, and underscores. An unknown key anywhere is an error. An unreadable or invalid file is exit 5, and the message names the path of the key at fault, such as `questions.kind.options`.
- The parser and the grouping live in `thinkthen-core` and touch no file. The binary reads the file and hands the text inward.
- `on` takes one pointer or several and works inside the evidence that `--field` selected. Several pointers build an evidence object by the rule of `--field`, and a key clash is an error in the file.
- Questions with the same `on` ride in one request, in file order. Requests for one record go out in the file order of their first question. Records never share a request.
- An object record gains one top-level field per question. Any other record yields an object of the named answers alone. A question name that the record already holds is exit 2 for that record before any request for it. An unresolved answer is `null`. The `score` value is the number the tool computes, as `score.md` says.
- `--details` prints `input`, `value`, `answers`, and `meta` as `result.md` gives them. `meta.usage` sums the record's requests, and each answer carries the digest of the request that produced it.
- `--dry-run` checks the file, prints the plan of `annotate.md`, sends nothing, and needs no key. With no evidence on standard input it still checks the file and prints the plan with no first record.
- `annotate` refuses `--threshold`, `--quiet`, and `--raw`, and each message says that a question carries its own threshold. It honors `--lines`, `--jsonl`, `--field`, `--input`, `--jobs`, `--record`, `--replay`, and `--cache` as record mode built them. `--jobs` bounds requests, and a record with three `on` sets uses three of them.
- How-tos 07 (add several judged columns in one pass) and 08 (check a document against a checklist) turn green in the form of ADR 0011, recorded through `sdlc/scripts/live`. How-to 08 combines its answers by the written rule of `sdlc/issues/2026-09-19-ideas-carried-from-the-design-captures.md`: any required no makes no, otherwise any required unresolved makes unresolved, otherwise yes. It never multiplies probabilities.

Excluded: the `rules` block, structured question values, templating inside the file, any eval engine, and `find`.

## Acceptance

- Unit tests in the core cover every refusal of the file grammar, with the path of the key named in the message.
- Integration tests against a local listener cover: one request per distinct `on` with the questions in file order, what each request's evidence holds under several pointers, the merge into an object record and into a text document, the name clash at zero requests for that record, `null` for unresolved, the stop at a failed record, order kept under `--jobs 4`, the plan under `--dry-run` with and without a first record, and each refused option.
- A property test holds that the output object of an object record holds every input field unchanged and exactly one new field per question.
- A recording made by `annotate` replays with no key and no request.
- The key and the evidence never appear in any error or Debug output.
- The pinned `decide` digest holds, and every committed recording still replays.
- The spec rung prints how-tos 07 and 08 green with the key unset and touches no network. The recordings hold no key.
- The ratchet equals the measured total, and the commit that raises it says what grew, why it earns its lines, and where duplication was looked for first.
- The whole ladder is green, and a second agent reviews the public surface change.
