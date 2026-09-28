# Site decide examples call decide inside a print

Status: Closed 2026-09-26 by the site landing. The Rust, C, and Polars decide samples bind the answer and assert it. No sample calls a print.

Found 2026-09-25 while a talk deck copied site examples line for line. The deck shows whole lines of each example and drops lines it does not need. Its owner asked for code with no print lines.

## What happens

On main `f3176b5d`, three decide examples under `site/src/data/examples/` make or report the call only on a print line:

- `decide__rust.json` line 8: `println!("{:?}: {text}", tt.decide(ask, text)?);`. The call sits inside the print.
- `decide__c.json` line 13: `printf("%d %.2f\n", a.outcome, a.probability);`. The print is the only body of the `if` around `thinkthen_decide`.
- `decide__polars.json` line 9: `print(df.with_columns(refund=tt.decide(ask, df["body"])))`. The call sits inside the print.

A reader who leaves the print line out loses the call, or keeps an `if` with no body. The deck keeps these three prints for that reason.

## Why it matters

A print shows nothing about ThinkThen. It adds a wrapper the reader has to strip. The Python library example `decide.py` in the deck binds the question to a name and asserts each answer, and reads cleaner.

## The fix

Bind each answer to a name, and leave printing out:

- Rust: `let answer = tt.decide(ask, text)?;`
- C: the `if` checks the status and the example ends with `a.outcome` bound or compared, with no `printf`.
- Polars: `df = df.with_columns(refund=tt.decide(ask, df["body"]))`

The other examples that print (`grep -l print site/src/data/examples/*.json`) can follow the same rule where the print carries no meaning.
