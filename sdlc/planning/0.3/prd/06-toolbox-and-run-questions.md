---
date: 2026-10-13
status: draft PRD
source: Ian's rulings in `toolbox.md` (2026-10-11 and 2026-10-12), `proxy.md`, `evals.md` and `mcp.md`; PRD 00 and its questions; PRD 02; the ChatGPT toolbox spec and `run` command-line notes of 2026-10-11; Ian's ThinkThen notes (data model, fixed and tunable) and the ThinkThen ideal state; a partner team's feedback mail of 2026-10-11 on the toolbox; ThinkThen 0.2's `specification/` (question-file, channels, records, result, settings, roadmap, sdk-boundary, transform) and `thinkthen --help`
---

# PRD 06 working file: toolboxes and `run`, the hard questions

This file backs [06-toolbox-and-run.md](06-toolbox-and-run.md). Each question gives the options, the reasoning and a recommendation. **Ian** marks the four that need his product direction. Claims about 0.2 cite the page in [`specification/`](../../../../specification/). PRD 01 owns question identity and wording versions, and PRD 02 owns evals; this file names where it leans on them.

## What a tool is

**Q1. Is a tool file a new format, or the 0.2 question file grown up?**
0.2 already has the one-step tool: a question file holds the verb, the wording, the options and the threshold, plus optional `name`, `wording_version`, `item_schema` and `context_schema` ([question-file.md](../../../../specification/question-file.md), ticket 0456). The ChatGPT spec proposes a fresh `thinkthen.tool/1` manifest that points at separate definition, schema and transform files. A new format doubles what an author learns and leaves 0.2 users with files to convert. Recommendation: a one-step tool file is a question file plus `description`, and optionally `inputs`, `input` and `needs`. A multi-step tool replaces the verb key with `steps`, and each step is a question file's body or a shell step. Moving `questions/refund.json` into a toolbox folder and adding a description makes it a tool. The 1 MiB cap, the duplicate-key refusal and the published JSON Schema carry over.

**Q2. JSON, YAML or TOML for tool files? (Ian)**
JSON is 0.2's grammar, has a published schema, parses in every one of the libraries, and agents write it without mistakes. It has no comments, and long wording sits on one line. YAML reads best for people and holds long text, but its implicit typing turns `no` into false and `1.10` into 1.1, and the Rust YAML crate most projects used is no longer maintained. TOML has comments and long strings and is native to Rust, but nested steps read poorly and a second grammar beside question files splits one concept in two. Long prose has a better home anyway: the description and help are short, and a long policy text is a file the tool names. Recommendation: JSON. Ian decides, because authors live in this format for years and it is costly to change once toolboxes are shared.

**Q3. How short is "a short pipeline"? What may a step do?**
Make, just and GitHub Actions all grew conditions, loops and matrices once steps existed, and each became a small programming language. ThinkThen's ideal state says it "will not sequence work that the host program already sequences". The ChatGPT CLI note warns against turning context assembly into a language. Recommendation: steps are a fixed list that runs top to bottom, every time. No condition, no loop, no branch on an answer, no tool calling another tool, no parallel steps. A job that needs more is a script that calls `run`. `toolbox check` refuses more than eight steps, which covers the partner team's four-step lookup with room to spare.

**Q4. Which functions may be steps, and where?**
0.2's record functions read records and print records: `filter` prints kept records as they arrived, `rank` prints records best first, `decide`, `choose`, `tag` and `score` print `{input, value}` per record, `annotate` prints records with answers ([records.md](../../../../specification/records.md)). `find` and `relate` read a whole set and print one result. Single-document `decide` and `choose` carry their answer in the exit code. If a mid-pipeline step could exit 1 for "no", ThinkThen would be choosing whether later steps run, which is sequencing on an answer. Recommendation: every step but the last reads and writes JSON Lines records. Only the last step may read one document or answer through its exit code. Any of the ten functions may be the last step.

**Q5. How does "keep five" happen without `head`?**
The partner pipeline is candidates, then `filter`, then `rank`, then keep five, then `find --none`. `head` is a shell step, so it needs trust, breaks in SQL and is missing on Windows. 0.2's `rank` already takes `--top N` ([rank.md](../../../../specification/rank.md)). Recommendation: a rank step takes `top`. The docs teach function settings before shell steps, so most tools stay function-only and run everywhere.

