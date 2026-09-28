---
title: "Retrieval-augmented decisions"
slug: rad
author: Ian Maurer
date: "2026-09-25"
draft: true
goal: "Show that putting the facts in the text turns a model's wrong answers into right ones."
blurb: "Your data sits in no model's memory. Put the facts in the text, then ask the model to decide. Beatles Bench shows what the facts fix."
---

Your own data sits in no model's memory. Your customers, your contracts, your patients: no model read them. So hand the model the facts in the text, and then ask it to decide. I call that retrieval-augmented decisions, or RAD.

The idea borrows from retrieval-augmented generation, or RAG. RAG finds the right documents and hands them to a chat model to write an answer. RAD hands the documents to a model that returns a decision and a probability. Your code has nothing to parse.

## What Jev knows from memory

We built Beatles Bench to find out what Jev knows. Jev is TypeSafe's model. A script set every right answer on the bench from Wikipedia and Wikidata. The [full results](https://github.com/botassembly/beatles-bench/blob/main/reports/results.md) give each run.

Jev knows the famous facts. It misses the fine ones: a year, a first album, which of two songs runs longer. Ask it whether A Day in the Life is on Abbey Road. Jev says yes, even inside a band of 0.1 to 0.9. That is wrong.

<!-- example: beatles/rad/1-memory -->

## What the facts fix

Put the song's catalog entry in front of the question and ask again. The entry names Sgt. Pepper's Lonely Hearts Club Band as the first album. Under the same band, Jev now says no.

<!-- example: beatles/rad/2-context -->

The bench asked the same way at scale. It drew 196 questions, mostly from Jev's misses, and asked each one twice. The first time, Jev answered from memory. The second time, the whole song catalog sat in front of the question, one line per song, with its singer, writers, length, and release date. From memory, Jev got 68 right. With the catalog, it got 184, in the [run of 2026-09-26](https://github.com/botassembly/beatles-bench#results).

The catalog broke an answer on "The Ballad of John and Yoko". Asked whether two Beatles share the lead, Jev said no from memory, and that is right. With the catalog, it said yes at 0.82, in the [open-book run](https://github.com/botassembly/beatles-bench/tree/main/results/runs/2026-09-26-thinkthen-jev-open-book). The line says Lennon. The title names two people. That may have fooled it.

## What it costs

Context costs input tokens. In the [open-book report](https://github.com/botassembly/beatles-bench/blob/main/reports/open-book.md), the whole catalog of 306 songs took a median of 12,214 input tokens a call. Send the one entry the question is about.

## What I don't know

This is one bench on one subject. I build ThinkThen. Weigh my Jev results with that in mind. The bench is public. Every answer on its pages replays from a saved recording for free. Run it yourself, and tell me where RAD breaks.

The [RAD page](/learn/beatles-bench/rad/) runs this question both ways.
