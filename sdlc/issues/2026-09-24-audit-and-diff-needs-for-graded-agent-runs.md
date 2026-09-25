# What audit and diff would need to grade agent runs

Status: Open. Tracked for after 0.1. Filed by Claude, the owner, on 2026-09-24.

botassembly wants to grade its agent runs with `thinkthen audit` and `thinkthen diff`. It would need four things beyond today's prototype. Ian's ruling holds: 0.1's audit and diff match the prototype measurement script and its goldens (`sdlc/issues/2026-09-24-audit-and-diff-move-into-0-1.md`). This issue adds nothing to 0.1. It records what ThinkThen already provides and checks that tickets 0113 and 0114 leave room to add each need later.

## The four needs

### 1. Graded scores

botassembly would grade on a scale, beyond right, wrong, or not sure.

- Already provided: the `score` verb places evidence on 2 to 10 named levels and prints a number from 0 to the top level (`specification/score.md`). A detailed score row carries `answer.kind` `score`, the level, and each level's probability (`specification/result.md`). `transforms/score/` reads accuracy, precision, recall, and F1 at a cut. `transforms/trials/` and `transforms/sweep/` read score rows too.
- 0113 and 0114 as designed: audit grades `decide` and `choose` only and refuses another verb with a usage error (0113, "Inputs" and "Failures"). The outcome enum holds right, wrong, tied, and unresolved. Neither blocks a later score path. Accepting `score` later turns a refusal into a new group kind with its own members. No 0.1 output changes.

### 2. Cost and latency per row

botassembly would weigh each graded row by what it cost and how long it took.

- Already provided for cost: every detailed row carries `meta.usage` (token counts), `meta.requests_sent`, and `meta.cached` (`specification/result.md`, ADR 0036). `transforms/cost/` prices input tokens and sets cached rows apart. ADR 0034 keeps count-only usage totals.
- Not provided for latency: no row carries a time. The pure core reads no clock (`CLAUDE.md`, "The pure core"). A latency member would need timing at the engine edge and a result-row addition, which is its own ticket.
- 0113 and 0114 as designed: both read only the members they grade and ignore `meta`. The 249 fixtures carry full `meta` objects, so the result reader already accepts members it does not use. Neither blocks a later cost summary, as long as the key reader also accepts members it does not use. See the builder notes below.

### 3. Repeated samples per case

botassembly would run each case several times and grade the spread.

- Already provided: `transforms/trials/` averages repeated observations once per case before a metric, and it prints one `thinkthen.trials/1` row per id with a trial count (`transforms/README.md`, "Average repeated trials once per case"). The metric transforms refuse repeated ids until that step runs.
- 0113 and 0114 as designed: audit refuses the same answer name, record id, and text twice (0113, "Duplicates"). diff refuses a record twice under one answer name (0114, decision 6). Today a caller averages with `trials` first. Neither refusal blocks later support, because relaxing a refusal changes no accepted input. Grading the spread later means a sample key joins the duplicate identity. The builder notes keep that identity in one place.

### 4. Run identity

botassembly would record which question file, data, and engine produced a run.

- Already provided: each detailed row carries `meta.tool` (name and version), `meta.question_sha256` or `meta.questions_sha256` for annotate, `meta.url`, `meta.model`, and `meta.requests`, the recording digests (`specification/result.md`). ADR 0021 makes `transforms/compare/` check the question digest and the evidence before it counts a pair. Data identity is the `input` each row carries. No member names the input file or digests the whole data set.
- 0113 and 0114 as designed: neither prints run identity. diff pairs by answer name and record id and points to `compare` for identity (0114, decision 3). Output rows carry no `schema` member (0113 and 0114, decision 2). Neither blocks a later identity summary, as long as the pages say that later versions may add members. See the builder notes below.

## What keeps the formats open

Builder notes on the 0113 and 0114 branches ask for three things. None changes 0.1 behavior or a golden.

- The key reader and the result reader ignore members they do not use. A test pins an extra member in each.
- The duplicate identity lives in one function in the core, so a sample key can join it later.
- `specification/audit.md` and `specification/diff.md` say that readers of audit and diff output should ignore members they do not know, because later versions may add them.

## Lever

Ian can overturn this triage. A ticket for any need opens after 0.1 ships, when botassembly asks for it.
