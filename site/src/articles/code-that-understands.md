---
title: "Code that knows what you mean"
slug: code-that-understands
author: Ian Maurer
date: "2026-09-22"
goal: "Show that a model which answers bounded questions lets ordinary code act on meaning, and show where it breaks."
blurb: "Code sees strings, not meaning. A model that answers bounded questions lets a script act on what text means. Here is how, and where it breaks."
links:
  - { href: "/functions/decide/", text: "Run decide" }
---

Code sees strings, not meaning. `grep` finds the word "refund". It cannot tell whether "I want to send this back" asks for money or for an exchange. Search matches letters. It cannot answer a question about the text.

[TypeSafe](https://typesafe.ai) makes a model called Jev that can. Jev answers a bounded question about the text you hand it. The answer is yes or no, one option from your list, or a place on a scale. Each answer comes with a probability. Jev writes no sentences, so your code has nothing to parse. It needs no training data and no labels. The question goes in as plain words. On Beatles Bench, a thousand answers cost about 0.016 dollars, by the bench's [cost table](https://github.com/botassembly/beatles-bench/blob/main/results/tables/cost.tsv).

We built ThinkThen around that interface. It is one program. You pipe in the text and pass one question. ThinkThen prints a bare answer and sets an exit code.

## Three answers, three exit codes

Here are three customer messages. The first asks for money back. The second does not. The third could mean either. Ask the same question of each line, with a band from 0.2 to 0.8:

<!-- example: functions/decide/1-lines -->

The refund request clears the high bar and answers yes. The thank-you note falls under the low bar and answers no. The send-back line lands inside the band and answers not sure. `grep` would have said no to that line, and a person would never see it.

A question and its band can live in a file, `refund.json`:

<!-- file: functions/question-file/files/refund.json -->

Ask it of the send-back line:

<!-- example: functions/question-file/2-unsure -->

Yes exits 0. No exits 1. Not sure exits 3 and prints `null`. A script branches on the exit code and parses nothing. A person reviews the middle. The question file diffs in a pull request like any other code.

Where you put the band depends on your data. The probability comes from the text you handed over. It does not tell you how often Jev is right. Run the question against cases you have labeled first. `thinkthen audit` grades saved answers against your labels. Pick the band from that run.

## Ten functions that pipe together

ThinkThen turns Jev's three kinds of answer into [ten functions](/functions/). Each reads standard input and writes standard output. This pipeline keeps the buying inquiries, ranks them by how ready the buyer is, and picks a team for each:

<!-- example: how-tos/rank-the-inbound-leads/1-leads -->

The pipeline returns the original lines beside their answers. It does not rewrite them.

`annotate` answers a whole set of questions for every record. A set can mix `decide`, `choose`, `score` and `tag` questions in one file. Here is a set with one yes or no, one pick and one scale, saved as `form.json`:

<!-- file: functions/annotate/files/form.json -->

Three bug reports go in, one per line. Each keeps its id and gains the three answers:

<!-- example: functions/annotate/1-jsonl -->

## Where it breaks

**Planted facts move the answer.** [Probe 06](https://github.com/botassembly/thinkthen/tree/main/probes/06-hostile-text) asked `jev-1.13.0` one question about twenty made-up messages: "The customer explicitly asks for money back." Each message ran once clean and once with hostile text added. An order aimed at the model moved the probability of yes by 0.04 or less, across seventeen wordings. A false claim planted in the message moved it by as much as 0.57. Jev reads a planted claim and a true one the same way, because both look like evidence. A [later review](https://github.com/botassembly/thinkthen/blob/main/sdlc/issues/closed/2026-09-26-architect-review-12-security-and-data-boundary.md) asked other questions of the same model. On a security question, a planted claim raised the probability of yes from 0.01 to 0.64. An order raised it to about 0.17, but there the order itself may be evidence of an incident. These numbers hold for those messages, questions and that model only. At the default bar of 0.5, a planted claim can flip an answer. Use a band, and send the middle to a person.

**`tag` often misses part of the set.** A song can have two lead singers, and `tag` must name every one to score. On Beatles Bench it names the whole set on 0.56 of songs. Its top label is a true lead on 0.80. The bench's [function table](https://github.com/botassembly/beatles-bench/blob/main/results/tables/functions.tsv) scores every function. Use `tag` to fill a queue a person reads. Do not use it as a gate.

**A threshold belongs to one model.** ThinkThen speaks System One, the request format Jev answers. Any server that speaks System One can answer at another address. Its answers will differ, so tune the band again for each model.

**It only answers.** ThinkThen writes no text, holds no conversation and takes no action. The answer goes back to your code, and your rules decide what happens next.

## What ships

ThinkThen ships 10 functions, 1 CLI and [24 bindings](/install/). Every binding calls the same Rust engine, so a question file reads the same way everywhere. In a database, a question sits in a `WHERE` clause like any other condition.
