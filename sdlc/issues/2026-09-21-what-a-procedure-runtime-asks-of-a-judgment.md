# What a procedure runtime asks of the judgment

Status: Open

Observed 2026-09-21 at `9fd4dcf`. Ian asked a steering agent to read this tool, an owned procedure runtime, and the `thruwire/foreman` experiment, and to name every gap that stands between the tool as it ships and a caller that must justify each judgment inside a sealed record. This page is that answer. It files an ask and authorizes nothing.

## The caller this page describes

A procedure runtime runs authored Markdown flows over folders. A stage runs an agent, writes an output file, and runs checks around it. A gate is an executable in any language that answers by exit code, and the runtime hands it the output path. The runtime keeps an append-only record of every run and promises that a finished run can be re-read with no network. Such a caller wants the tool in three places: a gate that decides whether a stage exits, a gate that decides whether a loop continues or which branch runs, and a judgment stage that turns one stage's output into structured fields for the next.

No item below asks the tool to learn what a stage is, what a flow is, or what a record is. Every item is a property of a command-line judge.

## What already answers the need

- **A question file is a complete declaration.** The first key names the verb, and the file carries the question text, the threshold, `on`, and the model. A caller holding a question file path and a pointer into its own artifacts holds the whole declaration. `specification/question-file.md`.
- **`--details` is one result object across all eight verbs** at `schema thinkthen.result/1`. `value` is `null` for an unresolved answer on `decide` and `choose`, and `answer.pick` names what an unresolved pick leaned toward. A caller can record a judgment it did not take. `specification/result.md`.
- **`--dry-run` checks a question set with no key and no request.** That is the offline structural check a runtime needs before it runs anything.
- **The band has a measurement behind it.** Answers inside the band flipped 5% to 14% between identical runs, and answers outside flipped 0.5% to 2%. `specification/threshold.md`. That number belongs in the long help of `decide`, where a gate author meets the option.

## The gaps, in the order to fix them

### 1. A single-verb row cannot be traced to the evidence it judged

`specification/result.md` gives each `annotate` answer a `request` field, "the digest that also names the recording entry." The `meta` table for a single-verb row stops at `tool`, `question_sha256`, `url`, `model`, `usage`, and `replayed`.

So an `annotate` row proves what was sent and a `decide` row does not. That matters because a gate is built on `decide`. The shape with a positive measurement is a narrow yes/no question over a real document through a short gate script. The row that gate writes names the question and never names the evidence.

Smallest fix: add `request` to the single-verb `meta` table, the same digest `annotate` already carries. One field in one place makes the recording the caller named reachable from every row of every verb.

### 2. Nothing bounds what a repeated caller spends

Filed already at `2026-09-20-a-status-command-for-configuration-and-usage.md` and item 1 of `2026-09-21-where-a-user-could-lose-trust-a-first-list.md`. That page recommends a stateless `--max-requests N` and no ledger in 0.1, because the price argument suits one command. Ian said on 2026-09-21 that he may not care about a cap.

A repeated caller is a different case, and the ruling of 2026-09-21 changed what it costs. Ian ruled that the cache and the configuration live in the XDG folders and that "never writes a file the user did not name" gains one stated exception for the tool's own XDG folders. The objection that a ledger needs a folder the user never chose is therefore gone.

What is still missing is a total that survives across invocations. `--max-requests N` bounds one run. A runtime that runs the same flow a hundred times, and a loop inside one flow, both spend across processes, and no option today can see it.

The shape that fits the existing rulings: a `ledger.jsonl` under `$XDG_STATE_HOME/thinkthen`, beside the folders Ian named. State is the right home and not cache, because a cache must be safe to delete and a ledger must not. Plain rows, so `transforms/cost/cost.jq` reads it. `thinkthen status` already promises the folders and the counts. One ceiling that refuses before the request that would cross it, on the rule ticket 0028 already set for `sdlc/scripts/live`: refuse before spending, never after.

**This is the item that decides whether a loop can call the tool at all.** Each other item is a convenience for a caller that already works.

### 3. `--dry-run` prints a plan, not the bytes

Item 11 of `2026-09-21-where-a-user-could-lose-trust-a-first-list.md` names `--dry-run` and `--field` as the honest answer to what leaves the machine. `--dry-run` names the pointers and the request count and never shows the request. A user then trusts a boundary and cannot verify it.

`specification/fixtures/systemone/` already pins exact request bytes and `encode` is already deterministic. Printing those same bytes under `--dry-run` costs a print, and it closes a rule that `2026-09-21-a-refused-request-hides-the-backends-reason.md` found broken: a dry run accepted `--jobs 1` that the live run refused at exit 2. A dry run that prints the bytes it would post is a dry run a caller can trust.

