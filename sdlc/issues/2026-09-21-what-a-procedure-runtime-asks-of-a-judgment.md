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

Filed already at `closed/2026-09-20-a-status-command-for-configuration-and-usage.md` and item 1 of `closed/2026-09-21-where-a-user-could-lose-trust-a-first-list.md`. That page recommends a stateless `--max-requests N` and no ledger in 0.1, because the price argument suits one command. Ian said on 2026-09-21 that he may not care about a cap.

A repeated caller is a different case, and the ruling of 2026-09-21 changed what it costs. Ian ruled that the cache and the configuration live in the XDG folders and that "never writes a file the user did not name" gains one stated exception for the tool's own XDG folders. The objection that a ledger needs a folder the user never chose is therefore gone.

What is still missing is a total that survives across invocations. `--max-requests N` bounds one run. A runtime that runs the same flow a hundred times, and a loop inside one flow, both spend across processes, and no option today can see it.

The shape that fits the existing rulings: a `ledger.jsonl` under `$XDG_STATE_HOME/thinkthen`, beside the folders Ian named. State is the right home and not cache, because a cache must be safe to delete and a ledger must not. Plain rows, so `transforms/cost/cost.jq` reads it. `thinkthen status` already promises the folders and the counts. One ceiling that refuses before the request that would cross it, on the rule ticket 0028 already set for `sdlc/scripts/live`: refuse before spending, never after.

**This is the item that decides whether a loop can call the tool at all.** Each other item is a convenience for a caller that already works.

### 3. The dry run shows the first record's request only, and prints no count

Corrected 2026-09-21. This item first said the dry run never shows the request, and that was wrong. Verified against the release binary at `9fd4dcf`: `decide --dry-run` and `filter --dry-run` over three records both printed `url`, `model`, `key_env`, the framing, the pointers, and the full request body with the text and the questions.

Three parts of the item stand:

- **It prints one request and never the rest.** Three records in, one request out, with nothing naming the number that was left.
- **It prints no request count.** A caller cannot see the size of the job before it starts.
- **It accepts a `--jobs` value the live run refuses**, which `closed/2026-09-21-a-refused-request-hides-the-backends-reason.md` already records.

The count matters more than this page first said. `recognize` and `relate` both send many questions in one request, and both designs depend on a printed request count and pair count. `closed/2026-09-20-a-status-command-for-configuration-and-usage.md` already asks for the count in one place. A caller that must bound its own spend needs it in the dry run.

Wave 1.5 reproduced this concretely: `--dry-run` accepts a JSONL file whose third record is invalid (exit 0) where the live run refuses at record 3 (exit 2). Found by experiment 218, wave 1.5.

### 4. A cut cannot travel inside a `score` question

`specification/roadmap.md` holds a threshold on `score` because `jq -e '. >= 2'` covers one script, and it names the trigger: a demo where the cut has to travel inside a saved question file.

A tuning tool is that demo. The roadmap already names the question file as the unit a tuning tool improves against labeled cases, and Optimizer is a sibling of this tool. When a degree-shaped judge keeps its cut in the gate script, the tuner edits two artifacts in step and the question file stops being the complete declaration described above. A `score` question inside a question set has the same hole.

### 5. A failed question inside a good request needs its own marker

Corrected 2026-09-21. This item first asked for a `null`, and that was wrong. `null` already means "not sure", and a caller branches on it, so a `null` that also meant "failed" could never be taken back.

A question set is one request, and today a failure ends the record. For a set with one essential question and four advisory ones, one backend refusal throws away the answers that worked.

What is needed is a marker of its own on that answer, plus a failure count in `meta`, with a transport or auth failure still ending the run. The rule that a failure is never a `null` holds and gets sharper: a failure is a third thing, distinct from yes, no, and not sure.

This is more urgent than a robustness nicety. `recognize` and `relate` send many questions in one request, so one refused question inside a good request is their everyday case, and the recognize experiments found that one refused word loses the whole name. For the frequency, `closed/2026-09-20-the-strict-probability-total-refused-two-live-replies-in-about-thirty.md` refused about two live replies in thirty, which is about one in fifteen.

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

## The caller's team reply, 2026-09-21

Both corrections accepted. Item 3 was checked against the release binary at `9fd4dcf` and the product side is right: `decide --dry-run` and `filter --dry-run` print the address, the model, the key variable, the framing, the pointers, and the full request body. Item 5 was checked against `specification/annotate.md` and the product side is right again: `null` already means "not sure", so a failure needs a third marker on the answer and a count in `meta`. Both items are amended above.

## Where the caller's team stands on timing

Ian's position: do it now if it changes the shape of anything, because after launch is worse.

**Shape-changing, and worth doing before the surface freezes:**

- **Item 1, the request digest.** One field now reaches every surface. Later it is nine changes. The product side's argument settles it.
- **Item 5, the marker for a failed question.** This is the largest shape decision on the page. A `null` that means two things is a contract a caller cannot leave, and `recognize` and `relate` multiply how often it happens.
- **Question 3 below, where the judged evidence lives.** This one is the caller's own shape and the product side found it. A run record that keeps only a digest whose entry the cache may prune breaks the promise that a sealed record can be re-read. The run directory gains a folder, and a published record layout is a compatibility surface.

