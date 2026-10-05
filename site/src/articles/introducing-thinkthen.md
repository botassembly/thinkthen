---
line: "Ask a bounded question about text and use the typed answer in your code."
title: "Introducing ThinkThen"
slug: introducing-thinkthen
author: Ian Maurer
date: "2026-10-01"
goal: "Introduce ThinkThen: code asks a model one bounded question about text, gets a typed answer back, and acts on it."
blurb: "ThinkThen lets code act on what text means. Ask one bounded question, get yes, no, one option or a number, and branch on it. Ten functions, 24 bindings, MIT."
card: /og/introducing-thinkthen.png
cardAlt: "The talk's title slide: Introducing ThinkThen, October 1, 2026"
youtube: YbzrlpAyCV4
links:
  - { href: "/install/", text: "Install ThinkThen" }
  - { href: "/functions/", text: "The ten functions" }
---

Code sees strings. It can't tell what they mean.

ThinkThen fixes that. Your code asks one bounded question about some text. It gets back a typed answer and acts on it.

## Ask a question, get a value

Here's a customer message. Does it ask for a refund?

<!-- example: functions/decide/3-one -->

The answer is `true`. Your code reads it as a value. There's no prose to parse.

Every answer has a shape you choose up front: yes or no, one option from your list, or a number on your scale.

## Ten functions you already know

I built ThinkThen around shapes every programmer knows. `decide` is an if that understands. `filter` is a grep that understands. `rank` is a sort that understands.

The other seven fill out the set. `choose` picks one option. `tag` names every label that fits. `score` places text on a scale. `find` picks the best line. `annotate` fills out a form. `recognize` finds names. `relate` finds how they connect.

The [functions page](/functions/) shows each one.

## Answers are exit codes

In a script, the answer is also the exit code. Yes exits 0. No exits 1.

Add a band, and the middle gets its own answer. Here the band runs from 0.2 to 0.8. This send-back line lands inside it:

<!-- example: functions/decide/2-case -->

Not sure exits 3, so a person reads it. A failure gets its own [exit code](/learn/answers/#exit-codes), such as 4 when the backend fails. A failure never looks like an answer.

## No training data to start

You don't train a model. You don't label a dataset first. The question is plain words.

You still measure before you trust it. Label some real cases, then run `thinkthen audit`. It grades saved answers against your labels, and you pick the threshold from that. [Test it before you trust it](/trust/) shows how.

## It runs in your language

ThinkThen is one command-line tool and [24 bindings](/install/). Every binding calls the same Rust engine.

The bindings include [DuckDB](/install/duckdb/), [SQLite](/install/sqlite/) and [PostgreSQL](/install/postgresql/). In those, a question sits in a SQL `WHERE` clause like any other condition.

## Cheap and fast

On Beatles Bench, 1,000 Jev answers cost about $0.016. That's from the bench's [cost table](https://github.com/botassembly/beatles-bench/blob/970907659abd5d8cefe1a975bca79b9d1b1c88df/results/tables/cost.tsv).

A live call through a library binding took a median of 138 ms on 2026-10-01. ThinkThen's own work was about 1 to 2 ms of it. From the command line, each run starts a new process and opens a new connection. A live command run took a median of 219 ms. The [overhead page](/learn/overhead/) has every measurement.

## An open standard

ThinkThen speaks System One, the request format [Jev](https://typesafe.ai) answers. Any System One server works.

Jev, from TypeSafe, is the default. Liquid AI's d1 works through the [Liquid backend](/install/backends/liquid/). A local model works through [Ollama](/install/backends/ollama/), with no key. `thinkthen check` tells you whether a server works with ThinkThen.

One caution: a threshold belongs to one model. Switch models, and you measure again.

## Test it like ordinary code

ThinkThen can record the answers a run gets. A test replays them later with no network and no key. [Caching and replay](/learn/caching/) shows how.

That's how this site works. Every example on it is a replayed test, and the build fails when one changes.

## Where it breaks

**Planted facts move answers.** In one [probe](https://github.com/botassembly/thinkthen/tree/2be1777e43c9b31a170107ece00e4587faf00ea7/probes/06-hostile-text), a false claim added to a customer message moved the probability of yes by 0.57. Jev reads a planted claim as evidence. Use a band, and send the middle to a person.

**`tag` misses part of the set.** On Beatles Bench it names exactly the right set of lead singers only 56% of the time. That's from the bench's [function table](https://github.com/botassembly/beatles-bench/blob/970907659abd5d8cefe1a975bca79b9d1b1c88df/results/tables/functions.tsv). Use `tag` to fill a queue a person reads.

**It only answers.** ThinkThen writes no text and takes no action. Your code acts.

## Proven in public

We built [Beatles Bench](/learn/beatles-bench/) to test this in the open. It asks 1,501 questions, most of them about Beatles songs. A script set every right answer from Wikipedia and Wikidata.

Jev gets 70.5% of them right from memory. The [model report](https://github.com/botassembly/beatles-bench/blob/970907659abd5d8cefe1a975bca79b9d1b1c88df/reports/models.md) compares it with other models and with plain search. I build ThinkThen, so weigh that result with care.

## Open source

ThinkThen is [MIT licensed](https://github.com/botassembly/thinkthen/blob/2be1777e43c9b31a170107ece00e4587faf00ea7/LICENSE). I build it at GenomOncology, and the code is on [GitHub](https://github.com/botassembly/thinkthen).

One line installs it:

<!-- install: shell -->

The [tutorial](/learn/tutorial/) asks your first question.
