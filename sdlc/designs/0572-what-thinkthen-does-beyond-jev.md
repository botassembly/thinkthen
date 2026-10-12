# 0572: What ThinkThen does beyond Jev

A read-only deep dive of the specification, ADRs, experiments and engine on 2026-10-12, for [ticket 0572](../tickets/0572-refresh-the-docs-to-sell-one-idea.md). Jev gives three raw question types. This file ranks what ThinkThen adds, so the docs explain and sell the right things. Measurements come from the cited specs, ADRs and experiments; most are single runs.

## Lead the docs with these

1. A decision your code can branch on: the not-sure band and exit codes 0, 1 and 3 (items 3, 4).
2. Pay once: packing many records into one request, and a saved-answer key per question (items 1, 2).
3. Test offline: record and replay with no key and no network (item 5).
4. Measure and tune for free: re-read saved answers with a new cut, `runs audit`, and "a high probability is not proof" (items 7, 50).
5. See before you send: `--plan` (item 8).
6. Structure from decisions: `recognize`, `find` with `none`, `relate`, each with its numbers (items 6, 9, 15).
7. Your data and keys stay put: the field pointer, key secrecy, and it never acts (items 11, 20, 41).
8. One engine in any language, on any backend you can swap and check, at about 1.7 ms of overhead (items 10, 18, 19).

Fast, cheap and no labelled data are Jev's merits, not ThinkThen's. ThinkThen's speed story is fewer requests: one request for 306 titles instead of 306.

## What is hard

- Packing records without them leaking into each other: the first packed form moved 11 and 12 answers with record order; quoting each record inside its own question fixed it (ADR 0055).
- Keeping saved answers valid however requests pack: requests close on size, a 50 ms pause and a 4,096-record cap, and each question keeps its own key (`records.md:106-114`, ADR 0111).
- Model noise: inside a not-sure band answers flipped 5 to 14% between identical runs, outside it 0.5 to 2%; a slower method counts as better only if it beats that noise (`threshold.md:43-45`, ADR 0054).
- One engine behind 24 surfaces that count text positions, booleans and deadlines differently (`recognize.md:103-112`, ADR 0041).
- Secrets on every path: debug output, cross-host redirects, recording permissions, passwords in addresses (`sdlc/planning/review-lessons.md`).
- Structure from yes/no and pick-one answers: valid name-boundary sequences in `recognize`, pair counts growing with the square in `relate`, with hard caps (255 names, 4,000 pairs).
- Honest token and money counts before a send, from measured bytes-per-token rates (`channels.md:110`, ADR 0108).
- Cache correctness across model aliases, versions, provider storage rules and key migrations (`cache.md:37-77`).
- Open: throughput. On 100,000 AG News rows the client kept 2.5 to 3.3 requests in flight and took 60 s (`thinkthen-exp/experiments/0017-agnews-100k-timing/`).
- Open: pages drifting from the program (`review-lessons.md:43`).

## The ranked list

Ranked by user gain over calling Jev directly, then by how clever the idea is. "0.1" shipped in 0.1.x; "0.2" is on main; "(?)" is unconfirmed.

