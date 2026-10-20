---
date: 2026-10-13
status: draft PRD
source: Ian's rulings of 2026-10-13 in `functions.md` and `data.md`; PRD 00 and its questions; ThinkThen 0.2's `specification/rank.md`, `find.md`, `choose.md`, `score.md`, `backends.md`, `roadmap.md`, `thinkthen help rank` and `help find`, and the find request fixture in `libraries/c/tests/door/golden.rs`; Ian's pairwise-rank idea; the clippings on jsort, jev-reranker and Jev as a reranker; Ian's data-model and fixed-and-tunable notes; the papers listed at the end
---

# PRD 03 working file: the hard questions for rank, find and the dial

This file backs [03-rank-and-find.md](03-rank-and-find.md). Each question gives the options, the reasoning and a recommendation. **Ian** marks the four that need his direction. Claims about 0.2 cite the page in [specification/](../../../../specification/). Numbers marked "about" are estimates for planning, not measurements.

## What 0.2 does today

- `rank` judges each record alone and sorts by the yes probability, or by the weighted level of a saved `score` question. Its help says it "never compares two records" and "never runs a tournament" ([rank.md](../../../../specification/rank.md), Cautions). Records share requests: each record rides inside its own question beside one fixed evidence sentence. On 306 Beatles titles this sent 11,468 input tokens where one title a request sent 88,933. `--top` saves no request.
- `find` sends 2 to 255 units in one request, each with an id `u001` to `u255`, and the model picks one. `--none` adds a `none` option and exits 3 when it wins ([find.md](../../../../specification/find.md)). The wire request is a System One `choice` question whose `instructions` are the caller's question as typed, with the unit ids as options and the units as a JSON list in `state` (fixture `FIND` in `libraries/c/tests/door/golden.rs`).
- The roadmap holds "a two-pass `find` beyond 255 units" because it "would need a merge rule and a second measurement" ([roadmap.md](../../../../specification/roadmap.md)). This PRD supplies both.

## The wire and the model

**Q1. How does a decision model, which answers yes/no probabilities and picks, answer a pairwise question in the System One wire shape?**
A System One model takes one `state` and named questions of three types: `noul` (yes/no), `choice` (pick one) and `score` (levels) ([backends.md](../../../../specification/backends.md), the `systemone` adapter). It never writes an ordering. Three forms can carry a comparison.
- (a) A `choice` between two items under the caller's question. This is exactly `find` with two units. It adds no words: the type itself asks "which one".
- (b) A `noul` question wrapping the caller's question in a fixed sentence, such as "Item A fits this better than item B". jsort uses this form and found it gives graded answers: a two-option choice "put 93% of its answers below 0.1 or above 0.9, where the noul left 28% of its answers between the two", at the same 95.6% correctness (jsort clipping, "Why a noul and not a choice").
- (c) Two `noul` questions, one per item, over a state that shows both. Each item is judged with its rival in view. This keeps an absolute yes probability but never forces a comparison.

Ian ruled that the question is always the caller's own and that no built-in "which is better" exists (`functions.md`). Form (a) honors that with no added text and reuses `find`'s wire, cache and result shapes. Its risk is saturated probabilities, which give a fit less to work with and may need more comparisons. Recommendation: (a) is the product form. The experiment runs (b) and (c) as arms on the same pairs. If (b) wins clearly, Q2 goes to Ian.

**Q2. If a fixed comparison sentence wins, may ThinkThen send it? (Ian)**
Form (b) puts ThinkThen's words around the caller's question. The caller still owns the question; ThinkThen owns one sentence that says "compare these two under it". The ruling forbids replacing the caller's question, and it is unclear whether a wrapper counts. Options: never wrap, and accept a weaker fit; wrap only when the experiment shows a clear gain; let the caller choose. Recommendation: wrap only if (b) beats (a) by the ship margin in Q27, show the exact sentence in `--plan` and `--details`, and fix it per release so stored answers stay comparable.

**Q3. How do pair questions pack into requests?**
A pair is small, and a tournament asks hundreds. Three packings exist.
- One pair per request, in `find`'s form. Simple, but 500 requests for 100 items, and about ten rounds each as slow as its slowest call.
- Many pairs per request, each pair self-contained: the two items travel as the two options' descriptions, beside 0.2's fixed evidence sentence. This mirrors how 0.2 batches records into questions (ADR 0111, [rank.md](../../../../specification/rank.md)). Each item's text repeats in each of its pairs.
- A window's items once in `state`, and many pair questions naming two ids each. This is the cheapest in tokens, but each pair is judged with the rest of the window in view.
The Perplexity route bills the state once per question ([backends.md](../../../../specification/backends.md)), so the third form saves nothing there. 0.2's batching moved "a few answers", and pairs may move more. Recommendation: the experiment compares the first two forms on the same pairs. Ship the packed form if it holds accuracy within the noise of repeats; otherwise one pair per request with `--jobs` concurrency.

