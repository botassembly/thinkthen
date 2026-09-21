# The Polars shape as the deck shows it

Written 2026-09-21 by the product side. Ian ruled the same day that Python's data frame is Polars, that plain Python lists stay first-class, and that all scaling runs in Rust code. The engineering plan is `sdlc/planning/polars-plan.md`. This page rules what a user types. The deck's new slide `15a-polars` is drawn from it, and that slide is the acceptance test.

## What a user types

One rule: **what goes in decides what comes out.** No new function names and no `polars` submodule.

| The text argument is | The function returns | Door in the plan |
| --- | --- | --- |
| A string | One answer | As today |
| A list of strings | A list of answers | As today |
| A Polars Series, `df["body"]` | A Polars Series of the same length | The Series door |
| A Polars expression, `pl.col("body")` | A Polars expression | The plugin expression |
| A Polars DataFrame with `on="body"` | A Polars DataFrame | The Series door |

- `decide` returns a Boolean column. `None` is "not sure".
- `choose` and `find` return a string column. `score` returns a float column. `tag` returns a list-of-strings column.
- `annotate` on a DataFrame adds one column per question in the form.
- `recognize` on a DataFrame returns a long frame: one row per name, with the source row number. `relate` returns one row per edge.
- `filter` and `rank` take a DataFrame with `on=` and return a DataFrame.
- The install line is `pip install thinkthen[polars]`. Plain `pip install thinkthen` still works, and lists still work without Polars.
- pandas is not supported and no page names it.

## The slide's code

```python
df = df.with_columns(complaint=tt.decide(ask, df["body"]))

urgency = tt.score("How urgent?", pl.col("body"), levels)
lazy.with_columns(urgency=urgency).collect()
```

The first call needs only the Series door. The second call needs the plugin expression, and the plan marks that door as unproven until its experiment 216 runs. If that experiment stops the plugin, the second half of the slide comes off and the first half stands alone. Nothing else in the deck depends on it.

## What the public copy may say

- "Rust reads the column where it sits, with no copy and no Python loop." This holds only after the plan's width measurement reads the same through a column as through a list. Until then the slide is a drawing of the finished product, like every surface slide.
- The overview slide says the functions run "in six languages, on Polars data frames, and inside three database engines". The surface count stays nine. Polars is a form of the Python library and of the Rust library, never a tenth surface.
- The social overview image lists Polars beside the three databases.

## Open for the build and library teams

1. Whether the Rust crate gets the same Series door behind a `polars` feature. The product side says yes, because a Rust user of Polars is the cheapest user to serve.
2. How a cancel reaches a call inside a lazy query. The plan names the deadline as the lever.
3. Whether `--dry-run` has a Polars form that prints the request count for a column before anything is paid.

## What Ian can overturn

All of it. The cheap ones: the one-rule shape against a `tt.polars` submodule, the optional install against a hard dependency, and the expression half of the slide.