1. **Pack many records into one request.** 306 titles in one request, 11,468 input tokens against 88,933 one at a time, accuracy within run noise. 0.1. ADR 0055; `rank.md:17`; `overhead.json`.
2. **A saved-answer key per question, whatever shares its request.** Add 20 records to 100 and only the 20 are asked. 0.1. `records.md:106-114`, ADR 0111.
3. **A not-sure band in the exit code.** `--threshold 0.1:0.9` gives yes, no or not sure; exit 0, 1, 3 let a shell route the middle to a person. 0.1. `threshold.md`, `channels.md:52-94`.
4. **A failed call never looks like an answer.** No failure becomes false, other or zero. 0.1. `channels.md:54-67`, `annotate.md:50`.
5. **Record and replay for offline tests.** `--replay DIR` needs no key or network; a miss stops with exit 5 and names the key. 0.1. `recording.md:5-17`.
6. **`recognize` built from many small decisions.** A five-way position question per piece, a best-path decode, then kind and edge questions; F1 0.865 on the repo's keys. 0.1; caller-defined kinds 0.2. `recognize.md`, `core/recognize/bilou.rs`.
7. **Re-read saved answers for free.** Change the cut with zero sends; `runs audit` tunes on half a labelled set and checks on the other across 20 splits, and can save the cut. 0.1. `audit.md`.
8. **`--plan` before sending.** The exact body, the key variable's name, request count, token range and where each setting came from; sends nothing. 0.1. `channels.md:96-136`, ADR 0123.
9. **`find` in one request, with `none`.** 20 requests and 11,063 tokens against 239 and 69,143 for `rank --top 1`; `none` right on 4 of 4 blank documents. At most 255 units. 0.1. `find.md:27-37`.
10. **One Rust engine under every binding.** Core overhead 1.74 ms of a 138 ms call; bindings add about 1 to 2 ms. 0.1. `overhead.json`.
11. **The field pointer decides what leaves the machine.** `--field /body` sends only that value. 0.1. `records.md:57-85`.
12. **A content digest per question.** `question_sha256` from a canonical form, threshold included. 0.1. `question-file.md:141-198`.
13. **A tuned cut remembers its batch size,** and warns when run at another. 0.1. `records.md:118`, ADR 0085.
14. **Shared and per-record context.** Putting the catalog in the context took Jev from 68% to about 97% on 1,075 Beatles questions. Shared 0.1; per-record 0.2. `records.md:98`.
15. **`relate` by pairs or one menu per source.** The menu raised precision from 0.420 to 0.749 for one-answer relations. 0.1 (?). `relate.md`.
16. **Relations stated in the text.** `recognize --relation` asks only rule-allowed pairs; precision 0.973, recall 0.783. Beta. `relate.md:60`.
17. **Ask only what can matter.** No question goes out whose answer cannot reach the output. 0.1. ADR 0054.
18. **Swap backends and provider types.** TypeSafe, Liquid, Perplexity, OpenRouter, Ollama, llama.cpp, MLX; OpenAI Decisions in 0.2. `backends.md`.
19. **`backends check`.** Fixed probes and small calls through all ten functions before you rely on a provider. 0.1; ten-function calls 0.2. `check.md`.
20. **Key secrecy.** Never in a plan, result, recording or error; keys in addresses refused; no redirects followed. 0.1. `backends.md:38-47`.
21. **One row, one model version.** `filter` and `rank` refuse mixed versions; the cache skips another model's answers. 0.1, tightened 0.2. `result.md:413-415`.
22. **Mutable aliases and provider storage rules** are honored. 0.1 and 0.2. `cache.md:43-65`.
23. **Split a too-big request in half once** on a size failure. 0.1. ADR 0051.
24. **`annotate` question sets** pack records and return flat fields. 0.1. `annotate.md`.
25. **Rank by probability, no pairwise calls,** with bounded memory under `--top`. 0.1. `rank.md:21-25`.
26. **Rank question sets merged by turns,** never comparing probabilities across questions. 0.2 (?). `rank.md:27-49`.
27. **`score` computes its own weighted position,** ignoring a vendor field that differed on 13 of 20 songs. 0.1. `score.md:27-35`.
28. **`tag` asks one yes/no per label,** each with its own cut. 0.1.
29. **Files and folders keep line positions,** and paths never enter evidence or cache keys. 0.2. `files.md`.
30. **grep-style display:** `-n`, `--around`, `--scores`, `--files-only`. 0.2.
31. **Clean stops and deadlines.** SIGINT finishes sent requests and prints completed output; one spelling of "no deadline" everywhere. 0.1. ADR 0041.
32. **Count-only usage totals** across every surface, no prompt or key stored. 0.1. `recording.md:32-42`.
33. **A spend limit before sending, and your own price.** 0.1; price pair (?). ADR 0108.
34. **`runs diff`:** which answers changed between runs or cuts. 0.1.
35. **A local MCP server with the ten function tools.** 0.2.
36. **Images on decide, choose and score,** refused elsewhere before reading files. 0.2.
37. **Per-record option shortlists for `choose`.** 0.1 CLI; native 0.2.
38. **Resume a failed run:** only failed questions are asked again. 0.1.
39. **A damaged cache never costs money;** migrations in one transaction. 0.2; cleanup 0.1. `cache.md:67-77`.
40. **Strict input reading:** no format guessing, duplicate keys refused, size and depth limits. 0.1. `records.md:35-94`.
41. **It never acts:** runs no commands, writes only named files, treats input as data. 0.1. `channels.md:17-23`.
42. **Ordinary Unix behavior:** results only on standard output, quiet `head -1`. 0.1.
43. **Split a two-fact question into two calls:** 51 of 60 right against 22 of 60 combined. 0.1 how-to.
44. **Two homes for each setting, with the source shown.** 0.1.
45. **Saved-question names, wording versions and input declarations.** 0.2.
46. **Backend profiles check limits locally** before any send. 0.1.
47. **Stable answer IDs** that survive repacking and replay. 0.2.
48. **Per-attempt facts:** timing, status, request ID, retries. 0.1/0.2 (?).
49. **A 50 ms pause flush** so live streams keep moving and files pack the same every run. 0.1. ADR 0053.
50. **Limits stated up front:** "a probability of 0.98 says nothing about absent facts"; the model approved every case at 0.98 where reviewers refused 23%. 0.1. `result.md:409-411`, refusals page.
