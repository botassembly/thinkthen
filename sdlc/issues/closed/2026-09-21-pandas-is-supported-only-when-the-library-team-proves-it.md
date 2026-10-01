# pandas is supported only when the library team proves it

Status: Closed on 2026-09-22. The five checks passed, with the two corrections this page records.

Written 2026-09-21 by the product side. Ian ruled the same day: "I'm good with supporting pandas if it actually works." This page is the thing to track. It replaces the earlier line on the Polars shape page that said pandas is not supported.

## The claim to prove

The library team says pandas needs no new code, because the door reads the Arrow stream form and imports neither library. The record behind that claim is `sdlc/planning/libraries/python.md:45` and local experiment 205's `FINDINGS.md:31`. No run has shown it on the current surface. Until the five checks below pass, no public page says pandas works.

## The five checks

| # | Check | Passes when |
| --- | --- | --- |
| 1 | A default pandas text column, object dtype, goes to `tt.decide` | The answers equal the answers for the same texts as a list. One crossing into Rust, 32 in flight |
| 2 | An Arrow-backed pandas column, `str[pyarrow]`, goes to `tt.decide` | The same answers. The buffer address is the same on both sides, the proof form 205 used |
| 3 | The wall time of check 1 and check 2 against the list form and the Polars form, on the width bench | The numbers are recorded. The manual states the slow one and the fast one plainly |
| 4 | What comes back | The page states the returned type for each of the two pandas columns, and the one line that turns it into a pandas Series. A test runs that line |
| 5 | A frame call, `tt.annotate("form.json", df, on="body")`, with a pandas frame | It either works, with a test, or it refuses with a sentence that names the fix. It never half works |

Also pin: the oldest pandas version the checks ran on, and a test run with pandas absent from the environment, so the library still imports nothing.

## What the product side rules now

- No pandas door is built. No pandas import, no per-value path, no pandas release matrix. The library team recommends against one, and the product side agrees.
- The Python manual page gets one table of three rows: a default column works at list speed, an Arrow-backed column works at full speed, and `astype("str[pyarrow]")` converts once.
- The deck and the social image do not name pandas. Polars is the data frame the copy shows. The answer for a person who asks is one sentence: "Your pandas column works today, and one `astype` line makes it fast." That sentence goes live only after the checks pass.
- Check 5 is the one that can embarrass us. A pandas user's first try will be a whole frame.

## Who holds it

The library team, inside the Polars work, after the surfaces experiment closes. The checks use the stand-in engine and cost nothing. The results go in that experiment's findings page, and this issue gets one closing line that names it.

## What Ian can overturn

All of it. The cheap one: naming pandas on the Python slide once the checks pass.

2026-09-21, closed: the five checks ran on the stand-in inside the surfaces work (branch `surfaces`, `libraries/python/NOTES.md` under "pandas checks"; the experiment's findings page is `FINDINGS.md` at the branch root). Checks 1, 3, and 4 pass on pandas 2.2.3 through 3.0.6. Check 5's bar is met by `1ac9250`: the frame call refuses with both remedies named — pass the column, or convert the returned Arrow frame with the one line the test runs. Two corrections to this page: the spelling `str[pyarrow]` is not valid past 2.2.3 (the valid forms are `string[pyarrow]` and, on pandas 3, the default `str`), and the fast path is pandas-3-only — a pandas 2 column crosses at list speed and still works, one crossing, 32 in flight. The live sentence for the product side should say both.
