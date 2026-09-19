# ADR 0013: The question file is the one direct file

- Status: Proposed. Ian had not read it when he ruled on the others, and the agent explained it on 2026-09-19. The amendment at the end adds his own question about reuse. The text keeps the word "recipe" as it was written, and ADR 0015 renames it
- Date: 2026-09-19

An agent proposal in answer to three questions from Ian. Can the tool run a recipe file directly? Would that be the way an assembly runtime such as botassembly uses the tool? Which format should such a file take: JSON, TOML, or Markdown with frontmatter? It becomes Accepted on his word. Nothing is built from it until then.

## Context

ADR 0012 proposes that a `jq` recipe is a folder of files and that the tool at most lists and shows built-in recipes. It declined a recipe file that the tool executes as a pipeline. Ian's question returns to that point with a new reason: a declarative runtime wants one file per decision, with no shell glue.

Three facts bear on the answer.

- The tool already runs one file directly. `thinkthen annotate FILE` reads a JSON file of named questions, options, levels, thresholds, and what each question may see. ADR 0007 rules that the file holds questions and nothing else, and that every file the tool reads is JSON.
- botassembly writes its assemblies as Markdown with frontmatter. Its routing stage is prose plus a bullet list of named options with descriptions, which is a `choose` question. Its stage checklist is a list of statements to verify, which is a set of `decide` questions. Its gates are shell scripts that answer with an exit code, which `decide` already is.
- Ticket 0008 showed where `jq` hurts. Three mistakes in the comparison recipe failed only at run time. Three other recipes read a missing probability as "below every cut" and scored the row as a no without a word. A policy is the place where that kind of slip does harm.

## Options

- **A. Add nothing.** The runtime calls `decide`, `choose`, and `annotate` from its stages and gates, and it turns its own Markdown into those calls. The policy that combines answers lives in the runtime or in `jq`.
- **B. A recipe file that the tool executes: steps before, the questions, and `jq` steps after.** It needs a `jq` engine inside the binary or a call out to `jq`. The first is a large dependency and a second language. The second breaks the rule that the tool never runs a command. The steps before the questions belong to the caller in every case.
- **C. The question file gains one optional block of rules.** A rule names values of earlier answers and gives an action and a reason. Rules are read in order, the first match wins, and a default closes the table. `annotate` then adds two fields to each record: `action` and `reason`. No `jq` is inside the tool, and no verb is added.
- **D. The tool reads Markdown with frontmatter, or TOML, as well as JSON.**

## Decision

1. **The question file is the one direct file, and a runtime calls it.** One file is one decision: a record goes in, and named answers come out. A runtime lowers its own stage files to this file and to the three verbs. For botassembly that means a routing stage becomes `choose` with option descriptions, a checklist becomes `annotate` with one `decide` per item, and a gate script calls `decide` as it can today. The `--details` row goes into the runtime's own run record. This needs no new feature.
2. **Option B stays declined,** for the reasons ADR 0012 gives.
3. **Option C is the one extension worth testing, and a demo decides it.** A rule is the same kind of thing as a threshold. A threshold turns one probability into a value, and a rule turns several values into an action. Both run locally after the judgment, and `--details` keeps every probability, so a later reader can still apply another policy. The table is closed on purpose: a condition is a question name with the values it may take, or a lowest score; every condition in a rule must hold; an unresolved answer matches no rule, so it falls to the default, and the usual default is "review". That last property removes the `jq` trap by construction. One digest then covers the questions and the policy, which is what an audit row wants. The flagship triage demo is written twice when `annotate` lands, once with a policy in `jq` and once with rules, and the comparison decides. If the `jq` form reads well and holds no trap, the rules stay out.
4. **JSON stays the only format the tool reads.** A question file must be editable by `jq`, because changing one threshold before a run is a recipe. One parser means one digest and no second set of rules for quoting. YAML frontmatter and TOML each bring a dependency. Markdown brings a document convention that the tool would have to define and parse: which heading is a name, which bullet is an option, which line is a setting. That convention is authoring, and authoring belongs to the layer above. botassembly already owns a Markdown format and its parser, so it writes the JSON. `--dry-run` shows exactly what the lowered file sends.
5. **Long prose is the one real cost of JSON.** If people who write question files by hand find it painful, TOML as a second spelling of the same structure is the smaller step, and it waits for that evidence.

## Consequences

Nothing changes now. The roadmap gains the rules block as a held idea with this test. The triage demo's ticket writes both forms. A note for the assembly runtime goes to its own repository when Ian takes up that work.

## Amendment, 2026-09-19: a verb reads one named question from the file

Ian asked how a user keeps a set pattern for `decide` or `choose`, with its options and its cut, and uses it again and again. He asked whether the answer is a Bash script.

Today the answer is a Bash function or a script. That works, and it has one weakness. How-to 13 tunes a cut on labeled cases. The user then types the question and the cut into a script by hand, and an eval cannot read a Bash function. The eval and the gate can drift apart, and nothing fails when they do.

Proposed, as item 6 of this decision: **every verb that takes a question can read one from a question file.**

```sh
thinkthen decide --from checks.json --name refund_request < message.txt
thinkthen annotate checks.json --jsonl < labeled-cases.jsonl
```

- `--from FILE` names the question file, and `--name NAME` picks one question in it. A file with one question needs no `--name`.
- The verb must match the question's type. `filter` and `rank` take a `decide` question.
- The question's options, levels, cut, and `on` apply as they do under `annotate`. The output and the exit code are the verb's own.
- A question typed on the command line beside `--from` is a usage error, and so is `--threshold` beside `--from`. A user changes the cut in the file with `jq`, and the file's digest changes with it.

The file is then the one saved form for every verb. The eval runs `annotate` over labeled cases, the gate runs `decide --from` on the same file, and the measurement applies to the gate because both read the same words and the same cut. The parser exists once `annotate` lands, so the cost is two options on five commands. A how-to proves the need: tune a cut in an eval, then ship the same file in the gate.

The agent recommends it. It waits for Ian's word, and it would land after ticket 0015.

