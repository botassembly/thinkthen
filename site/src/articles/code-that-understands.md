# Code that understands

<!--
Alternate titles:
1. The semantic `if`
2. The classifier you would have trained
-->

Every program I have written can match text. None of them can read it. `grep` finds the word "refund". It misses the customer who wants their money back and never says the word. For most of my career, I had to train a model to cross that gap.

We tried that with clinical notes. Around 2019, we got about 30% accuracy on the extractions that mattered. Labeling carried the whole cost. The model was stale on the day it shipped. Every new question made us start over. It was a terrible business. Nobody was happy.

This month, TypeSafe shipped a model called Jev. It answers a bounded question about the evidence you hand it: yes or no, one of a list, or a place on a scale. It needs no training data and no labels. You give it the evidence and one question. It returns an answer and a probability. It never writes a sentence. Your code has nothing to parse. The vendor lists input at $0.042 per million tokens and output as free. That price is cheap enough to call from a loop.

This is a classifier you do not train. There is no training step. Ask many questions about one piece of evidence and the answers come back in parallel. The probability was the training target. Nothing added it afterward.

I built ThinkThen around that interface. It is one binary. You pipe in the evidence and pass one question as an argument. ThinkThen prints a bare answer and sets an exit code.

## `decide`

Here's a support ticket:

```
I renewed once this morning, but my card shows two charges.
Please refund the duplicate.
```

And here's the whole idea:

```
$ thinkthen decide 'Does the customer ask for a refund?' < ticket.txt
true
```

The command returns exit code 0. The word "refund" appears in that ticket. `grep` would have caught this one. Now ask the same question and show the numbers:

```
$ thinkthen decide --details 'Does the customer ask for a refund?' < ticket.txt | jq .answer
{
  "kind": "yes_no",
  "probability": 0.99
}
```

ThinkThen passes the model's 0.99 through untouched. Every answer carries a probability.

Be careful what you read into that number. The probability describes the evidence you handed over. It doesn't measure how often the model is right. In one measurement of a model like this one, it approved every case at 0.98 while human reviewers had refused 23 percent of them. The evidence those reviewers used was never in the input. Run the question against cases you have labeled. That run tells you what a probability means for your question. Measure first, then pick a threshold.

## The threshold and the middle

Take a vaguer ticket:

```
I was charged twice. Can you fix this?
```

The customer doesn't use the word `refund`. Watch what the same question does:

```
$ thinkthen decide 'Does the customer ask for a refund?' < vague.txt
true

$ thinkthen decide --threshold 0.9 'Does the customer ask for a refund?' < vague.txt
false
```

The model assigns a probability of 0.74. At the default threshold of one half, the answer is yes. At a threshold of 0.9, the answer is no. You set that threshold against your own labeled cases. It holds for one model. A threshold doesn't carry from one model to another. We pointed the tool at a second model once. The question file and the band stayed the same. One model left the sample unresolved and sent it to a person. The other answered yes and routed it.

You can also define an uncertain band in a question file:

```json
{
  "decide": "Does the customer ask for a refund?",
  "true": "The customer asks for money back.",
  "false": "Anything else, such as a question or a complaint.",
  "threshold": "0.2:0.8"
}
```

```
$ thinkthen decide @refund.json < vague.txt
null
exit 3
```

Yes maps to exit code 0. No maps to exit code 1. Not sure maps to exit code 3 and prints `null`. Your script branches on three outcomes without parsing anything. A person reviews the uncertain middle.

That file also diffs in a pull request like any other code.

## Ten functions you can compose

The model answers three kinds of question: yes or no, one of a list, and a place on a scale. Diogo Almeida, TypeSafe's founder, puts them in code terms: "Choice maps into a switch statement on an enum. Nouls map to if statements. Scores map to sorting or thresholding." ThinkThen turns those answers into ten functions. This article shows four of them; the site shows all ten. The functions read standard input and write standard output.

`decide` answers one yes or no. `choose` picks one option from your list. `score` places the evidence on named levels. `tag` names every label that fits. `filter` keeps the records that pass, byte for byte and in their original order. `rank` orders records by how likely a yes is. `find` picks the one line that best answers a question. `annotate` fills out a form of named questions for every record. `recognize` finds the names in the evidence, and `relate` says how records connect.

You can pipe the record-oriented functions into each other:

```
$ cat leads.txt
We need 200 seats next quarter. Please send a quote.
Please remove me from this list.
Our team of six wants to buy today. How do we pay?
Loved your talk at the conference last week.

$ thinkthen filter 'Is this a buying inquiry?' --lines < leads.txt \
    | thinkthen rank 'Is this buyer ready to pay now?' --lines \
    | thinkthen choose 'Which team should take this?' enterprise smb --lines \
    | jq -r '"\(.value)  \(.input)"'
smb  Our team of six wants to buy today. How do we pay?
enterprise  We need 200 seats next quarter. Please send a quote.
```

The pipeline returns the original lines, each beside its team. It doesn't rewrite them.

I reach for `annotate` most. A question set contains any mix of `decide`, `choose`, `score`, and `tag` questions in one file. `annotate` answers the question set for every record:

```
$ thinkthen annotate form.json < report.txt
{"steps":true,"area":"export","impact":1.99}
```

ThinkThen answers three questions in one request.

## What it costs, and where it breaks

