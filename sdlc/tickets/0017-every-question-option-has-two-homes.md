---
flow: build
priority: 40
opens: crates spec specification README.md demos sdlc/ratchet.json sdlc/live-tokens sdlc/planning/documentation-plan.md probes
---

# 0017: Every question option has two homes

Status: in progress

## Outcome

Every structural option that the vendor's interface gives a yes/no question, a pick, and a placement has one home on the command line and one home in a question file, under the same word. `decide`, `choose`, and `score` read a question file as `@FILE`. The pages stop promising what the vendor never sends. Four live checks settle what reading could not.

## Current Facts

Ian ruled on 2026-09-19 that every structural option has an equal home in both places, and ADR 0013 holds the ruling, the question file, and the `@` form as items 1 to 6. `sdlc/planning/interface-audit.md` compared the vendor's published client and documentation with the tool, row by row. It found that `decide` never sends the texts for what true and false mean, that the wire code writes `null` for every option description, that evidence built from several pointers goes out as JSON text where the vendor also accepts a JSON object, and that `result.md` shows a `confidence` on a yes/no answer the vendor never sends. `sdlc/planning/ten-use-cases.md` holds the verdicts on the ideas ten use cases raised. `filter` and `rank` are not built, and ticket 0014 builds them after this ticket with the same options.

## Scope

- **The true and false texts.** `decide` gains `--true TEXT` and `--false TEXT`. Either may appear alone. They travel as the question's `criteria.true` and `criteria.false`. A request with neither is byte for byte the request of today, so every recording still replays and the pinned digest holds.
- **A description per option.** `choose` gains `--option LABEL=DESCRIPTION`, which may repeat. The first `=` splits the label from the description. Positional options and `--option` together are a usage error, because the order of options matters and two lists have no order between them. The description travels as the value under the label in `criteria`. An option with no description still sends `null`.
- **The question file.** The first argument of `decide`, `choose`, and `score` is the question, as text or as `@` and a path. A file holds exactly one question: `{"decide": TEXT, "true": TEXT, "false": TEXT, "threshold": CUT, "on": POINTER, "model": NAME}`, `{"choose": TEXT, "options": LIST or MAP, "threshold": CUT, "on": POINTER, "model": NAME}`, or `{"score": TEXT, "levels": LIST, "on": POINTER, "model": NAME}`. Every key beyond the first is optional where the command line makes it optional. An unknown key is an error that names the key. The command must match the file. A question that must start with `@` is written in a file. An unreadable or invalid file is exit 5.
- **Precedence, ruled by Ian on 2026-09-19: the command line, then the file, then the default.** A single value typed beside `@FILE` replaces the file's value: `--threshold`, `--true`, `--false`, `--model`, and `--field`, which replaces `on`. A list typed beside `@FILE` replaces the file's whole list and never merges with it: options for `choose`, as positional labels or as `--option` entries, and levels for `score`. The question text always comes from the file. The question that results must pass every check a typed question passes, and the message names the source of the value at fault.
- **Defaults.** `question-file.md` holds one table with a row for every setting: its command-line spelling, its file key, its default, and what is refused. The defaults: a cut of 0.5 for `decide`, no cut for `choose`, no threshold for `score`, no true or false text, no description, the whole record as evidence, and `jev-latest` as the model. Nothing has a default where a guess would hide a mistake: `choose` needs 2 to 255 options and `score` needs 2 to 10 levels from one source or the other, and their absence is a usage error.
- **What is refused,** with one test each: a command that does not match the file; two of `decide`, `choose`, and `score` in one file, or none; an option the command does not take, such as `--true` on `choose`; a band on `choose`, whether typed or in the file; a cut outside 0 to 1, a cut of 0, or a band whose low is above its high; fewer than 2 or more than 255 options; a repeated, empty, or non-text label; a label or a level that holds a control character, because a label is printed as the answer and a record may supply it; fewer than 2 or more than 10 levels; `--option` beside positional options; and an `--option` with no `=`.
- `--dry-run` prints the question that results. When a file is used, the plan also names the source of each setting as `file`, `command line`, or `default`, so a confused user can see what won. `question_sha256` digests the question that results and never the file's bytes. The same question gives the same digest whether it was typed or read, and an override shows up as a different digest in every row.
- **The seam stays clean.** Every new setting enters the neutral plan under the tool's own word, and the `systemone` adapter alone maps it to the vendor's field. `sdlc/scripts/lint` gains a check that the vendor's field names (`noul`, `criteria`, `systemone`, and the vendor's host name) appear in no source file outside the adapter's module, the default address, fixtures, and tests. ADR 0010's clarification of 2026-09-19 gives the reason: other backends will come.
- The parser and its checks live in `thinkthen-core` and touch no file. Ticket 0015 reuses them for each entry of a question set, so write them for that.
- `--details` rows carry `question_sha256` in `meta`, so a row names the exact question that produced it.
- **The pages.** `decide.md`, `choose.md`, `score.md`, `backends.md`, `channels.md`, `threshold.md`, and `result.md` follow. A new page, `question-file.md`, holds the grammar once, and `annotate.md` points at it for the shape of an entry and gains `true` and `false` on a `decide` entry. `result.md` loses the `confidence` on a yes/no answer. `backends.md` says that status 402 was seen live on 2026-09-19 and that the vendor's pages do not list it. `choose.md` says that one vendor page reports weaker picks above about 240 options. `score.md` says that the vendor's own `score` field is the same weighted average the tool computes, and that one vendor page warns against reading it as an exact size. `channels.md` gains one sentence for a gate: word the question so that yes permits the action, and treat every exit code other than 0 as a refusal. `README.md` gains a short section on what the tool is not for: a loop that needs many decisions a second, and a call from inside a program written in another language. `roadmap.md` gains three held entries with their reasons from `ten-use-cases.md`: a library over the core, a `models` listing, and a `serve` command that is declined.
- **Four live checks**, each a small job under `probes/` through `sdlc/scripts/live`, on made-up labeled cases that are committed before the first call:
  1. The endpoint accepts the true and false texts, and the record says how far they move the probability on forty labeled cases against the same cases without them.
  2. Real option descriptions against `null` on sixty labeled picks.
  3. Evidence from several pointers sent as JSON text against the same evidence sent as a JSON object, on forty labeled cases. If the object form is as good or better, the tool sends an object from then on, the page says so, and the affected recordings are recorded again. If it is worse, the text form stays and the page says why.
  4. The token budget for one request, by one request sized between the two numbers the vendor's pages give.
  The ceiling on questions in one request waits for `annotate`.
