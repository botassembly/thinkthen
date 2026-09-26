# Measure fixtures

These files define `thinkthen audit` and `thinkthen diff`. The source is a prototype measurement script at commit `be7cea2e4aa41097e7f629e35b62dadedeaca544`. Its three files had these SHA-256 values at that commit:

| Prototype file | SHA-256 |
| --- | --- |
| `scripts/tools/measure.py` | `7be206812f0e41011a9b31c59030c480f8b6bed945d7976db452f3908f0316a1` |
| `tests/test_measure.py` | `d64f2d9088c4f6237c0d2a2a8faf06dc51f92cd4674dab20b2a4f7231c01fa4e` |
| `scripts/tools/README.md` | `fae695523a16d1f2921ee9c73358c36e1613b58ee6acc9d720c6bc6e1d9e7fc7` |

`small/`, `249/`, and the fourteen files directly under `golden/` are byte-for-byte copies of the prototype's `tests/fixtures/audit/` at that commit. Two depart. Quick Fix qf-diff-warnings renamed the meta key in `small/annotate.jsonl` to `questions_sha256`, the key a real `annotate` line carries. The small fixtures share the placeholder digest `00`. It also recaptured `golden/diff-choose.jsonl`, the second departure, and `golden/table/diff-choose.txt` from `thinkthen diff` under the wider McNemar rule. Their McNemar p moved from 1.0 to 0.5. `249/` holds `thinkthen decide --details` lines replayed from an earlier experiment's recordings with every key unset: 272 yes/no questions about Beatles songs.

Three inputs are derived from `small/`. `decide-reversed.jsonl` holds the lines of `decide.jsonl` in reverse. `decide-key-noparts.jsonl` holds the key without `part`. `decide-key-odd.jsonl` holds the key without `part` and without `r6`.

The files under `golden/extra/`, `golden/table/`, and `replay/audit.jsonl` were captured from the prototype script at that commit, read with `git show` into a scratch file and run under Python 3.12.3 from this folder. `replay/key.jsonl` holds the labels of `transforms/rows/cases.jsonl`. `replay/audit.jsonl` grades the replay of `transforms/rows/recording` described in `specification/audit.md`.

| Output | Command line (from this folder) | Test |
| --- | --- | --- |
| `golden/audit-decide.jsonl` | `audit small/decide.jsonl small/decide-key.jsonl` | `audit::old_goldens_hold` |
| `golden/audit-decide-0.4.jsonl` | the same with `--threshold 0.4` | `audit::old_goldens_hold` |
| `golden/audit-decide-band.jsonl` | `audit small/decide-band.jsonl small/decide-key.jsonl` | `audit::old_goldens_hold` |
| `golden/audit-decide-bare.jsonl` | `audit small/decide-bare.jsonl small/decide-key.jsonl` | `audit::old_goldens_hold` |
| `golden/audit-choose.jsonl` | `audit small/choose.jsonl small/choose-key.jsonl` | `audit::old_goldens_hold` |
| `golden/audit-annotate.jsonl` | `audit small/annotate.jsonl small/annotate-key.jsonl` | `audit::old_goldens_hold` |
| `golden/audit-249.jsonl` | `audit 249/control.jsonl 249/key.jsonl --by verb` | `audit::old_goldens_hold` |
| `golden/audit-249-seed.jsonl` | `audit 249/control.jsonl 249/key-noparts.jsonl --by verb --seed 249` | `audit::old_goldens_hold` |
| `golden/extra/audit-decide-reversed.jsonl` | `audit small/decide-reversed.jsonl small/decide-key-noparts.jsonl` | `audit::old_goldens_hold` |
| `golden/extra/audit-decide-odd.jsonl` | `audit small/decide.jsonl small/decide-key-odd.jsonl` | `audit::old_goldens_hold` |
| `golden/extra/audit-choose-target-1.jsonl` | `audit small/choose.jsonl small/choose-key.jsonl --target 1` | `audit::old_goldens_hold` |
| `golden/extra/audit-249-question-seed-7.jsonl` | `audit 249/control.jsonl 249/key.jsonl --by question --seed 7` | `audit::old_goldens_hold` |
| `golden/table/audit-decide.txt` | `audit small/decide.jsonl small/decide-key.jsonl --table` | `audit::old_goldens_hold` |
| `golden/table/audit-choose.txt` | `audit small/choose.jsonl small/choose-key.jsonl --table` | `audit::old_goldens_hold` |
| `golden/table/audit-annotate.txt` | `audit small/annotate.jsonl small/annotate-key.jsonl --table` | `audit::old_goldens_hold` |
| `golden/diff-decide-cuts.jsonl` | `diff small/decide.jsonl --key small/decide-key.jsonl --compare-threshold 0.4` | `diff::goldens_match` |
| `golden/diff-decide-wordings.jsonl` | `diff small/decide.jsonl small/decide-b.jsonl --key small/decide-key.jsonl` | `diff::goldens_match` |
| `golden/diff-decide-nokey.jsonl` | `diff small/decide.jsonl small/decide-b.jsonl` | `diff::goldens_match` |
| `golden/diff-choose.jsonl` | `diff small/choose.jsonl small/choose-b.jsonl --key small/choose-key.jsonl` | `diff::goldens_match` |
| `golden/diff-249-cuts.jsonl` | `diff 249/control.jsonl --key 249/key.jsonl --compare-threshold 0.42` | `diff::goldens_match` |
| `golden/diff-249-soft.jsonl` | `diff 249/control.jsonl 249/soft.jsonl --key 249/key.jsonl` | `diff::goldens_match` |
| `golden/extra/diff-annotate.jsonl` | `diff small/annotate.jsonl --key small/annotate-key.jsonl --compare-threshold 0.75` | `diff::goldens_match` |
| `golden/extra/diff-249-cuts-nokey.jsonl` | `diff 249/control.jsonl --compare-threshold 0.42` | `diff::goldens_match` |
| `golden/table/diff-decide-cuts.txt` | `diff small/decide.jsonl --key small/decide-key.jsonl --compare-threshold 0.4 --table` | `diff::tables_match_byte_for_byte` |
| `golden/table/diff-decide-nokey.txt` | `diff small/decide.jsonl small/decide-b.jsonl --table` | `diff::tables_match_byte_for_byte` |
| `golden/table/diff-choose.txt` | `diff small/choose.jsonl small/choose-b.jsonl --key small/choose-key.jsonl --table` | `diff::tables_match_byte_for_byte` |