**Q6. Is a jq recipe its own concept?**
`toolbox.md` says jq is invoked, never bundled, optional and low priority. A tool-level `jq` key would add a fourth thing beside function steps and shell steps, with its own rules. A shell step that runs `jq` already does the job, and the ChatGPT spec's long jq contract (cardinality, degraded status, canonical transform input) becomes unnecessary. Recommendation: a jq recipe is a shell step whose program is `jq`, usually reading a `.jq` file in the toolbox, and `jq` is a declared need. One rule covers it. 0.2's `transform` catalog retires per PRD 00 Q9.

## The folder and how an agent learns it

**Q7. What is the folder layout?**
Options: one folder per tool with a manifest each (ChatGPT spec, GitHub Actions), one file per tool in a flat folder (0.2's `questions/`, just's one file), or one big manifest. A flat folder is the easiest for an agent to list and for a person to read. Recommendation:

```text
support/
  toolbox.json          {"name": "support", "description": "Questions about customer support tickets."}
  triage.json           a tool; the file name is the tool name
  triage.cases.jsonl    optional cases for the tool
  lookup.json
  lookup/candidates.sql files a tool's steps read, anywhere inside the folder
```

The toolbox name in `toolbox.json` must match the folder name, and each tool's `name` must match its file name, as 0.2 already requires for named questions. Every path a tool names stays inside the folder: no `..`, no absolute path, and no symlink that escapes, the rule 0.2 applies to `questions/`.

**Q8. Where do toolboxes live, and which one wins a name?**
0.2 reads named questions from `questions/` under the configuration home. The ChatGPT spec suggests the XDG data folder for installed releases. 0.2's settings follow one precedence: typed, then environment, then configuration file, then default ([settings.md](../../../../specification/settings.md)). Recommendation: the default place is `toolboxes/` under the configuration home, beside 0.2's `questions/`. The configuration file's `toolboxes` list adds folders, `THINKTHEN_TOOLBOXES` adds a path list for one shell or CI job, and `--toolbox DIR` names one for one call. Higher levels win, as every 0.2 setting does. Two toolboxes with one name at the same level refuse, naming both paths. `toolbox list` shows a shadowed toolbox with a warning, so no project silently borrows another's tool.

**Q9. What grammar do tool names use?**
PRD 00 Q16 found that `support/triage` fits neither 0.2's name grammar `[a-z][a-z0-9_-]{0,63}` nor the reserved wire field `question_id`, `[A-Za-z0-9._-]` of 1 to 128 characters ([result.md](../../../../specification/result.md)). Deep nesting such as `team/support/triage/v2` invites a second versioning scheme inside names. Recommendation: exactly two segments, `toolbox/tool`, each in 0.2's name grammar. A step adds a third: `concepts/lookup/match` is the question id of the `match` step. Widen the reserved wire field to admit `/` now, while no proxy reads it, as PRD 00 recommends.

**Q10. How does an agent learn a toolbox from help alone?**
Claude Code skills load only a name and a one-line description until a skill is needed, and then the body. MCP tools carry a name, a description and an input schema. just prints every recipe with its doc comment under `just --list`. Simon Willison's `llm` templates list by name and show their parameters. Every one of them uses two levels: a short list, then the full contract on demand. Recommendation: three levels, all generated from the tool files so help never drifts from behavior. `thinkthen help` names `toolbox list` and the number of tools found. `toolbox list` prints one JSON line per tool: name, description, input kind, named inputs, the last step's function and labels, needs, and the newest grade. `toolbox describe TOOL`, and `run TOOL --help` as the same text, prints the full contract: each input with its description, each step, the output shape, needs, the digest, the grade and a worked example taken from the first case. The MCP tool returns the same text (PRD 00 Q27).

**Q11. Who writes the descriptions, and how do we know they work?**
A generated contract tells an agent what a tool takes. Only the author can say when to use it. MCP and skills both live or die on the description line. Recommendation: `description` is required, one sentence, under 200 characters, saying what the tool answers and when to use it. Each named input needs its own description. `toolbox check` refuses a tool without them. The README's risk "a fresh agent given only a toolbox folder should use every tool correctly" becomes the self-description experiment in the PRD, and its failures rewrite the help rules before tickets.

