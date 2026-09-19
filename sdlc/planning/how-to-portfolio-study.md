# How-to portfolio study: software-development coverage, three themes, and a list of 20

A second agent wrote this study on 2026-09-19. ADR 0018 takes its list of 20 and its three page designs, and it changes the front window. Ticket 0018 had already landed when the study arrived, so ticket 0021 carries the change.

Read-only study, 2026-09-19. Sources: this repository (README, `specification/`, ADRs 0011, 0013, 0016, tickets 0014, 0015, 0017, 0018, `sdlc/planning/{documentation-plan,ten-use-cases,interface-audit}.md`, the issue on label tuning and the issue on the triage pipeline, `demos/`, `transforms/`), a survey of launch-week projects that use a decider in the development process, a saved page on a label-tuning tool, and two saved pieces on evals built from cheap parallel questions, the owner's statement of where the tool is going, and the source of the label-tuning tool. Nothing was written inside a repository. No key, `.env`, or `.zshenv.local` was opened. Nothing clinical or biological was read.

Throughout, "available" means available today or when a named ticket lands. `filter` and `rank` are ticket 0014. `annotate` is ticket 0015. The question file, `--true`, `--false`, and `--option LABEL=DESCRIPTION` are ticket 0017. `find` is slice 11.

---

# Part 1. Coverage of the software-development use cases

One row per project or pattern in the survey.

## 1. Foreman: a supervisor watching a coding worker

**The decision.** Is the worker on task, is it progressing, are the tests enough, is it stuck, is the job done, does a person need to step in? Six bounded judgments over one packet of evidence: a bounded diff, the tail of the tool output, the verification result, the ticket text.

**The command.** `annotate` with a question set, then a policy transform.

```sh
jq -n --rawfile ticket ticket.txt --rawfile log <(tail -n 40 run.log) --rawfile diff <(git diff -U2) \
  '{ticket:$ticket, log:$log, diff:$diff}' |
thinkthen annotate supervise.json --jsonl --cache runs/supervise --details |
jq -f transforms/policy/supervise.jq
```

**Verdict.** Covered when ticket 0015 lands. The question set holds `on` pointers so the progress question never sees the ticket and the completion question does. The continue / steer / stop / escalate policy is `jq` and stays outside the tool, which is the ADR 0013 ruling.

**What the tool lacks.** Nothing for the judging. The grace period, the retry counter, and the steering message are caller state, and the caller keeps them. A supervisor that must watch at a high rate hits the throughput limit below.

## 2. pi-warden: correct the agent instead of interrupting the person

**The decision.** Off-task change, project-rule violation, repeated failed strategy, a "done" claim with no test or build evidence.

**The command.** `annotate` for the four questions; the project rules travel either as a second pointer in the evidence or inside the question file as `@FILE`.

```sh
jq -n --rawfile rules AGENTS.md --rawfile diff <(git diff -U2) '{rules:$rules, diff:$diff}' |
thinkthen annotate warden.json --jsonl --field /rules --field /diff --details
```

**Verdict.** Covered when 0015 and 0017 land. The long rule text is exactly why the question file exists: a convention worth checking is longer than a comfortable command-line argument.

**What the tool lacks.** The steering itself. thinkthen judges and never acts, by `CLAUDE.md` and `channels.md`. The caller writes the correction.

## 3. hermes-jev-approvals: classify a flagged command

**The decision.** Approve, deny, or escalate a proposed shell command; separately, does it read a secret, does it send anything outward.

**The command.** `decide` with a band for the gate; `choose` when three named outcomes are wanted.

```sh
printf '%s\n' "$cmd" | thinkthen decide 'This command only reads files under the project.' \
  --threshold 0.2:0.8 --quiet; case $? in 0) run ;; 1) deny ;; 3) ask ;; *) deny ;; esac
```

**Verdict.** Covered today. Page 19 owns it. The author's two findings both land in the tool's own rules: word the question so yes permits the action (`channels.md`), and ask destructiveness and confidentiality as separate questions (a question set, one request).

**What the tool lacks.** Nothing. The author's real limitation, that the harness never routes some commands to the judge, is upstream of any judge.

## 4. JevLint: natural-language conventions as executable checks

