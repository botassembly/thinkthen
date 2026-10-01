# Harvest the Beatles and relate runs: efficiency, thresholds, and tuning

Status: Closed on 2026-09-25 after a check against main. Threshold how-to (0087), audit (0113), and diff (0114) landed. The open default-cut and automatic-rewording questions moved to 2026-09-25-audit-is-complete-for-0-1.md.

Ian asked on 2026-09-23 for a note to the build team. The team should mine our recorded Jev queries to make `recognize` and `relate` cheaper, and make thresholds and question tuning a first-class loop. Everything below is recorded and replays with no key.

## What to harvest

- Local experiment 235: 22 runs over 70 Beatles titles, all ten functions, 730 requests, 302,306 input and 42,569 output tokens. Each run folder holds `cmd.sh`, `requests.jsonl`, `answers.jsonl`, the recording, and a scored `output.txt` against the answer key in `dataset/`. R20 (recognize) and R21 (relate) went to Jev directly because no command build has either function. R22 sent R21's requests through `annotate`.
- Local experiment 236: relate sizing by dry-run (`sizing.txt`), duplicates, travel-policy contradictions, and ticket-to-incident matching with `choose`.

## Efficiency: what the requests show

1. **Recognize is heavy for short text.** R20 spent 18,999 input tokens on 23 titles of one to six words. Measure how much of each request is repeated instructions and per-word questions. Then test folding the word and kind steps together, as the demo did.
2. **Relate over two sets is choose.** R21 linked 10 songs to 4 singers and 4 albums as 80 pairs for 12,178 input tokens. Two `choose` questions per song ask 20 questions. The deep dive ranks "ask once per record against the candidates" second after splitting.
3. **Shared instructions ride in every request.** Sending them once per request is the cheap win the deep dive deferred. The harvest gives the byte counts to size it.
4. **Splitting is required this round** (`sdlc/planning/relate-design.md`, ruled 2026-09-23). Use R20 and R21 as replay fixtures for the split path.

## Thresholds: the data makes the case

The default cut of 0.5 says yes too often on knowledge questions. The recordings show it, and a stricter cut costs nothing on replay.

| Run | Cut 0.5 | Stricter cut |
| --- | --- | --- |
| R21 relate, 20 true edges | 18 right, 7 wrong | 0.9: 13 right, 0 wrong. 0.7: 17 right, 3 wrong |
| R20 recognize, 21 names | 3 of the 4 errors sit at 0.45 to 0.52 | 0.6 drops "Roll", "Mean", and the weak "Octopus's Garden" |
| R03 filter "Ringo sang it" | 32% of kept songs right | 0.8: 3 kept, all right |

Ask: the trust page and a how-to show the loop on real labeled cases. The loop runs with `--details`, joins a truth file, and prints right and wrong at each cut. Ian set `report` aside for `jq` transforms (`specification/roadmap.md`). So the loop is a documented transform, and it must also cover relate edges and recognize names, not only yes/no rows.

## Prompt optimization: tuning the question, not the model

Ian's word on 2026-09-23: this is prompt optimization, as GEPA does it. It is not fine-tuning.


"You train nothing" stays true. What a user tunes is the question: its wording, the option descriptions, and the cut. The question file already carries the tuned cut with the question (`specification/question-file.md`, rule 8). Ask:

1. A documented loop: labeled cases, run, sweep the cut, reword the question, and rerun. The cache answers every unchanged question for free.
2. A way to compare two wordings on the same cases by case id, as the set-aside `report` did. A transform is enough.
3. Later, measured before any build: automatic rewording against labeled cases, the way a prompt optimizer revises instructions from feedback. An outside write-up reports large gains for Jev this way. Record it as an experiment first.

Found by the product side from experiments 235 and 236. Ian can overturn any line.

## Added 2026-09-23: the default cut on knowledge yes/no questions

Experiment 242 found that the yes/no functions say yes too readily at 0.5 on knowledge questions. "Is this a Ringo song?" gave 32 yeses, and 7 were right. Yet the probabilities ranked the songs well (AUC 0.87). The same looseness showed in the DuckDB sample (241) and in relate (239). The cut is the problem, not the question's direction. The build team should weigh whether the default for `decide` and `filter` stays 0.5, using the harvested runs as evidence. The threshold how-to above becomes the first thing a user reads.

## Added 2026-09-23: what Beatles Bench found for tag and recognize

- `tag` at a cut of 0.5 over-tags. On songs with one lead singer, Jev gives the wrong Beatles 0.4 to 0.6, so extra labels cross the cut: Ringo was tagged on 51 songs against 10 true. The most probable label is right on 77% of those songs. `decide` shows the same middle band as a lean toward no. A per-question cut tuned on labeled cases, or a "top label" mode, would serve users better than one fixed 0.5.
- `recognize`, built from `annotate` as in experiment 222, misses name boundaries: it takes "album" into "the album X", keeps "Honey" and drops "Don't", and loses the middle of long titles. Exact matching scores songs at 0.23 precision and 0.15 recall. Counting overlap scores precision at 1.00 and recall at 0.54 for songs and 0.75 for albums. The shipped command needs a boundary test on multi-word titles.