**Q12. Should a tool carry a human version number?**
The ChatGPT spec has releases, release manifests and qualification states. Git already versions a folder, and the digest says exactly what ran. A third number invites the two to disagree. Recommendation: no version field in 0.3. Identity is the name plus the digest (Q13). PRD 01's wording version stays per question. A team that wants a release tags the git commit.

## Identity

**Q13. What does the tool digest cover?**
`toolbox.md` names a tool by "a hash of its definition". If the hash covers everything, fixing a typo in the help splits the tool's history in the store. If it misses a referenced SQL file, two different behaviors share one digest. The ChatGPT spec's VER02 hashes everything that affects behavior and records the rest as metadata. Recommendation: the SHA-256 of a canonical form of the resolved tool: the steps, each step's question in 0.2's canonical form, thresholds, `top`, `none`, model, each shell step's program and arguments, and the bytes of every file the tool names. It leaves out `description`, help, input descriptions, cases and needs, because none of them changes an output. Every answer carries the name, the digest and the step.

**Q14. Does a Windows checkout get the same digest?**
Git on Windows often checks text files out with CRLF line endings (`core.autocrlf`). Then the same commit hashes differently on Windows and Linux, and a context file sends different bytes, so one tool asks two questions. Recommendation: ThinkThen reads every text file a tool names as UTF-8 and turns CRLF into LF, both for the digest and for what it sends. A file that is not UTF-8 refuses at `toolbox check`. The tool's question then matches across operating systems, and the experiment in the PRD checks equal digests on all three.

**Q15. How do tool identity, question identity and wording versions fit together?**
`data-model.md` keys right answers on question and evidence. PRD 00 Q15 makes a tool's question id the tool id, or the tool id plus a step name. 0.2's question digest still includes the threshold. If the tool digest were the question id, every threshold change would orphan every right answer. Recommendation, consistent with PRD 01: three layers. The question id (`concepts/lookup/match`) keys right answers and survives rewording. The wording digest, and PRD 01's optional author number, changes when the wording changes. The tool digest changes when anything that affects output changes, thresholds included. Changing a step's labels makes it a new question, so `toolbox check` warns when a question id that has stored answers changes its labels, as PRD 00 Q17 says. Renaming a step or a tool is a new id, by choice.

## Inputs and wording

**Q16. How do named inputs and record fields enter the wording?**
The partner team asked for wording such as: does the concept NAME, with synonyms SYNONYMS, mean what QUOTE says? QUOTE is given once; NAME and SYNONYMS come from each record. Ian said yes. Prior art: `llm` templates use `$input` and named `$params`; Mustache and Jinja use double braces; GitHub Actions uses `${{ }}`. 0.2 already selects record parts with RFC 6901 pointers (`--field /body`). Recommendation: `{quote}` names an input and `{/name}` names a record field by pointer, so the two cannot be confused and pointers keep their 0.2 meaning. `{{` writes a literal brace. A placeholder works only inside string values. `toolbox check` refuses an unknown input name, and a record without the field refuses before it sends, as `--field` does today.

**Q17. Do per-record wordings break batching or caching?**
At first sight a wording that changes per record makes each record its own question. 0.2 already sends each record inside its own question: "The text is", the record, then the question, so many records share one request and each keeps its own cache key ([records.md](../../../../specification/records.md), ADR 0111). Equal records with different options already get separate questions. So a filled-in wording packs exactly as today. The cost is tokens: an input placed in the wording repeats in every record's question. Recommendation: short inputs go in the wording. A long input, such as a policy text, is named in the step's `context` and goes once per request as shared evidence, as 0.2's `--context FILE` does. The question id and wording digest come from the template, never from the filled-in text, so one tool step stays one question.

**Q18. How does a caller supply inputs on the command line?**
The ChatGPT note proposes `--file NAME=PATH`, `--arg NAME=VALUE`, `--stdin NAME` and `--input-json`. 0.2 already uses `--input FILE` for the evidence and requires one value per option ([channels.md](../../../../specification/channels.md)). jq, which every agent knows, uses `--arg NAME VALUE` and `--rawfile NAME PATH`. Recommendation: evidence arrives exactly as for functions, on standard input, as files after the name, or through `--input`. Named inputs use `--arg NAME=VALUE` and `--arg-file NAME=PATH`, one value each. A tool declares its evidence as `text`, `records` or `none`, so a tool that reads nothing never waits on a terminal. Libraries take the same inputs as a map.

