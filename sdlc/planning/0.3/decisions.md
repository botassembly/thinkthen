---
date: 2026-10-13
updated: 2026-10-14, after the six 0.3 experiments
source: the ten ThinkThen 0.3 PRDs written on 2026-10-13, in [prd/](prd/)
---
### Ian's PRD feedback

1. Let's reconsider some of the commands that we need to have library exposure with, especially around admin. I definitely want to be able to run toolbox commands from every library and every surface, but I don't know if I need to set up a toolbox using R or Python. I definitely want to support it using the Rust SDK and the CLI. The only question at this point is TypeScript and whether that waits for the buildout of the UI, if we need it.
2. I think the first version of the MCP doesn't need to have the admin stuff. It can just have the run. It just has the 11 functions (10 + run).
3. 
4. 


# ThinkThen 0.3: decisions for Ian

Updated 2026-10-14, after the six 0.3 experiments. The list now holds every question in the PRDs, one choice per question, most important first. Each question gives the options, what each one costs or gains, and a recommendation.

Questions 1 to 17 shape 0.3 and are the ones to answer first. Questions 18 to 24 belong to 0.4 and 25 to 32 to 0.5; they can wait, except 18 to 20, which let two 0.4 experiments start early. You can answer "all recommended" and name the ones you would change.

## What you already decided

- **Three releases.** 0.3, own your answers: the database, toolboxes and `run`, MCP as one tool, spreadsheets and publishing. 0.4, grade and improve: curation, evals and the ranking setting. 0.5, serve and structure: the proxy and document structure. The 0.3 database is designed for 0.4 from the start.
- **Libraries.** Every library gets only the ten functions and `run`. Toolboxes, answers, curation, evals and admin stay in Rust and the command line.
- **MCP.** It runs only the ten functions, `run` with configured tools, and help. Help lists the tools and shows what the session has spent.
- **Curation.** Curation means fixing data: find where models and labels disagree, or, without labels, put the answers most likely to be wrong first. Agents and notebooks curate through the command line; there is no library curation for now.
- **Reuse.** Saved answers are reused when the exact text matches, as in 0.2. A question has an identifier and version numbers. Looking up answers by question uses the latest version unless you name one.
- **Keep everything.** The database keeps every answer, including repeated calls. Clearing old answers is later work, so 0.3 has no cleanup command and no size limit.
- **DuckDB 1.4 is dropped.** The experiment showed DuckDB 1.4 lacks a piece ThinkThen needs, so one build cannot cover it. ThinkThen supports DuckDB 1.5 and 2.0.
- **Spreadsheets stay in 0.3.** The writer prototype passed, and the three files you opened in Google Sheets looked right.

## 0.3: answer these first

### 1. The promise about running commands

A tool can chain functions with programs, such as a database search before a `rank`. That breaks the promise in the 0.2 docs and your ideal-state note: "ThinkThen never runs a command." This is the most important question, because it is a public trust promise and hard to take back.

- **A. A new promise.** "ThinkThen runs only the steps a tool's author wrote, in order, and an answer never chooses whether or which command runs." Tools can include search and lookup steps. You must update the ideal-state note and the docs.
- **B. Keep the old promise.** Tools chain only the ten functions; a person or agent runs any program outside ThinkThen and pipes the results in. The promise stays simple, but tools become much weaker, and the retrieval pipelines a private trial-matching project asked for need outside glue.

**Recommended: A.** The new promise is still strict: an answer can never trigger a command.

### 2. Safety for tool steps that run programs

This only matters if you choose A in question 1. Every option keeps one safety rule: a tool never gives its caller more power to run programs than the caller already has. So the command line runs any toolbox you name, agents run only toolboxes set up when the server starts, SQL never runs program steps, programs never see a ThinkThen key, and a toolbox folder other users can change loses its program steps.

- **A. The safety rule alone.** No prompt and no sandbox in 0.3. Runs never stop to ask, which fits your ruling that tools must work in CI. A toolbox you run can do anything you could do yourself.
- **B. Ask before the first run** of a tool with program steps. Safer for people at a keyboard, but it breaks unattended and CI runs, against your ruling.
- **C. A sandbox for program steps.** The safest option, but it differs on every platform and is a large build.

