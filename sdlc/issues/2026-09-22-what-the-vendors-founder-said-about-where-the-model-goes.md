# What the vendor's founder said about where the model goes, and what the engine should do about it

Status: open. Filed 2026-09-22 by the product side from the Latent Space interview with the vendor's CEO (2026-09-21, transcript in the vault). Rulings below are the product side's. Ian can overturn any of them. The architect decides the order inside the 0.1 lane.

## What he said that touches the engine

1. **Every input slot is meant to be structured.** State, instructions, and criteria all take JSON objects, and templating them into strings is "the old way". The model is optimized for nested structure and they are "cooking hard in that direction".
2. **Models will ship fast and are not promised long-term support.** A temporary LTS on jev-1.13.0 is possible because so many people depend on it. Between versions the change is small; between jagged and smooth it is large.
3. **No determinism.** Same input may give slightly different output. Robustness (similar input, similar output) is the property they optimize. A deterministic model is possible and costs intelligence per dollar.
4. **Different model sizes and a cascade by confidence are coming** ("absolutely"). Fine-tuning is "in the cards", not promised.
5. **One state, many ids, one question per id.** His recommended pattern for a long state: pay for the state once and ask a question about each message inside it.
6. **A fourth primitive that is not a decision** is the hinted next launch. "There will be more types and they will map into programming primitives."
7. **No public benchmarks.** Trust comes from measuring your own workflow with your own labeled cases.

## Rulings

1. **Structured descriptions move up.** The issue `2026-09-21-the-question-file-cannot-carry-typesafes-structured-fields.md` was ordered after ticket 0065 and after recognize and relate. It now goes right after 0065 and before recognize and relate, unless the architect finds the recognize build needs it first, in which case it goes first. The measurement is done. Reason: the vendor is optimizing the model for structure, and a 0.1 that only sends strings will read as behind on launch day.
2. **The model name is pinned and a change is an event.** The question file and the cache key already carry the model name. The ruling adds: the tool never silently follows a moving alias. A run with a different model than the question file's pinned one either refuses with a plain message or takes an explicit flag, and the how-to for tuning a question file says to rerun the labeled set on every model change. The architect writes the ticket; it is small.
3. **The adapter table is the seam for a fourth primitive.** No new work now. The ruling is that the specification's adapter table (`specification/backends.md`) and the answer kinds in `result.md` are where a new vendor type lands, and nothing in the command grammar assumes there are exactly three. The architect checks that on the way through A7 (public Rust library shapes) and says so in the record.
4. **Robustness, not determinism, is what the demos test.** A replay demo pins recorded bytes and that is right for the gate. No page promises "same input, same output" from a live call. The wording issue (lane A1) gets one item: any line that implies determinism is cut.
5. **The per-id pattern is already ours.** `annotate` over one document and `find` over lines are that pattern. No new function. The tutorial says it in his words.
6. **A "null goes to a bigger model" how-to.** Exit 3 hands the record to a larger model, with no new function. The marketing side owns the page; the build owns nothing here.

## Not the engine's

The cascade by confidence across model sizes waits until the vendor ships sizes. The item stays here as the reason ruling 2 matters.