Ticket 0125 added members and table lines beside the old ones. `audit::old_goldens_hold` removes `precision`, `f1`, `r_precision`, `mean_level_distance`, and `steady`, and the table lines those members print, before it compares. Ticket 0131 adds `crossed` and `curve` and the `crossed` line to that list. Every other byte must match.

Ticket 0131 recaptured every `audit` file above, and `replay/audit.jsonl`, from the prototype with this patch. The patch pairs each answer with its confidence in the answer given, counts a tie at its share, shifts the interval by the bootstrap bias, and prints `by_bin`, the two tie members, and the ties table line. Only `calibration`, the tie members, and the ties line changed.

```diff
--- measure.py
+++ measure.py, patched
@@ -95,7 +95,7 @@
     for i in range(BINS):
         lo, hi = i / BINS, (i + 1) / BINS
         sel = [(p, t) for p, t in pairs if lo <= p < hi or (i == BINS - 1 and p == 1.0)]
-        gap += abs(sum(1 for _, t in sel if t) - sum(p for p, _ in sel))
+        gap += abs(sum(t for _, t in sel) - sum(p for p, _ in sel))
     return gap / len(pairs)
 
 
@@ -112,8 +112,14 @@
     rng = SplitMix64(seed)
     n = len(pairs)
     values = sorted(ece([pairs[rng.index(n)] for _ in range(n)]) for _ in range(DRAWS))
-    return {"error": ece(pairs), "interval": [quantile(values, 0.025), quantile(values, 0.975)], "bins": BINS,
-            "draws": DRAWS, "seed": seed, "note": NOTE}
+    est = ece(pairs)
+    bias = sum(values) / len(values) - est
+    shifted = sorted(max(0.0, v - bias) for v in values)
+    bins = [[(c, t) for c, t in pairs if i / BINS <= c < (i + 1) / BINS or (i == BINS - 1 and c == 1.0)] for i in range(BINS)]
+    return {"error": est, "interval": [min(quantile(shifted, 0.025), est), max(quantile(shifted, 0.975), est)],
+            "bins": BINS, "draws": DRAWS, "seed": seed, "note": NOTE,
+            "by_bin": [{"low": i / BINS, "high": (i + 1) / BINS, "n": len(b), "right": sum((t for _, t in b), 0.0),
+                        "confidence": sum(c for c, _ in b) / len(b) if b else None} for i, b in enumerate(bins)]}
 
 
 # ---------- input ----------
@@ -182,11 +188,11 @@
         answer = entry.get("answer") or {}
         self.p = answer.get("probability")  # p(yes) for decide
         self.probs = answer.get("probabilities")  # choose
-        self.tied = False
+        self.tied, self.holders = False, []
         if self.probs:
             top = max(self.probs.values())
             winners = [k for k, v in self.probs.items() if v == top]
-            self.top, self.pick, self.tied = top, winners[0], len(winners) > 1
+            self.top, self.pick, self.tied, self.holders = top, winners[0], len(winners) > 1, winners
 
     def has_probability(self):
         return self.p is not None if self.verb == "decide" else self.probs is not None
@@ -362,6 +368,8 @@
              "labeled": len(labeled), "unlabeled": len(ok) - len(labeled),
              "threshold": AS_RUN if rule is None else rule,
              "right": c["right"], "wrong": c["wrong"], "unresolved": c["unresolved"], "tied": c["tied"],
+             "tied_holding_key": None if decide else sum(1 for it in labeled if share(it, key) > 0),
+             "tie_share": None if decide else sum((share(it, key) for it in labeled), 0.0),
              "agreement": c["agreement"], "interval": wilson(c["right"], c["answered"])}
         for k in ("true_yes", "false_yes", "true_no", "false_no", "yes_recall"):
             g[k] = c[k] if decide else None
@@ -370,7 +378,7 @@
         g["auc"] = auc(pairs_yes) if pairs_yes else None
         g["disagreements"] = None if decide else disagreements(labeled, key, rule)
         if probs:
-            pairs = pairs_yes if decide else [(it.top, it.pick == truth(it, key)) for it in labeled if not it.tied]
+            pairs = [given(it, key) for it in labeled]
             g["calibration"] = calibration(pairs, seed)
             g["coverage"] = coverage_curve(labeled, key)
             g["suggested"] = suggest(labeled, key, rule, seed, target)
@@ -380,6 +388,19 @@
     return out
 
 
+def share(it, key):
+    return 1 / len(it.holders) if it.tied and truth(it, key) in it.holders else 0.0
+
+
+def given(it, key):
+    """The confidence in the answer given as run, and 1, 0, or a tie's share."""
+    said, want = it.said(None), truth(it, key)
+    right = 1.0 if outcome(said, want) == "right" else share(it, key)
+    if it.verb == "choose":
+        return it.top, right
+    return (it.p if said == "yes" else 1 - it.p if said == "no" else max(it.p, 1 - it.p)), right
+
+
 def disagreements(items, key, rule):
     n = {}
     for it in items:
@@ -476,6 +497,8 @@
         iv = g["interval"] or [None, None]
         out.append(f"  agreement {fmt(g['agreement'])} (95% {fmt(iv[0])} to {fmt(iv[1])}): {g['right']} right, "
                    f"{g['wrong']} wrong, {g['unresolved']} unresolved, {g['tied']} tied")
+        if g["tie_share"] is not None and g["tied"]:
+            out.append(f"  ties holding the key: {g['tied_holding_key']} of {g['tied']}, share {fmt(g['tie_share'])}")
         if g["verb"] == "decide":
             out.append(f"  said yes, key no: {g['false_yes']}   said no, key yes: {g['false_no']}   "
                        f"yes recall {fmt(g['yes_recall'])}   mean p(yes) {fmt(g['mean_probability'])}   AUC {fmt(g['auc'])}")
```

