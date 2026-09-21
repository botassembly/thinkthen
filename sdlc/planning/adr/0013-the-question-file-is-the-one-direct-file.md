# ADR 0013: The question file is the one direct file

- Status: Accepted by Ian, in rulings of 2026-09-19 and 2026-09-21. This page was written again whole on 2026-09-19, and git holds the earlier drafts
- Date: 2026-09-19

Ian's picture is the frame. The decider is a function. The evidence is its input. A question with its settings is its arguments. `decide`, `choose`, and `score` take the arguments for one question, and `annotate` takes the arguments for several. A user who has tuned a question and a cut wants to keep them and use them again, in a test and in a gate, without the two drifting apart. An assembly runtime such as botassembly wants one file per decision with no shell glue.

## The design

1. **A question file holds one question with its settings.** The first key names the command and holds the question's text.

   ```json
   {"decide": "Does this message ask for a refund?", "true": "It asks for money back.", "false": "It asks for anything else.", "threshold": "0.2:0.8"}
   {"choose": "Which team owns this?", "options": {"billing": "Charges and refunds.", "support": "Anything else."}, "threshold": 0.6}
   {"score": "How urgent is this?", "levels": ["Can wait.", "This week.", "Today."]}
   ```

   One key does the work of a `type` key and a `question` key, the file reads like the command line, and a file with two of the three keys or with none is refused. Every other key is optional where the command line makes it optional: `true`, `false`, `threshold`, `options`, `levels`, `on`, and `model`. An unknown key is an error that names it.
2. **A command reads the file as `@` and a path in the question's own place.** `thinkthen decide @refund.json < message.txt` prints what `decide` prints and exits as `decide` exits. The first argument is always the question, as text or as `@FILE`, and no option is added. `curl` reads `@` the same way. The command must match the file.
3. **Every structural option has two homes under one word.** Each setting the backend accepts for a yes/no question, a pick, and a placement has a spelling on the command line and a key in the file: `--true` and `true`, `--false` and `false`, `--threshold` and `threshold`, `--option LABEL=DESCRIPTION` and `options`, the levels and `levels`, `--field` and `on`, `--model` and `model`. `sdlc/planning/interface-audit.md` checked the vendor's pages and its published client row by row, and `question-file.md` holds the one table.
4. **The command line beats the file, and the file beats the default.** A single value typed beside `@FILE` replaces the file's value. A list replaces the file's whole list and never merges with it. The question's text always comes from the file. `--dry-run` names the source of each setting as `file`, `command line`, or `default`. `question_sha256` digests the question that results and never the file's bytes, so the same question gives the same digest whether it was typed or read, and an override shows in every row.
5. **Defaults exist only where a guess is safe.** A cut of one half for `decide`, no cut for `choose`, no true or false text, no description, the whole record as evidence, and the newest model. `choose` needs 2 to 255 options and `score` needs 2 to 10 levels from one source or the other, and their absence is a usage error.
6. **The test and the gate are the same command.** `thinkthen decide @refund.json --jsonl --details < labeled-cases.jsonl` measures the question, and `thinkthen decide @refund.json < message.txt` is the gate. Both read one file.
7. **A question set holds several questions, and `annotate` reads it.** Each entry has exactly the shape of a question file, and one parser reads both. `jq` lifts one question out of a set and builds a set from question files. The set takes an optional top-level `model`, and an entry's own `model` is refused, because one request has one model. No option on the command line overrides one entry, because the command line would have to address a question by name. `--model` and `--field` apply to the whole run.
8. **These are the only files the tool runs directly, and a runtime writes them.** One file is one decision: a record goes in and named answers come out. A runtime turns its own stage files into question files and the commands. For botassembly a routing stage becomes `choose` with option descriptions, a checklist becomes `annotate` with one `decide` per item, and a gate script calls `decide` as it can today. The `--details` row goes into the runtime's own run record. The tool never executes a file of steps, for the reasons ADR 0012 gives: it would need `jq` inside the binary or a call out to `jq`, and the tool never runs a command.
9. **JSON is the only format the tool reads.** A question file must be editable by `jq`, because changing one cut before a run is a one-line transform. One parser means one digest. YAML and TOML each bring a dependency, and Markdown brings a document convention that belongs to the layer above. Long prose is the one real cost of JSON. If people who write question files by hand find it painful, TOML as a second spelling of the same structure is the smaller step, and it waits for that evidence.

Ian accepted the names "question file" and "question set" and said he does not love either. "Template" was declined, because evidence is never spliced into the question. He may return to the names.

## Declined for version 0.1: a block of rules in the question set

A triage pipeline asks several questions with `annotate`, and something must then turn the answers into one action: draft a reply, block, or send to a person. Today that policy lives outside the tool, in a `jq` transform or in the script's own `case`. The idea is a small closed table inside the question set.

```json
"rules": [
  {"when": {"legal_threat": true}, "action": "escalate"},
  {"when": {"refund": true, "urgency_min": 4}, "action": "draft"}
],
"default": "review"
```

`annotate` would add `action` and `reason` to each row. One file and one digest would cover the questions and the policy. An unresolved answer would match no rule and fall to the default, which closes a trap that ticket 0008 found: a careless `jq` policy reads an unresolved answer as a plain no.

- **Option 1. Decline it for version one.** The flagship how-to is written once, with the policy as a tested `jq` transform that sends every unresolved answer to review. The idea stays on the roadmap.
- **Option 2. Compare on paper.** The flagship is written once for real, and a mock-up of the rules form sits beside it and does not run.
- **Option 3. Build it and test it.** The rules block is built, the flagship is written twice, and the better page wins. This means a second small language inside the tool, with its own parser, refusals, page, and tests, and code to delete if `jq` wins.

Ian chose option 1 on 2026-09-21. Rules are what a caller does with the answer, and they are no argument to the function, so they break the picture at the top of this page. Bash `case` and `jq` already do this job, and a shell user expects them to. The trap is fenced: the policy ships as a tested transform that fails closed. The cost is that the policy lives in a second file and the row's digest does not cover it. The idea returns only with evidence that the tested transform cannot express a needed policy clearly.

## Consequences

Ticket 0017 builds items 1 to 6 and writes `question-file.md`. Ticket 0015 builds item 7 over the same parser. Ticket 0041 builds the tested policy transform and the flagship how-to. A note for the assembly runtime goes to its own repository when Ian takes up that work.
