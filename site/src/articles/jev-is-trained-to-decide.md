---
title: "Jev is trained to decide"
slug: jev-is-trained-to-decide
author: Ian Maurer
date: "2026-10-02"
goal: "Answer a friend's question about why people are excited about Jev: it is fast, cheap and better at decisions, because TypeSafe trains it toward calibrated probabilities that code can act on."
blurb: "A friend asked me why people are excited about Jev. It is fast and cheap. It is also better at decisions than a chat model, because TypeSafe trains it to give a calibrated probability your code can act on."
card: /og/jev-is-trained-to-decide.png
cardAlt: "Jev is trained to decide. RLHF chat models please humans. RLVR reasoning models optimize benchmarks. RLCD trains Jev for programmatic use, with a calibrated probability of 0.84 over a threshold of 0.7. Fast, cheap, better at decisions."
links:
  - { href: "/trust/", text: "Test it before you trust it" }
  - { href: "/how-tos/search-youtube-transcripts-by-meaning/", text: "Search YouTube transcripts by meaning" }
---

A friend of mine asked me this on LinkedIn:

> Hey, long time no talk. What are you seeing with Jev? Why are people excited about it? I'm having a hard time understanding what problem it optimally solves.

It's a fair question. Jev is a classifier, and we've had classifiers for decades.

My short answer: Jev is fast, cheap, and better. It's better at the thing large language models do poorly. That thing is making decisions.

## Fast and cheap

In my launch talk I called the Jev API "extremely low cost, extremely fast" ([0:50](https://youtu.be/YbzrlpAyCV4?t=50)). I also said "you could always do this with a large language model, it just was slow and expensive" ([18:14](https://youtu.be/YbzrlpAyCV4?t=1094)).

Diogo Almeida runs TypeSafe, the company behind Jev. He says "Jev will be the name of models that will be on the frontier of intelligence per dollar" ([Latent Space, 6:43](https://youtu.be/cFx9Z3ZXca0?t=403)).

Speed and cost matter. The bigger reason is the third one.

## Chat models are trained to please you

A chat model learns from people's feedback. People rate its answers, and the model learns to give the answers people like. That training is called RLHF. Diogo puts it in three words: "[RLHF] is please humans" ([Latent Space, 1:23:12](https://youtu.be/cFx9Z3ZXca0?t=4992)).

Pleasing people has a cost. A Latent Space host asked whether chat models are "collapsing towards what you want to hear the most … instead of like their own internal confidence about a thing" ([7:15](https://youtu.be/cFx9Z3ZXca0?t=435)). Diogo answered with the problem he says "no one paid attention to": "the downsides of [RLHF], in particular, mode dropping" ([7:39](https://youtu.be/cFx9Z3ZXca0?t=459)).

Mode dropping means the model stops giving the rare answers. It "drop[s] the minority classes and just do[es] the really common ones" ([9:22](https://youtu.be/cFx9Z3ZXca0?t=562)). The model still sounds sure. Its sense of how sure it should be is gone. Diogo calls the effect "total poison" for "the probability distributions" ([9:47](https://youtu.be/cFx9Z3ZXca0?t=587)). His conclusion: "this is why strings are so bad at … [decision-making]" ([9:58](https://youtu.be/cFx9Z3ZXca0?t=598)).

Reasoning models have their own goal. "RLVR is optimized benchmarks" ([1:23:17](https://youtu.be/cFx9Z3ZXca0?t=4997)). They learn to get checkable answers right.

## Jev is trained to decide

TypeSafe picked a different goal. "RLCD is make it reliable for … programmatic use" ([1:23:30](https://youtu.be/cFx9Z3ZXca0?t=5010)). The reader of Jev's answer is your code.

Jev hands back a probability. A probability is useful only when it's calibrated. Calibrated means that when Jev says 0.8, it's right about 8 times in 10. Then your code can act on the number:

- Set a threshold.
- Branch on it.
- Send the cases Jev isn't sure about to a person.

In my talk I put it this way. You want "the computer to only take action when it's very confident in the … answer that it's rendering" ([5:37](https://youtu.be/YbzrlpAyCV4?t=337)).

Diogo sets a high bar for this. "The highest honor of reliability will be to get to the point where people can program against Jev without making example queries" ([a16z, 27:43](https://youtu.be/Ut3LOjKNJaE?t=1663)).

## An example: which tool to call next

An agent has a set of tools. At each step it has to pick one. In my talk I listed this as a use for decision models. They can give agents "guardrails like permissions or tool choice" ([16:17](https://youtu.be/YbzrlpAyCV4?t=977)).

Ask a chat model to pick, and it writes back a tool name in a sentence. It sounds just as sure on a hard step as on an easy one. Your code can't tell the difference.

Ask Jev to pick, and it gives each tool a probability. Say the search tool gets 0.84 and your threshold is 0.7. Your code calls search. Say no tool clears 0.7. Your code stops and asks a person. That check takes milliseconds and costs a fraction of a cent. That's "how you make those decisions quick and cheaply" ([16:36](https://youtu.be/YbzrlpAyCV4?t=996)).

## Is that good?

I think it is. Most of the work I want to automate is a chain of small decisions. Each decision needs an honest answer and an honest sense of how sure it is. A model trained to please me can't give me the second part. A model trained to be calibrated can.

Calibration is still a claim until you check it on your own cases. Label some real examples and measure before you pick a threshold. [Test it before you trust it](/trust/) shows how.

## The three talks

<!-- video: YbzrlpAyCV4 | Introducing ThinkThen | GenomOncology -->

<!-- video: cFx9Z3ZXca0 | Why I couldn't build Jev at OpenAI | Latent Space -->

<!-- video: Ut3LOjKNJaE | How Jev Turns AI Into Software That Gets Things Done | a16z -->

I asked ThinkThen to rank the transcripts. The how-to [Search YouTube transcripts by meaning](/how-tos/search-youtube-transcripts-by-meaning/) shows the method. The ranking found the Latent Space lines at 6:43, 7:15, 9:58, 1:23:12 and 1:23:17, the a16z line at 27:43, and my line at 5:37. I found the other lines by reading the transcripts.

Automatic captions name no speaker, and they spell Jev as "Jeff" and "Jeb". I checked each speaker by reading around the quote. Words in brackets fix a caption error or fit a quote into my sentence. An ellipsis marks words I cut. I quote my own talk from captions I corrected by hand.