## Running

**Q19. How do records stream through a multi-step tool?**
Options: buffer each step's whole output, then start the next; or run every step at once, joined by pipes, as a shell does. Buffering is simple but makes a filter over a large file wait for the end. Recommendation: steps run together and pass JSON Lines records, function steps inside the ThinkThen process and shell steps as child processes. Through a proxy each function step sends chunks through the function door, as `proxy.md` rules. A step that needs the whole set (`rank`, `find`, `relate`) begins its answer only after every earlier step has finished cleanly. This matches 0.2, where `rank` already prints only after the input ends.

**Q20. What happens when a step fails mid-run?**
A shell pipeline without `pipefail` hides an upstream failure, and the next step may compute a confident answer from half the records. In 0.2, "printed output after a failure is a prefix of the input", and `rank` prints no partial order ([records.md](../../../../specification/records.md)). Recommendation: any failed step stops the run. A step that needs the whole set never answers on partial input, so a failed `filter` before `find` prints no pick. Rows a streaming last step already printed stay printed and are each correct. Standard error names the failing step, its kind and its cause, and `--facts` reports the step. A function step keeps its 0.2 exit code. A shell step that exits nonzero gives exit 5, a local failure, with the step's own code in the message. A step may list extra exit codes that count as success, because `grep` exits 1 when nothing matches. SIGINT and SIGTERM reach child processes and end the run with 130 or 143, as 0.2 does.

**Q21. What exit code and output does `run` give?**
PRD 00 Q34 rules that a one-step tool prints and exits as its function and a multi-step tool prints and exits as its last step. The ChatGPT note wanted conventional exit codes for `run`, but shell gates depend on `decide`'s 0, 1 and 3 ([channels.md](../../../../specification/channels.md)). Recommendation: keep PRD 00's rule. Before the first step, a missing need exits 5 and a refused shell step exits 2, and neither runs anything.

**Q22. What does `--plan` show when a shell step produces the records?**
A function step after `psql` cannot know how many records it will get without running `psql`, and running a shell step under `--plan` would act. 0.2 promises that `--plan` sends nothing and calls no backend. Recommendation: `--plan` runs no shell step and sends nothing. It prints the steps, the resolved inputs, the needs and their check, and for each function step the requests it would make when its input is known, or "depends on step NAME" with the cap that bounds it. 0.2's spending caps already count across the whole process, so `--max-requests-total` covers every step of a run.

**Q23. Can a caller override a step's settings?**
0.2's rule is the command line, then the file, then the default. On a one-step tool, `--threshold` and `--model` mean what they mean on the function. On a four-step tool, `--threshold` has no obvious target. The ChatGPT spec's locks add a policy language. Recommendation: one-step tools take the function's typed settings. Multi-step tools take `--model`, applied to every step, and refuse `--threshold` with a message naming the step to edit. Per-step overrides wait for demand. The proxy's overrides are separate and always reported (Q25).

## Wire, results and the proxy

**Q24. What does a function step send through the proxy?**
The partner team asked that each function call carry the tool's name, hash and step, so the proxy routes, records outcomes and tunes per tool. 0.2 reserves a `proxy` block and refuses to activate it ([result.md](../../../../specification/result.md), [sdk-boundary.md](../../../../specification/sdk-boundary.md)). Recommendation: only the ThinkThen backend type sends `tool`, `digest` and `step`, with the question id `toolbox/tool/step`. A System One or OpenAI Decisions backend gets ordinary vendor bytes, as 0.2 requires. The proxy never receives a shell step, an argument list or a file the tool names, only the function call each step makes.

**Q25. How does a result report an override?**
When the proxy overrides a step's threshold, a local run and a proxy run of the same tool differ. 0.2 already reserves an `override` variant in result metadata. Recommendation: every detailed row carries `meta.run` with `tool`, `digest` and `step`, and the step's `threshold` with `threshold_from` set to `tool`, `call` or `proxy`. A proxy override also carries the code's threshold beside it. Earlier steps' answers are stored under the run's call id, so `answers list --call ID` shows them, and the final row stays small.