I ran the first 3,000 non-empty lines of *Pride and Prejudice* through one question per line: "The line is dialogue spoken aloud by a character." The bill was 3.6 US cents. Short records run about 1.2 cents per thousand. At that price, I can check every record instead of sampling.

Here's what I'd want to know before I trusted it.

**Measure accuracy on your own data.** The number that matters comes from your own labeled cases. The tool ships the `jq` transforms that sweep every threshold against those cases. Our one run on an old public set, 1,000 messages from the UCI SMS Spam Collection on 2026-09-20, got 96.8 percent with the bare question and 98.0 percent with a sentence each for true and false.

**Hostile text moves the answer a little, and planted facts move it a lot.** We wrote twenty made-up customer messages. Each one had a twin with hostile text appended. Seventeen twins carried a direct command to the judge, and every one of those moved the probability by 0.04 or less. Three twins carried a planted false claim about the case instead. Those moved the probability by 0.57, 0.24, and 0.02. At a cut of one half, 19 of 20 answers held. The tool reads a planted claim and a true one the same way. Both look like evidence to it. Twenty messages is a small test. Use a band and send the middle to a person.

**`score` is the weakest function.** One number hides the shape of the answer. Rubric scoring by a model of this kind rejected 18 to 46 percent of work that people had accepted. Our own run placed forty made-up incident reports on five levels. The order held. No report landed more than one level off. The exact level was right on 31 of 40. Use `score` for a queue a person reads. Don't use it as a gate.

**The tool supports one vendor.** One address selects the backend. The published wire shape includes fixtures. We tried a second System One model once, an open one running on a laptop behind a small local server. It answered the tool's exact requests. Ten runs came back clean, and 12 of 16 answers matched the first model. Four differed. Two of those four were wrong. That is one afternoon of evidence. We don't support that model.

**It writes nothing, remembers nothing, and runs nothing.** It won't summarize, rewrite, redact, or take an action on its own answer. A judgment goes back to your code, and your rules decide what happens next.

## What ships and what has not shipped

The command ships as a single binary. `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, and `annotate` are shipped. `recognize` and `relate` are built and not yet shipped. Relations found inside a text are marked a preview. They are the weak part.

The first release is numbered 0.1. The command, every library, and every database extension carry that one number. They all carry the same Rust engine. One engine means a question file means the same thing on every surface. Libraries in Python, TypeScript, Ruby, R, Rust, and C are drawn and not shipped. C opens the door to every other language. A Polars column is meant to cross into the engine once, without a copy. Extensions for DuckDB, SQLite, and PostgreSQL follow the libraries. In those extensions, a question reads like any other condition in a `WHERE` clause.

Your code could always match text. Now it can answer a bounded question about the evidence you hand it, from a fixed set of answers you wrote down. That is a narrow thing. It is also the thing I kept training a model to do.

You can find ThinkThen at [thinkthen.dev](https://thinkthen.dev).

## Sources

Every number above has a record in the repository. Each link is a path in [github.com/botassembly/thinkthen](https://github.com/botassembly/thinkthen).

- The price of $0.042 per million input tokens: [sdlc/records/0011-the-live-probe.md](https://github.com/botassembly/thinkthen/blob/main/sdlc/records/0011-the-live-probe.md). Output is free by the vendor's own published price list, not by our measurement.
- What a probability does not mean, including the 0.98 approvals against 23 percent human refusals: [specification/result.md](https://github.com/botassembly/thinkthen/blob/main/specification/result.md).
- How a threshold and a band are chosen: [specification/threshold.md](https://github.com/botassembly/thinkthen/blob/main/specification/threshold.md).
- The 3.6 cents for 3,000 lines and the 1.2 cents per thousand short records: [sdlc/issues/2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md](https://github.com/botassembly/thinkthen/blob/main/sdlc/issues/2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md).
- The 96.8 and 98.0 percent on 1,000 SMS messages: [sdlc/issues/2026-09-20-accuracy-round-on-three-public-sets-and-a-speed-rerun.md](https://github.com/botassembly/thinkthen/blob/main/sdlc/issues/2026-09-20-accuracy-round-on-three-public-sets-and-a-speed-rerun.md).
- The twenty hostile twins, their cases, their rows, and the analysis: [probes/06-hostile-text/](https://github.com/botassembly/thinkthen/tree/main/probes/06-hostile-text). The reading is in [specification/decide.md](https://github.com/botassembly/thinkthen/blob/main/specification/decide.md).
- The `score` warning and the forty incident reports: [specification/score.md](https://github.com/botassembly/thinkthen/blob/main/specification/score.md) and [probes/03-score/](https://github.com/botassembly/thinkthen/tree/main/probes/03-score).
- The second System One model and what it changed about the band: [sdlc/issues/2026-09-21-a-second-backend-tried-through-the-systemone-adapter.md](https://github.com/botassembly/thinkthen/blob/main/sdlc/issues/2026-09-21-a-second-backend-tried-through-the-systemone-adapter.md).
- The first release numbered 0.1 on every surface: [sdlc/issues/2026-09-20-the-first-release-is-0-1-on-every-surface.md](https://github.com/botassembly/thinkthen/blob/main/sdlc/issues/2026-09-20-the-first-release-is-0-1-on-every-surface.md).
- What version one leaves out: [specification/roadmap.md](https://github.com/botassembly/thinkthen/blob/main/specification/roadmap.md).
