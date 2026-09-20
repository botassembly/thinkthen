# ADR 0029: `tag` is any of many

- Status: Accepted by Ian on 2026-09-20
- Date: 2026-09-20

`tag` is the fourth single-judgment type: yes or no, one of many, any of many, and a position on a scale. It takes 1 to 20 ordered labels and asks one yes-or-no question per label in one request. A label description becomes its true criterion. The measured twenty-label request and the improvement from descriptions support this shape.

One single cut applies to every label and defaults to 0.5. The result is one JSON array per input record, including `[]` when no label passes. An empty array succeeds. `tag` gains no raw mode because a list has no unambiguous shell spelling that preserves one output record per input record.

A question file stores `tag` with `labels`, using the list-or-map grammar already used by `choose`. A question set may use the same entry inside `annotate`. Changing or reordering the labels changes the whole request and cache key, and the reference page tells users that answers near the cut can move.

Ian can overturn the limit, threshold, or prompt wording. Changing the output shape would break scripts and needs a new accepted decision.
