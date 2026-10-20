---
date: 2026-10-13
status: draft PRD
source: Ian's MCP rulings of 2026-10-11 in [../mcp.md](../mcp.md); his 2026-10-13 rulings in [../data.md](../data.md) and [../README.md](../README.md); [PRD 00](00-use-cases-and-surfaces.md) and its questions file; ThinkThen 0.2's `specification/` (mcp, settings, channels, records, files, roadmap) and `thinkthen --help`; thinkthen idea 0566; Ian's strategy notes and his later notes on the enterprise edition and the interface; ThinkThen's ideal state; BioMCP's `docs/reference/mcp-server.md`, `docs/blog/we-deleted-35-tools.md`, ticket 0435, ticket 1251 and `sdlc/planning/notes/2026-09-09-biomcp-interface-and-agent-behavior-study.md`; outside sources listed at the end
---

# PRD 07 working file: the hard questions on how agents connect

This file backs [07-agent-connection.md](07-agent-connection.md). Each question gives the options, the reasoning and a recommendation. **Ian** marks the three that need his direction. The PM may carry the rest into tickets, and Ian can overturn any of them. Claims about 0.2 cite [`specification/`](../../../../specification/) or the 0.2 help text.

## Measurements taken for this PRD

These numbers come from the 0.2 debug build of 2026-10-11 in `target/debug/thinkthen`, counted with the `cl100k_base` tokenizer that BioMCP also uses.

| What | Size |
| --- | --- |
| `thinkthen --help` | 1,836 bytes, 405 tokens |
| One function's `--help`, such as `decide` | 4,253 to 9,357 bytes, 916 to 2,043 tokens |
| All ten functions' `--help` together | about 17,000 tokens |
| 0.2's `thinkthen mcp` `tools/list`, ten tools | 1,140,254 bytes, about 271,000 tokens |
| One 0.2 MCP tool | about 43 KB of input schema and 71 KB of output schema |
| Options on `filter` | 38 |

