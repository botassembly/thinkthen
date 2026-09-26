# Site samples assert on unnamed answers

Status: Closed on branch `site/named-answers-deslop`. Every library and SQL sample names each answer, and `site/scripts/check-samples.mjs` holds the rule.

Found 2026-09-26 while a talk deck brought its library panes in line with a new rule from Ian. Evidence below was read on main at `22f50ab0`.

## Ian's ruling

Ian, 2026-09-26: every code sample stores each ThinkThen answer in a variable named for its meaning before it is used, then asserts on that name. For example, `is_spam = engine.decide(...)`, then `assert is_spam` or `assert not is_spam`. In Bash, `is_spam=$(thinkthen decide ...)`, then a test on "$is_spam". Where the exit code is the lesson, the sample keeps it and names its meaning. Names come from the question's meaning, never `result` or `answer`.

## What happens

The decide samples under `site/examples/functions/decide/` assert, and they print nothing. Most of them break the ruling:

- `rust.rs`: `assert_eq!(tt.decide(&refund, broken)?, Answer::Yes);`. The assert acts on the call.
- `typescript.ts`: `assert.equal(await tt.decide(question, broken), true);`. The assert acts on the call.
- `ruby.rb`: `raise unless ThinkThen.decide(question, broken) == true`. The check acts on the call.
- `c.c`: `thinkthen_answer answer;` and `assert(answer.outcome == THINKTHEN_YES);`. The answer has a generic name.
- `r.R`: `answers <- tt_decide(question, texts)`. The answer has a generic name.
- `polars.py`: `refund=tt.decide(question, tickets["body"]),` inside `tickets.with_columns(`. The answer goes into the frame with no variable first.

The other functions' samples under `site/examples/functions/` likely follow the same shapes.

## The fix

Store each answer under a name for its meaning, then assert on that name. For the refund question:

- Rust: `let broken_asks_refund = tt.decide(&refund, broken)?;`, then `assert_eq!(broken_asks_refund, Answer::Yes);`.
- C: `thinkthen_answer asks_refund;`, then `assert(asks_refund.outcome == THINKTHEN_YES);`.
- R: `asks_refund <- tt_decide(question, texts)`, then `stopifnot(identical(asks_refund, c(TRUE, FALSE)))`.
- Polars: `asks_refund = tt.decide(question, tickets["body"])`, then `assert asks_refund.to_list() == [True, False]`.

A check in the site build can hold the rule. The deck's check fails on an assert or print whose statement holds a ThinkThen call, on a call whose answer goes into no variable, and on a generic answer name.

## Resolution

Every library sample stores each answer under a name for its meaning and asserts on that name. The refund answers are named `is_refund`, after Ian's `is_spam`. Each SQL call carries an alias, and a filter or sort reads the alias. The Bash scripts that use an answer name it first: `refund_code=$?` before `case`, and a function such as `asks_for_refund` before `if`. A Bash transcript that shows the command's own output keeps its form, because its output block is the lesson. `site/WRITING.md` records the rule. `check-samples` fails an assert or print on a call, a SQL call with no alias, a bare `$?` in `case` or `test`, and a generic answer name.