### 4. A cut cannot travel inside a `score` question

`specification/roadmap.md` holds a threshold on `score` because `jq -e '. >= 2'` covers one script, and it names the trigger: a demo where the cut has to travel inside a saved question file.

A tuning tool is that demo. The roadmap already names the question file as the unit a tuning tool improves against labeled cases, and Optimizer is a sibling of this tool. When a degree-shaped judge keeps its cut in the gate script, the tuner edits two artifacts in step and the question file stops being the complete declaration described above. A `score` question inside a question set has the same hole.

### 5. One advisory question discards the essential answers

A question set is one request and a failure ends the record. For a set with one essential question and four advisory ones, a backend refusal on an advisory question throws away the answer that mattered.

The rule that a failure is never a `null` should hold, as `specification/annotate.md` says. What is missing is the middle case, a question that failed inside a request that otherwise succeeded. A `null` for that field plus a failure count, with a transport or auth failure still ending the run, keeps the safety rule and stops one weak question from taking the set with it.

## What this does not ask for

- **A band on `choose`.** Exit 3 already reports an unresolved pick, and an exact tie is unresolved with or without a cut. The single cut is the uncertainty rule and a band would state it twice. Ian asked for this to be checked and the answer was no change.
- **A library.** Calling the command is the ruled path for version one, and `specification/roadmap.md` is right that a binding is a second product. The result object and the exit codes serve a caller without one.
- **A required mark in a question file.** The roadmap is right that a required mark is policy. A caller names its essential question and applies its own rule to the answer.
- **A serve mode.** Declined, and correctly. `filter`, `rank`, and `annotate` already answer a whole stream inside one process.

## The ninth and tenth functions

`recognize` and `relate` are coming. `annotate` already covers many named questions over one document. Make `recognize` carry one question over many positions and `relate` carry one question over pairs, and give both the same result object, the same answer kinds, the same question file grammar, and the same exit-code table.

A caller will then treat ten verbs as one tool. That single property is worth more to a runtime than either new function, and it is the property most easily lost while two functions are added.

## The smallest outcome that closes this page

Two changes: `request` in the single-verb `meta` table, and the exact request bytes under `--dry-run`. Both are print statements against machinery that already exists. A caller can then carry a judgment into a sealed record and prove months later what the judge was shown and what it answered.

## What Ian can overturn

All of it. Items 1 and 3 are small and this page argues for them. Item 2 carries a ruling, because he already said he may not care about a cap and this page asks for the same cap with a different reason. Items 4 and 5 are a tuner's problem and a robustness problem, and both can wait.

Review trigger: the version one surface freezes, or 2026-10-21.

## The product side's reply, 2026-09-21

Ian asked the marketing side, which holds the product shape job, to review this page. Checked against the release binary at `9fd4dcf`.

- **Item 1, agreed, and it should land before the bindings freeze.** The result object is what every library and database returns under details, so one field added now reaches all nine surfaces for free. Added later, it is nine changes.
- **Item 3 is half wrong as written.** `--dry-run` does print the request. `decide --dry-run` and `filter --dry-run --lines` both printed `url`, `model`, `key_env`, the framing, the pointers, and the full `request` body with the text and the questions. What is truly missing: it prints the request for the first record only, it prints no request count, and it accepted `--jobs 1` where the live run refused. The `recognize` and `relate` designs both depend on a printed request count and pair count, so that part matters more than this page says.
- **Item 2, agreed on the need, with three conditions.** The count belongs in the engine layer, so a library and a database extension inherit it. A database runs the engine from many processes at once, so the ledger needs the same locking care as the cache. A ledger row holds counts and never a text or a question. The ceiling stays off unless the user sets it. A running total is also what the deck's cost slide lacks. Whether it is in 0.1 stays Ian's call.
- **Item 5, disagreed on the form, and it is more urgent than this page says.** `recognize` and `relate` send many questions in one request, so one refused question inside a good request is their everyday case. The recognize experiments found that a refused word loses the whole name. A failed question must never print `null`, because `null` already means "not sure". It needs its own marker on that answer, with the failure count in `meta`.
- **Item 4, agreed, and later.**
- **The ninth and tenth functions.** Agreed. Both design pages now say that `--details` prints the standard result object with the new value inside it.

Questions for the caller's team:

1. How many judgments does one flow run make, and how many runs a day? That sizes the ceiling.
2. Is a ceiling on requests enough, or must it be tokens?
3. A sealed record that holds only the digest points at a recording the default cache may prune at its 100 MB limit. Does the runtime name its own `--cache` folder inside the run's record? It should, and the manual should say so.
4. Must the digest cover the text after `--field` picked it, or the whole record?
5. Which per-question failures has the team seen in practice, and how often?
