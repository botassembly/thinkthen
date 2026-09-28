# ADR 0053: The pause fires in every mode, and batches have a member cap

- Status: Accepted 2026-09-26 by the coordinator after a fresh architect review of the batching design. Ian can overturn each item
- Date: 2026-09-26

This ADR amends ADR 0048 items 2, 8 and 12. It also adds a cache rule beside the sentence in `specification/records.md` that says each digest keeps "the first complete response". ADR 0048 says that changing one of its items takes a new ADR. This is that ADR. `sdlc/issues/2026-09-26-batching-design-review-before-0146.md` lists the findings. Section 14 of `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` holds the live measurements.

## Context

A fresh review, local experiment 273, read ADR 0048, tickets 0146 and 0154, and the planner on main. It ran 16 authorized live calls. The command cannot batch yet, so the review sent the planner's exact wire form through `annotate`. It found four defects in the accepted design and one cost that the design understated.

1. ADR 0048 item 2 turns the pause off under a named folder. A loop that writes one record and waits for its answer then blocks forever under `--cache DIR`, `--record DIR` or `--replay DIR`.
2. The planner adds a repeated record to the open batch without a limit check. Under `max`, a stream of a few distinct values forms one batch that closes only at a content cut or the end of input. The reader holds every record of the open batch, so memory grows with the run.
3. Every question file tuned today has no `batch` key, because the parser refuses one. Under item 8 such a file warns nobody when it runs at `max`.
4. The planner's own form over the 306 titles scored 272 to 274 right with 32 to 34 false yeses in three live runs. Item 12 quotes 276 to 278 and 27 to 29, from another client and another evidence shape. Today's filter scores 285 with 16 or 17.
5. The records of one batch are evidence for each other. One planted false claim raised the answers on ten wrong titles in a live test and pushed one over the cut. Item 12 names no threat.
6. A reply that fails one question still decodes when another question is answered. The engine installs it in the cache. Every rerun then reads the same reply and fails the same record without asking again.

## Decision

1. **The pause fires in every mode.** A live run closes the open batch after 50 ms with no new record, whether or not `--cache`, `--record` or `--replay` names a folder. It still fires only while the scheduler has asked for a batch and the open batch holds a record, so a file and a fast pipe never pause. A loop replayed as a loop pauses where it paused when recorded, and finds its batches. A pipe replayed with other timing can form other batches. `--replay` then stops at exit 5 at the first missing batch, and `--cache` pays for the batches that moved. A file replays byte for byte. The value stays fixed with no option.
2. **A batch closes at 4,096 members.** A member is one input record, repeats included. The batch closes as `limit` after the record that brings it to 4,096 members, unless a content cut or the size closed it first. The number equals the content-cut modulus. The planner counts it, so batches stay a function of the records and settings alone. The memory of a run is then bounded by `--jobs` batches of at most 4,096 records, plus the reader's read-ahead.
3. **A tuned file with no `batch` counts as tuned at batch 1.** A question file that holds a `threshold` and no `batch` names batch 1 as its tuned setting for item 8's warning. It does not set the batch. The run keeps its setting and prints the warning when that setting is not 1. `audit --write` writes `batch`, and `"batch": "max"` in the file also ends the warning. Ticket B16 builds it. (Amended by ADR 0085.)
4. **The stated cost is the tool's own form.** Item 12's figures become these. Over the 306 titles, one request in the planner's form scored 272 to 274 right with 32 to 34 false yeses. One title a request scored 285 right with 16 or 17 false yeses. Experiment 271's quoted control, one title a request with the quoted wording, scored 290 to 292. The default stays `max`, by Ian's ruling that speed wins. `--batch 1`, `THINKTHEN_BATCH=1` or a question file's `"batch": 1` gives today's requests and answers. Design test 9 reports against today's filter and the quoted control, and still gates nothing.
5. **Records of one batch are evidence for each other.** A record's text can move its neighbours' answers, and a false claim moves them most. The tool cannot tell a planted claim from a true one. `--batch 1` is the defence for records from different people or sources, or from any untrusted source. `records.md`, `decide.md` and ticket D1's page state this. Design test 9 adds one arm with a planted claim at full batch size. It reports and gates nothing.
6. **A cache installs no reply that failed a question.** When the folder is read back as a cache, by `--cache DIR`, `THINKTHEN_CACHE` or the default cache, a reply that holds a failed question is not installed. The run fails as today, and the next run asks again. `--record DIR` alone still writes that reply, so `--replay` reproduces the live run. (Amended below, 2026-09-26.)