**Q4. How does ThinkThen handle position bias?**
Pairwise LLM judges favor one position (Wang and others, 2023), and swapping the order flips some verdicts (Zeng and others, 2024). 0.2's `choose` saw 2 of 60 picks move when its list reversed ([choose.md](../../../../specification/choose.md), Cautions). Options: ask every pair in both orders and average (Qin and others, 2023), at twice the cost; or show each item first in half its comparisons and fit a first-position lean as a home advantage, as jsort does, which reported a lean of a few points. Recommendation: `all-pairs` asks both orders, because small sets can afford it. `tournament` balances positions and fits the lean, and `--details` and run facts report it. `window` shuffles positions between overlapping windows, which fights the middle-position loss reported for long contexts (Liu and others, 2023).

**Q5. What happens with ties and cycles?**
A decision model returns a probability, so a 0.5 answer is a tie and needs no special rule. A fractional fit treats 0.7 as seven-tenths of a win, which jsort already does with robust errors. Cycles (A over B, B over C, C over A) are real (Zeng and others, 2024). A fit gives the best single order through them, and split-half reliability falls when they are common. Recommendation: no tie option on the wire, since Davidson's tie model (1970) adds a parameter the experiment does not need. Report reliability; below 0.8, say so on standard error as jsort does. Exact score ties keep input order, as 0.2 does at every `--top` boundary.

## The score

**Q6. Which method turns many comparisons into one score from 0 to 1?**
Ian asked to test maximum, minimum, mean, median, Elo and fancier methods (`functions.md`).
- Mean, median, maximum and minimum of an item's pair results ignore who it faced. Under a schedule that pairs items by standing, a strong item meets strong items and its mean drifts toward 0.5.
- Elo updates in sequence, so its result depends on the order of games. A study of Elo for model evaluation found it unstable for exactly that reason (Boubdir and others, 2023), and Chatbot Arena moved to a Bradley-Terry fit (Chiang and others, 2024).
- Bradley-Terry (1952) fits one strength per item from all comparisons at once and accounts for opponents. Luce's choice rule (1959) and the Plackett-Luce model (1975) extend it to picks from groups, so one fit reads pairs and windows alike.
- Strengths are relative. They cannot say "nothing here is good", so a threshold on them fails across pools.
- Yan and others (2024) adjust pointwise relevance labels by the least amount needed to agree with pairwise preferences. The result keeps pairwise order and pointwise meaning. Perez-Ortiz and others (2019) unify ratings and pairwise comparisons on one scale.
Recommendation: the candidate to beat is anchored Bradley-Terry. Fit Luce strengths with a position lean and a small ridge. Then fit a monotone map from strength to the `alone` pass's yes probability, and report the mapped value as the score. Every listed method is an arm, and the experiment picks.

**Q7. How does the score correct for strong competition?**
Ian's example: the tenth line of a strong batch may beat the best line of a weak batch (`functions.md`). Two corrections stack. The fit corrects order, because a win over a strong opponent raises strength more. The anchor corrects level, because the `alone` pass judged each item with no rivals. Recommendation: the experiment measures drift directly. Place the same items in pools salted with strong items and with weak ones, and report how far each method's score moves. The ship gate needs the anchored score to drift no more than `alone`.

**Q8. What does a threshold mean on the new score?**
0.2's `rank --threshold P` keeps records whose yes probability is at least P ([rank.md](../../../../specification/rank.md)). Options: refuse thresholds outside `alone`; cut on strength; cut on the anchored score. Only the anchored score sits on the yes-probability scale. Recommendation: `--threshold` cuts the score under every setting, so a cut tuned under `alone` remains a sensible starting point. The help says a cut is tuned per model and per setting, as `data-model.md` already says for models. `eval grade` grades it like any other cut.

**Q9. Is the extra `alone` pass worth its cost?**
It adds one question per item. 0.2 packs these cheaply: 306 short titles in 11,468 tokens. It buys the level for Q6 and a starting order, so the first tournament round pairs plausible neighbors instead of a random ring. Recommendation: every comparing setting runs the `alone` pass first and `--plan` counts it. The experiment also runs a tournament without it, to price what it buys.

