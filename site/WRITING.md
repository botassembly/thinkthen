# Writing for thinkthen.dev

The rules for every page on the site live in one file: [`sdlc/decisions/2026-11-03-docs-rules-and-sitemap.md`](../sdlc/decisions/2026-11-03-docs-rules-and-sitemap.md). It holds the fifteen rules, the three page templates and the sitemap. Where anything else disagrees with it, it wins.

## The support inbox

Every function uses the same case on every page and in every language.

| Function | Question | Options | Text |
| --- | --- | --- | --- |
| decide, filter | Does the customer request a refund? | none | Please refund my order. / Where is my parcel? / I was charged twice for one invoice. |
| choose | What is the message about? | billing, shipping, other | Please refund my order. |
| tag | Which topics appear? | billing, shipping | I was charged twice and my parcel is late again. |
| score, rank | How urgent is the request? | routine, urgent | the three messages |
| find | Which line states the requested action? | none | Morgan works for Example Group. / Please refund my order. |
| recognize | none | person, organization | Morgan works for Example Group. |
| relate | none | works_for person organization | the find text |
| annotate | refund, topic, urgency | as above | the three messages |