**The decision.** Does this file or hunk break a written convention?

**The command.** `filter` over one record per changed file, with the rule in a question file; the exit code comes from the count.

```sh
git diff --name-only | jq -R '{path:., body:(input_filename|"")}' > /dev/null  # illustrative framing
git diff -U0 | jq -Rn '[inputs] | {hunk: join("\n")}' |
thinkthen filter @no-magic-numbers.json --jsonl --field /hunk > offenders.jsonl
[ -s offenders.jsonl ] && exit 1 || exit 0
```

**Verdict.** Covered when 0014 and 0017 land. This is part 2c below.

**What the tool lacks.** No verb sets a non-zero exit code because some record was kept. The shell line above does it, so no option is earned. The author's threshold flapping is answered by a band plus `--cache`: identical requests returned the same probability in the one live check the repository holds, so flapping is a claim to measure, not a fact to design around.

## 5. jev-lint over a knowledge base

**The decision.** Does this page contradict another page, or claim something now stale?

**The command.** Same as 4, `filter` over one record per page, with the other page carried as a second pointer.

**Verdict.** Covered when 0014 lands. It teaches nothing page 4 does not, so it earns no page of its own.

## 6. jev-review: review as a staircase of small decisions

**The decision.** Screen the risk, pick the evidence, name the defect mechanism, score the severity, route the finding.

**The command.** Four commands in a row: `score` for risk, `find` for the hunk that is the evidence, `choose` for the mechanism, `decide` for "route to a person".

```sh
git diff -U2 | thinkthen score 'How much of this change alters behavior?' none small large > risk.json
git diff -U0 | thinkthen find 'This hunk changes who may perform the action.' --lines --none
```

**Verdict.** Covered when 0014, 0015, and `find` land. `find` is the evidence-selection step the author calls the interesting choice, and the repository's live probe already measured it at a twentieth of the requests of `rank --top 1`.

**What the tool lacks.** Nothing new. The staircase is four commands in a script, which is the design.

## 7. Test adequacy

**The decision.** Do these tests establish the requested behavior? Distinct from "did the test pass", which is an execution result.

**The command.** `decide` over a record holding the requirement and the test diff.

**Verdict.** Covered when 0014 lands for a batch, today for one case. **Caution that belongs on any page that shows it:** `decide.md` records that outcome questions of this shape rejected 18% to 46% of work people had accepted, while narrow questions about a visible fact caught every planted mismatch. So the honest question is "the test file asserts on the returned status code", not "the tests are good". This is a real constraint on the use case, not on the tool.

## 8. Callstack: QA through a constrained action set

**The decision.** Which of the currently available actions to take next, and eventually pass, fail, or incomplete.

**The command.** `choose --options POINTER`, with the action list built per step by the runner.

```sh
jq -c '{screen: .snapshot, actions: .available}' step.json |
thinkthen choose 'Which action moves the booking forward?' --jsonl --field /screen --options /actions --raw
```

**Verdict.** Covered today. Page 21 is green and teaches exactly this.

## 9. agent-gate-loop: layered CI

**The decision.** After the cheap guards and the repository's own tests, does the change fulfil the issue, does it touch unrelated things, are the tests real, is the reviewer's finding worth blocking on.

**The command.** `annotate` and a policy, inside an ordinary CI job.

**Verdict.** Covered when 0015 lands. The cheap-guard-first ordering is a property of the caller's workflow and costs the tool nothing.

**What the tool lacks.** Nothing. "The action never merges" is the tool's own non-goal.

## 10. check-risk: decide how much scrutiny a change needs

**The decision.** A policy index, not a probability of failure: which checks and which reviewers this change requires.

**The command.** `score` for the judged part, `jq` to merge the deterministic facts.

```sh
git diff -U2 | thinkthen score 'How much does this change alter who may do what?' \
  'Nothing changes.' 'Behavior changes.' 'Permission changes.' |
  jq -e '. >= 1.5' >/dev/null && printf 'deep review\n'
```

**Verdict.** Covered today for the judging. Page 17 owns the shape (route by how hard it is).

## 11. jev-triage: the maintainer's next action

**The decision.** Type, severity, urgency, duplication, and what to do next: ask, investigate, decide, accept, close, wait.

