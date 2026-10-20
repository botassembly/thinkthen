---
date: 2026-10-13
status: design input for 0.3; rulings recorded, data model and commands to design with Ian
source: Ian's dictation on 2026-10-13; [`specification/cache.md`](../../../specification/cache.md) and [`recording.md`](../../../specification/recording.md) as of 0.2; Ian's notes on the data model and on decision knowledge
---

# ThinkThen data: the cache grows up

## What's in it for you (draft, to validate with Ian)

- **Every answer you paid for is yours, in a database on your machine.** You can query it, compare it and export it, with no proxy and no account.
- **Compare models on your own data.** Run the same questions on five models and see where they agree, where they differ and what each cost.
- **Fix the answers that matter.** Two models disagree, you or your agent settle the right answer, and that answer grades every model you try later.
- **Switch models on your terms.** Keep reusing old answers when you move to a cheaper model, or start fresh. It is a setting, not a rewrite.
- **Your path to your own model.** The answers you settled export as training data for a fine-tune.

## Rulings (Ian, 2026-10-13)

- **The simple cache becomes the full database.** SQLite in 0.3. Postgres as a backend later; 0.3 does not need it.
- **The data strategy works without a proxy.** It runs directly from the SDK and the command line. The proxy uses the same store and adds sharing and central control.
- **Whether the model is part of the key is a setting.** Keep the model in the key and a new model asks everything again. Leave it out and a new model reuses old answers. It trades cost against the goal of the run.
- **Two strategies are supported in the data model from day one:**
  1. Run a benchmark against several models, such as five, and compare them.
  2. Run two models, find where they disagree, and curate those cases.

  Whether a command line or a user interface drives them is open.
- **The command line comes first, built for agents.** Once agents can drive these uses from the command line, the user interface is easier to build on top of it.

## What 0.2 already has

- **One SQLite file per cache folder,** `thinkthen.sqlite`, private to its owner. Committed test fixtures are the same rows as `thinkthen.jsonl`.
- **A key per question as sent:** adapter, address, requested model, answering model, the evidence and the question. Change any one and it is a new key.
- **Each row keeps** the probabilities, its share of the token usage, when it was answered, the answering model and an observation id. Thresholds are applied on read, so moving a cut-off needs no new call.
- **Commands:** `cache prune` (by age or by answering model), `cache unused`, `cache convert`, and `status` counts. `audit` grades saved answers against a key file. `diff` compares two runs.
- **What it lacks:** a question identity separate from its wording and model, any notion of a run, a stored right answer, and history. A refresh replaces the held answer in place, so the old answer is gone.

## The gap, in one table

| The two strategies need | 0.2 has |
| --- | --- |
| Several models' answers to one question about one piece of evidence, side by side | Separate keys with nothing linking them except identical evidence and question bytes |
| A named run, such as "five-model benchmark, October" | Nothing; a run is whatever folder you pointed at |
| A settled right answer that grades every model | `audit` reads a key file you keep yourself |
| Every answer kept over time | One answer per key; a refresh overwrites |
| Export for fine-tuning | Nothing |

Ian's data-model notes already name the parts: question, wording, evidence, context, request and raw answer, reading rule, decision, judgment, setup. Its three keys map onto the setting above. The cache key holds the model. The truth key, question and evidence, does not. Leaving the model out of the lookup means looking up by the truth key plus the wording and context.

## A first sketch of the commands (lead's proposal, not ruled)

Each command reads and writes the same SQLite file the cache uses, works the same through a proxy, and prints JSON an agent can read.

- **Run on several models into one named run.** The same input and question sent to each named backend, stored together.
- **List disagreements in a run,** sorted by how far apart the probabilities are, with the evidence.
- **Record a right answer** for a question and a piece of evidence, with who judged it. A person, a script or a large model can do it.
- **Score every model in a run** against the recorded right answers. This is `audit` grown up.
- **Export** settled answers as training data, and any table as CSV or JSON.

## Ian's answers, 2026-10-13

