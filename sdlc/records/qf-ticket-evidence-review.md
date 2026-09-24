# Review of Quick Fix qf-ticket-evidence

Reviewer: a fresh read-only Opus session that did not write the work. This page restates its replies.

## First pass, commit 4d247755

Not accepted. Two blocking findings and four non-blocking ones.

1. Blocking. An empty bold item such as `- **Proof:**` passed, because `\s*\S` matched the first `*` of the closing bold. The item pattern now needs a character that is neither space nor `*`, and a self-test case pins it.
2. Blocking. The record said a review file existed before it did. This page now exists.
3. The fence strip knew only backtick fences. It now knows `~~~` too, and a self-test case pins it.
4. Authors could trip on `### Evidence`, numbered items, and lowercase labels. `sdlc/README.md` now says the labels are exact and the items are `-` list items.
5. The self-test pins the failure list, not `main`'s summary line. The record already says "the whole failure list". The lint rung shows the summary line and exit code.
6. The decision says "product ticket", and the check covers every numbered ticket. `sdlc/README.md` now says every numbered ticket here counts as a product ticket.

It confirmed the lint wiring, the filename match and exemption, the section bounds, the planted-bug counts, the `CLAUDE.md` size, and the README wording.

## Second pass, commit df6c6b27

Not accepted. One blocking finding and three non-blocking ones.

1. Blocking. The new item pattern refused real text that opens with emphasis, such as `- Proof: **ticket 0118** pins it.` The check now removes `**` from the line and needs text after the colon. Self-test cases for bold and italic openings pin it.
2. A backtick fence closed on `~~~`, and the reverse. A fence now closes only on the marker that opened it, and a self-test case pins it.
3. The product-ticket sentence broke the reference of "They" in `sdlc/README.md`. It moved to the end of the paragraph.
4. This page lacked the second pass. This section adds it.