**Q26. Does anything in 0.2's results already use the words "tool" and "run"?**
Yes, twice. `meta.tool` holds the binary's name and version, such as `thinkthen 0.2.0` ([result.md](../../../../specification/result.md)). `--facts` prints a `thinkthen.run/1` line ([channels.md](../../../../specification/channels.md)). Beside a tool named in every row and a `run` command, both read wrong, and PRD 00 Q2 says "run" means only "run a tool". The 2026-10-12 decision keeps no backward compatibility before 1.0. Recommendation: rename `meta.tool` to `meta.version` and the facts schema to `thinkthen.facts/1` in 0.3.

**Q27. May the proxy hold anything a tool step calls?**
Ian's 2026-10-12 ruling lets the proxy host "functions only ... perhaps with saved wording and thresholds". A step that named a proxy-held question instead of carrying its own wording would make the tool's behavior depend on the server, and its digest would no longer say what ran. Recommendation: in 0.3 every step carries its own wording. The proxy may override a threshold, reported as in Q25. Steps that call a saved function on the proxy wait for the proxy PRD to define saved functions.

## Security

**Q28. Who may run a shell step? (Ian)**
A toolbox cloned from someone else's repository runs their commands, much as `make` or an npm install script does. Ian ruled on 2026-10-11 that trust is set per toolbox and a run never stops to ask, because it may run in CI. On 2026-10-12 he ruled that the caller has total control over what runs on their machine. Prior art splits two ways. direnv and mise refuse a folder's file until the user allows it and ask again when it changes. VS Code opens untrusted folders in restricted mode. Make and just run whatever is there. The risk is not the person at a shell, who can already run anything. It is a caller that has no shell of its own: an agent that reaches ThinkThen only through MCP, a database user, a remote client. Recommendation: one rule, **a tool never gives its caller more power to run programs than the caller already has.**

- The command line and the libraries run shell steps from any toolbox the caller names, because a shell user or a program can already start processes.
- MCP runs tools only by name from toolboxes configured when the server starts, and refuses `--toolbox` in a command string, as PRD 07 Q17 also recommends. An agent that can write files but has no shell could still plant a tool in a configured folder, so the guide says never to give an agent write access to its toolbox folders, and PRD 07's `mcp --no-shell-steps` serves hosts that want no shell steps at all.
- SQL never runs shell steps (PRD 00 Q12).
- The proxy never sees a tool.
- No prompt and no sandbox in 0.3. A configured toolbox may pin a digest, and a changed digest then refuses shell steps until the pin moves.
- On Unix, a toolbox folder or tool file that another user can write refuses its shell steps, as 0.2 refuses `backends` from a configuration file another user can write ([settings.md](../../../../specification/settings.md)).

Ian decides, because a shared toolbox with shell steps is the first time ThinkThen runs someone else's code.

**Q29. How do inputs reach a shell step without injection?**
GitHub's security guide warns that `${{ }}` expressions pasted into a `run:` script let an attacker's issue title run commands, and tells authors to pass values as environment variables. Here, a record's text or the caller's QUOTE could carry a semicolon or a quote. Recommendation: a shell step is a program and a list of arguments, never a command string. ThinkThen starts the program directly, with no shell, and fills `{quote}` placeholders inside single arguments, so no value can become a second command. A step that runs `sh`, `bash`, `cmd` or `powershell` with a placeholder argument gets a `toolbox check` refusal. Such an author maps the input to a variable with the step's `env`, such as `"env": {"QUOTE": "{quote}"}`, and the script reads `"$QUOTE"`. On Windows, Rust's `Command` once let a crafted argument escape into a `.bat` file (CVE-2024-24576, fixed in Rust 1.77.2 by refusing unsafe arguments); ThinkThen builds with the fixed toolchain and refuses `.bat` and `.cmd` programs as steps.