## Amendment, 2026-09-26: a cache reads a partial entry as a miss

The coordinator ruled this on 2026-09-26, after the review of ticket 0158. It extends item 6 from what a cache installs to what a cache reads. An entry already in a folder can hold a reply that failed a question. A default cache may have written it before item 6 was built, or `--record DIR` alone may have written it. When the folder is read back as a cache, by `--cache DIR`, `--record DIR --replay DIR` on one folder, `THINKTHEN_CACHE` or the default cache, such an entry counts as a miss. The run takes the digest's lock and asks the backend again. It replaces the entry when the new reply answered every question, and keeps the old entry when the new reply failed a question too. `--replay DIR` alone still replays a partial entry byte for byte, with its recorded failure markers and exit code. Ticket 0158 builds it. Ian can overturn it.

## What this amends

| Where | What changes |
| --- | --- |
| ADR 0048 item 2, the pause bullet | The pause fires in every mode. A batch also closes at 4,096 members |
| ADR 0048 item 8 | "A file with no `batch` warns nobody" holds only for a file with no `threshold` |
| ADR 0048 item 12 | The cost figures of item 4 above, and the trust statement of item 5 |
| Batching design, open item 10 | The pause's absence under a recording folder is withdrawn |
| `specification/records.md` | The cache sentence names item 6. The batch rules name items 1, 2 and 5 |

## Which ticket builds each item

| Item | Built by |
| --- | --- |
| 1, 2 | Ticket 0146 (B4) |
| 3 | B16 |
| 4 | Ticket 0146 for the page sentences. B6 for the measured cost. D1 for the page |
| 5 | Ticket 0146 for `records.md` and `decide.md`. B6 for the planted arm. D1 for the page |
| 6 | Ticket 0158 |

## What Ian can overturn

The coordinator's rulings of 2026-09-26:

1. The pause in every mode, item 1, in place of refusing a named folder on a live pipe.
2. The member cap of 4,096, item 2.
3. A tuned file with no `batch` counting as tuned at batch 1 for the warning only, item 3.
4. The default staying `max` with the corrected cost stated, item 4. This keeps Ian's ruling that speed wins.
5. `--batch 1` as the defence for mixed or untrusted records, with no new isolation key, item 5.
6. `--record` alone still writing a reply that failed a question, item 6.
7. A cache reading a partial entry as a miss, and `--replay` alone replaying it byte for byte, by the amendment to item 6.

## Amendment, 2026-09-26: ADR 0055 narrows item 5

ADR 0055 takes the records list out of the evidence of `decide`, `filter` and `rank` batches. Without a context, their records are no longer evidence for each other, so item 5's caution no longer applies to them. It still applies to `choose` and `tag`. Ian can overturn it.

## Amendment for ticket 0212: caller-owned Rust iterators

Status: Ian approved this exception on 2026-09-28 after independent technical review. The accepted 50 ms command rule above remains in force. This amendment narrows its application to the Rust library's ordinary synchronous `Iterator` input.

`Batch::next()` reads a caller-owned iterator on the calling thread. An ordinary `Iterator::next()` has no nonblocking or idle signal, and its item need not be `Send`. When `next()` blocks waiting for another record, the library cannot both preserve caller-thread iteration and observe a 50 ms idle interval. A background reader would require `Send` and could run caller code on another thread, changing the existing Rust contract. A fixed small read-ahead cap would change the accepted `Max` batch shape and request identity.

**Rule:** The 50 ms pause continues for command inputs and any future source whose own API reports that it is idle. For an ordinary Rust iterator, `Max` closes a batch only at a content cut, size/profile limit, 4,096-member cap, source exhaustion, or local refusal. A caller that produces one record and waits for its answer selects `BatchSetting::Records(1)`; that preserves today's single-record request bytes and prompt output. `BatchSetting::Records(N)` closes at N records or an earlier accepted cut/limit. The library keeps input order, bounded batch memory, one first error, worker joining, and per-call facts. Ticket 0212 adds no public asynchronous source or timeout switch.

The cost is visible: `Max` over a blocking or unending synchronous iterator may wait for another record or a closing boundary before yielding its first row. A finite source groups until its accepted close, which can read as many as 4,096 records before the first row. This differs from the old one-record throttle-ahead test; that test uses explicit batch 1, while the separate Max test pins its actual batch-window bound. A held-input witness must prove that a one-record interactive iterator progresses under batch 1, while Max resumes only when the caller supplies a closing record or ends input.
