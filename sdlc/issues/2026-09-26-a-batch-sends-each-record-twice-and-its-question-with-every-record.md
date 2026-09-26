# A batch sends each record twice and its question with every record

Status: Open. Filed 2026-09-26 from the ten-function efficiency audit under ADR 0054, local experiment 275. Owner: the batching queue, before ticket 0146 lands. Does not block 0.1 by itself. Ticket 0146 fixes the batched request bytes, so a later change misses every cached batch.

## What happens today

The batch planner puts every record in the evidence and quotes it again in its own question.

- `crates/thinkthen/src/core/batch.rs` lines 433 and 434 build the evidence as `{"records":[…]}` over every record of the batch.
- Lines 417 to 423 build each record's question as `The text is <record>. <question>`. The record travels a second time, and the whole question travels once for every record.
- Ticket 0146's settings table already says it: "a batch carries each record twice, in the evidence and in its quoted question".

No one measured this form against a form that sends less. The batching record compares widths and wrapper shapes, and every arm kept both the list and the quote.

## Why it breaks ADR 0054

- Item 1 and the audit pattern "evidence sent more than once". The list repeats text that each question already quotes. The backend bills every question's instructions in full, so the question text is also billed once for every record.
- Item 5. The planner's form became the default for `decide`, `filter` and `rank` in ticket 0146, and for `choose`, `tag`, `score` and `annotate` in B8 to B10. It was never measured against the simplest alternative that sends less.

## Measured

Local experiment 275, model `jev-1.13.0`. The 306 Beatles titles in one request, keys from the Beatles Bench song table. Yes/no tasks cut at 0.7. Right answers of 306, one number a run.

The two alternatives:

- **Quoted.** No list. The evidence is one fixed sentence, and each question quotes its record as today.
- **Question in the evidence.** The evidence is `{"question": QUESTION, "records": […]}`. Each wire question only names its record: `The question holds for the text "<record>".`

| Task | Unbatched, one a request | Planner form | Quoted | Question in the evidence |
| --- | --- | --- | --- | --- |
| `filter` "It appears on the album Abbey Road", same bytes | 285 | 271, 274, 275 | 284, 285, 285 | 288, 292, 295 |
| The same, three shuffled orders | | 266, 267, 275 | 285, 285, 285 | 296, 298, 298 |
| `filter` "Ringo Starr sings lead on it" | 285 | 293, 294, 295 | 291, 293, 293 | 296, 296, 297 |
| `filter` "It was released before 1965" | 255 | 278, 279, 281 | 259, 260, 262 | 273, 274, 276 |
| `choose` first album of 14 options | 161, 164 | 172, 174, 176 | 156, 161, 163 | 169, 169, 172 |
| `tag` lead singers, exact label sets | 88, 93 | 140, 140, 143 | 111, 112, 116 | 93, 102, 110 |

Input tokens a run, planner against the two alternatives: 13,799 against 11,468 and 8,624 on yes/no, 17% and 38% fewer. `choose` 53,537 against 51,210 and 48,389. `tag` 65,717 against 63,390 and 35,167. The unbatched `filter` bills 88,933. Every request answered in 0.24 to 0.41 s. The list's share of the bill grows with record length, because each record is billed twice.

The noise:

- The same bytes sent three times moved 1 to 4 answers in every arm.
- The planner form moved with its neighbours. Three shuffled orders scored 266 to 275, and one added record lifted it to 285 and 288. That spread of 22 answers is five times the same-bytes noise that ADR 0054 item 3 cites.
- The quoted form gave identical counts in all three orders, so each question reads only its own quote. It scored like the unbatched request on two of three yes/no tasks and lost on the third.

What the numbers say:

1. On yes/no, putting the question in the evidence cost 38% fewer tokens. It won on Abbey Road by 13 to 24 answers on the same order and by 21 to 32 across orders, beyond the noise. It won on Ringo by about 2, at the edge of the noise. It lost on the release year by about 5, beyond the noise.
2. On `choose` it lost by about 4, at the edge of the noise, for 10% fewer tokens.
3. On `tag` it lost by 30 to 50 exact label sets. The planner form stays for `tag`.
4. The quoted form removes the neighbour effect, and ADR 0053 item 5's trust caution with it. It lost to the planner form on `choose`, `tag` and the release-year task.

The yes/no result rests on three tasks over one list of short titles. Long records were not measured.

## Recommended fix

1. For `decide`, `filter` and `rank`, put the question once in the evidence and send each record once, before ticket 0146 fixes the batched bytes. Before switching, run one more keyed yes/no task on longer records. Keep the planner form if that task loses beyond the noise.
2. Keep the planner form for `choose` and `tag`. B8 and B9 measure `score` and re-check these two with their own tasks.
3. State the measured order spread on the batching page that ticket D1 writes. Batching test 9 should report shuffled orders beside same-bytes repeats, because the order spread is the noise a batch user meets.

## Blocks 0.1

No. It should be settled before ticket 0146 lands, because each later change to the batch form misses every cached batch.