## The settings

**Q10. What does `window` mean for a model that cannot write an ordering?**
RankGPT slides a window of 20 down the list and asks the model to reorder each window (Sun and others, 2023). Top-down partitioning improves on its cost (Parry and others, 2024). Setwise asks a model to pick the best of a small set and sorts with those picks (Zhuang and others, 2024). A decision model picks one and returns a probability for every member, which is a whole Luce observation of the group. Recommendation: `window` asks a pick-one over groups of up to 20, places each item in about three overlapping groups, shuffles positions, and fits the same model as Q6. For 100 items that is about 15 questions. The experiment tries groups of 10 and 20 and two or three groups per item.

**Q11. Which tournament schedule: sort, bracket or Swiss?**
- Sort-based pairwise ranking (PRP-Sorting, Qin and others, 2023) costs about n log n comparisons, but its count depends on the answers, and one noisy comparison can misplace an item (Braverman and Mossel, 2008).
- A knockout bracket is cheap and finds a winner, but it orders the losers badly.
- TourRank runs grouped rounds and adds up points over several tournaments, which resists input order (Chen and others, 2024).
- A Swiss schedule with refitting pairs items near each other after each round, which is where a comparison says the most. jsort uses it at ten comparisons per item and reports a plateau from six to sixteen on readability. Active ranking theory supports adaptive pairs (Jamieson and Nowak, 2011; Heckel and others, 2019).
Recommendation: Swiss, with comparisons per item fixed by the experiment, about ten. Its count is fixed in advance, which makes `--plan` exact (Q17), and its cost grows with n, not n squared. The experiment runs sort-based and TourRank-style arms for comparison.

**Q12. What does `--top N` do under comparisons?**
In 0.2 `--top` saves nothing, because every record is judged ([rank.md](../../../../specification/rank.md)). Under a tournament, an item that clearly cannot reach the top N can stop being asked. jsort's rule found more of the true top ten at the same or lower cost in simulation. Recommendation: under `tournament`, `--top` prunes after an item's third comparison, and `--plan` reports the unpruned bound. Under `alone`, `--top` keeps 0.2's meaning.

**Q13. When is `all-pairs` allowed?**
All pairs in both orders cost n(n-1) questions: 9,900 for 100 items and 249,500 for 500. PRP's all-pairs form is its most accurate and its most expensive (Qin and others, 2023). The spend caps already exist (`--max-requests-total`, `--max-estimated-input-tokens-total`), but packing makes a request count a weak guard. Recommendation: `all-pairs` refuses past 100 items and names `tournament`. It counts wins by fit like every other setting, so its scores share one meaning.

## Find

**Q14. What do the settings mean for `find`?**
`find` returns one item or none (PRD 00, Q31). Its 0.2 form is already one window holding the whole set. Recommendation:
- `window`, the default: one request up to 255 items, as in 0.2. Past 255, groups of up to 255, the top few of each group by probability carried to a final group. Requests grow as n divided by 254, plus one.
- `tournament`: windows shortlist, then the finalists meet in pairs, then a final pick. Ian expects pairwise finding to be more accurate at a cost (`functions.md`), and the experiment measures it.
- `all-pairs` among at most 100 finalists.
- `alone` is refused and names `rank --top 1`, which already does it.
Without `--compare`, a set of 256 or more refuses and names the setting, as PRD 00 Q31 says. A builder who chose `find` for one request should not get several by surprise.

**Q15. How does `find` say nothing fits under a tournament?**
In 0.2 `none` is one more option beside the units, and it won 4 of 4 blank documents and 0 of 16 answerable ones ([find.md](../../../../specification/find.md)). Options: offer `none` in every group; offer it only in the final; check the winner alone with a yes/no question. A `none` in an early group would knock out a group whose best item is weak but still the best overall. A lone check of the winner loses the comparison that makes `find` sharp. Recommendation: `none` appears only in the final group, which holds the strongest finalists, so 0.2's meaning holds: nothing in the set fits better than nothing. The exit code stays 3. The experiment builds no-answer cases by removing every relevant passage from a query's pool.

## The name, the defaults and the plan

