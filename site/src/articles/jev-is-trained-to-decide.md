---
title: "Why are people excited about Jev?"
slug: jev-is-trained-to-decide
author: Ian Maurer
date: "2026-10-02"
goal: "Explain why people are excited about Jev: it is faster and cheaper, and the big bet is that a model trained for calibrated decisions beats one trained to please people."
blurb: "Jev is faster and cheaper, and potentially better. The big bet: software needs a model trained to make calibrated decisions, and chat models were trained to please people."
card: /og/jev-is-trained-to-decide.png
cardAlt: "Jev is trained to decide. RLHF chat models please humans. RLVR reasoning models optimize benchmarks. RLCD trains Jev for programmatic use, with a calibrated probability of 0.84 over a threshold of 0.7. Fast, cheap, better at decisions."
links:
  - { href: "/how-tos/search-youtube-transcripts-by-meaning/", text: "Search YouTube transcripts by meaning" }
---

People are excited about Jev because it is faster and cheaper, and potentially better. You can measure faster and cheaper today. Better is a bet.

Here is the bet. Large language models were trained with a human in the loop. They learned to give answers people like. Agents, automation and software need something else. They need a model optimized for making decisions. TypeSafe trains Jev for that. Nobody has proven yet that this approach wins. That is the big bet, and it is why people are excited.

<!-- video: YbzrlpAyCV4 | TT | ThinkThen introduction | 2026-10-01 | 24:38 -->
<!-- video: cFx9Z3ZXca0 | LS | Latent Space Pod | 2026-09-21 | 2:22:22 -->
<!-- video: Ut3LOjKNJaE | a16z | a16z Pod | 2026-09-28 | 42:24 -->

## Fast and cheap

In my launch talk I called the Jev API "extremely low cost, extremely fast" ([TT 0:50](https://youtu.be/YbzrlpAyCV4?t=50)). I also said "you could always do this with a large language model, it just was slow and expensive" ([TT 18:14](https://youtu.be/YbzrlpAyCV4?t=1094)).

Diogo Almeida runs TypeSafe, the company behind Jev. He says "Jev will be the name of models that will be on the frontier of intelligence per dollar" ([LS 6:43](https://youtu.be/cFx9Z3ZXca0?t=403)).

You can check speed and cost today. The bet is about how the model is trained.

## Chat models are trained to please you

A chat model learns from people's feedback. People rate its answers, and the model learns to give the answers people like. That training is called RLHF. Diogo puts it in three words: "[RLHF] is please humans" ([LS 1:23:12](https://youtu.be/cFx9Z3ZXca0?t=4992)).

Pleasing people has a cost. A Latent Space host asked whether chat models are "collapsing towards what you want to hear the most … instead of like their own internal confidence about a thing" ([LS 7:15](https://youtu.be/cFx9Z3ZXca0?t=435)). Diogo answered with the problem he says "no one paid attention to": "the downsides of [RLHF], in particular, mode dropping" ([LS 7:39](https://youtu.be/cFx9Z3ZXca0?t=459)).

Mode dropping means the model stops giving the rare answers. Diogo compares it to GANs, an older kind of image model. GANs "drop the minority classes and just do the really common ones" ([LS 9:22](https://youtu.be/cFx9Z3ZXca0?t=562)). The chat model still sounds sure. Its sense of how sure it should be is gone. Diogo calls the effect "total poison" for "the probability distributions" ([LS 9:47](https://youtu.be/cFx9Z3ZXca0?t=587)). His conclusion: "this is why strings are so bad at … [decision-making]" ([LS 9:58](https://youtu.be/cFx9Z3ZXca0?t=598)).

Reasoning models have their own goal. "RLVR is optimized benchmarks" ([LS 1:23:17](https://youtu.be/cFx9Z3ZXca0?t=4997)). They learn to get checkable answers right.

## Jev is trained to decide

TypeSafe picked a different goal. "RLCD is make it reliable for … programmatic use" ([LS 1:23:30](https://youtu.be/cFx9Z3ZXca0?t=5010)). The reader of Jev's answer is your code.

Jev hands back a probability. A probability is useful only when it's calibrated. Calibrated means that when Jev says 0.8, it's right about 8 times in 10. Then your code can act on the number:

- Set a threshold.
- Branch on it.
- Send the cases Jev isn't sure about to a person.

In my talk I put it this way. You want "the computer to only take action when it's very confident in the … answer that it's rendering" ([TT 5:37](https://youtu.be/YbzrlpAyCV4?t=337)).

Diogo sets a high bar for this. "The highest honor of reliability will be to get to the point where people can program against Jev without making example queries" ([a16z 27:43](https://youtu.be/Ut3LOjKNJaE?t=1663)).

## An example: which tool to call next

An agent has a set of tools. At each step it has to pick one. In my talk I listed this as a use for decision models. They can give agents "guardrails like permissions or tool choice" ([TT 16:17](https://youtu.be/YbzrlpAyCV4?t=977)).

Ask a chat model to pick, and it writes back a tool name in a sentence. It sounds just as sure on a hard step as on an easy one. Your code can't tell the difference.

Ask Jev to pick, and it gives each tool a probability. Say the search tool gets 0.84 and your threshold is 0.7. Your code calls search. Say no tool clears 0.7. Your code stops and asks a person. That check takes milliseconds and costs a fraction of a cent. That's "how you make those decisions quick and cheaply" ([TT 16:36](https://youtu.be/YbzrlpAyCV4?t=996)).

## Will the bet pay off?

I think it will. Most of the work I want to automate is a chain of small decisions. Each decision needs an honest answer and an honest sense of how sure it is. A model trained to please me can't give me the second part. A model trained to be calibrated can.

Calibration is still a claim until you check it on your own cases. Label some real examples and measure before you pick a threshold. [Test it before you trust it](/trust/) shows how.

## How I found these quotes

I put this article together with ThinkThen. The how-to [Search YouTube transcripts by meaning](/how-tos/search-youtube-transcripts-by-meaning/) shows the search. ThinkThen found the lines from my talk about speed, cost and tool choice. It found Diogo's a16z line about programming against Jev. It pointed me to the Latent Space passage about intelligence per dollar. I found the training quotes and my line about confidence by reading the transcripts.

Words in brackets fix a caption error or fit a quote into my sentence. An ellipsis marks words I cut.
