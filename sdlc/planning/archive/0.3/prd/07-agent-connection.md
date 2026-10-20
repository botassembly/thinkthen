---
date: 2026-10-13
status: draft PRD
release: 0.3
source: Ian's MCP rulings of 2026-10-11 in [../mcp.md](../mcp.md); his 2026-10-13 rulings in [../data.md](../data.md); [PRD 00](00-use-cases-and-surfaces.md); ThinkThen 0.2's `specification/mcp.md`, `settings.md`, `channels.md` and `records.md`, its help text and a measurement of its MCP catalog; thinkthen idea 0566; BioMCP's MCP design, ticket 0435 and its 2026-09-01 agent study; Anthropic's and OpenAI's published tool-design guidance. The 28 worked questions and sources are in [07-agent-connection-questions.md](07-agent-connection-questions.md)
---

# PRD 07: How agents connect

An agent with a shell uses the command line. An agent without one uses one MCP tool. Both give the same words, help, JSON and exit codes.

## Who and what's in it for them

- **The agent** gets a third kind of search, beside keyword and vector search, for under 1,000 tokens of context before its first call. Proof: CI measures the tool description, the server instructions and `help` against a budget. 0.2's ten-tool MCP catalog measures about 271,000 tokens.
- **The agent** learns every function, tool and job from help alone. Proof: the prototype below.
- **The person** an agent acts for is named on every write it makes, and is never billed past a cap the agent cannot raise. Proof: a demonstrated refusal and the recorded actor.
- **The builder** maintains one grammar. The tool uses the command line's parser, so no second schema can drift.

## Use cases

U5, U6, U7, U19, U20, U48 and U49 from PRD 00, and the agent side of U8, U15, U25 and U26.

## The design

**One tool.** `thinkthen mcp` serves one tool, `thinkthen`, over local stdio, with two fields. `command` holds a command line, such as `filter "Mentions a KRAS mutation?" --jsonl --input hits.jsonl`, split by shell quoting rules with no expansion, pipes or shell. A leading `thinkthen` is optional, so a line copied from help works. `input` holds what the command line would read on standard input, so long evidence needs no quoting.

**One reply.** The reply is standard output plus one closing line, such as `exit 3: not sure`. A no or a not sure is an answer. Exit codes 2, 4, 5 and 70 mark an error. A reply over the budget shows its first lines, counts the rest, and names the option that writes the whole result to a file.

**Help in layers.** The tool description, about 200 tokens, names the jobs and functions and says to ask `help`. The MCP instructions, about 150 tokens, say when ThinkThen fits. `help` lists every command by job, under 600 tokens. `help decide` gives examples and one line per option, under 800; `help decide --long` gives today's full text. The command line prints the same layers. `help` lists the configured tools, one line each; `run TOOL --help` gives a tool's inputs, needs and cases result. Errors list the valid choices and the command to run next. Successful replies suggest nothing, because BioMCP's agent study found suggestions misled agents. `help guide` prints the agent guide from idea 0566, which also ships as a skill file.

**What MCP runs.** The ten functions, `run` with configured tools, and `help`, by Ian's 2026-10-14 ruling. Never `toolbox`, `answers`, `curate`, `eval`, `status`, `admin`, `login`, `proxy` or `mcp`; an agent with a shell uses the command line for those. Nothing over MCP deletes, so the tool truthfully declares itself not destructive.

**Settings fixed at start.** The MCP configuration sets the backend, address, key, database, toolbox, file roots, caps and person. A call may set function options, `--plan`, a configured setup by name, and a lower cap. It can never name an address or key, or raise a cap.

**Who acted.** The person is `--user NAME` or the current user at start. The agent is `THINKTHEN_AGENT`, or the name the MCP client reports. Every write records both and the surface `mcp`, as claimed; 0.3 verifies no one.

**Cost.** Session caps on live requests and estimated input tokens cover every call. Cached answers cost nothing. A refusal names the cap, the spend so far and the setting that raises it. `help` shows the session's spend under its caps.

**Files.** The server reads only inside the roots the client declares, or its start folder, plus any `--root DIR`. Tools run by name from the configured toolbox only.

**Proxy.** The MCP server is a client like the command line. With a ThinkThen backend it sends function calls to the proxy, runs tools locally, and curates in the proxy's store.

## What must be true

- **One tool serves agents as well as typed tools.** A prototype runs 24 tasks, each with a check written first, in four arms: a string command, an argument list, ten slim typed tools, and the command line through a shell. Three agents run each arm: a frontier model, a small fast model, and an open-weight model in a second host, three times per task, from fresh context, with answers replayed so ThinkThen spends nothing. Tasks cover every function, files and folders, a BioMCP results file, a toolbox tool, `--plan` before a 5,000-record run, a cap refusal, a refused command, a quoted question and an oversized reply. One tool passes when its success sits within five points of the best MCP arm on every model, under 10% of its calls fail, and catalog plus help costs under 2,000 tokens a task. If the list wins on quoting, the field becomes a list. A second pass tests the guide.
- **Help stays small.** CI holds each layer to its budget. BioMCP's catalog grew 3.6 times when no bound was enforced.
- **Parallel calls work.** 0.2 answers a second active call with a busy error. 0.3 runs four at once under one set of caps.

## Install

The packaging experiment showed an agent can start the server with nothing else installed: `uvx thinkthen mcp` and `npx thinkthen mcp` answered an MCP start request offline on Linux. PRD 09 owns the packages.

## Out of 0.3

Remote MCP over HTTP. Sampling: the server never asks the client's model anything. Resources, prompts and elicitation. Typed tools, unless the prototype earns them. Pipes.

## Questions for Ian

Ian's open questions for every PRD are in one list, ordered by importance, at the start of the PRD PDF. This PRD's are numbers 8 and 9. Answered: MCP runs the ten functions, `run` and help only (Ian, 2026-10-14).