**Q30. What environment does a shell step see?**
A step that inherits everything can read `THINKTHEN_API_KEY` and every cloud credential in the caller's shell. 0.2 promises that a key goes only to the address the user named. Recommendation: a shell step gets the working essentials (`PATH`, `HOME`, locale, temp folder, and on Windows `SystemRoot` and `PATHEXT`) plus the variables its `needs` lists. It never gets a ThinkThen key. Its working folder is the toolbox folder, so `-f candidates.sql` means the same file from anywhere, and caller files arrive through `--arg-file`. This is a cheap, portable boundary, not a sandbox.

**Q31. What do declared needs check, and when?**
The partner team asked that a run on another machine fail before the first step, not halfway. A database connection cannot be proven without connecting. Recommendation: `needs` lists programs and environment variables. Before the first step, `run` checks each program on `PATH` and each variable as set and nonempty, and refuses with one line naming every missing need. `toolbox check` runs the same check and reports it. Connection checks wait: the first step fails fast anyway, and a check command would be one more shell step to trust.

## Surfaces

**Q32. Do shell steps work across the libraries and on Windows?**
The Rust core spawns processes for every library, so behavior matches. Some hosts cannot spawn at all: iOS apps, sandboxed mobile Flutter apps, browsers running WebAssembly, and SQL. Windows has no `head`, `grep` or `sh` by default, but has `psql.exe` and `jq.exe`. Recommendation: argument-list steps run the same on Linux, macOS and Windows, resolving `.exe` through `PATHEXT`. A host that cannot start processes refuses shell steps with the same message SQL gives, and function-only tools run there. `toolbox check` warns about programs that are missing on Windows by default, and the docs steer reshaping to function settings and `jq`.

**Q33. What does `run` return in a typed language?**
0.2's libraries return typed results per function. A tool's return type depends on its last step, which a compiler cannot see in a string name. Options: a plain JSON value; a sum type with one variant per function; generated code per toolbox, as protobuf does. Generated code is heavy and ties builds to toolbox files. Recommendation: `run` returns a `RunResult` that names the last step's function and holds its typed result, with one accessor per function that refuses a mismatch, plus the JSON. Rust and C# use an enum, Python and TypeScript a tagged union, C a tagged struct. `run` is tier one in every language, as PRD 00 says.

**Q34. How does SQL call a tool?**
SQL functions need a fixed return shape, and 0.2's differ by function: `thinkthen_decide` is a scalar, `thinkthen_rank` a table, `thinkthen_find` a struct ([databases/duckdb/README.md](../../../../databases/duckdb/README.md)). A path inside a query would let any database user read files on the server. Recommendation: `thinkthen_run(tool, evidence[, settings])` returns the last step's output as JSON, taking one value or a JSON array of records. It refuses a tool with a shell step. It reads toolboxes only from a folder the database administrator sets in server configuration, never from a path in the query.

## Cases, grades and sharing

**Q35. How does a tool carry its cases and show its score?**
`evals.md` proposes a cases file beside each tool, and PRD 02 runs it with `eval bench TOOL` when no answer key is named. A grade recorded into the shared folder would show the author's number for the author's model to someone running a different one. Recommendation: `TOOL.cases.jsonl` holds one case per line: the evidence, the named inputs and the expected final output. Cases stay out of the digest. Grades live in the store, keyed on the tool digest and the model. `toolbox list` and `describe` show the newest grade for this digest, mark one from an older digest as stale, and say "not graded here" with the command to run when there is none. The first case doubles as the worked example in help.

**Q36. What does the GEPA recipe need from the toolbox?**
GEPA (Agrawal and others, 2025) improves prompts by reflecting on failures in natural language, and it needs a score per candidate. DSPy modules let an optimizer rewrite a signature's instructions while the fields stay fixed. `fixed-and-tunable.md` lets optimization change wording, descriptions and context, never labels. Recommendation: the recipe is a documented script, not a command. It copies a tool file, lets the user's own large model rewrite one step's wording, runs `eval bench` on the tune part, keeps the best, and compares on the held part with `eval compare` (PRD 02). ThinkThen never calls the large model (PRD 00 principle 8). The tool file's plain JSON is what makes the recipe short.

**Q37. How are toolboxes shared without a server?**
The proxy never serves tools, so sharing is a folder. An install command that clones from a URL would add network fetches, which the ChatGPT spec rightly forbids at run time. Recommendation: share with git. A user clones a repository and adds its path to the configuration file or `THINKTHEN_TOOLBOXES`. No install, update or registry command in 0.3. A repository may hold several toolboxes as sibling folders.

