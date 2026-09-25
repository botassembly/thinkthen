# Show what changes when the cut moves, free from the cache

Status: Closed on 2026-09-25 after a check against main. 0114 added diff --compare-threshold; 0113 audit prints the coverage curve.

Ian's feature idea, 2026-09-23. A user tuning a threshold wants to see which answers flip between two cuts: what gets added and what gets removed going from 0.4 to 0.5. The cache holds every probability, so a rerun at a new cut sends nothing and costs nothing. Today the user reruns twice and diffs by hand, or writes a `jq` transform over `--details`.

## The ask

Rerunning any command at a new cut from the cache prints only the rows whose outcome changed, marked added or removed, each with its probability. One possible spelling is `--compare-threshold 0.4`. Near a band, the rows moving into or out of "not sure" show too. For `relate` and `recognize`, the rows are edges and names.

A request that would miss the cache is refused under this flag, so the comparison can never spend money. The output ends with one count line, such as "0.5 → 0.4: 9 added, 0 removed".

## Why

The Beatles runs showed that the cut matters more than the wording on knowledge questions. The Ringo filter had 32 yeses at 0.5 and 7 right (experiment 242). In the graph (239), the ruled 0.5 gave precision 0.87 and recall 0.60, and 0.4 gave 0.83 and 0.68. A user sees that trade only by comparing runs. The deck shows it as a slide, "Move the bar, see what changes, at no cost".

## Relation to `report`

Ian set `report` aside for `jq` transforms (`specification/roadmap.md`). This flag is narrower: one comparison, on the command the user already runs, reading only the cache. The build team says whether it belongs in the command or ships as a documented transform first.

Ian can overturn any line.

## Added 2026-09-23: accuracy at each coverage level

Given labeled cases, the same cache-only pass can report a curve: for each cut, the share of cases that still get an answer and the share of those answers that are right. A cut of 0.9 answers fewer cases and gets more of them right. The curve shows the user where to set the cut before trusting a question, and it costs nothing once the answers are saved. Doug Turnbull's public study of query classification (https://softwaredoug.com/blog/2026/09/22/jev-query-understanding.html) makes this curve its main result: Jev trades coverage for accuracy smoothly, and a chat model that answers everything sits at one point.
