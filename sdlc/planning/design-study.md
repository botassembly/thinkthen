# thinkthen design study

**Read ADR 0007 first.** Ian replaced the `thinkthen decide VERB` grammar, the symmetric pass mark, and the saved question format described below with the flat-verb surface on 2026-09-19. The measurements in section 5 and the reasoning about fit still stand.

**A study. Nothing here is ruled.** Written 2026-09-18 from seven design captures Ian made in chats with an AI, from a survey of the Rust repositories in his workspace, and from three measurements of a decider model taken the same day. Section 9 lists what Ian has to rule on. Decisions an agent already made are ADRs under `adr/`.

## 1. What thinkthen is

`thinkthen` puts a decider model in the shell. A decider model never writes text. It reads a state, answers a typed question, and returns probabilities. The question types are yes/no, pick one from a list, and rate on a scale. TypeSafe's Jev is the first decider model. Others will follow, hosted and local.

The division of labor is fixed. The shell sequences programs and acts on files. `jq` reshapes data by rule. `thinkthen` judges meaning and does nothing else. It never runs a command, never edits a file it was not asked to write, and never interprets free text as an instruction.

A command reads as a sentence: think, then decide. Families of commands sit under the name: `thinkthen decide` first, `thinkthen resolve` and others later if they earn a place.

## 2. How the design grew

The seven captures were written in this order. None of it was built. None of it was measured.

| Capture | What it added |
| --- | --- |
| A tool named `decide` | Five verbs: `if`, `which`, `how`, `where`, `rank`. Code parses the command. The model reads only the condition |
| A fuller `decide` design | `match` and `segment`. Saved decision files with `run`, `validate`, `eval`, and replay. A three-layer result. A pass mark with an unsure middle. An exit-code table. An audit channel. Backends described by capability |
| A Bash primer | Fifty shell idioms for scripts that call the tool. No commands |
| The tool renamed `sem` | Five families: `decide`, `fold`, `reduce`, `patch`, `resolve`. Utility families. About forty-five subcommands |
| A derived-field layer | A store of named, versioned fields with refresh, history, and policy simulation |
| Add columns | CSV and TSV input with JSONL enrichment through `annotate` |
| JSON from a decider | The technique under all of it. Code finds candidates, the decider picks, code copies the pick |

The surface grew ninefold across three chats with no code behind it. This repository keeps a size ceiling that makes every line cost. Version one has to be small, and each later family has to arrive with a real use.

## 3. What the captures agree on

These hold across the captures and this study keeps them.

- Code parses the command line. The model reads only the condition, the options, and the evidence. No mode treats free text as a command.
- Arguments carry options. Standard input carries data. Standard output carries results. Standard error carries diagnostics. An optional audit channel carries structured events.
- A result has three layers. The answer is what the backend said. The assessment is the local acceptance decision. The metadata names the backend and model. A no, an unsure, and an error never collapse into one value.
- A pass mark is symmetric by default. A probability at or above the mark is a yes. A probability at or below one minus the mark is a no. Anything between is unsure. An asymmetric pair of marks is the alternative.
- A result with no pass mark is unassessed. It is never accepted by default.
- `where` and `rank` emit the original records. A pipeline keeps working on its own data.
- One output row per input row, in input order. No row is dropped for being unsure. No failure turns into `false`, `other`, or zero.
- `--on` takes a JSON Pointer. Only the pointed value and explicit context reach the backend. The pointer is also the disclosure boundary.
- `--plan` shows the intended computation and never calls a model. `check` always means a local check.
- Choosing never executes. A choice can return no match.
- A run can be recorded, and a recording can be replayed with no network.

## 4. Where the captures disagree