**The command.** `annotate` with all three question types, then a policy that names the action and the rule that fired.

**Verdict.** Covered when 0015 lands. This is page 16, the flagship, in another costume. The uncertainty queue is the band: an unresolved answer is `null` and `null` is the review pile.

## 12. Compaction of stale tool history

**The decision.** Keep the pair, keep the call with a shortened result, or drop the pair.

**The command.** `filter` for keep-or-drop, `choose` for the three-way.

```sh
jq -c '{i:.index, text:.tool_result}' history.jsonl |
thinkthen choose 'What should happen to this tool result?' keep shorten drop --jsonl --field /text --raw
```

**Verdict.** Covered when 0014 lands. ADR 0016 already folds it into page 03 as one sentence, which is right: it teaches no new mechanism.

## 13. SkillRanker: pick the procedure for this step

**The decision.** Which skill fits, and "none of these".

**The command.** `rank --top N` to shortlist, then `find --none` over the shortlist's fuller text.

**Verdict.** Covered when 0014 and `find` land. Page 15 owns the clean "nothing fits".

## 14. LangChain: classification as a harness primitive

**The decision.** Route to a cheap or a strong model; check a tool call before it runs.

**The command.** `choose easy hard` or `score`, and `decide` with a band.

**Verdict.** Covered today. Pages 17 and 19. Their architectural point, that a decider is not a chat model, is the tool's first sentence.

## 15. jev-agent-failure-benchmark: diagnose a failed run

**The decision.** Which agent, which step, which error category.

**The command.** `choose --options POINTER` for the agent (the roster differs per trace), `find` for the step, `choose` for the category.

```sh
jq -c '{trace: .text, agents: .roster}' failure.json |
thinkthen choose 'Which agent caused the failure?' --jsonl --field /trace --options /agents --raw
```

**Verdict.** Covered when 0014, 0015, and `find` land. It reuses pages 21 and 15 and earns no page.

## New-feature ideas these use cases raise, judged under the project rule

The rule: a command or an option enters only when a how-to cannot be written without it.

| Idea | Verdict | Reason |
| --- | --- | --- |
| An exit code meaning "some record was kept" for a CI gate | **Decline** | `[ -s offenders.jsonl ]` or `jq -e` is one line, and part 2c's page is written with it |
| Line-level findings in place of file-level | **Nothing to build** | `find` names the line; `filter` over hunks narrows the record |
| A rules block inside the question set | **Decline, as ADR 0013 already recommends** | The supervisor, the approval gate, and the CI loop all want a different policy, and each is a `jq` transform or a `case`. Three callers wanting three policies is an argument for keeping policy out |
| A `--context` option for recent history | **Decline** | `tail` and several `--field` pointers build it |
| A `serve` command or daemon for a supervisor loop | **Decline** | Already ruled in `ten-use-cases.md`; a `coproc` over record mode serves a loop |
| A steering or correction output | **No fit** | The tool judges and never acts |
| Pinning and listing a model version in CI | **Held, minor** | `--model` pins it; `thinkthen models` is on the roadmap and blocks no how-to |
| A library over the pure core for Python and TypeScript harnesses | **The real gap, after version one** | Foreman, pi-warden, the compaction extension, the LangChain middleware, and SkillRanker all live inside a program. Records, recordings, exit codes, and transforms buy them nothing |

## The straight answer

**Yes, with two exceptions, and neither is a missing command or option.**

Every software-development use case in that document is either covered today or covered when tickets 0014, 0015, and 0017 land. Nothing in the list asks for a verb, an option, or a file format the specification does not already hold. The question file of ticket 0017 turns out to be load-bearing for this whole family, because a written convention or a rubric item is longer than an argument a person wants to type twice.

The two exceptions:

1. **In-process use.** Five of the fifteen patterns are extensions or middleware inside a Python or TypeScript harness. A shell binary is the wrong shape there, and the honest answer is the library over `thinkthen-core` that `ten-use-cases.md` already puts after version one.
2. **Rate.** A supervisor that judges every tool call in a fast loop pays a process start plus about a third of a second per request, with `--jobs` between 1 and 32. Batch supervision at every step, or a `coproc`, is the shape that works; ten decisions a second is not.

