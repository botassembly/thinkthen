# Invalid answer distributions are accepted

Status: Open

Found 2026-09-20 in the full-project review at `2c32524` and confirmed again after ticket 0024.

The `systemone` decoder checks each probability separately and checks that every requested label has a value. It never checks that the values total one, and it ignores keys for labels the question never sent.

A score reply with three probabilities of `1.0` is accepted. The tool computes a score of `3.0` on three levels numbered 0 through 2, outside the scale it documents. A choice reply can likewise claim full probability for several options and still produce a winner. An extra wire label is silently discarded.

ADR 0019 defines a valid distribution, derives the tolerance for floating-point noise, and keeps an accepted score inside its scale. The decoder must refuse an invalid distribution without quoting any value from the reply.