| Question | The captures say | This study proposes |
| --- | --- | --- |
| The name | `decide`, then `sem` | `thinkthen`, ruled by Ian on 2026-09-18. It is free on crates.io and in the organization |
| The result shape | A flat object in one capture. A nested three-layer object in two | The nested object, with a schema version |
| Output selection | `--annotate` and `--details` in one capture. `--emit` with named values in two | `--emit` |
| Input kinds | `text`, `json`, `jsonl`, `lines`. CSV and TSV are input framings only | One enum that includes CSV and TSV when their reader lands |
| Durable evidence | A recording directory in one capture. A derived-field store in another | One recording directory first. The store waits for a use |
| Saved question files | JSON in every capture | Markdown with frontmatter. See section 6 |

## 5. What measurement says

Three experiments ran a decider model over about two thousand sealed records of real agent work. The experiments live outside this repository.

- A narrow yes/no question about a fact visible in the evidence worked. One question caught every planted mismatch and wrongly rejected about 2% to 3% of good work.
- Picking from a fixed list was stable. Reversing the option order changed none of fifty picks.
- Judging quality or completeness did not work. Outcome criteria rejected 18% to 46% of accepted work.
- High confidence was wrong in one test. The model approved every case at 0.98 while human reviewers had refused 23%. The evidence it needed was absent from what it was shown.
- Answers inside the unsure band flipped between identical runs 5% to 14% of the time. Answers outside it flipped 0.5% to 2%.
- The vendor's documents say option order and added options shift the odds, and rubric scoring is the weakest primitive.

Three consequences follow.

1. `if`, `which`, and `where` stand on measured ground. `rank` by yes-probability follows the vendor's own reranking recipe. `how` rests on the weakest primitive and ships with that warning in its help text.
2. A pass mark is trustworthy only after it is measured on labeled cases for one model version. `eval` and replay matter more than any new verb. They are also what lets a user try a second vendor: replay the recorded evidence through the new backend and compare.
3. An unsure answer has to stay visible and cheap to route. The exit code, the status column, and the `--unknown` policy all exist for that.

## 6. Proposed version one

One family, six verbs, and the machinery that makes a pass mark honest.

| Slice | What it delivers |
| --- | --- |
| 1 | `thinkthen decide if CONDITION` over text on standard input. One backend. The three-layer JSON result. `--min-prob`, `--status`, the exit-code table, `--plan` |
| 2 | `thinkthen decide which OPTION...` and `--from FILE`. An abstain option and a minimum gap |
| 3 | `--record DIR` and `--replay DIR`. One content-addressed directory serves as recording, replay source, and cache. The test suite replays fixtures and needs no network |
| 4 | JSONL and lines framing, `--on`, `--id`, and `where`. Bounded parallel requests with input order kept. A request cap, a rate limit, and a token ledger |
| 5 | `thinkthen decide run FILE`. A saved question file holds several questions over one state and sends them in one request |
| 6 | `thinkthen eval FILE --cases FILE`. It reports wrong accepts and wrong rejects at each pass mark for one backend and model version |
| 7 | `rank` and `how` |
| 8 | `annotate` enrichment for JSONL, then CSV and TSV input with the same JSONL output |

Parked until a real use arrives: `match`, `segment`, `fold`, `reduce`, `patch`, `resolve`, `derive`, subprocess adapters, shell completion.

**The saved question file is Markdown.** The captures use JSON. A person who owns the work should be able to read the file, and the sibling project botassembly already writes its questions as Markdown. Frontmatter holds the settings. A heading holds each question. A list holds the options, each as a name in a code span followed by its description. `--plan` prints the compiled request as JSON, so the machine form is always one command away. Cost: this repository has to specify and test a small Markdown grammar.

## 7. Backends

The wire format of the first decider model is one request. The body carries a `state`, a `model`, and a map of named typed `questions`. The response carries `answers` under the same names. Version one speaks exactly this shape.