### Ticket 0125 fixtures

`abbey/rows.jsonl` holds 70 `decide --details` lines saved by a benchmark's run over song titles, one question: is the title a Beatles song on Abbey Road? `abbey/key.jsonl` is that benchmark's key, by title. `abbey/key-tune.jsonl` is the same key with `"part": "tune"` on every line, from `jq -c '. + {part: "tune"}'`. Graded with `--id /input`.

`better/` holds hand fixtures for `steady.better`. Each has eight records and a key with no parts, so audit reads twenty seeded splits of four held records each. The held parts at seed 0 hold these records, split 0 first:

| Split | Held records | Split | Held records |
| ---: | --- | ---: | --- |
| 0 | 1, 4, 5, 8 | 10 | 1, 5, 6, 8 |
| 1 | 3, 5, 6, 7 | 11 | 2, 3, 4, 8 |
| 2 | 4, 5, 6, 7 | 12 | 2, 5, 7, 8 |
| 3 | 1, 4, 5, 8 | 13 | 2, 3, 4, 7 |
| 4 | 3, 4, 6, 7 | 14 | 1, 4, 7, 8 |
| 5 | 1, 2, 4, 6 | 15 | 2, 4, 5, 6 |
| 6 | 1, 4, 6, 8 | 16 | 1, 3, 6, 8 |
| 7 | 1, 3, 4, 6 | 17 | 2, 3, 5, 6 |
| 8 | 3, 5, 7, 8 | 18 | 1, 3, 4, 5 |
| 9 | 2, 4, 6, 8 | 19 | 1, 3, 6, 7 |

- `better/decide.jsonl`: records 1 to 4 are keyed yes at p 0.9, 0.8, 0.75, and 0.7. Records 5 to 8 are keyed no at p 0.6, 0.55, 0.3, and 0.2. The run's rule is 0.5, which calls 5 and 6 yes. A tuning part holding record 5 tunes 0.61. One holding 6 and not 5 tunes 0.56. One holding neither tunes 0.5. The splits tune 0.61 ten times, so `steady.cut` is 0.61. It beats 0.5 on every held part that holds 5 or 6, and ties on the rest: splits 11, 13, and 14. `better` is 17. Counting each split's own bar instead gives 7.
- `better/tie.jsonl`: yes at p 0.9, 0.85, 0.8, and 0.7, and no at 0.3, 0.2, 0.15, and 0.1. Every split tunes 0.5, the run's own rule, so every held part ties and `better` is 0. Counting a tie as a win gives 20.
- `better/choose-reach.jsonl`, run as printed with no threshold, `--target 0.9`: records 1 to 4, 7, and 8 pick the keyed option at top 0.95, 0.9, 0.85, 0.8, 0.7, and 0.65. Records 5 and 6 pick wrong at top 0.6 and 0.55. `steady.cut` is 0.61 again. A held part holding 5 or 6 sits below the target as run and at 1.0 at 0.61, so the cut wins by the target. The other three held parts reach the target both ways with the same right answers, so the cut does not win. `better` is 17.
- `better/choose-more.jsonl`, saved under a threshold of 0.9, `--target 0.8`: every pick is right. Records 1, 2, and 7 printed their pick at top 0.95, 0.92, and 0.99. The other five printed null at top 0.85, 0.8, 0.75, 0.7, and 0.6. Every split tunes 0.01. Every held part holds a null record, so 0.01 answers more at equal agreement 1.0 and wins. `better` is 20. Comparing agreement alone gives 0.

