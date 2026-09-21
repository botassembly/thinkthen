# Five small closings for the recognize team

Written 2026-09-21 by the product side. The harvest package at `experiments/225-recognize-harvest-package/` is accepted. An audit found every part the request listed: the exact words, the rules with tests, forty made-up cases with recordings in the ruled shape, `relate` as its own set, and the map. No more experiments are asked for. Four page fixes remain, and none needs a paid run.

1. **Mark the method page as superseded on shape.** `experiments/RECOGNIZE-PRODUCT-SPEC.md` still prints `PER`, `head`, `tail`, `type`, word positions in the main object, a `depth` dial, and a margin term. Put three lines at its top: the public shape is `sdlc/planning/recognize-design.md`, the rules and cases are the harvest package, and the table of rulings on the design page settles each difference. Do not rewrite the page.
2. **Fix one sentence.** The `relation_threshold` row says to raise the bar to find more relations. Raising a bar finds fewer.
3. **Write the overlap rule down.** The closing note to Ian describes it: probability weighting, with leftmost-longest as the tie-break. The audit found those words in no page. Add the rule to `rules/rules.md` with one test case, and state there that pure leftmost-longest was measured and lost, with the two numbers. Add the possessive rule the same way if `rules.md` lacks it: a trailing `'s` trims off and a middle one stays.
4. **State two defaults.** `word_threshold` defaults to 0.0 with no reason given. Say why or say it was never varied. Say the same for `repairs`, which is off and unmeasured.

5. **One free comparison, from recordings only.** Added later the same day, after Ian asked that every number match what the model reports. Rescore the saved runs with one change: gate a name on the lowest model probability behind it, with connector words left out, in place of the least-times-mean number. Report both on the same corpora. If the plain number holds, a name carries a real `probability` and the word "confidence" leaves the product. The reason is `sdlc/issues/2026-09-21-one-rule-for-every-number-the-tool-prints.md`. No paid run.

Not asked: the 222 demo file stays as it is. The deck's `recognize` slide now reads case C01 from the harvest package.

Ruled by the product side, for the team to know: `relate` reads records only, and names a user already has are passed as records with a kind field. No `depth` option exists on the command. Both rulings are on the two design pages, and Ian can overturn them.

Closed 2026-09-21. All five items are done with no paid run. Item 5's answer: no plain model probability gates as well as the computed number, so the product side named the computed number `strength`. The record is `sdlc/issues/2026-09-21-one-rule-for-every-number-the-tool-prints.md`. The recognize program is closed.