**Q16. What is the dial called, and what are its settings? (Ian)**
The name crosses the command line, 24 languages, SQL settings, tool files and MCP (PRD 00, principle 2), so a rename later costs every surface.
- `--compare alone|window|tournament|all-pairs`. It names what the model sees and uses Ian's own words for the settings. "Compare alone" reads oddly, but it is the honest name for 0.2's behavior.
- `--method`. Plain, but it says nothing, and `rank`'s help already uses "the method is fixed".
- `--effort low|medium|high|max`. It speaks of cost and hides how, so `--plan` would have to explain it. It also cannot express `find`'s default.
- `--judge`. It collides with "judgment", which `data-model.md` uses for a right answer.
Recommendation: `--compare`, with the four values above, and the same key `compare` in every library, SQL settings object and tool file.

**Q17. What does `--plan` print for each setting, and must it be exact?**
0.2 prints the first request and a whole-input count line. Under a Swiss schedule, which pairs meet depends on answers, but how many does not. Pruning under `--top` and refusal splits can only lower or raise the count. Recommendation: `--plan` prints the setting, items, questions per stage (the `alone` pass, windows, pairs, final), rounds, and "at most" bounds on requests and estimated input tokens, using 0.2's byte estimate and the largest item. Illustrative for 100 short passages of about 75 tokens: `alone` about 10,000 tokens; `window` about 25,000; `tournament` about 100,000; `all-pairs` about 1.8 million. At OpenRouter's listed $0.042 per million for Jev ([backends.md](../../../../specification/backends.md)), the last is under a dime. The experiment checks every run against its plan.

**Q18. Do the defaults keep 0.2's behavior?**
PRD 00 principle 5 makes 0.2 behavior the default. Recommendation: `rank` defaults to `alone` and `find` to `window`, which equals 0.2 up to 255 items. A tool file may set `compare`, and the usual precedence applies: typed, environment, tool file, configuration ([settings.md](../../../../specification/settings.md)). The `rank` help line "never runs a tournament" becomes "judges each record alone unless `--compare` says otherwise".

## Storage and reuse

**Q19. How are pairwise answers stored in the database? (depends on PRD 01)**
0.2 keys each answer on the exact question as sent: adapter, address, models, evidence and question ([cache.md](../../../../specification/cache.md)). Ian ruled that the stored answer records that it compared two pieces of evidence under the caller's question (`data.md`). `fixed-and-tunable.md` already makes `find`'s whole set its item. Recommendation: a comparison is stored as a `find` answer whose evidence is the ordered set of its two items, each item keeping its own evidence id. Each order is its own answer. The schedule's seed is fixed by default, so a rerun asks the same pairs and costs nothing. A rerun reuses the pairs it already asked, and `all-pairs` reuses any pair a tournament asked in the same order. PRD 01 needs set evidence with ordered members; it needs no new answer kind.

**Q20. What happens when a reuse rule lets answers from two models enter one fit?**
PRD 00 Q18 makes the reuse rule a setting, and U21 lets a cheaper model reuse old answers. A fit across two models mixes two scales, which jsort refuses by default. Recommendation: follow the reuse rule, because rules apply on read. The result names every answering model in the fit and warns when there are two or more. A strict rule re-asks.

