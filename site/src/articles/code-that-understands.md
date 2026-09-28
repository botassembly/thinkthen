<!-- Goal: show that a model which answers bounded questions lets ordinary code act on meaning, and show where it breaks. -->

# Code that knows what you mean

Every program I have written can match text. None of them can read it. `grep` finds the word "refund". It misses the customer who wants their money back and never says the word. For most of my career, I had to train a model to cross that gap.

We tried that with clinical notes around 2019. Labeling carried the whole cost. The model was stale on the day it shipped. Every new question made us start over. Nobody was happy.

TypeSafe makes a model called Jev. It answers a bounded question about the evidence you hand it: yes or no, one of a list, or a place on a scale. It needs no training data and no labels. It returns an answer and a probability. It never writes a sentence, and your code has nothing to parse. On [Beatles Bench](https://github.com/botassembly/beatles-bench), a thousand answers cost 0.015 dollars.

I built ThinkThen around that interface. It is one binary. You pipe in the evidence and pass one question as an argument. ThinkThen prints a bare answer and sets an exit code.

## `decide`

Here's a support ticket, and here's the whole idea:

<!-- example: functions/decide/3-one -->

The command returns exit code 0. The word "refund" appears in that ticket, and `grep` would have caught this one. The [detailed result](/reference/details/) also carries the model's probability and run facts.

Be careful what you read into that number. The probability describes the evidence you handed over. It doesn't measure how often the model is right. Run the question against cases you have labeled. That run tells you what a probability means for your question. Measure first, then pick a threshold.

## The threshold and the middle

Some messages are plain and some are not. Here are three. The first asks for money back. The second does not. The third could mean an exchange or money back. Ask the same question of each line, with a band from 0.2 to 0.8:

<!-- example: functions/decide/1-lines -->

The refund request clears the high bar and answers yes. The thank-you falls under the low bar and answers no. The send-back line lands inside the band and answers not sure.

You can save the question and its band in a file, `refund.json`:

<!-- file: functions/question-file/files/refund.json -->

Then ask it of the send-back line:

<!-- example: functions/question-file/2-unsure -->

Yes exits 0. No exits 1. Not sure exits 3 and prints `null`. Your script branches on three outcomes without parsing anything. A person reviews the middle. The file diffs in a pull request like any other code.

## Ten functions you can compose

The model answers three kinds of question: yes or no, one of a list, and a place on a scale. TypeSafe calls a yes or no question a Noul. Diogo Almeida, TypeSafe's founder, puts the three in code terms: "Choice maps into a switch statement on an enum. Nouls map to if statements. Scores map to sorting or thresholding."

ThinkThen turns those answers into [ten functions](/functions/). They read standard input and write standard output, and you can pipe them into each other:

<!-- example: how-tos/rank-the-inbound-leads/1-leads -->

The pipeline returns the original lines, each beside its team. It doesn't rewrite them.

I reach for `annotate` most. A question set holds any mix of `decide`, `choose`, `score`, and `tag` questions in one file. `annotate` answers the set for every record. Here is a set with one yes or no, one pick, and one scale, saved as `form.json`:

<!-- file: functions/annotate/files/form.json -->

Three bug reports go in, one per line. Each keeps its id and gains the three answers:

<!-- example: functions/annotate/1-jsonl -->

## Where it breaks

Here's what I'd want to know before I trusted it.

**Measure accuracy on your own data.** The number that matters comes from your own labeled cases. `thinkthen audit` grades saved answers against your labels.

**Planted facts move the answer.** A live run judged twenty made-up messages, once clean and once with hostile text added. On that one question and one model, a command aimed at the model moved the probability of yes by 0.04 or less, in seventeen wordings. A false claim planted about the case moved it by as much as 0.57. The [decide specification](https://github.com/botassembly/thinkthen/blob/main/specification/decide.md) records the run. A [later review](https://github.com/botassembly/thinkthen/blob/main/sdlc/issues/2026-09-26-architect-review-12-security-and-data-boundary.md) asked other questions. On a security question, a command raised the probability of yes from 0.01 to between 0.16 and 0.18, and a planted claim raised it to 0.64. The command itself may read as a sign of an incident. A planted claim moved a `choose` answer from shipping to billing. The tool reads a planted claim and a true one the same way. Both look like evidence to it. At the default cut of 0.5, a planted claim can flip an answer. Use a band and send the middle to a person.

**`tag` has the lowest strict score on Beatles Bench.** A song can have two lead singers, and `tag` must name every one to score. It names the whole set on 0.29 of songs. Its top label is a true lead on 0.75. The bench's [function table](https://github.com/botassembly/beatles-bench/blob/main/results/tables/functions.tsv) scores every function. Use `tag` for a queue a person reads. Don't use it as a gate.

**The tool speaks one interface.** It sends TypeSafe's System One requests. Any server with that interface can answer at another address, and some of its answers will differ. A threshold tuned on one model doesn't carry to another.

**It writes no text, holds no conversation, and runs nothing.** It won't summarize, rewrite, redact, or take an action on its own answer. A judgment goes back to your code, and your rules decide what happens next.

## What ships

The command ships as a single binary with all ten functions. Libraries for Python, TypeScript, Ruby, R, Rust, and C call the same Rust engine. Extensions for DuckDB, SQLite, and PostgreSQL do the same. One engine reads a question file the same way everywhere. In a database, a question reads like any other condition in a `WHERE` clause.

Your code could always match text. Now it can answer a bounded question about the evidence you hand it, from a fixed set of answers you wrote down. That is a narrow thing. It is also the thing I kept training a model to do.
