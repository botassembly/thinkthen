# Windows over long text

Status: Closed on 2026-09-25 after a check against main. Ticket 0059 closed it as a refusal with a clear message (build-queue-2026-09-21.md A2). Earlier status: Open

Ian asked on 2026-09-21 what a windowed reading of a long document should cost and look like: given a big chunk of text, the caller windows through it, and each window could carry some text before and some after as context. The record answers the numbers and leaves the shape open.

What the record holds:

- The bill is the request, not the words. A short record bills about 290 input tokens, of which about 256 is fixed per-request overhead (`sdlc/issues/2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md:76-85`), at $0.042 a million input tokens (`sdlc/records/0011-the-live-probe.md:11`). About 1.2 cents a thousand judgments.
- Evidence rides one request at about 32,000 tokens and one whole request at about 64,000, measured live: a 33,663-token request returned 200, and the packing probe found no lower ceiling (`specification/records.md:140-145`).
- The tool never splits text. Evidence is one string (`specification/backends.md:91`), and no sentence or word segmentation exists anywhere. A retired verb `segment` once cut documents at yes/no boundaries and was held off the roadmap (`specification/roadmap.md:51`).
- Windowing is upstream today: "An endless stream has to be cut into windows upstream" (`specification/rank.md:21`). `find` takes the other shape: its whole bounded set of 2 to 255 units rides one aggregate request (`probes/find-0040/README.md:18-21`).

The cost math for Ian's shape: the before-and-after context is simply more characters in the one evidence string, and 100 extra tokens cost about four millionths of a dollar. The floor is per request, so fewer, richer windows beat more, thinner ones; the limit on richness is the 32,000-token evidence ceiling, shared by the window, its context, and the questions.

The open question: does a windowed reading earn a verb on the roadmap, and if so, does the window own its context — the before-and-after strings inside one request, with the window size set in the caller's unit and checked against the token ceiling at usage time? Or does windowing stay the caller's job, with the tool only promising the ceilings and the floor honestly?

What Ian can overturn: all of it. The cheap overturn is doing nothing: the ceilings and the floor are already promised, and the caller can window today.