`verbs/` holds one hand fixture per verb for `audit_verbs::each_verb_grades`.

- `tag.jsonl`: labels `x` and `y`, six records. The key lists the labels that apply, and `t6` is null. Label `x`: said yes on t1, t2, t5 and no on t3, t4; the key says yes on t1, t2, t4, t5. That is 4 right, one false no, precision 3/3, recall 3/4, f1 6/7. Label `y`: said yes on t2, t3, t5; the key says yes on t3, t5. That is 4 right, one false yes, precision 2/3, recall 2/2, f1 4/5. The pooled row holds 12 answers, 10 labeled, 8 right, precision 5/6, recall 5/6, f1 10/12.
- `score.jsonl`: levels low, mid, high, every key line in the tuning part. At the midpoint cuts 0.5 and 1.5, s2 (0.7, keyed low) and s5 (1.6, keyed mid) are wrong: 4 right, mean level distance 2/6. The first cut rises to 0.71, the nearest place above 0.7 and at most 0.9. The second rises to 1.61, the nearest place above 1.6 and at most 1.9. Then all six are right, and no move gets more.
- `rank.jsonl`: five questions, one per edge row of the ticket's R-precision table. R counts the labeled relevant rows present: 2 (rows at p 0.9 and 0.7, with 0.8 between: 0.5), 1 (`m4` to `m7` are keyed yes but absent: 1.0), 0 (null), 0 (null), and 2 with a tie at p 0.7, where the earlier `p2`, keyed no, takes the second place: 0.5.
- `find.jsonl`: five lines with no `input`, so the ids are the line numbers. Line 1 picks `u002`, right. Line 2 prints null with `none` on top, right. Line 3 prints null with `u001` and `none` tied, so it is tied. The key says `u001`, one of the two, so ticket 0131 gives it a share of 0.5. Line 5 ties `u001` and `u002` and prints `u001`, the first unit, as `find` does for a tie among real units. The key says `u001`, so it is right and earns no share, and `tied_holding_key` stays 1. Line 4 picks `u003` where the key says `u001`, wrong.
- `annotate.jsonl`: three records with a `decide`, a `choose`, and a `score` member. `urgent`: a1 right, a2 a false no, a3 a false yes. `effort`: levels small and large at the cut 0.5, so a3 at 0.6 reads large where the key says small.

`write/` holds the question files and keys for `audit_write`. `decide.json` is the payment question of `transforms/rows` with CRLF line ends, an escaped `threshold` key, and a threshold of 0.9. `key.jsonl` is `replay/key.jsonl` with `C-12` keyed no and a part on every line: `C-15` and `C-29` held, the rest tuned. The tuning part holds `C-12` keyed no at p 0.58 and `C-39` keyed yes at p 0.81, and every other record sits outside that range, so the bar is the lowest cut above 0.58: 0.59. On the held part, 0.59 gets `C-15` (no, p 0.51) and `C-29` (yes, p 0.79) right. Both 0.5 and 0.9 get one of them wrong. `set.json` is `demos/14-grade-a-batch/checks.json` with its threshold set to 0.95. `set-key.jsonl` keys only `correct`, from the cases' `human_correct`, with `E-05` held. The tuning part puts `correct` at p 0.98, 0.62, and 0.94 for yes and 0.02, 0.02, and 0.53 for no, so the bar is 0.54. It gets `E-05` (yes, p 0.94) right where 0.95 does not. `score.json` and `rank.json` hold the questions of demos 17 and 06.

### Ticket 0131 fixtures

`given/` holds hand fixtures for the tie share, the calibration pairs, the curve, `crossed`, and `--by POINTER`. `given/key.jsonl` keys `yesno.jsonl`, `choose.jsonl`, and `verbs/tag.jsonl`.

