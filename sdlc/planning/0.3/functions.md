---
date: 2026-10-13
status: design input for 0.3; rulings recorded, experiments to design
source: Ian's dictation on 2026-10-12 and 2026-10-13; Ian's idea notes on segment, compact and chunk, and on pairwise ranking
---

# New functions in 0.3

Ian, 2026-10-13: "My goal for 0.3 is to get back to doing some functions." 0.2's rule of no new functions ends with 0.2.

## Segment and chunk: a document becomes a graph

### What's in it for you (draft, to validate with Ian)

- **Turn a long Markdown document into a map you can work with.** Sections, paragraphs, lists and tables become nodes, and their links become edges. Your code or agent then walks the map instead of reading the whole document.
- **Cuts where the meaning changes, not every 500 characters.** A decision model predicts where each segment starts and ends, so a chunk never splits a table from its caption or a rule from its exception.
- **Every chunk traces back to its place in the source.**

### Rulings (Ian, 2026-10-13)

- **Chunking and segmenting are the headline,** ahead of compact. Their use is document understanding: a model such as Jev predicts where segments live.
- **The mental model is a graph:** a Markdown document turned into nodes and edges, then processed as a graph. Ian is unsure others will grasp this at first; the docs must show it, not assume it.
- **Each step needs prediction** (2026-10-12). If segmenting turns out to need no classifier, that is a finding.

- **The graph is part of the output** (2026-10-13). It is nodes and edges, or a tree: a document is a hierarchy. Its exact form is still vague and needs design.
- **Whether chunk, segment and compact are their own functions is open.** Design the options before deciding.

## Compact

- Keep what is salient for a question and drop or shorten the rest. Ian: compact is a kind of ranking. It may not be its own function at all, but a documented way to use `rank` for compaction.

## The ranking dial

### What's in it for you (draft, to validate with Ian)

- **You choose cost or accuracy with one setting.** The cheapest setting scores each item once. The top setting compares every item with every other, which is worth it when there are five items and the answer matters.
- **The cost of each setting is known before you spend it,** through `--plan`.

### The problem (Ian, 2026-10-13)

- Pairwise judgment is stronger: "this is better than that" gives the model something to compare against. A score given in isolation, or inside one batch, has nothing outside its batch to compare with.
- Ian's example: 100 lines in 10 batches. Each batch is only compared within itself. The 10th-best line in a strong batch may beat the best line of a weak batch, and batch scores will not show it.
- Comparing everything with everything grows with the square of the count. The algorithm must avoid that for large sets.

### The dial, cheapest to dearest

1. **Score each item on its own.** One judgment per item. This is today's `rank`.
2. **Windows.** Rank items in overlapping groups that slide down the list, so strong items carry forward.
3. **A tournament.** Pairwise comparisons arranged like a sort or a bracket, so the count grows with n times the log of n, not n squared. Elo-style ratings fit here.
4. **All pairs.** Every item against every other. Sensible for a handful of items.

### Rulings (Ian, 2026-10-13)

- **The question is always the caller's own** (2026-10-13). There is no built-in "which is better" question. Pairwise ranking asks the caller's question, such as "Is this song on Abbey Road?", of two items at once.
- **The experiment must turn comparisons into one score from 0 to 1** that a threshold can cut. Options to test: maximum, minimum, mean, median, Elo-style ratings and fancier scoring. An item facing strong competition will likely score lower than the same item facing weak competition, and the scoring must correct for that.
- **An experiment first; ship only if it wins.** It becomes a `rank` setting, not a new function.
- **Find good datasets** with a known right order to measure it. Ian is unsure such sets exist for this.
- **A strong blog article** once the algorithm is worked out.

### Lead's starting points for the experiment

- Published methods to compare: pairwise ranking prompting with a sliding window or a sort (Qin and others, 2023), setwise ranking (Zhuang and others, 2024), listwise sliding windows as in RankGPT (Sun and others, 2023), and tournament ranking such as TourRank (2024).
- Datasets with graded relevance and known order: the TREC Deep Learning 2019 and 2020 passage sets and the BEIR collection. Each has queries, candidate passages and human relevance grades.
- The measure: agreement of the top k with the known order, against the number of calls and tokens each setting spent.
- Ian's notes from Twitter mention a paper. The experiment brief should find it in Ian's clippings.

- **The dial applies to `rank`, `find` and compaction** (2026-10-13). Pairwise finding is probably more accurate than window-based finding, at a cost.
- **TREC Deep Learning and BEIR are the starting datasets** (2026-10-13). Report back if they fail to meet the need.