**Q21. Do right answers for comparisons need their own table?**
`evals.md` (a lead's draft, not a ruling) lists "a comparison of two items as its own kind of answer and of right answer". The datasets grade each item, and any pair's right answer follows from two item grades. A pairwise right answer is only needed when people judge pairs directly. Recommendation: in 0.3, right answers stay per item, as grades or yes/no, and ordering measures (U34) read them. Pairwise right answers wait for a user who needs them. This diverges from the `evals.md` draft and saves PRD 01 a table.

## Inputs and outputs

**Q22. What do saved score questions, question sets, `--true`, `--false` and `--context` do under comparisons?**
- A saved `score` question: the comparison sends its question text, and the `alone` pass with its levels sets the anchor, its weighted value divided by the number of levels minus one. Turbopuffer found a ten-level score the best way to rerank with Jev (clipping "Jev is a state-of-the-art search reranker"), so it is also the strongest baseline.
- Question sets: `alone` only in 0.3; others refuse before sending.
- `--true` and `--false` shape the `alone` pass. A pick-one has no place for them, so comparisons send the question alone. An experiment arm sends them as shared context.
- `--context` rides once as shared evidence, as in 0.2.
- 0.2's pattern of copying the query into every record with jq ([rank.md](../../../../specification/rank.md), "Ranking against a query") would show the query twice in every pair. The docs move the query into the question.

**Q23. What do `--details` and SQL show?**
A pair answer per row would make output grow with comparisons. Recommendation: each row's answer carries the score, the strength, its standard error, the comparisons it took part in, the setting and the fit's reliability. Pair answers stay in the database, linked by id. SQL's `thinkthen_rank` today returns `key`, `rank` and `probability` ([rank.md](../../../../specification/rank.md), SQL). Rename `probability` to `score` on every surface, since 0.3 keeps no backward compatibility before 1.0 (PRD 00, Q3) and `--scores` already says "score". Under `alone` the score equals the yes probability.

**Q24. Where does the comparison loop run when a proxy is in use?**
`proxy.md` says `rank` sends scores in chunks and sorts on the client, and `find` sends the whole set at once. Principle 6 bars the proxy from adding a behavior a local run cannot reproduce. A tournament needs a fit between rounds. Recommendation: the schedule and the fit run in the caller's SDK. Each round is an ordinary call through the proxy and lands in the proxy's store. A run takes about ten round trips, and `--plan` names the rounds so latency is no surprise.

## The experiment and the gate

**Q25. Do TREC Deep Learning and BEIR meet the need? (Ian asked for a report if they fail)**
TREC Deep Learning 2019 and 2020 have 43 and 54 judged passage queries with grades from 0 to 3 and deep pools (Craswell and others, 2020 and 2021). Reranking BM25's top 100 is the shared setup of RankGPT, PRP, Setwise and TourRank, so results compare with published ones, and BM25 alone scores about 0.51 and 0.48 nDCG@10. The top 1,000 candidates test `find` past 255. BEIR adds 18 domains (Thakur and others, 2021). Five small sets fit: TREC-COVID, NFCorpus, SciFact, Touché-2020 and DBPedia-entity, about 50 queries each. They meet ordering, threshold transfer (grade 2 or more counts as relevant) and no-answer tests. They miss one thing: the caller's own question that is not a search, such as Ian's "Is this song on Abbey Road?". Recommendation: add two small checks beside them. The Beatles songs, which ThinkThen already owns, give a yes/no truth. The CommonLit readability corpus, which jsort used, gives an order fitted from teachers' comparisons.

**Q26. Do these datasets fit decision models' context limits?**
MS MARCO passages run about 75 tokens, so a pair runs about 200 to 320 tokens with overhead, and a window of 20 about 2,000. A `find` of 255 passages runs about 20,000 tokens and about 90 KB, near the 96,000-byte default request size. Limits in [backends.md](../../../../specification/backends.md): the built-in route refuses past 65,536 input tokens, OpenRouter's Jev route lists 32,000, Clef's pages warn of truncation near 2,000 tokens of text state, the tested llama.cpp setup used 4,096, and Laya reads 512, which made it refuse every Fed statement in jsort's run. BEIR documents run longer, and Robust04 and TREC-NEWS run far longer, so they are left out. Recommendation: all four settings fit Jev and d1 on these sets. `window` with 20 does not fit Clef safely, and only pairs fit small local models. The experiment truncates BEIR documents in its own preparation, as published rerankers do; the product never truncates and keeps refusing oversized items. One local model run tests pairs only.

**Q27. What does "wins" mean for the ship gate? (Ian)**
Ian ruled that the dial ships only if the experiment wins (`functions.md`). Recommendation: a setting ships when all of these hold.
1. Order: on TREC Deep Learning 2019 and 2020 together, its nDCG@10 beats the better of the two `alone` baselines, yes/no and ten-level score, by at least 0.02, with a paired bootstrap 95% interval above zero. It also beats that baseline on three of the five BEIR sets.
2. Models: it holds on Jev and on one other hosted decision model.
3. Cut: a threshold chosen on half the queries keeps F1 no worse than `alone` by more than 0.01 on the other half.
4. Drift: the same item's score moves no more between strong and weak pools than under `alone`.
5. Cost: every run stays inside its `--plan` bounds.
For `find`, a setting ships when its top-one accuracy beats `window` on the top 1,000 candidates, and its none answers hold on no-answer queries. Each setting ships or drops on its own. `all-pairs` ships only with `tournament`, since they share the fit. A losing result is published too.

**Q28. What does the experiment cost, and which data may it use? (Ian)**
Estimate per hosted model: tournaments at ten comparisons per item on 97 queries in two wire forms, three repeats, all-pairs on each query's top 20, windows, the top-1,000 `find`, and one pass on 250 BEIR queries with longer items. That totals about 250 to 300 million input tokens, about $12 at OpenRouter's $0.042 per million for Jev and similar for d1 at $0.04 ([backends.md](../../../../specification/backends.md)). MS MARCO, under TREC Deep Learning, is licensed for non-commercial research only, and several BEIR sets carry non-commercial or research terms. The workspace rules require authorization for licensed data and paid services. Recommendation: approve both datasets for the experiment and for published numbers with credit, and a cap of $150 across two models, which covers reruns.

**Q29. How does the experiment guard against luck?**
Recommendation: three repeats on the same bytes and two shuffled input orders, as ADR 0055 did for batching ([rank.md](../../../../specification/rank.md)). Paired tests across queries, never across passages. Hold out the threshold half. Fix seeds. Report every arm, including arms that lose.

**Q30. How does compaction use the dial?**
PRD 00 Q32 makes compaction `rank` over segments with a size budget, and Ian ruled that the dial applies to it (`functions.md`). Recommendation: compaction takes `--compare` through `rank` with no new option here. The segment and compaction PRD owns the budget and the segment unit. `--top` pruning under `tournament` matters most there, because compaction keeps a few segments and drops the rest.

## Sources

- Bradley, R. A., and Terry, M. E. (1952). Rank analysis of incomplete block designs. *Biometrika* 39.
- Luce, R. D. (1959). *Individual Choice Behavior*. Wiley.
- Plackett, R. L. (1975). The analysis of permutations. *Applied Statistics* 24.
- Davidson, R. R. (1970). On extending the Bradley-Terry model to accommodate ties. *JASA* 65.
- Braverman, M., and Mossel, E. (2008). Noisy sorting without resampling. SODA.
- Jamieson, K., and Nowak, R. (2011). Active ranking using pairwise comparisons. NeurIPS.
- Heckel, R., Shah, N., Ramchandran, K., and Wainwright, M. (2019). Active ranking from pairwise comparisons and when parametric assumptions do not help. *Annals of Statistics* 47.
- Perez-Ortiz, M., and others (2019). From pairwise comparisons and rating to a unified quality scale. *IEEE Transactions on Image Processing* 29.
- Craswell, N., and others (2020). Overview of the TREC 2019 Deep Learning Track. arXiv:2003.07820. The 2020 overview is arXiv:2102.07662.
- Thakur, N., and others (2021). BEIR: a heterogeneous benchmark for zero-shot evaluation of information retrieval models. NeurIPS Datasets and Benchmarks. arXiv:2104.08663.
- Sun, W., and others (2023). Is ChatGPT good at search? Investigating large language models as re-ranking agents (RankGPT). EMNLP. arXiv:2304.09542.
- Qin, Z., and others (2023). Large language models are effective text rankers with pairwise ranking prompting. NAACL Findings 2024. arXiv:2306.17563.
- Zhuang, S., Zhuang, H., Koopman, B., and Zuccon, G. (2024). A setwise approach for effective and highly efficient zero-shot ranking with large language models. SIGIR. arXiv:2310.09497.
- Chen, Y., and others (2024). TourRank: utilizing large language models for documents ranking with a tournament-inspired strategy. WWW 2025. arXiv:2406.11678.
- Parry, A., MacAvaney, S., and Ganguly, D. (2024). Top-down partitioning for efficient list-wise ranking. arXiv:2405.14589.
- Yan, L., Qin, Z., Zhuang, H., and others (2024). Consolidating ranking and relevance predictions of large language models through post-processing. EMNLP. arXiv:2404.11791.
- Zeng, Y., and others (2024). LLM-RankFusion: mitigating intrinsic inconsistency in LLM-based ranking. arXiv:2406.00231.
- Wang, P., and others (2023). Large language models are not fair evaluators. arXiv:2305.17926.
- Liu, N. F., and others (2023). Lost in the middle: how language models use long contexts. arXiv:2307.03172.
- Boubdir, M., and others (2023). Elo uncovered: robustness and best practices in language model evaluation. arXiv:2311.17295.
- Chiang, W.-L., and others (2024). Chatbot Arena: an open platform for evaluating LLMs by human preference. arXiv:2403.04132.
- jsort, by keltokhy, OpenJev fork: pairwise Bradley-Terry sorting with Jev, Swiss pairing, first-position lean and readability benchmark.
- jev-reranker, by hotchpotch: listwise, pointwise and all-pairs reranking with Jev.
- Turbopuffer: Jev as a reranker with a ten-level score.
