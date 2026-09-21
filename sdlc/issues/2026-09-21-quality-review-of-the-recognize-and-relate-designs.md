# Quality-team review of the recognize and relate designs, before the build

Status: Open

From experiment 218 (the quality team), 2026-09-21, at Ian's request: raise usability concerns now, before the build. The designs are `sdlc/planning/recognize-design.md` and `relate-design.md`; the method record is `experiments/THINKTHEN-RECOGNIZE-MASTER-REPORT.md`. Everything below comes from the QA seat: wave 1 tested the eight shipped functions from the keyboard and the script loop, and the caller review found what the keyboard missed. These are the things a user will hit first, ordered by how often the measurements say they will hit them.

1. **The tuning path is measured but not mapped.** The biggest user-facing levers are known and quantified: the span gate (0.5 default, 0.95 for a precision-priority point at 0.9529/0.30), the per-kind walls (ORG 0.604, MISC 0.484 at corpus scale), and the connector bar. A user whose names are dropped, or whose output is noisy, currently has no page that says which dial to turn and what it costs. Recommendation: the manual ships one symptom-to-dial table (names dropped → lower the gate, with the measured curve; too much noise → raise it; organization-heavy text → expect the wall), and one line saying custom kinds are only as good as their descriptions, because the measured numbers are for the three stock kinds.

2. **The dropped-name diagnosis needs the formula's inputs.** The design promises each word's probabilities under `--details`, and `--threshold 0` returns every candidate. That answers "what did the model see." It does not answer "why did this span fail," because the confidence formula discounts by margin and binds at the weakest member. Recommendation: `--details` on a name carries the formula's inputs — the weakest member word, the margin, and which words were connector-admitted — so the Nathan-der-Weise class of failure is readable after the fact instead of only through `--threshold 0` archaeology.

3. **Two tuned runs must not look identical.** The span-selection policy (probability-weighted with leftmost-longest tie-break, and any future pure leftmost-longest mode) and the formula variant (exclusion versus geometric mean) change answers and are invisible in the output. Recommendation: `meta` under `--details` names the policy and the formula that ran, the way it names the model.

4. **Expose leftmost-longest as a question-file mode.** The closing session priced it: probability weighting won 0.75 to 0.62, so it stays the default, but Ian's own mental model is leftmost-longest and a debugging user wants the predictable policy. Recommendation: one question-file key, documented beside the measured cost of choosing it. This is the "say the word" item; the quality team says the word is worth saying.

5. **One name for the relation number.** `relate-design.md` already suggests it; the quality team endorses: relations print `probability` in both functions, and `confidence` stays on names only, as the one sanctioned exception the vocabulary already documents. Wave 2's parity check will hold every surface to this.

6. **`--threshold` means different things in the two new functions.** In `recognize` it is the name bar (beside `--relation-threshold`); in `relate` it is the edge bar. A script author moving between them misreads it. Recommendation: `recognize` spells `--name-threshold` and `--relation-threshold`; `relate` keeps `--threshold`, its only bar. If the shorter names stay, the help of each says in one line which bar it is.

7. **The offset round-trip is the conformance case, not just a number.** The design rules offsets as characters in the user's text with each surface converting once. The assertion that pins it is `text[start:end] == entity.text` on every surface, with an accented letter and an emoji in the pinned case, because JavaScript's UTF-16 indexing and Rust's byte indexing both bite exactly there. Recommendation: the shared conformance suite carries that assertion verbatim, and each library page documents its conversion in one sentence.

8. **The relate record limit should teach while it refuses.** 255 is a guess borrowed from `find`, which is fine as a guard. Recommendation: the refusal names the limit, the pair arithmetic (N records makes N×(N-1)/2 pairs), and the narrowing move; wave 2 tests 254, 255, and 256 on every surface.

9. **No public price until the packing fix lands, then a fresh measurement.** The 0.02 cents per sentence falls several-fold with the menu-once fix, and the refusal tax inside one full run was about 15 cents on 89. The repository rule already says a number names its record; hold the price pages to it. The refusal-bug fix and the repo tolerance fix are preconditions the quality team echoes from the master report.

10. **Refusal and retry counts belong on the run line, not only in `meta`.** The caller measured about 1 reply in 15 refused. The design counts refused questions in `meta`; the quality team asks for the same visibility the retried-send ruling won for the shipped functions: a finished recognize or relate run prints its refused and retried counts where a script can see them without `--details`.

Wave 2's plan already carries the first cases (offsets over an accent and an emoji, a name at a piece edge, the same name twice, no names, `*` at each end, and the 10/50/200 pair-count growth). Added from this review: the offset round-trip assertion per surface, a connector-style dropped-name case driven through the tuning path, one refused piece inside an otherwise good recognize run, the 254/255/256 relate boundary, and the identical-looking-tuned-runs check from item 3.

Found by experiment 218, ahead of wave 2.
