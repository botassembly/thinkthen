---
title: "Retrieval-augmented decisions"
slug: rad
author: Ian Maurer
date: "2026-09-25"
draft: true
goal: "Show that putting the facts in the text turns a model's wrong answers into right ones."
blurb: "Your data sits in no model's memory. Put the facts in the text, then ask the model to decide. On Beatles questions Jev missed from memory, the song catalog turned most answers right."
---

Your own data sits in no model's memory. Your customers, your contracts, your patients: no model read them. So hand the model the facts in the text, and then ask it to decide. I call that retrieval-augmented decisions, or RAD.

The idea borrows from retrieval-augmented generation, or RAG. RAG finds the right documents and hands them to a chat model to write an answer. RAD hands the documents to a model that returns a decision and a probability. Your code has nothing to parse.

## What Jev knows from memory

We built Beatles Bench to find out what Jev knows. Jev is TypeSafe's small, fast model. Every answer on the bench comes from Wikipedia and Wikidata. From memory, Jev gets more of the Beatles questions right than vector search does. A big chat model gets more right, and it takes far longer to answer.

Jev knows the famous facts. It misses the fine ones: a year, a first album, which of two songs runs longer. Ask it whether A Day in the Life is on Abbey Road. Jev says yes at 0.94. That is wrong.

<!-- example: beatles/rad/1-memory -->

## What the facts fix

Put the song's catalog entry in front of the question and ask again. The entry names Sgt. Pepper's Lonely Hearts Club Band as the first album. Jev now says no at 0.04.

<!-- example: beatles/rad/2-context -->

The bench asked the same way at scale. It took questions Jev had mostly missed and asked each one twice. The first time, Jev answered from memory. The second time, the whole song catalog sat in front of the question, one line per song, with its singer, writers, length, and release date. The catalog turned most of the misses right.

The catalog broke one answer. Jev read "The Ballad of John and Yoko" and said two Beatles share the lead. The line says Lennon. The title names two people. That may have fooled it.

## What it costs

Context costs input tokens. From memory, the call above reads 291 input tokens. With the one entry, it reads 368. The whole catalog costs far more.

You rarely need the whole catalog. Send the one entry the question is about.

## What I don't know

This is one bench on one subject. I build ThinkThen. Weigh my Jev results with that in mind. The bench is public. Every answer on its pages replays from a saved recording for free. Run it yourself, and tell me where RAD breaks.

The [RAD page](/learn/beatles-bench/rad/) runs this question both ways.