The 0.2 MCP catalog alone is larger than most agents' whole context. A host that loads tool definitions up front cannot use it. This matters for 0.2's release gate, which asks that a fresh agent "runs the MCP server inside an agent" (Ian's strategy notes), and the thinkthen PM should know. For 0.3 it settles the direction: ten full typed schemas are not an option.

## What BioMCP learned

BioMCP is the model Ian named. Its history has three steps.

1. **36 tools to one command string.** The Python version's 36 tools cost about 16,600 tokens. The Rust version served one `biomcp` tool taking a command line (`docs/blog/we-deleted-35-tools.md`).
2. **One string to one string plus six typed tools.** A benchmark pilot found 18% of 638 MCP calls failed, 112 calls, "almost all because the agent guessed the CLI wrong": 38 nonexistent flags, 33 limits out of bounds, 11 wrong section names (ticket 0435). Typed `search` and `get` schemas now enumerate the valid entities and sections and bound the limit. The command string stays as an escape hatch for everything else.
3. **The 2026-09-01 study.** With four tools, Claude Code finished 27 of 31 tasks, and 29.1% of tool calls failed. 19 failures invented a section name and 15 wrote query syntax into a plain-text field. The task still succeeded in most of those runs; the failures cost turns and tokens. Static disclosure of available filters helped. Suggested next steps after a reply hurt in three recorded cases. The catalog grew 3.6 times, from 1,628 to 5,922 tokens, because the command tool's description was generated from a reference that kept growing and the release had no enforced bound. The study also noted that its host deferred tool schemas, which may have raised the error rate (BioMCP study note, 2026-09-09).

Two later facts matter. A top-level `oneOf` in a tool schema is rejected by OpenAI and Gemini function calling, so BioMCP flattened its schemas (ticket 1251). And BioMCP's tools are free, read-only lookups, so its HTTP server can run without authentication (`docs/reference/mcp-server.md`). ThinkThen's calls cost money and some write.

What carries over: one tool keeps context small; failures cluster where values are enumerable and the agent guesses; static affordances help and dynamic advice can hurt; a size bound that nothing enforces will be broken.

## Shape of the tool

**Q1. One tool, ten typed tools, or one tool plus a few typed tools?**
Options: one `thinkthen` tool taking a command; one typed tool per function, as 0.2 has; BioMCP's mix. Outside guidance agrees that fewer tools help. Anthropic advises building "a few thoughtful tools" and warns that "too many tools or overlapping tools can also distract agents." OpenAI suggests fewer than 20 functions at the start of a turn. Anthropic measured 58 tools across five servers at about 55,000 tokens, and its tool search raised accuracy from 49% to 74% on Opus 4 by loading definitions on demand. The RAG-MCP paper found tool selection accuracy of 13.62% when every schema sat in the prompt, against 43.13% with retrieval. Against one tool stands BioMCP's 18% to 29% failed-call rate. ThinkThen's guess surface differs from BioMCP's: the function names are ten fixed words, and the questions, labels and levels are free text the agent writes anyway. The likely guesses are option names, of which `filter` has 38, and framing flags. Errors that list the valid options fix most of that in one retry. Typed schemas would cost real context: even slimmed to 400 tokens each, ten tools spend 4,000 tokens before the agent asks anything. Recommendation: one tool in 0.3, as Ian ruled. The prototype includes a ten-slim-typed-tools arm. Typed tools return only if that arm beats one tool by more than five points of task success at similar context cost, and then only for the functions where it wins.

**Q2. Does `command` take one string or an argument list?**
A string matches every example in help and in agents' training, and BioMCP uses one. An argument list such as `["decide", "Is it the customer's fault?"]` avoids shell quoting, and models emit JSON arrays reliably. The quoting risk shrinks once evidence leaves the command (Q3), because the question and a few options remain. Accepting both as exclusive fields would put a `oneOf` in the schema, which some hosts reject, or two optional fields with a rule only the server knows. Recommendation: one string, split by POSIX shell quoting rules with no variable expansion, globbing, pipes or redirection. The prototype runs an argument-list arm; if quoting errors make the string lose by more than five points, the field becomes a list.

**Q3. Where does the evidence go?**
In 0.2's MCP, text evidence travels in typed fields. On the command line it arrives on standard input or as files. Putting a long email inside a quoted command string forces double escaping, once for the shell grammar and once for JSON, which is the failure BioMCP's agents met with query syntax. Recommendation: a second field, `input`, holds exactly what the command line would read on standard input. Files go by path, as on the command line. The schema has two flat properties and no `oneOf`.

**Q4. What does a reply look like?**
0.2's MCP returns the complete result object twice, as structured content and as JSON text, under an output schema of about 71 KB (`specification/mcp.md`). Both copies cost context. The command line's JSON lines are what help documents. The exit code carries the answer for `decide`: 0 yes, 1 no, 3 not sure (`specification/channels.md`). Recommendation: the reply is standard output as text, then one closing line such as `exit 3: not sure`. No output schema and no structured copy. Exit codes 0, 1, 3, 6 and 7 are answers or partial completions, so the reply is not an error. Exit codes 2, 4, 5 and 70 set the MCP error flag, and standard error's message comes with them.

**Q5. How large may a reply be?**
0.2's MCP allows replies up to 192 MiB (`specification/mcp.md`). Anthropic restricts tool replies in Claude Code to 25,000 tokens by default and advises pagination and truncation "with sensible defaults" that steer the agent. A `filter` over a large file can print far more. Recommendation: the server holds a reply budget, 20,000 tokens by default and set at start. Past it, the reply carries the first lines, a count of the lines held back, and the option that writes the whole result to a file, which the spreadsheets and output work defines (PRD 00, U8 and U9). The server never cuts a reply without saying so.

**Q6. Can a command use pipes or chains over MCP?**
Agents with a shell pipe ThinkThen through `jq` and `head`. MCP has no shell, and adding a pipe grammar would make the server a shell. Tools already chain steps (`toolbox.md`). One gap remains: idea 0566's most common case is a BioMCP reply shaped `{"results": [...]}`, which needs `jq '.results[]'` before `filter --jsonl`. 0.2 reads a JSON array of records only in `relate` (`specification/records.md`). Recommendation: no pipes. Ask the functions PRD for a records pointer, such as `--records /results`, so a function reads the records inside one JSON document. That serves the shell user too, and removes `jq` from the guide's first example.

## Teaching through help

**Q7. How does an agent learn the whole surface without reading 17,000 tokens?**
Reading every function's full help costs about 17,000 tokens. Anthropic's code-execution article and its tool search both rest on progressive disclosure: load names first and details on demand. The command line convention already has two lengths, `-h` and `--help`. Recommendation: four layers, each with a token budget that CI measures on the released build. The tool description stays under 250 tokens and names the seven jobs, the ten functions and `help`. `help` stays under 600 tokens, one line per command grouped by job. `help JOB` and `help ACTION` stay under 800 tokens, with two examples, the options one line each, and the exit codes. `help ACTION --long` prints today's full text. The command line prints the same layers: `thinkthen decide -h` is the short form and `--help` the long one. PRD 00's `help --json` prints the whole tree for programs, not for agents.

**Q8. Does the server use MCP instructions, resources or prompts?**
MCP lets a server send instructions at initialization, and some hosts place them in the agent's context. Resources and prompts reach the agent only in hosts that surface them, and BioMCP's help resource duplicates its help command. Recommendation: send instructions of about 150 tokens saying when ThinkThen fits: judging many records or files by meaning, a third kind of search, planning before a large run. Serve no resources or prompts in 0.3; help through the tool reaches every host.

**Q9. What do errors say?**
BioMCP's study found that telling an agent what is available helped, and telling it where to go next after a success sometimes misled it. Anthropic advises errors that "clearly communicate specific and actionable improvements." Recommendation: every refusal names what was wrong, lists the valid values when they are enumerable, offers the nearest option name for a misspelling, and names the command to run next, such as `help filter`. Successful replies carry no suggested next steps. The one exception is a row that already names its own follow-up, such as a `curate queue` row that includes the `curate settle` command for that row; that is data, not advice.

**Q10. How does an agent learn a toolbox?**
Ian ruled that the toolbox describes itself (`toolbox.md`). A toolbox may hold dozens of tools. Recommendation: `toolbox list` prints one line per tool: its name, its function or steps, and its one-sentence purpose. `toolbox describe TOOL` prints its inputs, what its shell steps need, its threshold and its cases result when it has one. `run TOOL -h` prints the same as `toolbox describe TOOL`, so the agent finds it either way. The toolbox PRD owns the folder; this PRD only requires that both commands fit the 800-token layer per tool.

**Q11. How does an agent learn the curate, eval and answers jobs?**
These jobs are loops, not single calls: find disagreements, settle, freeze a key, grade. A list of commands does not teach a loop. Recommendation: `help curate` and `help eval` open with the loop in three to five numbered command lines that work as written against the local database, then list the actions. Each output row carries the ids the next command needs, so the agent never builds an id itself. The curation PRD owns the commands; this PRD sets the help shape.

**Q12. What is the agent guide from idea 0566, and where does it live?**
Idea 0566 asks for a skill that teaches when to use `filter`, `rank` and `find` over JSON records, with BioMCP-shaped examples. A file in one host's skill folder would serve one host and drift from help. Recommendation: one text, printed by `thinkthen help guide`, under 1,500 tokens, also shipped as a skill file in the release for hosts that load skills. It covers when ThinkThen beats keyword search, vector search or reading the text yourself; which function fits which job; previewing with `--plan` and reusing answers; settling answers and naming the person; and two worked BioMCP pipes. The MCP instructions point to it. The prototype measures it by running one arm with and without it.

**Q13. Command line or MCP: which should an agent use?**
Mario Zechner and BioMCP's own blog argue that a command line beats MCP for an agent with a shell, at a fraction of the context. Anthropic's code-execution article reaches a similar place: agents that write code against tools spend far less context. Many hosts have no shell: chat apps, some editors and sandboxed agents. Recommendation: the docs say it plainly. With a shell, use the command line. Without one, use MCP. Both give the same help, JSON and exit codes, so an agent that learns one knows the other. The prototype includes the shell arm as the reference.

## What MCP may do

**Q14. Which commands may the MCP tool run? (Ian)**
Ian's 2026-10-11 ruling names the ten functions, `run` and help (`mcp.md`). His 2026-10-13 ruling says people and agents settle disagreements by using the tool (`data.md`), and PRD 00's surface table marks `answers`, `curate` and `eval` "MCP all" and Set up "MCP: status only". PRD 00's Q28 instead allows every `admin` action except deletion and restore, which contradicts its own table. Recommendation, following the table:

| Allowed over MCP | Refused over MCP |
| --- | --- |
| The ten functions and `run` | `admin` in every form: backends, database, prune, users |
| `help`, `toolbox list`, `describe`, `check` | `login` |
| `answers list`, `show`, `stats`, `export` | `proxy`, `mcp` |
| `curate queue`, `settle`, `key`, `outcome`, `export` | |
| `eval grade`, `compare`, `bench` | |
| `status` | |

`admin backends` sends paid probes and the other `admin` actions change or reveal the machine's setup; none is a job an agent does for its person. Every allowed write appends, because nothing in the store is overwritten (PRD 00 Q20). A refused command names the command line as the place to run it.

**Q15. Which settings may a single call set?**
0.2 already rules that MCP tool arguments "never select a key or route" (`specification/mcp.md`). A command string can carry `--url`, `--backend`, `--cache` or `--replay`, so the server must refuse them. `eval bench` needs to name several models. Recommendation: the server's start settings fix the backend, address, key, database, toolbox, roots, caps, reply budget and person. A call may set any function option, `--plan`, a lower cap, and names of setups the person's configuration already defines. A call that names an address, a key, a database or a recording folder is refused with a message naming the configuration as the place to change it.

**Q16. What does the one tool declare in its MCP annotations?**
MCP tool annotations describe a whole tool: read-only, destructive, idempotent and open-world. With one tool, the flags must hold for every command it accepts. Hosts use them to decide when to ask the person. Recommendation: not read-only, because functions send paid requests and curation writes; not destructive, which is true only because MCP refuses every deleting command; not idempotent; open-world, because requests go to a model provider. This makes Q14 load-bearing: adding `admin prune` to MCP would make the annotation false.

**Q17. May `run` execute tools with shell steps over MCP?**
Tools may run shell steps on the caller's machine (`toolbox.md`). A host with no shell would gain one through any tool in the toolbox. If an agent can also write files into the toolbox folder, it can run any command. Recommendation: MCP runs tools only by name from the toolbox fixed at start, never from a path the call names. Shell steps run, because the person who installed the toolbox wrote them. `mcp --no-shell-steps` refuses them for hosts that grant no shell. The guide tells people not to give agents write access to their toolbox folder.

**Q18. Which files may the server read?**
0.2's MCP reads any path the user can read and states that it does not confine the file system (`specification/mcp.md`). A prompt-injected agent could then send a private key to a model provider through `decide`. MCP lets a client declare roots, the folders the server may use. A stdio server also starts in a folder the host chooses, which may be the home folder or `/`, so relative paths are unreliable. Recommendation: read only inside the client's roots, or the start folder when the client declares none. `--root DIR` adds a folder. Relative paths resolve against the first root, and `status` shows the roots. This changes 0.2's behavior, which PRD 00's principle 5 allows because confinement is a security limit, not a new judging behavior.

## Who acted

**Q19. How does the server know which person an agent acts for?**
Ian ruled that an agent records the person it acts for (`data.md`). 0.3 has no sign-in, and `login NAME` makes a user current (PRD 00 Q38). Letting a call name the person would let an agent act as anyone. Recommendation: the person is fixed when the server starts: `mcp --user NAME`, else the current user from `login`, else the default user. No command over MCP changes it. Through a proxy, the local server sends the person and agent with each write, and the proxy records them as claimed until sign-in.

**Q20. How does the server name the agent?**
PRD 00 Q39 proposes `THINKTHEN_AGENT`. An MCP client reports its own name and version at initialization. Recommendation: `THINKTHEN_AGENT` in the server's configuration wins; otherwise the client's reported name and version. Every call and write records the agent, the person and the surface `mcp`, which 0.2 already supplies as a closed surface identity (`specification/mcp.md`). The record says "as claimed" until sign-in exists, because any process can claim any name.

## Cost

**Q21. How does a person stop an agent looping on paid calls? (Ian)**
0.2 has process-wide caps on requests and on estimated input tokens, set when `mcp` starts and with no default limit (`specification/settings.md`, `thinkthen mcp --help`). Ian's notes on the enterprise edition record Cursor users exhausting credit in a few prompts and recommends caps on by default. Cached answers already cost nothing. Recommendation: MCP turns session caps on by default, 1,000 live requests and 2 million estimated input tokens, raised only in the server's configuration. A call may lower a cap and never raise one. A refusal names the cap, the spend so far and the setting that raises it, and `status` shows the session's spend. `--plan` works in every sending command, `run` and `eval bench` included (PRD 00 Q30). Ian sets the numbers, because a cap can stop a legitimate long run halfway.

**Q22. How do long runs and parallel calls work?**
0.2's MCP allows one active call and answers a second with a busy error (`specification/mcp.md`). Agents issue tool calls in parallel, and each busy error is a failed call the agent must reason about; Anthropic advises hiding such infrastructure. Long runs meet host timeouts. Recommendation: up to four calls run at once under one set of caps, and a fifth waits. Long runs send MCP progress notifications and honor cancellation as 0.2 does. The guide tells agents to `--plan` a large run first and to use the command line for very large ones. MCP's newer long-running task methods wait until hosts support them widely.

## Reach

**Q23. Remote MCP over HTTP in 0.3? (Ian)**
The ideal state says the proxy serves the functions "over HTTP and MCP". BioMCP serves unauthenticated HTTP safely because its calls are free and read-only. A remote ThinkThen MCP endpoint without sign-in would spend the operator's keys for anyone who finds it. It could not read the agent's files, and it could not run tools, because the proxy never serves tools (Ian, 2026-10-12). MCP's HTTP authorization is OAuth sign-in, which is out of 0.3. Recommendation: stdio only in 0.3. A local MCP server pointed at a proxy gives a team the shared store and keys. Remote MCP arrives with sign-in, under the same action names.

**Q24. How does MCP relate to the proxy?**
Ian ruled one store per caller and that the proxy adds no behavior a local run cannot reproduce (PRD 00). Recommendation: the MCP server is a client exactly like the command line. With a ThinkThen backend, function calls go through the function door; `run` executes locally and sends its function steps; `answers`, `curate` and `eval` use the proxy's store. File paths are read on the agent's machine before anything is sent, as Ian ruled in `mcp.md`.

**Q25. Does the server ever ask the client's model anything?**
MCP lets a server ask the client's model to write text, called sampling, and to ask the person a question, called elicitation. Ian ruled that large models call the tool and the tool never calls them (`data.md`). Recommendation: never sampling. No elicitation in 0.3; cap refusals and `--plan` do the job in every host, and elicitation support varies.

**Q26. What are the names and the configuration line?**
PRD 00 Q8 keeps `thinkthen mcp`. Agent configurations already copy `{"command":"thinkthen","args":["mcp"]}`. Hosts show tools as server plus tool name. Recommendation: the server is `thinkthen mcp`, the tool is `thinkthen`, and 0.2's configuration line keeps working unchanged. Each command keeps its action name, such as `curate.settle`, in the call record (PRD 00 Q29).

## Proof

**Q27. What prototype retires the one-tool risk?**
The README names this risk. BioMCP's history shows that context cost and failed calls pull in opposite directions, so the prototype measures both.

- **Build.** A thin one-tool MCP server in the experiment repo that splits the command and calls the 0.2 command line, with hand-written short help for the layers in Q7. Answers replay from committed recordings, so the runs spend nothing on ThinkThen and every run sees the same answers. A toolbox stand-in uses 0.2's named questions through `@NAME`.
- **Arms.** A: one tool, string command plus `input`. B: one tool, argument list plus `input`. C: ten slim typed tools, each under 400 tokens with no output schema, plus a help tool. D: the command line through a shell. A second pass runs arm A with and without the guide.
- **Agents.** A frontier model, a small fast model, and an open-weight model in a second host. Fresh context for each task, at most 30 turns, three runs per task per arm per model: 24 tasks, four arms, three models, three runs, 864 runs, plus 216 for the guide pass. Hosts that defer tool schemas are run both ways where they allow it.
- **Tasks.** 24, each with a check written before any run. Eight single pieces of evidence: each of `decide`, `choose`, `score`, `tag`, `recognize`, one question holding an apostrophe and quotes, one not-sure answer the agent must report, one image. Eight record and file tasks: `filter` a JSONL file, `rank` a folder by windows, `find` with nothing fitting, `annotate` a CSV, `relate`, a BioMCP-shaped results file, a three-file folder with line positions, a reply over the budget. Four discovery tasks: pick the right function for a vague request, run a named tool, preview a 5,000-record run with `--plan` and keep under a stated budget, report a cap refusal. Four traps: a refused command, a path outside the roots, a call naming an address, a misspelled option.
- **Measures.** Task success by the written check; failed calls per task; tokens of catalog, instructions and help read; total tokens, turns and dollars per task.
- **Pass.** Arm A's success sits within five points of the best MCP arm on every model; under 10% of its calls fail; catalog, instructions and help cost under 2,000 tokens per task at the median; and it sits within ten points of the shell arm. If B beats A on quoting, the field becomes a list. If C beats A by more than five points at similar context, typed tools for those functions go to Ian.
- **Cost.** Only the agents' own tokens; replay keeps ThinkThen's cost at zero. The PM prices the runs before they start.

**Q28. What happens to 0.2's MCP catalog size?**
0.2 ships as built (`mcp.md`). Its 271,000-token catalog makes it unusable in hosts that load schemas up front, and its release gate expects a fresh agent to run it. Recommendation: report the measurement to the thinkthen PM, so the 0.2 docs set expectations or the output schemas shrink before the gate. 0.3 replaces the catalog, and CI enforces the 0.3 budgets from the first build.

## Sources

- Anthropic, "Writing effective tools for agents — with agents", 2025. https://www.anthropic.com/engineering/writing-tools-for-agents
- Anthropic, "Introducing advanced tool use on the Claude Developer Platform", 2025: 58 tools at about 55,000 tokens; tool search accuracy 49% to 74% on Opus 4 and 79.5% to 88.1% on Opus 4.5; tool use examples 72% to 90%. https://www.anthropic.com/engineering/advanced-tool-use
- Anthropic, "Code execution with MCP", 2025: 150,000 tokens to 2,000 by loading definitions on demand. https://www.anthropic.com/engineering/code-execution-with-mcp
- OpenAI, function calling guide: fewer than 20 functions at the start of a turn, and tool search for larger sets. https://developers.openai.com/docs/guides/function-calling
- Gan and Sun, "RAG-MCP: Mitigating Prompt Bloat in LLM Tool Selection via Retrieval-Augmented Generation", arXiv 2505.03275, 2025: 13.62% to 43.13% tool selection accuracy. https://arxiv.org/abs/2505.03275
- Mario Zechner, "What if you don't need MCP?", 2025. https://mariozechner.at/posts/2025-11-02-what-if-you-dont-need-mcp/
- Model Context Protocol specification 2025-11-25: tools and annotations, roots, progress, cancellation, sampling, elicitation, authorization. https://modelcontextprotocol.io/specification/2025-11-25
- BioMCP: `docs/blog/we-deleted-35-tools.md`, `docs/reference/mcp-server.md`, `sdlc/tickets/archive/0435-typed-mcp-tool-surface-schema-typed-tools-replace-single-command-string.md`, `sdlc/tickets/1251-publish-flat-mcp-tool-schemas-and-finish-the-argument-checks.md`, `sdlc/planning/notes/2026-09-09-biomcp-interface-and-agent-behavior-study.md`.
