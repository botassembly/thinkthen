---
title: "Why are people excited about Jev?"
slug: why-people-are-excited-about-jev
author: Ian Maurer
date: "2026-10-02"
goal: "Answer a friend's question about why people are excited about Jev, with quotes from three talks, and show how ThinkThen found them."
blurb: "A friend asked me why people are excited about Jev. I asked three talks with ThinkThen. Jev answers bounded questions about text fast and cheap, with no training, so code can call it."
card: /og/why-people-are-excited-about-jev.png
cardAlt: "Why are people excited about Jev? Fast, cheap, no training, and built for code."
links:
  - { href: "/how-tos/search-youtube-transcripts-by-meaning/", text: "Search YouTube transcripts by meaning" }
  - { href: "/functions/rank/", text: "rank" }
---

A friend of mine asked me this on LinkedIn:

> Hey, long time no talk. What are you seeing with Jev? Why are people excited about it? I'm having a hard time understanding what problem it optimally solves.

It's a fair question. Jev is a classifier, and we've had classifiers for decades.

So I asked three talks. One is my own launch talk. The other two are interviews with Diogo Almeida, the CEO of TypeSafe, the company behind Jev. ThinkThen found the passages that answer my friend. Each link below opens the video at the quoted line.

## The short answer

Jev answers a bounded question about text. It answers fast and cheap, and you don't train it. Code can call it on every record.

**It's fast and cheap.** In my talk I called the API "extremely low cost, extremely fast" ([0:50](https://youtu.be/YbzrlpAyCV4?t=50)). You could always ask a large language model these questions. As I put it, "you could always do this with a large language model, it just was slow and expensive" ([18:14](https://youtu.be/YbzrlpAyCV4?t=1094)). Diogo says "Jev is meant to be optimized for intelligence per dollar" ([Latent Space, 6:24](https://youtu.be/cFx9Z3ZXca0?t=384)).