**Q38. Should 0.3 ship example toolboxes? (Ian)**
Traction is the measure, and a builder learns fastest from a toolbox that runs. The PRD 02 experiment already builds a BANKING77 toolbox with cases. A public lookup example with a SQLite candidate search would show a multi-step tool without a private database. Recommendation: ship two example toolboxes in the repository, a one-step support router on BANKING77 with cases and a three-step lookup over a small public vocabulary, and use them as the launch demo and the self-description experiment's material. Ian decides, because it shapes what 0.3 shows the world.

## The promise

**Q39. Does "ThinkThen never runs a command" survive? (Ian)**
0.2's channels page says the tool "never runs a command" ([channels.md](../../../../specification/channels.md)). The ideal state lists "Thinkthen will never run a command or take an action on a judgment" as a non-goal and says it will not sequence what the host sequences. Ian's 2026-10-11 ruling adds shell steps, and PRD 00 principle 11 keeps only "judging never becomes acting". The old sentence is now false, and the ideal state is Ian's to edit. Recommendation: replace it with two sentences that stay true. ThinkThen runs only the steps a tool's author wrote, in the order written, every time. An answer never chooses whether or which command runs. Q3 and Q4 enforce the second sentence.

## Sources

- ThinkThen 0.2 specification: [question-file.md](../../../../specification/question-file.md), [channels.md](../../../../specification/channels.md), [records.md](../../../../specification/records.md), [result.md](../../../../specification/result.md), [settings.md](../../../../specification/settings.md), [rank.md](../../../../specification/rank.md), [roadmap.md](../../../../specification/roadmap.md), [sdk-boundary.md](../../../../specification/sdk-boundary.md), [transform.md](../../../../specification/transform.md); `databases/duckdb/README.md`; `thinkthen --help`.
- GNU Make manual, recipe execution: https://www.gnu.org/software/make/manual/html_node/Execution.html
- just manual (recipes, parameters, `--list` doc comments, `windows-shell`): https://just.systems/man/en/
- GitHub Actions metadata syntax, composite actions (`shell` required on run steps, described inputs): https://docs.github.com/en/actions/reference/workflows-and-actions/metadata-syntax
- GitHub Actions security hardening, script injection and passing values through the environment: https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions
- Model Context Protocol, tools (descriptions, input schemas, annotations treated as untrusted): https://modelcontextprotocol.io/specification/2025-06-18/server/tools
- Invariant Labs, tool poisoning attacks through MCP tool descriptions (2025): https://invariantlabs.ai/blog/mcp-security-notification-tool-poisoning-attacks
- Claude Code skills (name and description first, body on demand): https://docs.claude.com/en/docs/claude-code/skills
- Simon Willison, `llm` templates (`$input`, named parameters, `-p`): https://llm.datasette.io/en/stable/templates.html
- DSPy signatures and modules: https://dspy.ai/learn/programming/signatures/ and https://dspy.ai/learn/programming/modules/
- Agrawal and others, "GEPA: Reflective Prompt Evolution Can Outperform Reinforcement Learning", arXiv:2507.19457 (2025)
- direnv (`direnv allow`, re-allow on change): https://direnv.net/
- mise `trust`: https://mise.jdx.dev/cli/trust.html
- VS Code Workspace Trust: https://code.visualstudio.com/docs/editor/workspace-trust
- npm scripts and `--ignore-scripts`: https://docs.npmjs.com/cli/v10/using-npm/scripts
- Rust security advisory CVE-2024-24576, Windows batch file arguments: https://blog.rust-lang.org/2024/04/09/cve-2024-24576.html
- Git attributes and line-ending conversion: https://git-scm.com/docs/gitattributes
- jq manual (`--arg`, `--rawfile`): https://jqlang.org/manual/
- Bash `set -o pipefail`: https://www.gnu.org/software/bash/manual/bash.html#The-Set-Builtin
- Command Line Interface Guidelines: https://clig.dev/
- XDG Base Directory Specification: https://specifications.freedesktop.org/basedir/latest/