**Recommended: A.** It matches how people already treat scripts they choose to run.

### 3. Letting people read the database directly

"You own your answers" is strongest when people can query them with plain SQL. The schema experiment showed that 0.4's notebooks need nine more views than the six first planned, so the read contract is still changing.

- **A. Private.** Only ThinkThen commands read the database. Nothing to keep stable, but the ownership claim is weak.
- **B. Public views, marked preview in 0.3 and stable in 0.4.** People can query now, and the views get one round of change once curation and evals use them.
- **C. Public views, stable from 0.3.** The strongest promise now, but every 0.4 change then needs a migration and a compatibility note.
- **D. Public tables.** Simplest, but every table change breaks someone.

**Recommended: B.**

### 4. `answers` or `decisions`

The name for your stored results appears in the command line, SQL and docs, and is hard to change later.

- **A. `answers`.** Plain, and it cannot be misread as the output of `decide`.
- **B. `decisions`.** Matches "rent your models, own your decisions," but `decisions list` reads like a list of `decide` results only.

**Recommended: A.** The tagline can still say "decisions."

### 5. `login` with no password

0.3 needs a way to say who is acting, so every stored answer and right answer names a person or an agent. Real sign-in comes later.

- **A. `login NAME`,** with a plain notice that 0.3 checks no password. Nothing is renamed when sign-in arrives, but the word promises more than 0.3 does.
- **B. `whoami` or `user set NAME`.** More honest now, but it gets renamed or doubled when sign-in arrives.

**Recommended: A,** with the notice.

### 6. Spreadsheets: a new command, or options on the functions

You called Excel "a new tool." The question is how people type it.

- **A. Options on the existing functions:** `--excel`, `--output`, `--in-place`, `--into` and `--fill` on `decide`, `choose`, `score`, `tag` and `annotate`. The command list stays short, and every function that gives one answer per row works with spreadsheets. A toolbox tool can still wrap one sheet's job under a name.
- **B. A new `excel` command.** Easier to discover ("grep for Excel"), but it is one more top-level word that repeats what the functions already do.

**Recommended: A.**

### 7. Spreadsheets: allow output files

0.2's rule says ThinkThen prints JSON lines and never writes output files.

- **A. Change the rule for table formats.** Excel and CSV runs write a new file by default, and writing into the original makes a backup first, as you ruled.
- **B. Keep the rule.** People convert the JSON themselves, which defeats "an agent hands over a sheet and gets a sheet back."

**Recommended: A.**

### 8. Spending caps for agents

An agent in a loop can send far more requests than its person expects. 0.2 has no cap by default.

- **A. No cap by default,** as in 0.2. Nothing ever stops halfway, but one runaway loop can spend a lot.
- **B. A cap on by default:** 1,000 live requests and 2 million input tokens per agent session, raised only in the configuration file. Help shows the spend. A long job can stop halfway, but the stored answers make the rerun free.

**Recommended: B.**

### 9. Agents on the network

- **A. Local only in 0.3.** The agent connection runs on your own machine. Nobody else can spend your keys, but a remote agent cannot use it.
- **B. Over the network now.** Without sign-in, anyone who can reach it spends your keys, and a remote server can neither read the agent's files nor run its tools.

**Recommended: A,** until sign-in exists.

### 10. The command inside the Python and npm packages

Today `pip install thinkthen` gives the library only. The packaging experiment showed that adding the command works: `uvx thinkthen` and `npx thinkthen` started the agent server with nothing else installed.

- **A. Yes, in 0.3.** Agents and Python or JavaScript users get the command in one step, and the agent server can be listed in the MCP Registry. The Python download grows from 5.5 to 11 MB, npm's from 2.5 to 8 MB, and each install by about 15 MB.
- **B. No.** Packages stay small, and the command comes only from the installer, Homebrew or crates.io.

**Recommended: A.**

### 11. Which publishing channels to polish

