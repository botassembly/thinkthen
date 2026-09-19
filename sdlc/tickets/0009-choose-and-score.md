---
flow: build
priority: 50
opens: crates spec specification/fixtures sdlc/ratchet.json sdlc/live-tokens demos/02-route-a-ticket demos/05-sort-a-folder demos/17-rate-and-sort demos/20-not-stated-or-false demos/README.md sdlc/planning/documentation-plan.md
---

# 0009: `choose` and `score`

Status: landed

## Outcome

`thinkthen choose QUESTION OPTION...` and `thinkthen score QUESTION LEVEL...` work on one document as `specification/choose.md`, `score.md`, `result.md`, `threshold.md`, and `channels.md` describe. All three of the backend's question types are then in the shell. Four how-tos are green.

## Current Facts

Tickets 0005 to 0007 landed the flat `decide`, the two variables, the live script, and the removal of profiles. The adapter translates the yes/no type alone. ADR 0009 item 2, accepted in ADR 0010, rules that a detailed result keeps the probability of every option or level and the vendor's `confidence`. `score` has no demo today. `sdlc/issues/2026-09-19-ideas-carried-from-the-design-captures.md` holds advice that belongs in the help of both verbs.

## Scope

- `choose`: two or more options, at most 255, blank labels refused. The bare value is the winning label as a JSON string, or `null` when the winning probability is under the cut or two options tie exactly. `--threshold T` takes a single cut, and a band is a usage error. `--raw` prints the label with no quotes. `--quiet`, `--details`, and `--dry-run` behave as on `decide`. The exit codes are those of `channels.md`.
- `score`: two to ten ordered levels, lowest first. The bare value is the backend's probability-weighted position as a number. `score` takes no threshold. The help shows `jq -e '. >= 2'`, says that one number hides the shape of the distribution, carries the measured-weakness warning, and points to `choose` with ordered labels for branching.
- The `choose` help says how to tell "not stated" from "false" with labels such as `supported`, `contradicted`, and `not_stated`.
- The adapter translates both types to and from the wire format in the pure core, with fixtures. `answer` keeps every probability and `confidence`. The `question` object carries `options` or `levels`. The request bytes of `decide` do not change, so every pinned digest holds.
- Live recordings through `sdlc/scripts/live` turn four how-tos green in the form of ADR 0011: demo 02, demo 05, a new demo 17 (rate on a scale, sort by it, and test it with `jq -e`), and a new demo 20 (tell "not stated" from "false"). If a live answer disagrees with what a page asserts, the page changes to what the model really said when its point survives, and the record says so. If the point does not survive, the builder stops and reports.

Excluded: `--lines`, `--jsonl`, `--field`, `--options POINTER`, and `--input`. Build Settled sections only.

## Acceptance

- Table tests pin the cut and the exact-tie rule of `choose`, and the weighted score on known distributions. A property test holds that the bare `choose` value is always one of the options or `null`.
- Integration tests against a local listener cover the bare values, `--raw`, `--quiet`, `--details` with every probability and `confidence`, `--dry-run`, the refusals (one option, 256 options, a blank label, a band on `choose`, a threshold on `score`, one level, eleven levels), and the exit codes.
- The pinned `decide` digest holds. The key and the evidence never appear in any error or Debug output. The recordings hold no key.
- The spec rung prints the four new green demos with the key unset and touches no network.
- The live tokens spent are in `sdlc/live-tokens` and in the table in `sdlc/planning/plan.md`.
- The ratchet equals the measured total, and the commit that raises it says what grew, why it earns its lines, and where duplication was looked for first.
- The whole ladder is green, and a second agent reviews the public surface change.
