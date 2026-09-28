# A recording folder that predates the backend marker cannot be extended

Status: Open. Filed 2026-09-27 from the GEPA tuning experiments 296 and 297. Evidence lives in `~/workspace/experiments/297-gepa-loop-tests/`; the full write-up is `notes/2026-09-27-optimization-lessons.md`.

## What happens today

A recording folder holds `.thinkthen-backend.json`, one identity derived from the adapter name and the endpoint URL. A folder that holds entries but no marker is refused on write: `the recording folder predates backend binding; replay it read-only or choose a new folder`. Such a folder can be replayed, and it cannot be extended with new answers.

## Why it matters

The cheapest baseline for a tuning round is a previous run's answers. A loop copies a recorded run, replays the seed for free, and sends only the changed wordings live. That is exactly the folder the tool refuses to extend. A loop can also run into the refusal for a second reason: the request digest covers the model string, so a run recorded with `jev-latest` misses when the loop names `jev-1.13.0`, and the copied folder is unusable either way.

## Measured

Experiments 296 and 297. The bench's 2026-09-26 recordings, 1,501 entries, were copied into the experiment folder. The marker was written by hand as `sha256("systemone\nhttps://api.typesafe.ai/v1/systemone")` to make the folder usable. After that the seed replayed for free: 279 live requests for experiment 296 and 710 for 297, instead of a baseline call for every one of the roughly 1,100 seed cases across both experiments. Experiment 296 lost its first replay attempt to the missing marker, and the first recording-folder error named only read-only replay as the alternative.

## What to change

One of two additive forms:

1. On first write, publish the marker when every existing entry records the same adapter and endpoint as the folder being bound, and refuse a folder whose entries disagree.
2. A `cache bind` subcommand that does the same thing deliberately and reports what it found before writing the marker.

Either form widens what works. No existing folder changes behavior, and a mixed folder stays refused.

## Factual preparation, 2026-09-28

At main `e58aceae`, `Entry` stores the adapter, URL and request, and `Entry::inspected` can recompute the exchange digest. The filename digest by itself does not reveal the old endpoint. Matching stored fields and digest establishes internal consistency, not authenticated origin: another writer of an explicit folder can forge both. The marker gate, safe publication and read-only replay behavior remain necessary. Experiment 296's model-alias miss is a separate complete-request identity; binding a marker cannot make it a cache hit. The recovery choice and a temporary mixed-folder/concurrent-binder proof remain open. See `sdlc/records/2026-09-28-tuning-loop-intake-preparation.md`.