ThinkThen publishes to 21 channels, and few are finished.

- **A. Three tiers.** Six front doors polished every release: GitHub with the installer, Homebrew, crates.io, PyPI, npm and DuckDB. Nine listed and checked every release: R-universe, RubyGems, NuGet, Maven Central, pub.dev, Packagist, Go modules, and the PostgreSQL and SQLite downloads. Six as downloads only: C++, Swift, Zig, Ada, COBOL and Objective-C. Ten more channels wait. A listed channel with almost no downloads for two releases becomes a candidate to drop.
- **B. Polish all 21.** Every channel looks finished, but each release costs far more work.
- **C. Fewer front doors,** such as GitHub, PyPI and npm only. Less work, but Rust, Homebrew and DuckDB users meet unfinished pages.

**Recommended: A.**

### 12. Who posts on outside sites

The DuckDB pull request went out under your name without your knowing.

- **A. Agents post words you approved.** You approve the exact text once, the PM posts exactly that text, and replies to maintainers stay yours. Routine version bumps use a one-line text you approve once.
- **B. You post everything yourself.** Full control, but every release waits on you.

**Recommended: A.**

### 13. The file format for tools

- **A. JSON,** the same as today's question files. One grammar and no new parser, but no comments.
- **B. YAML.** Easier to write by hand, but whitespace mistakes change meaning, and it adds a second grammar.
- **C. TOML.** Readable and strict, but awkward for nested steps, and also a second grammar.

**Recommended: A.**

### 14. European CSV files

In much of Europe, Excel saves CSV with semicolons and a local character set.

- **A. Stay strict.** Read commas and tabs in UTF-8 only, and print a message that says exactly how to fix the file or to use `--excel` on the original workbook. Cheap; a `--delimiter` option can come later.
- **B. Read semicolons and local character sets.** Friendlier, but guessing a character set can silently garble text.

**Recommended: A.**

### 15. An outside user before release

A team outside ThinkThen trying 0.3 catches problems before the public does.

- **A. Ask a private trial-matching project** to try the database and toolboxes once they work. It already wants classifier-based retrieval and gave toolbox feedback. Friendly and quick, but it is still an internal team, and it costs its time.
- **B. Find an outside developer.** A more honest test, but slower to arrange.
- **C. No outside user** before release.

**Recommended: A.**

### 16. Example toolboxes

- **A. Two examples:** a BANKING77 support-request router and a three-step lookup over a public vocabulary. They serve as the launch demo and as experiment material.
- **B. One example.** Less to maintain, but it shows only one kind of tool.
- **C. None.** People start from the docs alone.

**Recommended: A.**

### 17. The leaderboard on Awesome ThinkThen

Your toolbox ruling says "no model comparisons; ThinkThen supports every model." The Awesome ThinkThen list publishes a Beatles Bench leaderboard that ranks models, and the top three are a statistical tie. No change goes out until you say go.

- **A. Keep the ranked leaderboard.** It attracts attention, but it crowns a winner.
- **B. Each model's results in its own guide,** with no ranking. Fully neutral, but harder to compare.
- **C. One table sorted by name,** with each result's range of uncertainty shown. People can compare, and nobody is crowned.

**Recommended: C.**

## 0.4: can wait, except 18 to 20

### 18. Spending on the evals experiment

It runs two models, an agent settling answers and a model that rewrites wordings, on the public BANKING77 dataset. Running it now lets its results shape the 0.4 plan.

- **A. Approve a $10 cap now.**
- **B. Wait until 0.4 starts.** No spending now, but 0.4's design waits for the results.

**Recommended: A.**

### 19. Data for the ranking experiment

The experiment compares ranking settings on standard search benchmarks. TREC Deep Learning and several BEIR sets allow only non-commercial research use, which matters for a company's product test and for published numbers.

- **A. Use every set,** and publish everything. The best-known benchmarks, but license risk.
- **B. Use every set, but publish numbers only from commercially licensed ones.** Better internal evidence, but running the product test on non-commercial data is itself doubtful.
- **C. Commercially licensed sets only.** Clean, but it leaves out TREC Deep Learning, the benchmark most readers know.

