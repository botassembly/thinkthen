# Feedback on the `annotate` plan for ticket 0015

Status: Open. Two points change work inside the ticket. The rest can wait until it lands.

Ian shared the builder's second plan for ticket 0015 on 2026-09-20 and asked the agent that holds the marketing and library-design job for feedback. The plan is sound and needs no change of direction. It fixes the contradictions in the old ticket, keeps the command consistent with the other verbs, and takes in the stumble register: the hint for a stray second file, the swapped-file test, and the cold pass with an empty environment. The internal library target with a thin command shim is the right step toward the one-crate ruling, and cases kept as data serve the shared tests later.

## 1. The paid probe in the plan has already been run

The plan ends with a paid probe of 1, 5, 10, 20, and 40 questions per request, plus the models listing and the response header names. A spike ran exactly that on 2026-09-20 under Ian's direct go-ahead. `2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md` holds the result. It landed on main after the builder read the probe plan, so the builder saw "running". Forty questions in one request changed no answer across 120 clear questions and billed 20.8 times fewer tokens. No ceiling was found, and that supports the plan's choice to impose no question-count cap.

Do not buy those numbers twice. The paid step still has to record how-tos 39 and 14. Spend the rest on two things the spike did not measure and `annotate` does every day:

- **Mixed types in one request.** The spike packed yes or no questions only. A real question set puts `decide`, `choose`, and `score` side by side. Ask the same questions alone and together, and count the answers that move.
- **Questions near the cut.** Every spike question was a clear case, and the single arm scored 100 percent. A handful of made-up borderline cases shows whether packing moves an answer that sits near 0.5.

Both fit in a small reservation. The spike's 120-question run billed about 132,000 tokens across all five arms.

## 2. Say what a change to the set costs, in the reference page

This one is least surprise. Every question with the same `on` rides in one request, and a recording or a cache entry is keyed by the whole request. A user who adds one question to a set of ten, and reruns under `--cache`, pays for all ten again on every record. An old column can also change. The spike's tagging run shuffled twenty questions and flipped 3 answers of 400. It added two questions and flipped 3 of 400. The flipped answers sat near the cut.

Neither fact is a defect. Both surprise a user who was not told. The Cautions section of the reference page should say three things. A change to a group re-asks the whole group. An answer near the cut can move when its neighbors change. A user who needs a stable column keeps the probability under `--details`, or gives that question its own `on`.

`specification/annotate.md` also still quotes the vendor's figure of twelve times cheaper over thirteen questions. The repository now has its own number with a record behind it. The rewrite in this ticket should quote that one.

## 3. Smaller notes for this ticket

- **The model-mismatch message.** Ian ruled that a record fails when its replies name different model versions. The alias `jev-latest` answered as `jev-1.13.0` on 2026-09-20, so a vendor rollout in the middle of a long run will trip this. The diagnostic should name both versions. It should also say that a rerun under `--record` or `--cache` answers the finished records from disk, and that `--model` pins a version.
- **The order after this ticket.** The first draft listed the correction pass, cache coalescing, `find`, page 16, transforms, release preparation, and libraries. It left out `tag` and the CSV and TSV framing. The second draft names both under "do not begin", so I read them as still in version one. Please confirm the order in the ticket. The fold into one published crate needs its own ticket before release preparation, as point 1 of `2026-09-20-steering-on-the-version-one-completion-plan.md` says.

## 4. What can wait until it lands

- `tag` will expand one named question into one yes or no question per label inside the same request. The grouping code should not be bent for that now. Wire names are already `q1` onward and private, so the recording format does not change when `tag` arrives.
- `--details` could carry the vendor's request id and the versioned model per request. That is a follow-up from the status finding.
- Under `--lines`, the bare row holds the answers alone, with no copy of the line. The specification settled that, and `--details` carries the input. I will watch for it in the cold pass and add it to the stumble register if a new user trips on it. The how-to for lines should show `--details` or a `paste` beside the source file.

## What Ian can overturn

All of it. Point 1 saves money and is the one I would relay today. Point 2 is a paragraph of documentation. The builders can take or leave the notes in section 3.
