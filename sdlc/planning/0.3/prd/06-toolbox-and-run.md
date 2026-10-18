---
date: 2026-10-13
status: draft PRD
release: 0.3
source: Ian's rulings in `toolbox.md` (2026-10-11 and 2026-10-12), `proxy.md`, `evals.md` and `mcp.md`; PRD 00, PRD 02; the ChatGPT toolbox spec and `run` note of 2026-10-11; a partner team's toolbox feedback of 2026-10-11; ThinkThen 0.2's `specification/` and `thinkthen --help`. The 39 worked questions and sources are in [06-toolbox-and-run-questions.md](06-toolbox-and-run-questions.md)
---

# PRD 06: Toolboxes and `run`

A tool replaces a question. A toolbox is a folder of tools an agent learns from help alone. `run` is the eleventh function.

## Who and what's in it for them

- **The builder** names a question or a short pipeline once and calls it the same way from every surface. Proof: `run` of a four-step lookup prints byte for byte what the hand-written pipe prints.
- **The agent** runs one name instead of rebuilding a pipe, and learns every tool from help. Proof: the self-description experiment below.
- **The data person** stores each answer with its tool, digest and step, as versioned evidence. Proof: a query grouping stored answers by digest.
- **The operator** routes, overrides and records outcomes per tool. Proof: a local proxy trace grouped by tool.
- **Everyone** sees each tool's score on its own cases. Proof: PRD 02's BANKING77 arm A.

## Use cases

U13 to U17 from PRD 00, with U6, U15's SQL path, U19 and U20 for `run`, and U33's GEPA recipe.

## The design

**A tool file is a question file grown up.** A one-step tool is 0.2's question file plus a one-sentence `description`. A multi-step tool lists `steps`: functions with their wording, or shell steps, each a program and its arguments. Steps run top to bottom, every time. Only the last step may read one document or answer through its exit code. No step branches, loops or calls another tool.

```text
concepts/
  toolbox.json        {"name": "concepts", "description": "Find the vocabulary concept a phrase means."}
  lookup.json
  lookup.cases.jsonl
  lookup/candidates.sql
```

```json
{"name": "lookup", "description": "Pick the concept a quoted phrase names, or none.",
 "input": "none", "inputs": {"quote": "The phrase as written in the source."},
 "needs": {"programs": ["psql"], "env": ["DATABASE_URL"]},
 "steps": [
  {"step": "candidates", "run": ["psql", "-At", "-v", "q={quote}", "-f", "lookup/candidates.sql"]},
  {"step": "match", "filter": "Does {/name}, also called {/synonyms}, mean what \"{quote}\" says?"},
  {"step": "order", "rank": "Is {/name} the concept \"{quote}\" names?", "top": 5},
  {"step": "pick", "find": "Which concept does \"{quote}\" name?", "none": true, "on": "/name"}]}
```

`{quote}` fills a named input and `{/name}` a record field. 0.2 already quotes each record inside its own question, so filled-in wording batches and caches as today.

**The surface** follows PRD 00:

```text
thinkthen run TOOL [FILE...] [--arg NAME=VALUE] [--arg-file NAME=PATH] [--plan] [--details] [--toolbox DIR]
thinkthen toolbox list | describe TOOL | check [DIR]
thinkthen eval bench TOOL                      # PRD 02, on the tool's cases
```

Python calls `thinkthen.run("concepts/lookup", args={"quote": q})`. Every library returns the last step's function and typed result. SQL's `thinkthen_run(tool, evidence)` returns JSON. The `toolbox` commands live in Rust and the command line only, by Ian's 2026-10-14 ruling.

**Finding and learning tools.** `run` looks in `--toolbox`, `THINKTHEN_TOOLBOXES`, the configuration file's `toolboxes`, then `toolboxes/` under the configuration home. Names are `toolbox/tool`; a step's question id is `toolbox/tool/step`. `toolbox list` prints one JSON line per tool: description, inputs, output, needs and grade. `describe` and `run TOOL --help` print the full contract, with the first case as a worked example, all generated from the tool files.

**Identity.** The tool digest covers everything that can change an output: steps, wording, thresholds and named files' bytes, with line endings made LF. Help and cases stay out. Every detailed row carries `meta.run` (tool, digest, step) and `threshold_from` (tool, call or proxy). A ThinkThen proxy receives the same three fields with each function step, and never a shell step. 0.2's `meta.tool` becomes `meta.version`, and `thinkthen.run/1` becomes `thinkthen.facts/1`. PRD 01 owns question ids and wording versions.

**Running.** Steps run together and pass JSON Lines. `rank`, `find` and `relate` answer only after every earlier step finishes cleanly. A failed step stops the run and names itself; a failed shell step exits 5. Otherwise a tool exits as its last step (PRD 00). `--plan` runs no shell step and sends nothing. Spending caps cover every step.

**Shell steps.** A tool never gives its caller more power to run programs than the caller already has. The command line and libraries run any toolbox the caller names. MCP runs tools only by name from toolboxes configured at start, and refuses a path (PRD 07). SQL and hosts that cannot start processes refuse them. ThinkThen starts programs with no shell, fills placeholders inside single arguments, and passes only declared variables, never a key. It checks needs before the first step. A folder another user can write loses its shell steps.

**Cases and sharing.** `TOOL.cases.jsonl` holds evidence, inputs and the expected output. Grades live in the store by digest and model, so a cloned toolbox says "not graded here". Teams share toolboxes with git. jq is a shell step. GEPA is a recipe over `eval bench`.

## What must be true

| Risk | What retires it |
| --- | --- |
| Agents misuse tools learned from help | Fresh agents from two model families, given only help and three toolboxes, do 30 tasks. Target: 90% right on the first call |
| `run` differs from the pipe it replaces | Run both from one recording, locally and through a local proxy, and diff |
| A failed step yields a confident wrong answer | Fault injection: a shell step dies after N records, and `find` prints nothing |
| Shell steps open a hole | A refusal suite: injection, escaping paths, writable folders, unconfigured MCP paths, SQL |
| Windows differs | CI on three systems: equal digests and replayed output |
| Filled-in wording costs too much | Tokens and agreement against fields as evidence |

## Out of 0.3

Tools on the proxy, and steps that call a function saved there. Conditions, loops, parallel steps and tools calling tools. Per-step overrides from the command line. Sandboxing. Install, update and registry commands. Grades that travel in the folder. A version field. YAML.

## Questions for Ian

Ian's open questions for every PRD are in one list, ordered by importance, at the start of the PRD PDF. This PRD's are numbers 1, 2, 13 and 16.