One caution is not a gap but must appear on any page in this family: narrow questions about visible facts hold up, and "is this good" questions do not. The measured numbers are in `decide.md` and `score.md`.

---

# Part 2. Three pages, in the form of ADR 0011 and ADR 0016

## (a) Tune a question file

**Number: 41.** ADR 0016 reserves two new pages from ticket 0017: "say what yes and no mean" and "tune a question once and use the same file in the test and in the gate". The second one **already is this page**; it should be written as this page and numbered 41, with 40 taking the true and false texts. The repository assigns no numbers to either yet, so nothing has to be renamed.

**Title.** How to tune a question file and use the same file in the gate

**Scenario, four words.** Expense claims needing receipts.

**The one command it teaches.** `decide` reading its question as `@FILE`.

**Steps.**

1. Judge twenty-four labeled claims with the question file, saving the rows: `thinkthen decide @receipt.json --jsonl --field /body --details --cache runs/v1 < tune.jsonl > run-v1.jsonl`.
2. Score the run at the file's cut with `score.jq`, and see where it is wrong.
3. Change the file, not the command: add what yes and no mean, keep the cut, judge again into `run-v2.jsonl` with a second cache folder.
4. Compare the two runs with `compare.jq`, check the winner on the held-out claims, then run the same file as the gate: `thinkthen decide @receipt.json --quiet < claim.txt`.

**What it asserts.**

- The two runs carry different `meta.question_sha256` and the same case ids, and `compare.jq` names the question as what changed.
- The second run's F1 on the held-out half, as a pinned number from the committed rows.
- `--dry-run` prints the question that resulted and names each setting's source as `file`, `command line`, or `default`, so a `--threshold` typed beside `@FILE` is visibly the winner.
- The gate exits 0 on a claim the tuned file calls yes, from the same recording.

Runs from recordings under `--replay`, depends on `jq` and the committed transforms alone.

**Closing line on an automatic tuner.** An optimizer drives this loop by rewriting step 3: the labeled cases, the metric transform, and the comparison stay exactly as they are, and each round writes one more question file and one more run, so nothing in the tool changes and every round is already traceable by its digest.

**Is a one-line `jq` conversion from jev-align's best candidate honest today?** No. Four reasons, each checked in that project's source:

1. The target does not exist. `--true`, `--false`, and the question file are ticket 0017 and are not built.
2. jev-align's saved `current_candidate` in `state.json` is an untagged union of four shapes (`BinaryTaskSpec` is `{instructions, true_criteria, false_criteria}`, multiclass is `{instructions, criteria}`, score is `{instructions, levels}`, and there is a multilabel shape). With no type field, a one-liner has to sniff keys.
3. Multilabel has no single-question home in thinkthen; it is a question set of several `decide` questions, which is `annotate`, not a question file.
4. Nothing maps to `threshold`. jev-align applies 0.5 everywhere, so a converted file arrives with its cut untuned, which is the very thing page 41 teaches.

After 0017 lands, this is honest for the binary shape alone, and the page must say the cut still has to be swept:

```sh
jq '{decide: .current_candidate.instructions, true: .current_candidate.true_criteria, false: .current_candidate.false_criteria}' \
  .jev-align/runs/<id>/state.json > receipt.json
```

## (b) Judge an agent with thinkthen in place of a language model as judge

**Which existing number owns it: 14**, "Grade a batch with a reusable definition of named checks". It is red and still written against the removed `report`, so it is rewritten over `annotate` and the transforms anyway. ADR 0016 already says 14 absorbs 29 and 31. This page is that rewrite, with the subject named: the thing being graded is an agent's work.

**Title.** How to grade an assistant's answers with a rubric instead of a second model

**Scenario, four words.** Office assistant answering policy questions.

**The one command it teaches.** `annotate`.

**Steps.**

1. The case file and the rubric: one JSON object per case with `id`, `request`, `context`, and `reply`; a question set with three narrow `decide` questions, one `choose` for the failure kind, and one `score` for severity. The grounding question names `on` as `/context` and `/reply` and never sees the trusted answer.
2. Grade every case in one pass: `thinkthen annotate rubric.json --jsonl --details --cache runs/grade --jobs 4 < cases.jsonl > graded.jsonl`.
3. Report with `counts.jq` per check: how many yes, how many no, how many unresolved, and the cost with `cost.jq`.
4. Check the judge itself against the twelve cases a person labeled, and link to page 25 for the accuracy, precision, and recall of the judge.

