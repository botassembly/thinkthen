---
flow: build
priority: 40
opens: crates spec specification README.md demos sdlc/ratchet.json sdlc/live-tokens sdlc/planning/documentation-plan.md probes
---

# 0017: Every question option has two homes

Status: waiting on ticket 0013

## Outcome

Every structural option that the vendor's interface gives a yes/no question, a pick, and a placement has one home on the command line and one home in a question file, under the same word. `decide`, `choose`, and `score` read a question file as `@FILE`. The pages stop promising what the vendor never sends. Four live checks settle what reading could not.

## Current Facts

Ian ruled on 2026-09-19 that every structural option has an equal home in both places, and ADR 0013's amendment holds the ruling, the question file, and the `@` form. `sdlc/planning/interface-audit.md` compared the vendor's published client and documentation with the tool, row by row. It found that `decide` never sends the texts for what true and false mean, that the wire code writes `null` for every option description, that evidence built from several pointers goes out as JSON text where the vendor also accepts a JSON object, and that `result.md` shows a `confidence` on a yes/no answer the vendor never sends. `sdlc/planning/ten-use-cases.md` holds the verdicts on the ideas ten use cases raised. `filter` and `rank` are not built, and ticket 0014 builds them after this ticket with the same options.

## Scope

- **The true and false texts.** `decide` gains `--true TEXT` and `--false TEXT`. Either may appear alone. They travel as the question's `criteria.true` and `criteria.false`. A request with neither is byte for byte the request of today, so every recording still replays and the pinned digest holds.
- **A description per option.** `choose` gains `--option LABEL=DESCRIPTION`, which may repeat. The first `=` splits the label from the description. Positional options and `--option` together are a usage error, because the order of options matters and two lists have no order between them. The description travels as the value under the label in `criteria`. An option with no description still sends `null`.
- **The question file.** The first argument of `decide`, `choose`, and `score` is the question, as text or as `@` and a path. A file holds exactly one question: `{"decide": TEXT, "true": TEXT, "false": TEXT, "threshold": CUT, "on": POINTER}`, `{"choose": TEXT, "options": LIST or MAP, "threshold": CUT, "on": POINTER}`, or `{"score": TEXT, "levels": LIST, "on": POINTER}`. Every key beyond the first is optional where the command line makes it optional. An unknown key is an error that names the key. The command must match the file. Options, levels, `--true`, `--false`, `--option`, or `--threshold` typed beside `@FILE` are a usage error, and the message says to change the file. `on` works as `--field` does, and `--field` beside a file that holds `on` is a usage error. A question that must start with `@` is written in a file. An unreadable or invalid file is exit 5.
- The parser and its checks live in `thinkthen-core` and touch no file. Ticket 0015 reuses them for each entry of a question set, so write them for that.
- `--dry-run` shows the texts, the descriptions, and the file's digest as `question_sha256`. `--details` rows carry the same digest in `meta`, so a row names the exact question that produced it.
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
- Integration tests against the local listener cover: the exact request body with each new option from the command line and from a file, and the two bodies are byte for byte equal for the same question; the unchanged body with no new option; each clash beside `@FILE`; the command that does not match the file; `--option` beside positional options; and the plan under `--dry-run`.
- A property test holds that a question typed on the command line and the same question read from a file produce the same request.
- The key and the evidence never appear in any error or Debug output.
- The pinned `decide` digest holds, and every committed recording still replays, except the recordings that check 3 replaces on purpose.
- The record gives each live check's counts, says the cases are few and made up, and states what each result changed. The live spend stays under 150,000 input tokens.
- The spec rung prints the two new how-tos green with the key unset and touches no network. The recordings hold no key.
- The ratchet equals the measured total, and the commit that raises it says what grew, why it earns its lines, and where duplication was looked for first.
- The whole ladder is green, and a second agent reviews the public surface change.