- `yesno.jsonl`: five `decide` answers. The pairs are y1 said yes at 0.9, right; y2 said no at 0.1, key yes, so 0.9 and wrong; y3 said yes at 0.5, key no, wrong; y4 said no at 0.6 under a bar of 0.7, so 0.4 and right; y5 said no at 0.35, so 0.65 and right. The bins give 0.4 (one pair, right 1), 0.5 (0.5, right 0), 0.6 (0.65, right 1), and 0.9 (two pairs, right 1): `|1 − 0.4| + |0 − 0.5| + |1 − 0.65| + |1 − 1.8|` is 2.25, over 5 is 0.45. The curve is `{0.9, 2, 1}`, `{0.65, 3, 2}`, `{0.5, 4, 2}`, `{0.4, 5, 3}`.
- `choose.jsonl`: g1 picks `a` at 0.6, right. g2 picks `a` at 0.6, key `b`, wrong. g3 ties `a` and `b` at 0.45, key `c`, share 0. g4 ties `a` and `b` at 0.4, key `b`, share 0.5. g5 ties all four at 0.25, key `d`, share 0.25. So `tied` 3, `tied_holding_key` 2, `tie_share` 0.75. The bins hold 0.2 (n 1, right 0.25, confidence 0.25), 0.4 (n 2, right 0.5, confidence 0.425), and 0.6 (n 2, right 1, confidence 0.6). The error is `(0 + 0.35 + 0.2) / 5`, 0.11. The curve is `{0.6, 2, 1}`, `{0.45, 3, 1}`, `{0.4, 4, 1.5}`, `{0.25, 5, 1.75}`.
- Both, with `verbs/tag.jsonl` beside them and `--pooled`, give 10 answers: the tag labels stay out. The bins give 0, 0.25, 0.5, 0.15, and 0.8 over 10, which is 0.17.
- `crossed.jsonl`: the tuning part holds p 0.2 (no), 0.4, 0.6, and 0.8 (yes). Accuracy is 4 of 4 from 0.21 to 0.4, and the tie rule nearest 50 picks 0.4. The held part holds 0.3 (yes), 0.45 (no), 0.55 (yes), and 0.7 (no), where the best cuts give 2 of 4 and 0.5 is nearest 50. The held part at 0.4 gets only 0.55 right. The tuning part at 0.5 gets 0.4 wrong. That is 4 right of 8. Under `--optimize f1` the held part tunes 0.3, with f1 2/3. The tuning part at 0.3 gets all four right, so 5 of 8 in all.
- `by.jsonl`: six lines whose inputs carry `category` and `a/b`. `/category` gives `lead` with two `decide` and one `choose` answer, `tail` with two `decide`, and `3` with one `choose`.

### Ticket 0135 fixtures

`sets/` holds hand fixtures for `recognize` and `relate` grading. Every value below is worked by hand.

`names.jsonl` holds seven `recognize --details` record lines run at 0.3, with kinds PER, ORG, and LOC. `names-key.jsonl` keys each. Places are `start-end`, end exclusive.

| Id | Said (kind, places, strength) | Key | `strict` | `overlap` |
| --- | --- | --- | --- | --- |
| a | PER 0-5 0.9, ORG 10-20 0.4 | PER 0-5, ORG 10-20 | 2 matched | 2 matched |
| b | PER 0-6 0.8, LOC 30-35 0.35 | PER 0-5, LOC 35-40 | 2 extra, 2 missed | PER matched; LOC 30-35 only touches 35-40: 1 extra, 1 missed |
| c | ORG 0-8 0.95 | LOC 0-8, PER 12-15 | 1 extra, 2 missed | the same: the kinds differ |
| d | none | PER 0-4 | 1 missed | the same |
| e | PER 0-3 0.7, PER 4-8 0.6 | PER 0-8 | 2 extra, 1 missed | 0.7 takes PER 0-8; 0.6 is extra |
| f | PER 0-10 0.5 | PER 0-4, PER 6-10 | 1 extra, 2 missed | takes PER 0-4, the first in key order; PER 6-10 missed |
| g | PER 0-3 0.4, PER 2-9 0.9 | PER 0-4, PER 5-9 | 2 extra, 2 missed | 0.9 goes first and takes PER 0-4; 0.4 overlaps only PER 0-4 and is extra; PER 5-9 missed |

Ten names are said and twelve keyed. `strict` matches 2, so 8 are extra and 10 missed: precision 2/10 = 0.2, recall 2/12 = 0.166667, f1 4/(4 + 8 + 10) = 0.181818. `overlap` matches 6 (a 2, b 1, e 1, f 1, g 1), so 4 are extra and 6 missed: precision 0.6, recall 0.5, f1 12/22 = 0.545455. Taken in output order, g's 0.4 would take PER 0-4 and 0.9 would take PER 5-9, for 7 matched.

`edges.jsonl` holds three `relate --details` lines with no `input`, so their ids are 1, 2, and 3. Each ran at 0.6 with a directed `calls` and an `either` `peers`. Line 1 says `calls a→b` 0.9, `calls c→a` 0.7, and `peers b→c` 0.8, keyed `calls a→b`, `calls a→c`, and `peers c→b`: `calls a→b` matches, `calls c→a` is extra and `calls a→c` missed, since `calls` has a direction, and `peers` matches in reverse. Line 2 says `calls a→b` 0.65, keyed the same. Line 3 has `meta.failed_questions` 1, so it fails and its key line counts nowhere. As run: 3 matched, 1 extra, 1 missed, f1 6/8 = 0.75. Every key line is `part: "tune"`. By cut: 0.6 to 0.65 keep everything (0.75); 0.66 to 0.7 lose line 2's match (4/7); 0.71 to 0.8 also lose the extra (4/6); 0.81 to 0.9 also lose `peers` (2/5). The cut is 0.6. Grid cuts below 0.6 would read 0.75 too, and 0.5 is nearer 0.5.