**What it asserts.**

- Each output row holds every input field unchanged plus exactly one new field per question, and an unresolved answer is `null` and never `false`.
- `--dry-run` prints the plan whose `input.on` shows the grounding check seeing `/context` and `/reply` and nothing else.
- The per-check counts and the run's input-token cost, as pinned numbers from the committed rows.
- The judge's agreement with the human labels on the one check that has them.

Runs from recordings, `jq` and the committed transforms only.

**Does it belong in the front window? Yes.** It is the page that answers "can I use this instead of a model as judge", which is the question a reader arrives with, and it is the cheapest honest demonstration that several questions ride in one request. It goes sixth in the window, before the flagship.

## (c) One page from software development

**The pick: lint a change by meaning and fail the build.** It beats the risk score, which page 17 already shows as routing by difficulty, and it beats the stuck check, which is page 01's pattern pointed at `tail run.log`. It shows one thing no other page shows: **many records in, a build-failing exit code out, with the offending records themselves as the report the agent reads**. It does not repeat 19, which is one command judged before it runs and fails closed, and it does not repeat 39, which is several questions about one message.

**Number: 43**, a new page in the software-development slot of the list below.

**Title.** How to lint a change by meaning and fail the build

**Scenario, four words.** House style for shared code.

**The one command it teaches.** `filter`.

**Steps.**

1. Turn the change into records, one per hunk: `git diff -U0 | jq -Rn '...' > hunks.jsonl`, shown as one line with the split rule stated.
2. Check the convention, which lives in a question file so the wording is reviewable: `thinkthen filter @no-magic-numbers.json --jsonl --field /hunk --replay recording/ < hunks.jsonl > offenders.jsonl`.
3. Make it the gate: print the offending hunks for the author or the agent, and exit 1 when the file is not empty.
4. Say what to do about a borderline hunk: run the same rule with `decide --details` instead, sweep the saved rows, and link to page 41 for tuning the wording and the cut.

**What it asserts.**

- The hunk that breaks the convention is printed byte for byte and the clean hunk is not.
- The gate exits 1 on the change with an offender and 0 on the clean change, both from the recording.
- The kept record count matches the printed report's count.
- A second run with a reworded rule file produces a different `question_sha256`, so a CI log says which rule ran.

Runs from recordings, no network, no outside project.

---

# Part 3. A crisper list

## The proposal: 20 pages

ADR 0016 keeps 27: `01, 19, 27, 18` / `04, 02, 20, 17, 21` plus two new from ticket 0017 / `03, 06, 12, 15` plus one new / `39, 07, 16` / `14, 30, 23, 13, 24, 25, 37, 28`. Below, the two ticket-0017 pages are numbered 40 (say what yes and no mean) and 41 (tune a question file), the new record page is 42 (serve a loop from one long-lived process), and 43 is the software-development page of part 2c.

### Kept, with what each one owns