- A backend profile names a base URL, a model, the environment variable that holds its key, its limits, and its default pass marks. Limits cover tokens per question, tokens per request, options per choice, and requests per minute.
- Overriding the URL is enough for any server that speaks the same shape. A local decider model joins by serving that shape.
- A key belongs to one profile. It is never sent to another host. Overriding a URL on a profile drops its key unless the user names a key for the new host.
- This repository publishes the wire format as its own specification with fixture requests and responses. Another implementer can test against the fixtures and claim compatibility.
- A vendor with a different shape gets a small translating server first. A subprocess adapter that exchanges JSON over standard input and output is the later option. Dynamic plugin libraries are refused.
- Pass marks do not carry between models. They live in the profile, and `eval` is how a profile earns them.

On speed. A hosted decider answers in one to two tenths of a second and caps requests per minute. The backend sets throughput on a big file, and the language does not. Rust buys a single binary with nothing to install, a fast start inside gates that fire many times, and steady memory on long streams. A local backend will shift the balance, and measurement decides any tuning then.

## 8. How it fits

- **Botassembly.** A botassembly `gate` is a program that judges a stage's output. A proposed `decide` program would answer a `CHOOSE` or a `LOOP`. Each becomes a two-line script that calls `thinkthen`. The option list in a saved question file has the same shape as the body of a `CHOOSE.md`, so `--from CHOOSE.md` can work with neither project knowing the other. If the runtime later gains a decider setting, it can run `thinkthen` and carry no backend code of its own.
- **Bench.** A checker is an executable that prints a score. A checker that wraps `thinkthen decide if` adds a judged check that abstains when unsure.
- **Small tools.** A prose lint, an inbox sweep, and a Markdown fixer become shell scripts over `where` and `which`. None needs its own program.
- **Spreadsheet enrichment.** A consumer that classifies rows gets `--as` columns with a status column beside each value.
- **The experiment harness.** It stays the place where a decider model is measured. `thinkthen eval` takes over the pass-mark sweep once it exists.
- **The platform.** Ian named a larger platform this tool will join. This study could not find it in his notes. See question 2.

## 9. Open for ruling

Ian ruled on 2026-09-18, and ADR 0003 records it. Questions 1, 3, 4, 5, and 9 are answered: a standalone shell primitive for a wide audience, the whole command line planned with `decide` first, the family word kept, a backend as a URL and a simple adapter, and Bash as the first job. Questions 2 and 8 wait, because every botassembly decision is deferred. Questions 6, 7, and 10 stay open. Sections 6 and 8 above predate the ruling. `plan.md` and `specification/` now lead.

1. **First user.** Ian and his agents, or outside developers. The answer sets when the repository goes public and how much polish version one carries. Recommendation: Ian and botassembly first, private until slice 6 works, then public under the MIT license that botassembly's public siblings use.
2. **The platform.** What it is, and what it needs from this tool on day one.
3. **Scope of version one.** Section 6, or a wider first release that includes a second family. Recommendation: section 6. Cost: the "semantic shell" story waits.
4. **Grammar depth.** `thinkthen decide if` or the shorter `thinkthen if`. Recommendation: keep the family word. The name is half a sentence that the family word finishes, and later families arrive with no renaming. Cost: one extra word on every command line.
5. **How other backends plug in.** One wire format with a URL override, compiled-in adapters per vendor, or subprocess adapters. Recommendation: one wire format, published with fixtures. Cost: a vendor with another shape needs a translating server.
6. **Saved question files.** Markdown, JSON, or both. Recommendation: Markdown as the authored form, JSON as the printed plan.
7. **Where pass marks live.** The question file, the backend profile, or a flag. Recommendation: the profile holds defaults per model, the file may mark a question as essential or advisory, and a flag overrides both.
8. **Botassembly's decider.** Whether the runtime should run `thinkthen` when it gains a decider setting. Recommendation: yes. Cost: a decider in botassembly then needs this binary installed.
9. **The first real job.** The job version one must do well before anything else is built. Candidates: a shadow gate on botassembly stages, an inbox sweep of Ian's notes, a prose lint for his writing rules, spreadsheet enrichment. Recommendation: the shadow gate. It already has measured numbers to beat.
10. **Relations with the first vendor.** Whether to tell TypeSafe about the tool and the published wire format before it goes public. Only Ian can weigh that.
