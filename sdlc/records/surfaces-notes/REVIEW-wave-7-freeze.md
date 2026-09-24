# Review of the wave-7 freeze raises and ratchet changes

Second-agent review of the nine commits that `sdlc/records/surfaces-freeze-2026-09-24.md` names as waiting. The reviewer wrote none of them. Reviewed at `f13afd89` on `surfaces-wave7`, on 2026-09-24. Seven commits move the ceiling from 36,510 to 36,829. Two change the ratchet script or its self-test.

| Commit | Change | Verdict |
| --- | --- | --- |
| `ce2aef10eb1f51ac20cc88e174d4ba29992da5b2` | ceiling 36,510 to 36,531 | ACCEPT |
| `b590a8994e0181c0d10c747b17b1c53f534f0faa` | ceiling 36,510 to 36,716 | ACCEPT |
| `d74d95fb248fcb65b0d30ccf31c0d45002f49b9c` | ceiling 36,716 to 36,730 | ACCEPT WITH FOLLOW-UP |
| `d048261edac1145775f9cdb951c5eede392df5c4` | ceiling 36,730 to 36,797 | ACCEPT WITH FOLLOW-UP |
| `385afd29a72973e55b3226a3ed5942c1715ecf05` | ceiling 36,797 to 36,802 | ACCEPT |
| `417433b48d6fd99d36024645976c6b70adaef243` | ceiling 36,802 to 36,808 | ACCEPT |
| `ff57ef565186314293ffc2c12c242642e5feac25` | merge, ceiling 36,829 | ACCEPT |
| `35229b813ea8845f31f8796a2847470924000508` | ratchet script | ACCEPT WITH FOLLOW-UP |
| `1a3df4931e1265cfe3fa4c6d81bc18628bcbb7a8` | ratchet self-test | ACCEPT |

## What I checked

- I recounted each commit's tree with the ratchet's rule from `git show` of each `.rs` file. Each new ceiling equals its measured count.
- I read each commit's diff against its parent.
- `git show --remerge-diff ff57ef56` shows only the ceiling conflict, resolved to 36,829. `git diff 40c8f705 ff57ef56 -- '*.rs'` is empty.
- In a scratch clone I committed this record and the ratchet went green. I then planted an unreviewed raise, an unreviewed script edit, a prefix verdict, and a verdict inside a fenced block. Each turned the ratchet red and named the planted commit.

## Follow-ups

- `d74d95f`: the new `snapshot()` test helper sits inside the doc comment of `refusal`. The helper now carries `refusal`'s doc, and `refusal` has none. Move the helper below `refusal`.
- `d048261`: the check "covers this extent or refuse with `UNREADABLE`" is written out five times. One helper would save about eight lines. A null batch child is refused with `UNREADABLE`, and a sentence naming the null child would read better.
- `35229b8`: the ratchet stops at the first kind of waiting commit, so a pending raise hides a pending script change. `1a3df49` works around that in the self-test. Report both lists in one run, and the self-test loop can go. The guard does not cover `sdlc/surfaces-ratchet-reviews.json` or the rungs that call the ratchet. The anchor check on `GRANDFATHER_UNTIL` bounds the first gap.

## Who can overturn this

Ian can overturn any verdict here.