| # | Page | Owns |
| --- | --- | --- |
| 01 | Gate a script step on a yes/no answer | `decide`, `--quiet`, the exit code as a shell test |
| 19 | Gate a risky command and fail closed | the band, all four outcomes, `case $?`, `--details` on a gate |
| 27 | Test a script with no network | `--record`, `--replay` |
| 18 | Compare two deciders on one recording | `--url`, `--model`, `--timeout`, `--max-retries`, replaceability |
| 02 | Branch on a label with `choose` and `case` | `choose`, `--raw`, `--option LABEL=DESCRIPTION`, sorting a folder |
| 40 | Say what yes and no mean | `--true`, `--false`, and telling "not stated" from "false" |
| 17 | Route a request by how hard it is | `score`, levels, `jq -e` as the cut |
| 21 | Pick the next action from a list that changes at every step | `choose --options POINTER`, a label map inside the record |
| 41 | Tune a question file and use the same file in the gate | `@FILE`, precedence, `--dry-run` naming each source, `compare.jq` |
| 03 | Keep only the records that match a meaning | `filter`, `--field`, `--dry-run` as the disclosure proof, compaction in a sentence |
| 06 | Put the best matches first | `rank --top N`, several `--field` pointers for a query |
| 12 | Resume a long run that stopped | `--cache`, `--jobs`, `--input`, the prefix after a failure |
| 15 | Find the line that answers a question | `find`, `--none`, `--lines` |
| 39 | Screen one message for several hazards at once | a question set, all three question types in one request, the checklist rule |
| 16 | Build a triage pipeline that drafts, blocks, or asks a person | the flagship: `annotate`, a policy transform, audit rows, three output files |
| 14 | Grade an assistant's answers with a rubric | `annotate` over records, `on` pointers, `counts.jq`, part 2b |
| 13 | Pick a threshold from labeled cases | `sweep.jq`, `band.jq`, the holdout split |
| 25 | Check the judge against human labels | `score.jq`, `calibration.jq`, hostile and hard cases |
| 28 | Know what a run cost | `cost.jq`, `meta.usage` |
| 43 | Lint a change by meaning and fail the build | `filter` as a CI gate, part 2c |

### Merged, and where each idea lands

| Page | Into | What moves |
| --- | --- | --- |
| 20 (not stated or false) | **40** | The whole scenario. What yes and no mean is the fix for "not stated", so one page teaches both |
| 24 (compare two runs) | **41** | `compare.jq` is step 4 of tuning. A comparison with nothing to compare was always a thin page |
| 07 (judged columns) | **14** and **16** | Several judged columns per record and the name clash go to 14; the spreadsheet view goes to 16's output files |
| 30 (mix exact and judged checks) | **14** | One sentence: an exact check is a `jq` field on the record, by ADR 0008 item 6 |
| 23 (a run that can be traced and replayed) | **27** and **14** | The recording is 27; the row carrying the model version and the digests is 14 |
| 37 (hard and hostile cases) | **25** | Validating the judge and testing it with hostile cases are the same page. The measured injection numbers stay in `decide.md` |
| 04 (act only when sure) | **19**, **16**, **13** | The band goes to 19, the review pile to 16, the coverage-against-accuracy trade to 13's band section |
| 42 (serve a loop from a long-lived process) | **12** | One sentence and a `coproc` line. It teaches no command and no option |

### Dropped outright, and why nobody misses them

- **04**: four ideas, none of them its own. Everything in it is taught better elsewhere; ADR 0016 had already stripped it to three piles, and the three piles are the band.
- **07**: green-adjacent and useful, but after 14 and 16 exist it is the same command with a duller purpose.
- **23**: it was a checklist of things other pages already do. A reader who has run 27 and 14 has a traceable run and does not know it needs a page.
- **30**: one sentence, not a page.
- **37**: the hostile case belongs beside the human labels, not in a page of its own, because both answer "should I trust this judge".
- **42**: a shell trick. `ten-use-cases.md` already declined a `serve` command; a page for the workaround teaches the tool nothing.
- Already gone under ADR 0016 and staying gone: 05, 08, 09, 22, 26, 29, 31, 32, 34, 35, 36, 38.

I am willing to drop **04**, which is green today, and to rewrite **02** and **17** rather than keep their current scenarios. Ian can overturn any of it.

### Matrix 1: every command and option against its page

| Command or option | Page |
| --- | --- |
| `decide` | 01, 19, 40, 41, 13 |
| `choose` | 02, 21 |
| `score` | 17, and a severity column in 14 and 16 |
| `filter` | 03, 43 |
| `rank`, `--top N` | 06 |
| `annotate`, question set | 39, 16, 14 |
| `find`, `--none` | 15 |
| `--threshold`, single cut | 03, 13, 43 |
| `--threshold`, band | 19, 13 |
| `--quiet` | 01 |
| `--raw` | 02 |
| `--details` | 19, 14, 13, 28 |
| `--dry-run` | 03 (what leaves the machine), 41 (the source of each setting) |
| `--input` | 12 |
| `--lines` | 15 |
| `--jsonl` | 03, 06, 21, 14, 16, 43 |
| `--field`, one pointer | 03 |
| `--field`, several pointers; `on` | 06, 14 |
| `--options POINTER` | 21 |
| `--record`, `--replay` | 27 |
| `--cache` | 12 |
| `--jobs N` | 12, 14 |
| `--url`, `--model` | 18 |
| `--timeout`, `--max-retries` | 18 |
| `@FILE` question file, precedence | 41, 43 |
| `--true`, `--false` | 40, 41 |
| `--option LABEL=DESCRIPTION` | 02 |
| `sweep.jq`, `band.jq` | 13 |
| `counts.jq`, `score.jq`, `calibration.jq` | 25, 14 |
| `compare.jq` | 41 |
| `cost.jq` | 28 |
| a policy transform | 16 |

