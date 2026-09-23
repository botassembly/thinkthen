# Question wording helpers, proven on held-out data

## Problem

Small changes in how a question is worded move a model's answers. Three are cheap and well known: state the task twice, add a few worked examples, and, for left-to-right chat models, put the options before the text. Today a user finds these by trial and error, and nothing tells them whether a change helped or only fit their test cases.

## Proposal

Offer each helper as an explicit, opt-in field of the question file:

1. `repeat: true` states the task a second time after the text.
2. `examples:` holds a few labeled cases. ThinkThen places them in the question and never in the text under judgment.
3. `options_first: true` places the options before the text, for any backend that reads a flat prompt (a chat-model backend). The System One API sends the text and the question as separate fields, so it has no effect there.

A helper becomes a default only with evidence. The evidence is a held-out gain larger than noise on a public benchmark, from a pass that tunes on one half and reports the other. The Beatles Bench audit (2026-09-23) runs that test for Jev and for GLM-5.3 Flash. Its result decides each default.

## Tuning loop

Pair the helpers with the harvest issue (`2026-09-23-harvest-the-beatles-and-relate-runs-for-efficiency-thresholds-and-tuning.md`): answers already in the cache let a user compare wordings on their labeled cases for the cost of the new requests only, and a split into tuning and held-out halves keeps the result honest.

## Open

- Whether `examples` count toward the request size limit and the cache key (they should: a changed example is a changed question).
- Whether a chat-model backend belongs in ThinkThen at all. Beatles Bench runs GLM-5.3 Flash beside Jev through a separate script.

Ian can overturn any line.

## Added 2026-09-23: Beatles Bench as the proving ground, and the cache

Ian's direction: if a helper works, use Beatles Bench to design ThinkThen's own wording, including the questions that `tag`, `recognize`, and `relate` write for the user. A changed wording changes the request, so it misses the cache and costs new calls. The old answers stay in the cache, and the two wordings compare side by side. A default must also hold on a second set outside the Beatles (reversal-general, or the site's recorded examples), so it does not fit one band.
