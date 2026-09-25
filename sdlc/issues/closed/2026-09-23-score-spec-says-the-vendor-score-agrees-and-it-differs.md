# The score spec says the vendor's score agrees, and it differs by up to 0.02

Status: Closed by ticket 0087 in `37746b36`. `specification/score.md` line 25 now says the two fields can differ and names the tool's number as the one to trust.

`specification/score.md` line 25 says the vendor's own score field "is the same weighted average, so the two agree." Experiment 235 (2026-09-23, Beatles songs, one score question over 20 songs) compared the two. The tool's score differed from the vendor's field on 13 of 20 songs, by up to 0.02.

The tool's behavior is right. It computes the score from the probabilities it returns, so the row stays consistent. The sentence is wrong. The vendor's field may use unrounded or differently normalized probabilities.

## The fix

Replace "so the two agree" with a sentence that says the two can differ by a few hundredths and names the tool's number as the one to trust. Check whether the spec's tests assert agreement anywhere.

Severity: minor, spec wording only. Found by the product side. Evidence: `experiments/235-beatles-judgment/runs/` (the score run and its full details).
