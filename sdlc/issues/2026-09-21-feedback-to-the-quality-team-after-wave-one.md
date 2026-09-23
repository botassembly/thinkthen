# Feedback to the quality team after wave one

Status: Closed on 2026-09-22. The feedback was delivered and the work it asked for landed.

From the product side, 2026-09-21, at Ian's request. Wave one was good work. 345 checks, 21 findings, each with a reproduction and a severity a user would recognize, a one-page list in the order of embarrassment, and a proposed plan with gates and costs. The "what would not embarrass us" paragraph is as useful as the findings, and the next wave should keep it.

## What wave one missed, and why

A second team reviewed the tool as a program that calls it, and found four things first. The four checks are now in the experiment's README. The lesson is about the tester's seat. Wave one sat at a keyboard. A caller sits inside a loop, holds the result object for months, and sends whatever size its last stage wrote.

1. The result object differs between functions. `annotate` carries a request digest and the single functions do not. Compare every key of `--details` across every function.
2. `--dry-run` and the live run disagree on what they accept, and the dry run prints the first record only and no count.
3. Nobody tested the spread of real input sizes. A caller measured 40 bytes to 1.4 MB.
4. Nobody tested one refused question inside a good request. A caller sees about 1 reply in 15 refused.

Two more came from the product side's own probes, outside the wave: an over-limit text is refused at status 400 and the user is told nothing of the reason, and a threshold tuned on one model silently changed an answer on another.

## What to watch more closely from here

1. **Test from three seats.** A person typing, a script in a loop, and a program that keeps the result. Name the seat on every finding.
2. **Every paid path.** The 20 accidental paid calls were the wave's best finding and also a process slip. Before any run, state in the log which address each command will reach, and run with no key set unless the step needs one.
3. **The two new functions.** `recognize` and `relate` arrive with shapes no other function has: a result of no fixed size, offsets into the user's text, many questions per request, and a pair count that grows with the square. `sdlc/planning/recognize-design.md` and `relate-design.md` are the expected behavior. Offsets across an accented letter and an emoji, a name at the edge of a piece of a long text, the same name twice, a text with no names, and a relation rule with `*` at each end are the first cases.
4. **The second backend.** Rerun the core of wave one against the local open model in `experiments/220-thinkthen-second-backend/`. The tool's behavior must not depend on which model answers, and its messages must not name the first vendor.
5. **The public pages against the tool.** The examples page failed as printed and the deck taught words the tool does not use. From now on the deck, the README, the examples page, and the help are inputs to every wave, and every command printed on any of them is run.
6. **Numbers.** Every printed number names its record. The wave found two that do not. Sweep all public pages for the rest.
7. **Severity by money first.** The order in `FOR-IAN.md` put the missing cache above the two findings that silently cost money. The product side would put silent spend first on any list, because a user forgives a missing feature and does not forgive a surprise bill. Ian rules the order.

## What the product side owes the quality team

The deck findings are the product side's to fix: the two names for the third and fourth outcomes, the use-case flow that goes red, the vocabulary slips, and the cost number with no record. Each fix will name the finding it closes.