**Not shape-changing, and it can wait:**

- **Item 2, the ledger.** Plain out-of-band rows. A running total added later changes no result and no record. Three rulings are cheap now: a row holds counts and never text, the ceiling is off unless the user sets one, and the count belongs in the engine so a library and a database inherit it. The build can wait.
- **Item 4, the score cut.** Deferrable, with one ruling worth taking now on the caller's side: a cut lives in the question file where the verb supports one and in the gate script where it does not. That keeps the gate examples from being rewritten later.
- **Item 3, the dry-run count.** Deferrable for this caller and a dependency for `recognize` and `relate`. Not this caller's shape.

## Answers to the five questions

### 1. Judgments per run, and runs a day

Measured from this machine's own home at `~/.local/share/bot/runs` on 2026-09-21. It holds 2,040 runs from 2026-08-06 to 2026-09-16.

| Measure | Value |
| --- | --- |
| Runs in the home | 2,040 over 42 days |
| Runs a day | 206 at the peak, and about 49 across the whole span |
| Stage attempts in one run | median 5, p90 7, max 9, mean 4.0 (120-run sample) |
| Stage attempts that are a retry | 18.8% (208 attempts in 60 runs) |
| Checks in one stage attempt | 1 to 4, mean 3.33 (193 attempts) |

So one run makes 4 to 13 judgments if every check became one, and a peak day makes 800 to 2,700. At the measured price of about 1.2 cents per thousand judgments, that is 1 to 3 cents a day. A ceiling sized on requests should sit well above 2,700 and well below anything that would surprise a user.

### 2. Requests is not enough. It must be tokens, and it must be a per-request guard

Measured stage output sizes, which are the evidence a gate would judge (136 outputs across 40 runs):

| Measure | Bytes |
| --- | --- |
| min | 40 |
| median | 2,750 |
| p90 | 16,253 |
| max | 1,433,440 |
| mean | 54,365 |

The spread is 40 bytes to 1.4 MB, a range of about 36,000 to 1. One request at 1.4 MB is roughly 350,000 tokens, five times over the vendor's 64,000 limit, and a count-based ceiling lets that call through.

So the ceiling needs two parts: a running count for the loop, and an input-size guard that refuses before the send. The second is the one that matters, and it is the pre-flight check `2026-09-25-docs-how-tos-and-spec-claims-owed.md` already asks for.

This also settles a caller-side design rule. A gate must never pipe a raw stage output at a judge, because the p90 is 16 KB and the tail is 1.4 MB. The manifest has to select and bound, the way the `thruwire/foreman` experiment bounds a diff at 20,000 characters and each output tail at 12,000.

### 3. The run does not name its own recording folder today, and it must

Two halves, and the first is better news than the question suggests.

**The details row already lands.** `specification/elements/record.md:393` retains a gate's exit code, its executable file, its hash, and its exact evidence capture, and `check.capture` holds the exact output. A gate that prints a `--details` row therefore puts that row in the sealed record with no new mechanism anywhere.

**The evidence behind the row has no home.** The run should name its own `--record DIR` inside the stage attempt directory and must never point a judgment the sealed record depends on at the shared XDG cache, because a cache is prunable and a run record is not. The digest then resolves for as long as the record exists, which is the promise this caller makes.

This is a change to the record layout, so it is a shape decision and it belongs before the layout is published. The manual should say the same thing, which is what the product side asked for.

### 4. After `--field`, and the whole record is a different field

The digest covers the text after `--field` picked it. That is forced by the recording design and needs no new decision. The recording key is `sha256(adapter, address, request bytes)`, and the request bytes carry the post-`--field` state. `request` is "the digest that also names the recording entry" (`specification/result.md:113`), so it can only be the digest of what was sent.

The whole record is already a separate field. In record mode, `--details` carries `input`, "the whole record, including parts that were never sent" (`specification/result.md:94`). Keep the two apart. `request` proves what left the machine, `input` proves what the caller held, and merging them would lose the boundary property. A single document has no record, so `request` is the only digest it can have.

### 5. No per-question failures seen, because no judgment has ever run

The honest answer is none. Botassembly has never run a judgment. The b03 experiment found no `chose`, `loop_done`, `subflow_call`, or `fanout_done` event in 2,039 records, and every check in the 2,040 runs measured above is a checklist, a schema, or a gate script.

The nearest real number is on the ThinkThen side, and it argues for the product side's position. `closed/2026-09-20-the-strict-probability-total-refused-two-live-replies-in-about-thirty.md` refused about two live replies in thirty, about one in fifteen, and that is a whole-reply refusal. If one bad question kills the reply, a ten-question `recognize` request fails about one time in fifteen.

That is the everyday case, and it is why the marker for a failed question belongs before `recognize` and `relate` land and not after.