A Latent Space host said Jev took "the faster but cheaper side of the quadrant" ([54:02](https://youtu.be/cFx9Z3ZXca0?t=3242)). Diogo agreed, and he named the catch. He called it "a very [load-bearing] statement while holding intelligence constant. That's the hard part" ([54:14](https://youtu.be/cFx9Z3ZXca0?t=3254)).

**You don't train it.** Jev is zero-shot. In my talk I said that means "you don't have to do what you'd have to do in the past with machine learning models" ([1:57](https://youtu.be/YbzrlpAyCV4?t=117)). You don't label data, train a model, or host it. You write the question in plain words.

**It's built for code.** Diogo describes a "class of models where the goal is for code to be the consumer" ([Latent Space, 5:32](https://youtu.be/cFx9Z3ZXca0?t=332)). On a16z his pitch is a complaint: "my favorite elevator pitch for Jev is where the [expletive] is all the automation?" ([1:41](https://youtu.be/Ut3LOjKNJaE?t=101)). His answer is "[TypeSafe] is making AI for software" ([2:12](https://youtu.be/Ut3LOjKNJaE?t=132)).

TypeSafe also explained it well. In my talk I gave them credit for more than the model: "they've told a good story" ([1:03](https://youtu.be/YbzrlpAyCV4?t=63)).

So what problem does it solve best? Small judgments about text, at volume, inside software. Triage an alert. Route a ticket. Keep the log lines that matter. Pick which model should handle a task. In my talk I said "people are very excited about the idea of … doing model selection" ([16:23](https://youtu.be/YbzrlpAyCV4?t=983)). A big model could always do these jobs. Jev makes them cheap enough to run on every record.

Diogo gave the sharpest version when he compared how models are trained. I found it by reading the transcript, not from the ranking. "[RLHF] is please humans" ([1:23:12](https://youtu.be/cFx9Z3ZXca0?t=4992)). "RLCD is make it reliable for … programmatic use" ([1:23:30](https://youtu.be/cFx9Z3ZXca0?t=5010)). RLCD is TypeSafe's name for how it trains Jev.

## How I found it

These are the three talks:

<!-- video: YbzrlpAyCV4 | Introducing ThinkThen | GenomOncology -->

<!-- video: cFx9Z3ZXca0 | Why I couldn't build Jev at OpenAI | Latent Space -->

<!-- video: Ut3LOjKNJaE | How Jev Turns AI Into Software That Gets Things Done | a16z -->

I pulled each talk's captions with `yt-dlp` and turned them into timed lines. Then I joined the lines into passages and asked ThinkThen to rank them.

Here are the same steps on part of my talk. `paste` joins every ten lines into one passage:

<!-- example: how-tos/search-youtube-transcripts-by-meaning/2-passages -->

`rank` judges every passage and puts the likeliest yes first:

<!-- example: how-tos/search-youtube-transcripts-by-meaning/3-rank -->

The top passage holds the line about large language models being slow and expensive. The next two say you don't need to train it.

Across all three talks, I joined every 20 lines into a passage and got 292 passages. I asked "Does this passage explain why people are excited about Jev?" These are the top seven, with a short quote from each:

| Rank | Talk | Who | Quote |
| --- | --- | --- | --- |
| 1 | [Introducing ThinkThen, 18:14](https://youtu.be/YbzrlpAyCV4?t=1094) | Ian Maurer | "you could always do this with a large language model, it just was slow and expensive" |
| 2 | [Introducing ThinkThen, 1:38](https://youtu.be/YbzrlpAyCV4?t=98) | Ian Maurer | "it's zero shot" |
| 3 | [Latent Space, 54:02](https://youtu.be/cFx9Z3ZXca0?t=3242) | the host | "you've done the faster but cheaper side of the quadrant" |
| 4 | [Latent Space, 52:27](https://youtu.be/cFx9Z3ZXca0?t=3147) | Diogo Almeida | "It is the best thing at intelligence per dollar …" |
| 5 | [Latent Space, 1:35:46](https://youtu.be/cFx9Z3ZXca0?t=5746) | Diogo Almeida | "I'm also anti-demos. I want to make sure that it works reliably." |
| 6 | [Introducing ThinkThen, 0:50](https://youtu.be/YbzrlpAyCV4?t=50) | Ian Maurer | "extremely low cost, extremely fast" |
| 7 | [Latent Space, 12:37](https://youtu.be/cFx9Z3ZXca0?t=757) | Diogo Almeida | "we are back to like early internet energy" |

The a16z talk shows up on a second question: "Does this passage explain what problem Jev solves that LLMs do not?" Diogo's elevator pitch ranked second there.

Each question over all 292 passages took about 0.4 seconds and cost about $0.0026. The [measurement](https://github.com/botassembly/thinkthen/blob/main/site/examples/how-tos/search-youtube-transcripts-by-meaning/files/measured.txt) has each run.

The captions needed care. Automatic captions name no speaker. They spell Jev as "Jeff" and "Jeb", and they hide a swear word. I checked each quote's speaker by reading around it. I quote my own talk from captions I corrected by hand.

## What ThinkThen could do better

This search showed me five gaps. I've filed each one as a request.

1. **Say where a record came from.** `--details` gives the probability but not the input line number. I couldn't pull the passages before and after a hit.
2. **Show the records around a hit**, like `grep -C`. A hit often starts mid-thought. In the run over three talks, the top passage starts with "into their memory".
3. **Print the score in plain output.** Plain `rank` prints the order without the scores. The scores would show where the answers stop being relevant.
4. **Join lines into passages.** ThinkThen leaves the windows to you. `paste` works, but passages split mid-sentence.
5. **Give `rank --details` a value.** It prints `"value": null`. It should print each record's rank.

`filter` was strict here. It kept 4 of the 292 passages at its default cut, as the measurement shows. Use `rank` to explore, and `filter` once you know the cut you want.

## Try it on a talk you care about

The how-to [Search YouTube transcripts by meaning](/how-tos/search-youtube-transcripts-by-meaning/) has every step, from `yt-dlp` to `filter`.