**Recommended: C.**

### 20. Spending on the ranking experiment

The PRD estimates about $12 per model, across two hosted models.

- **A. $30.** Covers the estimate with room for one rerun.
- **B. $150,** as the PRD asked. Room for more models and sets.

**Recommended: A.**

### 21. The name of the ranking setting

The setting says what the model sees while ranking: each item alone, a window of items, a tournament or all pairs.

- **A. `--method`.** Plain, and it does not clash with `eval compare`.
- **B. `--compare`,** as the PRD proposed. It says what the model does, but `--compare` and `eval compare` would mean different things.

**Recommended: A.**

### 22. Answer keys settled by agents alone

- **A. By default, answer keys use answers settled by people or by audited agents.** A key made only by agents is allowed and labeled with that agent's measured error rate.
- **B. Treat agent answers like people's.** Faster, but errors in the key go unnoticed.
- **C. People only.** Most trusted, but slow and costly.

**Recommended: A.**

### 23. The bar for shipping a ranking setting

- **A. A clear win.** A setting ships if it beats ranking items alone by at least 0.02 on the standard search score (nDCG@10), with the 95% range above zero, on most of the chosen sets and on two models. Each setting ships or drops on its own.
- **B. Any measured gain.** More settings ship, but some will be noise.
- **C. A stricter bar.** Fewer, surer settings, but good ones may be dropped.

**Recommended: A.**

### 24. A fixed comparison sentence

Some ranking settings may work better if ThinkThen wraps the caller's question in one fixed yes or no sentence.

- **A. Allow it if it clearly wins,** and show it in `--plan`.
- **B. Never add words** to the caller's question. Purer, but a better setting may be lost.

**Recommended: A.**

## 0.5: can wait

### 25. A password for the proxy

You ruled that 0.3 has no access control. The proxy can be reached from other machines.

- **A. None.** Simplest, but anyone who can reach the proxy spends your keys.
- **B. One optional shared password,** required whenever the proxy listens beyond your own machine. Plainly not sign-in.

**Recommended: B.**

### 26. `segment` as a new function

`segment` would turn a document into a tree of parts. Chunking becomes a reading option and compacting a `rank` option.

- **A. A new top-level function** beside the ten and `run`. Clear, but it is a twelfth word everyone learns.
- **B. Part of `recognize`.** No new word, but `recognize` returns a flat list, not a tree.
- **C. Decide after the segment experiment.**

**Recommended: C,** leaning A.

### 27. One program or two

- **A. The proxy lives inside the `thinkthen` program,** kept out of the libraries. One install, one version.
- **B. A separate program.** Smaller downloads for people who never serve, but two versions to keep in step.

**Recommended: A.**

### 28. OpenAI-format requests through the proxy

- **A. Pass them straight to OpenAI first,** and translate them for other models once tests show the formats match. Cheap and safe.
- **B. Translate for any model now.** The stronger pitch, at a higher cost and risk.

**Recommended: A.**

### 29. Small models

- **A. Route to small models hosted elsewhere** until an experiment shows they can answer these questions.
- **B. Run small models inside the proxy.** One box does everything, but the proxy grows and must run models.

**Recommended: A.**

### 30. Document formats

- **A. Markdown and plain text only.** Other tools convert PDF and HTML first. PDF tables failed in two earlier experiments.
- **B. ThinkThen converts PDF and HTML itself.** Easier for users, but a large and fragile build.

**Recommended: A.**

### 31. Ship the free parser regardless

The parser finds the author's own headings and sections with no model call.

- **A. Ship it whatever the experiment shows,** and add model questions only if they win. Agents get a free map of every document.
- **B. Ship nothing until the experiment finishes.**

**Recommended: A.**

### 32. Which languages get the document tree

- **A. Every library,** like the other functions. Consistent, but a tree type in 24 languages is a lot to keep in step.
- **B. Rust and the command line first.** Other languages read its JSON output, and the libraries get it once it proves itself.

**Recommended: B.**