No empty row.

### Matrix 2: every use case against its page

The ten from `ten-use-cases.md`:

| # | Use case | Page |
| --- | --- | --- |
| 1 | Real-time loops, ten decisions a second | **None, on purpose.** The README says it is out of reach, and ticket 0017 adds that section |
| 2 | Picking the next action in a multi-step run | 21 |
| 3 | Gating a tool call as safe or unsafe | 19 |
| 4 | Routing to a cheap or a strong model | 17 |
| 5 | Goal and stuck checks in a loop | 01, with `tail run.log` named in one line; also 16's supervisor policy |
| 6 | Context compaction by keep or delete | 03 |
| 7 | Picking a skill or tool from a long list | 15, shortlisted by 06 |
| 8 | Guardrails on outputs and traces | 39 |
| 9 | Ticket and email triage at volume | 16, with 12 for the volume |
| 10 | Reranking retrieved passages | 06 |

Software development, from part 1:

| Pattern | Page |
| --- | --- |
| Supervising a coding agent (Foreman, pi-warden) | 16, with 14 for measuring the supervisor |
| Permission and approval classification | 19 |
| Semantic linting of code and of documents | 43 |
| Staged code review, evidence selection | 43 and 15 |
| Test adequacy | 14 |
| CI gate over an agent's change | 43 |
| Risk-based scrutiny | 17 |
| Maintainer triage and next action | 16 |
| Transcript compaction | 03 |
| QA over a constrained action set | 21 |
| Diagnosing a failed multi-agent run | 15 and 21 |

Evals and tuning:

| Need | Page |
| --- | --- |
| Structured cases with stable ids | 14 |
| A reusable definition of named checks | 14, 39 |
| Detailed results that keep no, unsure, missing, and failed apart | 19, 14 |
| A run that can be traced and replayed | 27, 12 |
| Local reporting | 14, 25, 28 |
| Picking and changing a threshold | 13 |
| Comparing two runs | 41 |
| Validating the judge, including hostile cases | 25 |
| Tuning the question itself | 41, 40 |
| Comparing two decider models | 18 |

### The front window of seven, simplest to strongest

| Order | # | Page | Why it is here |
| --- | --- | --- | --- |
| 1 | 01 | Gate a script step on a yes/no answer | The first command, an `if`, an exit code |
| 2 | 02 | Branch on a label with `choose` and `case` | The second question type, still one file in |
| 3 | 03 | Keep only the records that match a meaning | The step up to many records, and `--field` as the boundary |
| 4 | 06 | Put the best matches first | Ordering, and a query carried in the record |
| 5 | 43 | Lint a change by meaning and fail the build | The first page where the tool earns a place in someone's build |
| 6 | 14 | Grade an assistant's answers with a rubric | Evals, several questions in one request, and the judge checked against people |
| 7 | 16 | Build a triage pipeline that drafts, blocks, or asks a person | The flagship: all three question types, a policy, audit rows |

All three question types appear (yes/no in 01, a pick in 02, a placement inside the rubrics of 14 and 16). The window no longer shows every command: `find` (15) and `score` on its own (17) sit just outside it. That is a deliberate trade. The window's job is the strongest arc a newcomer can read in order, and the full list of 20 carries the guarantee that every command and option has a page.

### What this changes in the repository

ADR 0016's list and front window, `documentation-plan.md`, `demos/README.md`, and the README's window all move together. Ticket 0018 is the ticket that would carry it, and its scope grows by the drops and the two merges above. No behavior of the binary changes, and no new command or option enters.