- **It is a database, not a cache.** Nothing is thrown away. It keeps every answer ever received and gives the benefits of caching through caching rules.
- **Caching rules choose the key.** A rule says whether the model, the model version and the wording version are part of the key. Saving money, collecting data for optimization and tuning, and benchmarking are all valid goals, and each wants a different key. The indexes must support every key the rules allow, and the rules are configurable.
- **A question has an id and versioned wordings.** The id names the question's ideal form. Each wording is an exact form, reworded or optimized, with a version number for tracing. The data can be read across versions, and a caching rule can include or exclude the version.
- **One database.** The SDK and the proxy each connect to one database by default, SQLite for now. A setting, such as an environment variable, points at another.
- **The tool never calls a large model.** Large models call the tool: an agent is a large model in a loop calling tools. People and agents settle disagreements by using the tool. Every change records who made it, a person or an agent, and an agent's change also records the person it acts for.
- **A new command, perhaps `db`,** covers the database, caching rules, administration, evals, audit and diff. Its shape needs design.
- **`run` runs a tool from a local toolbox.** Learning what a toolbox holds is part of help.
- **Evals shape the database.** Evals start in 0.3 and may not finish. The big picture of evals must be designed now, because a database designed without it will be wrong. See `evals.md`.
- **Export is JSONL.**

## Draft tables (lead's proposal, to review with Ian)

Each row says what the table holds. Names are placeholders.

| Table | One row is | Key parts |
| --- | --- | --- |
| question | A question's ideal form: its id, function and options | The id, given or derived |
| wording | One exact form of a question, numbered | Question id and version; a hash of the text and option descriptions |
| evidence | The thing judged: a record, a passage, a pair of items | A hash of its content; an optional source and source version |
| context | Added help that is not the thing judged | A hash of its content |
| answer | One model's raw probabilities for one wording, evidence and context | Every part above, plus address, model, model version, time, tokens, latency, run and actor |
| run | A named group of answers with a purpose: everyday use, benchmark, comparison, optimization | Its name, purpose, setups, actor |
| judgment | The right answer, a grade or an order for a question and evidence | Question id and evidence only; actor, time, reason and the quote that supports it |
| dataset version | A frozen, numbered version of a named set | Name and version number, with its own id |
| dataset member | One judgment included in one dataset version | Dataset version id and judgment id |
| outcome | What happened downstream after an answer | The answer id |
| user | A person or an agent; an agent's action also names the user it acts for. One default user to start | Name and kind |

**A caching rule** names which answer parts must match for a stored answer to be reused. Wording text, evidence and context always match. The address, the model, the model version and the wording version are each optional. The 0.2 behavior, where everything must match, is one rule among several. When more than one stored answer matches, the newest wins, and the result says which answer it reused.

**Nothing is overwritten.** A refresh adds a new answer. "The answer" for a key is the newest one that the rule accepts.

## Ian's answers, 2026-10-13, on the draft

- **Least surprise is the operating principle.** The default caching rule is the 0.2 one: everything must match. Saving money across models is a rule you turn on.
- **A user model, not an authorization model.** 0.3 has no login. A users table starts with one default user. A command lets a person say who they are, much as a coding tool's subscription sign-in records an identity: it creates the user record and makes it current, with no password check. Later a person can add another user and act as them. Authentication comes in a later version. Agents record themselves and the user they act for.
- **Frozen, numbered sets, normalized.** A benchmark or gold set has a name and numbered versions, each with its own id. Membership is its own table: one row for each judgment a version includes. Adding a judgment makes a new version.
- **No canned ranking question.** Pairwise ranking asks the caller's own question, such as "Is this song on Abbey Road?", of two items at once. The stored answer records that it compared two pieces of evidence under that question. Turning many comparisons into one score is the ranking experiment's job (`functions.md`).

## To design next

1. Review the draft tables and caching rules above with Ian, checked against every use in `evals.md`.
2. The `db` command: its subcommands, and where `audit`, `diff` and the cache commands move.

## A boundary to settle in the thinkthen repo

The thinkthen repo's rules say the proxy owns curation and the SDK adds no business policy. This ruling puts runs, comparison and recorded right answers in the SDK. The PM needs to update that boundary when this design is handed off.
