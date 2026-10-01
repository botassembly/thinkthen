---
title: "Retrieval-augmented decisions"
slug: rad
author: Ian Maurer
date: "2026-09-25"
draft: true
goal: "Show that putting the facts in the text turns a model's wrong answers into right ones."
blurb: "Your data sits in no model's memory. Put the facts in the text, then ask the model to decide. Beatles Bench shows what the facts fix."
---

No model has read your customers, your contracts or your patients. Hand the model those facts in the text, then ask it to decide. I call that retrieval-augmented decisions, or RAD.

The name borrows from retrieval-augmented generation, or RAG. RAG finds the right documents and hands them to a chat model. The chat model writes an answer. RAD hands them to a model that returns a decision and a probability. ThinkThen sends the documents as `state`.

## What Jev knows from memory

Jev is TypeSafe's model. We built Beatles Bench to find out what it knows. A script took every right answer on the bench from Wikipedia and Wikidata. The [full results](https://github.com/botassembly/beatles-bench/blob/main/reports/results.md) give each run.

Jev knows the famous facts. It misses the fine ones, such as a release date, a first album, or which of two songs runs longer. The [open-book report](https://github.com/botassembly/beatles-bench/blob/main/reports/open-book.md) counts the misses by topic. Ask it whether A Day in the Life is on Abbey Road. With a band of 0.1 to 0.9, Jev still says yes. That's wrong.

<!-- example: beatles/rad/1-memory -->

## What the facts fix

Put the song's catalog entry in front of the question and ask again. The entry names Sgt. Pepper's Lonely Hearts Club Band as the first album. Under the same band, Jev now says no.

<!-- example: beatles/rad/2-context -->

The bench asked this way at scale. It drew 196 questions, mostly from Jev's misses, and asked each one twice. The first time, Jev answered from memory. The second time, the whole song catalog sat in front of the question, one line per song, with its singer, writers, length, and release date. In the [run of 2026-09-26](https://github.com/botassembly/beatles-bench#results), Jev got 68 of those 196 right from memory, and 184 of 196 with the catalog. That sample leans on misses. The first run, of 2026-09-23, measured Jev at 68% from memory on all 1,075 questions the catalog covers. The [open-book report](https://github.com/botassembly/beatles-bench/blob/main/reports/open-book.md) weights that run back to the whole set and estimates about 97% with the catalog.

The catalog also broke an answer. Asked whether two Beatles share the lead on "The Ballad of John and Yoko", Jev said no from memory, and that is right. With the catalog, it said yes at 0.82, in the [open-book run](https://github.com/botassembly/beatles-bench/tree/main/results/runs/2026-09-26-thinkthen-jev-open-book). The catalog line names Lennon. The title names two people. That may have fooled it.

## What it costs

Context costs input tokens. In the [open-book report](https://github.com/botassembly/beatles-bench/blob/main/reports/open-book.md), the whole catalog of 306 songs took a median of 12,214 input tokens a call. The same report asked 38 lead singer questions with only the song's own catalog line. In the runs of 2026-09-23 and 2026-09-24, Jev got 36 of 38 right with the one line, at a median of 335 input tokens. With the whole catalog, it got 35 at 12,145. Send the one entry the question is about.

## Limits

This is one bench on one subject. We build ThinkThen, so weigh our Jev results. The bench is public. Every answer on its pages replays from a saved recording for free. Run it yourself, and tell me where RAD breaks.

The [RAD page](/learn/beatles-bench/rad/) runs this question both ways.
