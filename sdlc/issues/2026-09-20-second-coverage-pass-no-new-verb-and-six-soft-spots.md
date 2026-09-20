# Second coverage pass: no new verb is needed, and six soft spots remain

Status: Open

Ian asked on 2026-09-20 whether the seven verbs can do everything in two of his notes, or whether a function is missing. The first note lists twenty proposed semantic shell commands with a shared contract. The second is an outside article that surveys what people built in the first week of a public System One model: routing, context filtering, tool gating, worker supervision, fast control loops, fuzzy data queries, and a pile of odd projects. This pass follows the earlier one in `2026-09-20-use-cases-from-the-notes-that-no-page-teaches.md`.

## The finding

No job in either note needs an eighth verb. Every job lands in one of four places: a verb does it, a pipe of verbs and plain tools does it, a library does it, or the tool refuses it on purpose because it writes no text and computes no embeddings. The roadmap already weighed most of the flags these notes ask for, and its bar still holds: an option enters when a demo cannot be written without it.

## The article, pattern by pattern

| Pattern | What people built | How the verbs do it |
| --- | --- | --- |
| Routing | Pick a model, an agent, a tool, or an HTTP route by meaning | `choose`, and `choose --options POINTER` when the candidates travel in the record. Routing inside a web server is a library job |
| Context filtering | Judge tool results before an agent reads them, pick which files to open, decide what survives compaction word for word, rerank search hits | `filter`, `rank --top N`, and `score`. "Survives word for word" is the promise `filter` already makes: a kept record is written back byte for byte |
| Tool gating | Allow, ask, or deny a tool call. Check a tool result for an injected instruction | `decide` with a band. Yes is allow, no is deny, and unresolved is ask. The three exit codes are the three policies. Enforcement stays in the host's code, which matches the rule that the tool judges and never acts |
| Worker supervision | Several standing questions about one worker: is it stuck, is it drifting, did the tests pass | `annotate` with a question set over one document, then plain code reads the fields and decides |
| Fast control loops | Browser steps, computer use, games. Build the legal moves, pick one, repeat | `choose --options POINTER` in record mode through one long-lived process, which ticket 0024 landed, or a library. A fresh process per step is too slow for this |
| Fuzzy data queries | A yes or no, a label, or a rank inside SQL and graph queries | `psql` or `duckdb` writes JSON lines, then `filter`, `rank`, or `annotate --jsonl`. Walking a graph by choosing which edge to follow is `choose --options POINTER` in a loop, which is a library job |
| Everything else | Approve a pull request, block ads, strip page clutter, ask one question of every function in a codebase, score every sentence of a transcript | `decide` over a diff. `filter` over page blocks, which is a job for the JavaScript build. `filter --jsonl` after a code tool splits the functions. `score --lines` after a sentence splitter |

## The twenty proposed commands

| Proposed | Verdict |
| --- | --- |
| grep, rank, check | `filter`, `rank`, and `decide` with a band |
| extract | `find` for one unit, `filter` over units for many |
| classify, with many labels and abstain | `choose` with a threshold for one label. `annotate` with one `decide` per label for many. `how-to-portfolio-study.md` already ruled this way |
| verify, relate | `choose` over a pair, with labels such as supported, contradicted, mixed, and insufficient. `--field` takes several pointers, so both sides travel as one evidence object |
| diff, comm | Pair the records upstream, then `decide` whether each pair says the same thing |
| normalize, join | `choose` or `find` against the reference list, up to 255 entries |
| uniq | A shell loop: `find --none` asks whether a new record repeats one already kept. It costs one request per record and holds up to 255 kept records |
| split | Held as `segment`. The windowed recipe covers it |
| outliers with a baseline | `rank` on a question about what is unusual. The baseline has to ride inside each record, because `--context FILE` is held |
| calibrate | The sweep transform, then the chosen threshold goes into the question file |
| cluster, sample, similarity mode | Out. They need embeddings |
| summarize, rewrite, redact a span | Out. The tool writes no text |

The shared contract in that note is already met: records pass through untouched so ids survive, three-way decisions exist, `--cache` exists, and the cost transform exists. Two parts are met by a pipe: a separate file for the uncertain rows is `decide --jsonl --details` and a `jq` split, and "which span supports this" is a `find` after the `decide`.

## The six soft spots

None of these is a missing verb. Each is a place where a new user will stumble.

1. **The 255 ceiling on two-list jobs.** Normalize, join, and uniq stop at 255 candidates. A longer list needs a cheap first cut upstream, by `grep`, by a database, or by an embedding search. The roadmap holds the two-pass `find`. A recipe page should show the first cut.
2. **Splitting text into units.** The verbs read lines or JSON lines. Real text comes as paragraphs, sentences, and functions. Paragraphs are one `awk` line. Sentences and functions need a named neighbor tool. This is the most likely first request for a new flag after launch. It stays a recipe until a demo cannot be written.
3. **Many labels read back as a list.** `annotate` writes one true or false field per label. A user who wants `["billing", "urgent"]` needs one `jq` line. The page for many labels should end with it.
4. **Agent hosts read exit codes their own way.** Tool gating is the clearest pattern in the article, and the three exit codes fit it exactly. Each host still needs a small mapping. One coding agent's hook contract blocks a call on exit 2, and this tool uses exit 1 for a no and exit 2 for a usage error. The how-to for a host hook has to show the three-line `case` that maps them. Check each host's contract when the page is written.
5. **Speed inside a loop.** A control loop cannot pay for a new process on every step. The answer is one long-lived record-mode process or a library. Ticket 0024 landed the loop and the README names it. No how-to under `demos/` shows it.
6. **Text only.** The computer-use projects read a screen. This tool sends text and nothing else, and the specification never mentions an image. A user has to turn the screen into text first. This is a line for the refusals page.

## Two things the article suggests beyond this tool

- A database extension is a separate product over the same core. The pipe covers a script. It cannot put a judgment in the middle of a query plan.
- The JavaScript build makes browser jobs possible. A key inside a browser extension is a disclosure problem, and the library documents have to say so.

## What Ian can overturn

The verdict that no verb is added. The strongest case for an eighth verb is a many-label `tag`. This issue first recommended against it. Ian asked for it later the same day, and a check of the vendor documents showed that every label rides in one request. The recommendation reversed. See `2026-09-20-tag-a-fourth-question-type-for-many-labels.md`. Every job in the two notes can still be done without it.