`cut.jsonl` holds two `recognize` lines run at 0.3, both tuned. r1 says one right name at 0.9. r2 says one right name at 0.46 and three extra at 0.47, 0.48, and 0.49. F1 is 4/7 from 0.3 to 0.46, 2/6, 2/5, and 2/4 at 0.47 to 0.49, 2/3 from 0.5 to 0.9, and 0 above. The cut is 0.5. Matches over records is 2/2 from 0.3 to 0.46 and picks 0.46, the nearest to 0.5.

The hand-written `--lines` relate line in `audit_sets` carries `f2c4e88c6a7b11bd98a7fed97bcc8412f0e6ca7da0b45b0c9bb2014cd41da3fc`, from `printf '%s' '{"verb":"relate","fields":null,"relations":[{"name":"calls","source":"*","target":"*","reads":"calls","either":false}],"threshold":0.01}' | sha256sum`. It also holds a `value` and a `question` with `threshold` and `relations`, so it reaches the digest check.

`audit::every_fixture_keeps_its_checksum` fails on a missing, extra, or changed file.

| File | SHA-256 |
| --- | --- |
| `249/control.jsonl` | `c9abe7ba38243fe5fbf577a94754067de873be4b0b00339ec31eed40138d8bdb` |
| `249/key-noparts.jsonl` | `b6386622b98cf36523623ab22516629bfe38742e35f7f4d96f2db8cd87bd387b` |
| `249/key.jsonl` | `8e5e8fb7c799b0c64d1502167fc8f010d4a598202b060b0fd407600b68429b59` |
| `249/soft.jsonl` | `12dd593528336582c930e9f4472708018fd5bd67d9f1a3337e613773e734a08b` |
| `abbey/key-tune.jsonl` | `489151528893b560763f9abca1cc0d81a735e1a63310de444d19295b94f7f9e7` |
| `abbey/key.jsonl` | `3b12e168ca9715a775bf3bc759c20f6e018f7d1d1763a8c6352c59d2a42c242e` |
| `abbey/rows.jsonl` | `70b54a7636132e4576538107d45b2ff1841709de3eceb2a8b5970404b2678952` |
| `better/choose-more-key.jsonl` | `4400e79ce28f6f352d9ec7db606df7647879ec40aee49c52e210013be5776d99` |
| `better/choose-more.jsonl` | `9bd9459368a823269e6de6057f9228f212fbfd4ad066eecc52ce96f5482d692f` |
| `better/choose-reach-key.jsonl` | `641ab843cc9a72c3d8adca4e77e1da3393d40853dd2ca6c3f4270d921febb15e` |
| `better/choose-reach.jsonl` | `cfdc382795a8d25dfad725f6c0e3e1cd6691b1a684d1b2353fbd7f67162a37e6` |
| `better/decide-key.jsonl` | `8b47ff1106e77c8fd4c00802006c2fe9ca449e4e9e94b00e65bf6b04ad8898b0` |
| `better/decide.jsonl` | `07082f6c1c542f5e70f21e579df3f824280c4dd59e8b015f439e1e72b41aedd8` |
| `better/tie-key.jsonl` | `8b47ff1106e77c8fd4c00802006c2fe9ca449e4e9e94b00e65bf6b04ad8898b0` |
| `better/tie.jsonl` | `d74c6d3b93823583fc02fcd2b9ba3be5ef11a98e7ab5bfa32c495d492cdf0386` |
| `given/by-key.jsonl` | `2380d356e8c93b6136ddc594557cf62ad369c95818f8becc432d3721ed96cb5e` |
| `given/by.jsonl` | `8d9094499bf640f84258a1d0495c4e934e5a231023e7789a493357055ab1b2b8` |
| `given/choose.jsonl` | `6cc988a7406d39ea0f40fe11cd2c7ba8e609258be4e8750a63393703d8c0dbd1` |
| `given/crossed-key.jsonl` | `a530ed13dc777260b3e5612ec9437d695b46e5921ab5f4ffc052fd8c16aa4e8c` |
| `given/crossed.jsonl` | `b42e456bb619f9d5eccc35954770603f4b7d0a8c63a67e9af2df7e86fcc43020` |
| `given/key.jsonl` | `8b56e25deac98b1f208f662a4ecb223ca08a11283977e7bb40ab049f7f97d8e5` |
| `given/yesno.jsonl` | `ddabfebfc1d7ef71d45593e4c7d27e92ec971c7ea4bf901b757ac9567b84a94b` |
| `golden/audit-249-seed.jsonl` | `73d9def49b9dfe2bcf47f9bedad4c2328c5dd93929dc39503728a88e7fca5f70` |
| `golden/audit-249.jsonl` | `7b40c727fe9450e84a71c36c6dd93bab1971250cc0fd39d46cb4c90bc1d2ed62` |
| `golden/audit-annotate.jsonl` | `5ec5f3aa346ff09737125b2b84661c08837f620d2fce8f55a95bc15aea551e26` |
| `golden/audit-choose.jsonl` | `4c52c78749a4412c14f713a94b5e583adf0fc2fcac8818687d51ad5be5043625` |
| `golden/audit-decide-0.4.jsonl` | `b4cae90f891fc6937a9cbc2050a7747e068326b8b34e8d4c22204bdc83a30cda` |
| `golden/audit-decide-band.jsonl` | `6017904ac849ab831ca582f3720ae144fa0a16b512a9e1d16fd486db007713a2` |
| `golden/audit-decide-bare.jsonl` | `c0cdc94d99aaba8e814ff5afa8b58e577f19375c03ed3faa19ea7b08f5b60179` |
| `golden/audit-decide.jsonl` | `ae5ad8f6e13aa58c15185a674d0a733d21cc4999991d2bd675c78f62b0bed6bf` |
| `golden/diff-249-cuts.jsonl` | `76dfffe53438500267a548456deea5fdfc454ab05fec0a0e284ba52208475dd3` |
| `golden/diff-249-soft.jsonl` | `69b1c96922b4c0c5bdc40ab919e0023eb08bac4b10baafd20d3af61b5f53ea44` |
| `golden/diff-choose.jsonl` | `fd677ab15a786ceaae633b89ddb46badbc36a953f721ef0d4d16e13c1600e44f` |
| `golden/diff-decide-cuts.jsonl` | `ae00a2514257d35c36658c3c63abe2c92e9dbc711950bdcef7864f891d081682` |
| `golden/diff-decide-nokey.jsonl` | `ad99f8572fa59ab30168859f119114892656e0e61efbe0e633cba5c93727a284` |
| `golden/diff-decide-wordings.jsonl` | `593398f0617796069712dcbad935a0b181ed57e1a78c362e41fcaf15c0ccc3f5` |
| `golden/extra/audit-249-question-seed-7.jsonl` | `4e7f367dc12253c0c6397ee8e1066c3a2543614f3cabdbf39531708663321b54` |
| `golden/extra/audit-choose-target-1.jsonl` | `9a2ab92c655ab9aa3029d8c0bb7d55ec8e0a4cb0609c065d0adfeb460310b717` |
| `golden/extra/audit-decide-odd.jsonl` | `d700950a309b244a8403871ec10c54f76dd27466e6cef515bcb44241ab0c9654` |
| `golden/extra/audit-decide-reversed.jsonl` | `b89c70a5f8203bb5d7534f36f0760d091c44cd94428665e051a4a446a2949500` |
| `golden/extra/diff-249-cuts-nokey.jsonl` | `1fc7ba84a3a666004872ea0e619c5451f64daa67a6bda1aefa9540a398a44458` |
| `golden/extra/diff-annotate.jsonl` | `ab7ac6f3064bfea00a0ca9d075183f1f1c33947e507584c9206a27f52d51514a` |
| `golden/table/audit-annotate.txt` | `1a6729b3aa5f9c0b8e0462235cd05deb42661ccca81c44888f8b3b05918c74a1` |
| `golden/table/audit-choose.txt` | `5f81c2893de087d4dc50dc4b0372614ad99a7b5768f69495118070f88997770e` |
| `golden/table/audit-decide.txt` | `bc3bf0c7acaf5635fb9ed3492eb235d535f37cc34b12dc8737e16c9ea2cd4b1d` |
| `golden/table/diff-choose.txt` | `7f1a11677eba121025ab46b1ad5ffdd6ec09e3cb4ac0d0519960b5ba7301c186` |
| `golden/table/diff-decide-cuts.txt` | `c56e94bc2e3c09c525e7ef8fc3e2190a5af4216c5d91d769c4176cfea06536c8` |
| `golden/table/diff-decide-nokey.txt` | `82edef392a2ddee9528a931453abd42c57d99a5c67a98e2984ffcc6e8be9f9c1` |
| `replay/audit.jsonl` | `d0a1e138ae38922f55363f2f4d446d61580013657dcc4049f69ad1f211e08e8e` |
| `replay/key.jsonl` | `6469595eef17159ed9563de7b84bdc4396249fb2bb8a704397e5cc31c0619042` |
| `sets/cut-key.jsonl` | `eb4ae5c67d4accc03efcdacbd4d4918c9479d803165f1f1e3985f94943bdd7ca` |
| `sets/cut.jsonl` | `5238ba287efe3605ce1238b110214505685459c553bfed73b9223ca89a86ec9a` |
| `sets/edges-key.jsonl` | `f45486609c9923d9549fe032d9b58d58a0b9517b76f718e51ea369f145ee7b58` |
| `sets/edges.jsonl` | `ccaa648abd1ec49a8ae0355f5df81d36de5667686c6deaf5b82a7135afcdfe1c` |
| `sets/names-key.jsonl` | `382523d51f5aa82757ca75bab931a6f3fb450ecbf07e7bb2e90c2f2698edffac` |
| `sets/names.jsonl` | `20e6a04fb06cffae4b4ac0ec65664dce5263a000aba3798de9b2939771f107ee` |
| `small/annotate-key.jsonl` | `fcaebb1e268c468d697e6bc6852c4b5aab5198e544552711d7d5c1b27e523b26` |
| `small/annotate.jsonl` | `c827c6ca44198b10c0561b49d930d85f74bdba88549169e1cc187e2320c24270` |
| `small/choose-b.jsonl` | `b227bc45a7fbcb64db89352bda57d0da5eb53a9b9df01f2332c956dc844d8d51` |
| `small/choose-key.jsonl` | `f908ef59696a2fc7ba06001e3bbaaf53e52427a4adf5d8f2d347e8bb7ca0ddfa` |
| `small/choose.jsonl` | `b8f2178160860be3a58b4801e72eb68ded2cdfc0fd1675c34a23e95f8e9be7cb` |
| `small/decide-b.jsonl` | `a63d035f5a867b8080cca87efba5aeaec0ecbf3b53ffb6339f7c80ca1b0f785f` |
| `small/decide-band.jsonl` | `c8dc93088bc80ffbefa5087c5639a48da1eb5a10408ec760c4547e68674aa2a6` |
| `small/decide-bare.jsonl` | `b9f53267f0fb1fa0d89250f42c3c6f2bbd4c975548dde94668b55b545dad2e11` |
| `small/decide-key-noparts.jsonl` | `ebd896c265f68c7a4e71387ab8af8b495ae5f9406d9248b4cb2f9fadd87d0e25` |
| `small/decide-key-odd.jsonl` | `5aea71cae954171b2003947a27294ab86c8e17a746124cb04b0d9b5100d96128` |
| `small/decide-key.jsonl` | `10461a043dd3aa8feb8b5647212abd31f779511b274df5ddd2e9269403b59df4` |
| `small/decide-reversed.jsonl` | `e51b0c73bd7183715e3cda4bf71a02e0a0f4ca22cdc051ecedf83430a497ea86` |
| `small/decide.jsonl` | `fd6546d99ca631ec96569e432a8ec6d40889c312a445728be9a678196b0a06c6` |
| `verbs/annotate-key.jsonl` | `d39de81ae6d31ba4f2dc748c259d689a0e9ccb538d744f3a4221d1403454b63f` |
| `verbs/annotate.jsonl` | `6ad41b300b9de9c2fc680b32da7627ee6f5d7bb0c9eaa7fe17d7e641a5ca8704` |
| `verbs/find-key.jsonl` | `9aacc7cfeb2ae55030779e536b09b739f170efdc44df7e962948069615057128` |
| `verbs/find.jsonl` | `15a956b7f23c1e7f21046ae6a06b949e14fb44c4679bc53cc02cfe9eacf37e9b` |
| `verbs/rank-key.jsonl` | `c45b0db08b0b382c6f28c96a35f76f2a9b600ddae3009cc50910f519854cb8f4` |
| `verbs/rank.jsonl` | `a47981836cdd181ae6905a1adac191c63adf90d6936e43d54e6c3b402dd2d64d` |
| `verbs/score-key.jsonl` | `be01ccf4a2ec3478c4628cce5a07f73b263251497db8c467d585d1ded7b92c37` |
| `verbs/score.jsonl` | `fb70310ebc6d57166fc88c2701be55b902e0c3cb96a6bed9c66fa38e909e8f35` |
| `verbs/tag-key.jsonl` | `df891f08ee40f159c81b5172dfe91cf2a19189422b510fd8fdb5e19245e217a8` |
| `verbs/tag.jsonl` | `789fef009c2dd692d1444eaf824b680189665d1f064d05bc4ec2060f54ef4c32` |
| `write/decide.json` | `9f95c830aa63f2e905f530067cb8139af4d334787cb9ddeb9b30ac73c96ea683` |
| `write/key.jsonl` | `2208a2f79f308ff14ceb303be29d8cf0dc44f7f84cac22ecddec152d25265864` |
| `write/rank.json` | `237516ad81b76d4cf9508a44d59c637f88a97ca5d5c5865567e16ba1e7d799d3` |
| `write/score.json` | `4388b91d7342267bfb7a1547d4c41e1276fb5667c0ecc999417e656f6793976a` |
| `write/set-key.jsonl` | `ade50a6f5cc0262d4b5b7c9d39fc2287d61825015a0318633d82683917f755dd` |
| `write/set.json` | `ba2b5e1c0d8c2f142cbdd19e744847eb5a18c9d2f5014ec96c7d22d5033c3979` |
