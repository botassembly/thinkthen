---
title: "Retrieval-augmented decisions"
slug: rad
author: Ian Maurer
date: "2026-09-25"
draft: true
blurb: "Your data sits in no model's memory. Put the facts in the text, then ask the model to decide. On 196 Beatles questions, that moved Jev from 39% to 95%."
---

Your own data sits in no model's memory. Your customers, your contracts, your patients: no model read them. So hand the model the facts in the text, and then ask it to decide. I call that retrieval-augmented decisions. RAD for short. Please use the name.

The idea borrows from retrieval-augmented generation, or RAG. RAG finds the right documents and hands them to a chat model to write an answer. RAD hands the documents to a model that returns a decision and a probability. It writes no prose. Your code has nothing to parse.

## What Jev knows from memory

We built Beatles Bench to find out what Jev knows. Jev is our small, fast model. The bench holds 306 songs and 1,501 questions, and every answer comes from Wikipedia and Wikidata. From memory, Jev gets 68% of the Beatles questions right. Vector search gets 38%. A big chat model gets 96%, and it takes about 8 seconds an answer.

Jev knows the famous facts. It misses the fine ones: a year, a first album, which of two songs runs longer. Ask it whether Hold Me Tight runs longer than Ticket to Ride, and it picks Hold Me Tight at 0.68. That is wrong.

## What the facts fix

On 2026-09-25 we asked Jev 196 questions twice. We drew them mostly from questions it had missed before. From memory, it got 76 right, or 39%. Then we pasted the song catalog in front of each question, one line per song, with its singer, writers, length, and release date. It got 186 right, or 95%.

Hold Me Tight now reads 2:32 and Ticket to Ride reads 3:09. Jev picks Ticket to Ride at 0.99.

The catalog broke one answer. Jev read "The Ballad of John and Yoko" and said two Beatles share the lead. The line says Lennon. The title names two people. That may have fooled it.

## What it costs

The whole catalog costs about 12,000 input tokens a question. Jev costs $42 per billion input tokens, and output is free. So 1,000 answers from memory cost about 1.5 cents. With the whole catalog, they cost about 51 cents.

You rarely need the whole catalog. Send the one entry the question is about, and it costs tens of tokens. In the worked examples, one entry per song turned every miss right.

## What I don't know

This is one bench on one subject. I build ThinkThen, so weigh my Jev numbers with that in mind. The bench is public. Every answer on its pages replays from a saved recording for free. Run it yourself, and tell me where RAD breaks.

The [RAD page](/learn/beatles-bench/rad/) runs one of these questions both ways.
