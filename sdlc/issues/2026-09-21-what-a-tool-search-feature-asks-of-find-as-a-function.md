# What a tool-search feature asks of `find` as a function

Status: Open

Ian pointed on 2026-09-21 at an open-source adapter for the Model Context Protocol that shipped a search over its tools the same day. A user describes a job in words, and the adapter names the right tool out of about a hundred. It calls the System One backend directly through the vendor's SDK, from a long-lived Node process. A read-only survey of its code shows what a real caller needs from the pick-one verbs as library functions. This page authorizes nothing. It feeds the ADR 0017 rewrite and step 2 of the engine plan.

## What the feature does

It sends one pick-one request per search. Every candidate tool is an option with a description cut to 512 bytes, and a `none` option sits beside them. It caps the list at 127 candidates. It reads the whole probability list: it ranks the tools by probability, and it answers "nothing fits" when `none` wins, when `none` ties the top, or when the top falls under 0.2. On a timeout, a rate limit, or an unavailable service it falls back to its own keyword ranker. On a malformed reply it reports the error.

In this project's words the feature is `find --none --details`: a pick over data units, where `choose` picks over labels the author wrote.

## What it asks of the function surface

1. **`find` returns the ranking.** A caller wants the top five with their probabilities. The winner alone does not serve. `--details` already carries every probability in input order, and the function's result needs the same list.
2. **A floor on the winner.** The feature refuses a top pick under 0.2. `find` has `--none` and no threshold today. The specification says the live run gave no error to choose a floor from. A library caller can apply its own floor if the result carries the probabilities, and point 1 covers it.
3. **The error says whether a second try could help.** The feature falls back on a timeout, a rate limit, and an unavailable service, and it stops on a reply it cannot trust. The engine plan's kinds are usage, backend, local, cancelled, and defect. One `backend` kind hides that difference. The error needs a way to tell a busy backend from a refused reply.
4. **A deadline for one call.** The feature gives every search 5 seconds. A host that answers a person needs a per-call deadline beside the cancel token.
5. **Descriptions on units.** The feature sends a name and a description for each candidate. `find --jsonl` sends the whole record or the `--field` part, and that covers it.

## What stays the caller's

Choosing which 127 candidates to send out of a larger set, storing the key, per-script spending budgets, and the keyword fallback. None of it belongs in the engine.

## What the library would replace

About 165 of the feature's roughly 900 lines: building the request, checking the reply, and mapping timeouts and errors. It would add two things the feature lacks: answers saved on disk for equal requests, and recorded replies for tests in place of a hand-written network mock. `find` also accepted 255 units in the ticket 0040 reach check, against the feature's cap of 127.

## What Ian can overturn

All of it. Points 3 and 4 are the two that change the engine plan.
