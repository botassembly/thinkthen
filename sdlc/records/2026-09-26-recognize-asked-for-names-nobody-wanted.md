# Recognize asked for names nobody wanted

Report of 2026-09-26, written for Ian. ADR 0054 turns it into a rule.

## The issue

`recognize person` should return the people in a text. On main it returns every name, people, songs, places and companies, and labels all of them `person`.

It happens because `recognize` works in two steps.

1. **Find the names.** One fixed question asks the model to mark every name in the text, in the news sense: people, organizations, places, events, products and creative works. The caller's kinds never appear in it.
2. **Sort them.** For each name found, the tool asks which of the caller's kinds it is. With one kind that question has one answer, so the tool skips it and labels everything `person`.

## Why the approach was bad

- It ignored what the caller asked. The caller said `person`. The first step looked for every kind of name anyway.
- It wasted work. The model found songs and places that nobody wanted, and the caller paid for them.
- The first repair made it worse. It kept step 1 as it was and added "person or none?" in step 2 to throw the extras away. That pays twice: once to find unwanted names, then again to discard them. It treated the symptom and roughly doubled the cost at one kind.
- Nobody checked the obvious fix. The fixed wording was measured and kept, and no one tried putting the caller's kinds into the first question. The design followed the letter of an old ruling instead of its purpose.

## The better approach

Filter at the earliest step. Tell the first question what the caller wants: find only names of these kinds. Names of other kinds are never marked, so they cost nothing later.

- At one kind, no second question is needed.
- At several kinds, the second question sorts only among the caller's kinds.
- `relate` follows the same rule. It asks only about pairs that some relation type allows. With "person wrote song", it never asks about a person and a place.

Wrong-kind names never reach the relation step, so relations get cheaper and cleaner too. Local experiment 274 measures this against today's behavior and against the `none` repair. The cheapest version that is not worse beyond the noise wins.

## Speed against accuracy

1. Every answer from the model has noise. Three runs over the same 306 titles disagreed on 3 to 5 titles (section 14 of `2026-09-26-batching-and-recognize-evidence.md`). That error is always there.
2. Accuracy counts only when it beats the noise. A slower method that is better by less than the run-to-run spread is not better. It only costs more. One request for all 306 titles takes about a third of a second, against 13 seconds for one title a request. Its answers move within that same noise, so batching wins.
3. Cheaper usually means more accurate too. Fewer, more focused questions give the model fewer chances to be wrong. Not looking for songs when the caller asked for people is faster and more correct at once.
4. Pay for accuracy only when the gap is real and matters. When a method is worse by clearly more than the noise, on something users rely on, the loss is measured and stated plainly, and a setting such as `--batch 1` gives the slower method. A slow path never becomes the default for a gain nobody could detect.

The rule: do not ask what can be ruled out, prefer the fastest method unless its loss is larger than the noise, and measure before choosing.

## What the measurement showed

Local experiment 274 measured the two repairs on the 100-sentence key and a sample of the Beatles Bench, with 2.9 million tokens of authorized live calls. Run-to-run noise was about 1.5 F1 points.

- Today's build scored 27 to 32 F1 at one kind. About 120 wrong-kind names a run reached the relation step.
- The `none` repair scored 62 to 76 at one kind, with 5 or 6 wrong-kind names a run. At five kinds it matched today, 72.5 against 72.6. At one kind it costs 382 input tokens a word against 215.
- Putting the caller's kinds into the detection question halved that cost at one kind. It matched the `none` repair within noise on `person`, `place` and a described `work`. It lost 16 points on a bare `work` and 13 on Beatles `person`, beyond the noise, and it passed fragments of other names as names. Adding the restriction to the fixed list changed nothing, because the model ignored it.

So the `none` repair ships under ADR 0054 item 4: the cheaper method's loss is beyond the noise. Labelled detection is the measured alternative. It stays open for one kind when every kind carries a description, which Ian can choose.
