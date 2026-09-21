# ADR 0015: Names, the license, and a check on every push

- Status: Accepted by Ian on 2026-09-19, except where an item says the agent chose
- Date: 2026-09-19

Ian asked what a recipe is, whether it is only `jq`, and what to call a rerunnable job. He then ruled on the names, on ADR 0012, on `find`, on the license, and on a check at every push.

## 1. Four names, and the word "recipe" retires

| Thing | Name | What it is | What runs it |
| --- | --- | --- | --- |
| What to ask, with its options, levels, and cuts | question file | JSON | `thinkthen` |
| `jq` that reads saved rows | transform | One `.jq` file | `jq` |
| A whole worked example that can be run again | how-to | A folder under `demos/`: the page, the inputs, the question, the transform, the recording | The spec rung |
| A user's own job over the user's own input | pipeline | A Bash script | Bash |

A transform is one of two kinds. A metric reads a whole run and prints numbers. A policy reads one row and names an action.

`recipes/` becomes `transforms/`. The living pages change to the new words. ADRs and records written before this one keep the old word, because they are history.

"Playbook" is held for a question file that carries rules. It enters only if the rules block of ADR 0013 is accepted.

Ian asked why the question file is not called a template. The agent's answer, which he can overturn: "template" promises slots that the input fills. The question file has none. The evidence travels beside the question and is never written into it, and that separation is a safety property. A user who reads "template" will look for slots and ask for them.

## 2. ADR 0012 is accepted as amended

Transforms are files in the repository now. When three families of transform exist, the tool gains `thinkthen transform list` and `thinkthen transform show NAME`, and both only print. A built-in `jq` engine, a pipeline file that the tool runs, and a transform manager stay declined.

Ian said he never meant `jq` inside the binary. He meant that the tool could start `jq` as a separate process. He left the choice to the agent, and the agent declines it. The tool never runs a command, and a pipe already joins the two programs. Under `transform show`, a user writes `jq -f <(thinkthen transform show counts)` and copies nothing.

## 3. `find` is accepted

Ian accepted item 1 of ADR 0014. `find` is built with a `none` option. The test on documents of 100 to 250 lines still runs first, and the page states the size that held.

## 4. The license is MIT

Ian ruled MIT. The agent chose the copyright line "Ian Maurer", as his other public repositories have it.

## 5. No talk with the vendor is needed

The agent had listed a conversation with the vendor as a step before release. Ian ruled that the interface is an open one and no conversation is owed. The lever is removed.

## 6. A check runs on every push

GitHub Actions runs the gate ladder on every push. No rung needs a key, and no rung reaches the paid service. Ian said "unit tests". The agent runs all four rungs, because the demos are tests too and they replay recordings. A key is never stored in GitHub, and a live run stays a job done by hand through `sdlc/scripts/live`.

## 7. A second agent reads everything once for coherence

After the pages change, a second agent reads the whole repository once and reports every place where two living pages disagree. It fixes the plain slips and brings the rest back as questions.

## Consequences

Ticket 0016 carries the rename, the page edits that ADR 0014 names, the license, and the check. The tickets for features stay held until Ian lifts the hold.

## Implementation note — 2026-09-21

The trigger in item 2 has fired. The repository now has three transform families: whole-run metrics, a row policy, and a reviewed-action monitor. Ten `.jq` files total 1,408 lines, the transform index has seven executable Bash blocks, and nineteen how-tos are green. The evidence-based agent decision is to decline `report`; the transform stage therefore stays as files and closes without that command. Ian can overturn this decision.

The fifth outcome still enters at release preparation as the read-only `thinkthen transform list` and `thinkthen transform show NAME` surface. It lists or prints selected embedded transforms, never starts `jq`, interprets a transform, reads user files, or combines reports. The catalog's membership, names, and package location remain for its implementation ticket after ADR 0017's one-crate move. Ian can overturn that timing.
