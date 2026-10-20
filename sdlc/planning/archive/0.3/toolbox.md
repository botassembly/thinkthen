---
date: 2026-10-11
status: design input for 0.3, shaped by Ian's rulings of 2026-10-11
source: Ian's dictation on 2026-10-11, a ChatGPT toolbox spec and a ChatGPT note on the `run` command line, both of 2026-10-11, compared with ThinkThen 0.2 and the question catalog it replaces
---

# ThinkThen Toolbox

0.3 is the proxy and the toolbox. A tool replaces a question. A toolbox is a folder of tools that an agent can run, read and understand. The toolbox is local only. The proxy serves functions, never tools.

## Ian's ruling, 2026-10-12: toolboxes are local only

- A toolbox lives on the caller's machine and runs only there. The caller has total control over what runs on their computer.
- The proxy never stores, serves or sends tool definitions, scripts or shell commands. A server that tells a client what to run is a security hole, even with a trust setting.
- The proxy hosts functions only: prediction calls, perhaps with saved wording and thresholds, that return answers and probabilities between 0 and 1. A function step in a local tool may call those.
- This replaces the 2026-10-11 lines below that let a tool live on the proxy and be fetched, and the "Shell steps from a proxy" trust design.

## Ian's rulings, 2026-10-11

- **Tool replaces question.** A toolbox is a collection of tools. The toolbox replaces the question catalog.
- **The toolbox describes itself.** It is a folder whose descriptions and help let an agent learn what each tool does and how the toolbox works, without outside documentation.
- **A tool is a short pipeline (Ian, 2026-10-11, from a private trial-matching project's mail).** Most tools wrap one function. A tool may also chain steps: function steps such as `filter`, `rank` and `find`, and shell steps such as a database search, `head` or jq. That project's example: Postgres candidates, then `filter`, then `rank`, then keep five, then `find --none`. No new function is needed, because 0.2 functions already read and print records.
- **The client runs the whole tool.** Locally and through a proxy, the caller's process runs every step. Shell steps run on the caller's machine. Function steps go to the configured backend, which is the proxy's function door when the backend type is ThinkThen. The proxy never runs a shell command. (Superseded 2026-10-12: a tool never lives on the proxy.)
- **`run`, not `execute`.** `thinkthen run TOOL` invokes a tool, from the XDG folder or a folder given by path. `run` is the eleventh function. It works in every library and the CLI, locally or against a proxy. The Rust core runs the steps, so every library runs shell steps the same way. A program calls `run("support/triage", ...)` as it calls `decide(...)`.
- **MCP is separate.** The MCP redesign lives in `mcp.md`. MCP invokes the ten functions and `run`.
- **jq is invoked, never bundled.** A tool may carry an optional jq recipe that filters or reshapes its answer, like a small script piping into jq, so the toolbox returns the shape the caller expects. It is lightweight, optional and low priority.
- **GEPA as a recipe.** GEPA need not ship in 0.3. It should work as a recipe, so a user can run GEPA on their own toolbox.
- **Evals are a separate design.** See `evals.md`.
- **Redesign.** `audit`, `diff`, `find`, and the jq handling in `transform` need fresh designs.
- **Scope is fine.** Twelve tickets is small for this team. Competitors and name collisions are not a concern. ThinkThen is the open-source way to run your own proxy over your own data.
- **No model comparisons.** Marketing says ThinkThen supports every model. People test models on their own data and later fine-tune their own. Enterprise supports that later.

## What 0.2 already has

| Toolbox part | 0.2 today |
| --- | --- |
| A tool: the function, wording, options, levels and threshold | A saved question file, with author, wording version and declared inputs |
| Call by name | `@NAME` reads `questions/NAME.json` under the XDG config folder |
| A folder of tools | The named questions folder, and question sets for `annotate` |
| Context from files | `--context FILE`, per-item context (0407) and `--pointer` |
| See the request without sending it | `--plan` prints what would be sent and stops |
| Grading | `audit` grades saved answers against a key and suggests a bar. `diff` compares runs |
| jq | Built-in transforms in `thinkthen transform` |

0.2 results already carry the model, the question's SHA-256, the answer ID and the score in `--details`, and not sure in exit code 3. `find --none` and `choose` thresholds let an answer say nothing fits.

New in 0.3: the toolbox folder and its self-description, multi-step tools, `run` in every library and the CLI, named inputs assembled by the tool, optional jq recipes, and the GEPA recipe. The proxy serves the functions that tools call.

## What a tool carries (from a private trial-matching project's asks, 2026-10-11)

- **Named inputs and record fields in the wording.** A tool asks per record, for example: does the concept NAME, with synonyms SYNONYMS, mean what QUOTE says? QUOTE is a named input given once; NAME and SYNONYMS come from each record.
- **An identity.** A tool is named by a hash of its definition. Every result adds the tool's name and hash to the model and question hash 0.2 already returns, so a caller can store an answer as evidence with its version.
- **Declared needs.** A tool lists what its shell steps require, such as `psql` and a database connection. A run checks them first and fails with a clear message before the first step.
- **Step identity on the wire.** Each function call a tool makes carries the tool's name, hash and step number, so the proxy routes, records outcomes and tunes at the level of the tool.
- **Overrides in the result.** When the proxy overrides a step's threshold, the result says which threshold ran, so a local run and a proxy run of the same tool can be compared.

## Shell steps from a proxy

Ruled out on 2026-10-12. Tools are local only, so no proxy hands a client a shell command, and no trust setting is needed.

## Locks, explained

A lock is the ChatGPT spec's word for a tool setting the caller may not change on a call, such as the wording, the threshold or the model. Today the caller wins: a typed value beats the question file. The 2026-10-06 ruling already settles this. The code owns its threshold, and the proxy may override it, so a business controls decisions in one place. Locally, the caller keeps winning. No new decision is needed.

## Single-player and proxy mode

Each 0.3 feature must say how it behaves in both modes. The starting rule: the toolbox and `run` work the same with no proxy. The proxy adds a shared cache, central overrides, routing and outcome records. It never adds a tool behavior that a local run cannot reproduce.

## To design with Ian

1. The toolbox folder: the manifest, where descriptions and help live, and how an agent reads them.
2. The redesign of `find`, `transform` and jq recipes.
