# Surfaces ceiling review, surfaces-wave7 freeze

Fresh read-only review of the nine commits named in `sdlc/records/surfaces-freeze-2026-09-24.md`, at `f13afd89` in `/home/ian/workspace/worktrees/thinkthen-surfaces-freeze`. I wrote none of them. Nothing in the worktree was edited.

## How I checked

- Recounted non-blank `.rs` lines at each commit with the ratchet's rule (directories and excludes from `sdlc/surfaces-ratchet.json`), reading files through `git show`. Every new ceiling equals its measured count: 36,531 / 36,716 / 36,730 / 36,797 / 36,802 / 36,808 / 36,829.
- Read each commit's diff against its parent.
- Built a scratch clone, ran the ratchet, planted violations, then deleted the clone. Results are in the two check-script sections below.

## ce2aef10eb1f51ac20cc88e174d4ba29992da5b2: ACCEPT

DuckDB relate queue message. The ceiling goes from 36,510 to 36,531 (+21), which matches the measured count. `queue_limit_message` (8 lines) replaces `time_limit_message` at the two places where the query never ran: the queue wait and zero time left. The unit test (14 lines) pins the sentence. The message is a distinct sentence. Folding it into `time_limit_message` would need a flag, so no duplication remains to cut.

## b590a8994e0181c0d10c747b17b1c53f534f0faa: ACCEPT

This commit changes only the ceiling, from 36,510 to 36,716 (+206). The code is in its parent `3b7f876d` (file-mapping size bound: `Segment`/`Backing`, `readable_until`, `file_until`, `probe_until`, `file_end`, and tests for three backings). The old merged-range parser `readable_ranges` is deleted, and `reach` and `covers` now share `readable_until`. There is no second copy. The field parser is custom because a map path can hold spaces. `PastEnd::each` repeats the open-options line twice, which is trivial. Side note: `3b7f876d` itself leaves the count at 36,716 against a ceiling of 36,510, so that single commit is red. It is not a mover, and the gate only judges HEAD, so this does not block. A future lane should land the code and its raise in one commit.

## d74d95fb248fcb65b0d30ccf31c0d45002f49b9c: FINDINGS (minor), verdict ACCEPT WITH FOLLOW-UP

One memory-map snapshot per call instead of one per batch. Growth is +14, which matches the count. The unused `SchemaGuard::held` and two local `snapshot` closures are deleted in favor of one test helper, which is good. `build_frame` reuses the stored snapshot. Finding: the new `fn snapshot()` helper sits inside `refusal`'s doc comment (arrow.rs around line 2295 at HEAD). `snapshot` now carries `refusal`'s four-line doc, and `refusal` has none. Move the helper out. This does not block.

## d048261edac1145775f9cdb951c5eede392df5c4: FINDINGS (minor), verdict ACCEPT WITH FOLLOW-UP

Checks that the buffer, child, and schema pointer tables can be read. Growth is +67, which matches the count: about 25 lines of checks and 38 lines of unit test. The growth is justified by a real crash (SIGSEGV) that the new test catches. Findings: the pattern `if !memory.covers(ptr, n * size_of::<T>()) { return Err(UsageError::new_err(UNREADABLE)); }` appears five times. A small helper would save about 8 lines, so this is borderline duplication worth a follow-up. A null batch child is refused with the `UNREADABLE` text, and a sentence that names the null child would be clearer.

## 385afd29a72973e55b3226a3ed5942c1715ecf05: ACCEPT

R deadline accepts `I(5)`. Growth is +5 (the AsIs class test in `deadline_of`), which matches the count. It is minimal. `I()` over a classed value still refuses.

## 417433b48d6fd99d36024645976c6b70adaef243: ACCEPT

Test cleanup: the named temp file is removed when its mapping drops. Growth is +6, which matches the count. It is minimal.

## ff57ef565186314293ffc2c12c242642e5feac25 (python4 merge): ACCEPT

`git show --remerge-diff` shows only the ceiling conflict (36,531 against 36,808), resolved to 36,829. That is 36,510 + 21 + 298, and it matches the measured count. `git diff 40c8f705 ff57ef56 -- '*.rs' sdlc/surfaces-ratchet.json` is empty, so the later `w7/fast` merge adds no Rust.

## 35229b813ea8845f31f8796a2847470924000508 (ratchet script): FINDINGS (non-blocking), verdict ACCEPT WITH FOLLOW-UP

The check did not get weaker. Every change tightens it:
- Verdicts and grandfather keys need the full 40-character SHA and must match exactly. Before, a 7-character prefix counted. I checked each rewritten grandfather key: it keeps its old prefix, and the script still requires every key to be an ancestor of `GRANDFATHER_UNTIL`.
- A missing or bad ceiling file now fails. Before, a missing file exited 0.
- A malformed reviews file fails with a message.
- A verdict inside a fenced code block or an HTML comment no longer counts.
- An uncommitted raise in the working tree fails.
- A new guard makes any commit after `f6a7faea` that touches the script or its self-test need a verdict. This commit is inside that guard.
- The mover rule is the same as before: a commit counts only if it differs from every parent and holds the file. It moved into `walk()`, which is shared.

Findings, none blocking:
1. The script stops at the first kind of waiting commit. A pending raise hides a pending script change, and `1a3df493` exists only to work around that. Collect both lists before exiting, then delete that loop.
2. The guard does not cover `sdlc/surfaces-ratchet-reviews.json`. The ancestor-of-`GRANDFATHER_UNTIL` rule bounds that gap. The guard also does not cover the rungs that call the ratchet (`scripts/check_surfaces.sh` and the lint rung), and nothing proves a reviewer is really a second agent. These limits existed before this commit.

## 1a3df4931e1265cfe3fa4c6d81bc18628bcbb7a8 (self-test): ACCEPT

This commit changes only the self-test's clone baseline. The baseline now loops up to three times and appends each waiting SHA as an accept, so the self-test's cases stay independent of reviews still pending on the branch. The accept lives only in the temporary clone, and the real ratchet still judges every commit. It does not make the check weaker.

## Proof the check still goes red (scratch clone, deleted afterward)

- Baseline at `f13afd89`: red, and it names exactly the 7 ceiling movers (the script check never runs).
- Committed the record below: green, `36829/36829`, exit 0. So the record satisfies all nine SHAs, including the two script commits.
- Planted an unreviewed raise (+1 `.rs` line, ceiling 36,830, with a body): red, and it names the planted SHA.
- Planted an unreviewed edit to `surfaces-ratchet.mjs`: red, and it names the planted SHA.
- Planted a 12-character prefix verdict for that edit: still red.
- Planted a full-SHA verdict inside a ``` fence: still red.
- Control, a full-SHA verdict outside any fence: green.
- Planted an uncommitted ceiling raise in the working tree: red.
- `sdlc/scripts/surfaces-ratchet-self-test` with the record in place: exit 0, and every case ends in ok.

## Record to commit

Path: `sdlc/records/surfaces-notes/REVIEW-wave-7-freeze.md`. The check accepts any `REVIEW-*.md` under `sdlc/records`, committed at HEAD, with either `Verdict: ACCEPT <full sha>` or a table row whose first cell is the backticked full SHA and whose last cell is `ACCEPT` or `ACCEPT WITH FOLLOW-UP`. Lines inside fences and HTML comments are ignored. Commit the file verbatim, with the content between the markers below. This one file clears all nine SHAs; I proved that in the scratch clone.

----- BEGIN sdlc/records/surfaces-notes/REVIEW-wave-7-freeze.md -----
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
----- END -----