- **How-tos.** Two turn green in the form of ADR 0011: say what yes and no mean, and tune a question once and use the same file in the test and in the gate. `documentation-plan.md` gains them and the five pages that `ten-use-cases.md` names, each with the slice that turns it green.

Excluded: `filter`, `rank`, `annotate`, `find`, `transform list`, any `models` command, any `serve` command, and any cut on `confidence`.

## Acceptance

- Unit tests in the core cover every refusal of the question file grammar, with the key named in the message.
- Integration tests against the local listener cover: the exact request body with each new option from the command line and from a file, and the two bodies are byte for byte equal for the same question; the unchanged body with no new option; each override beside `@FILE` with the plan naming its source; every refusal in the list above; the command that does not match the file; `--option` beside positional options; and the plan under `--dry-run`.
- A property test holds that a question typed on the command line and the same question read from a file produce the same request.
- The key and the evidence never appear in any error or Debug output.
- The pinned `decide` digest holds, and every committed recording still replays, except the recordings that check 3 replaces on purpose.
- The record gives each live check's counts, says the cases are few and made up, and states what each result changed. The live spend stays under 150,000 input tokens.
- The spec rung prints the two new how-tos green with the key unset and touches no network. The recordings hold no key.
- The ratchet equals the measured total, and the commit that raises it says what grew, why it earns its lines, and where duplication was looked for first.
- The whole ladder is green, and a second agent reviews the public surface change.
