# recognize hides its fixed news-document wording

Status: Open

Reported from a user's benchmark on 2026-09-25 and checked against main at `535cb3e7`. Verdict: partly right. The fixed wording is a recorded choice, not a bug. The claim that the kind question offers a fixed set of kinds is wrong. The real gap is that no page tells the user about the fixed wording or its limits.

## What the user sees

The user ran `recognize` with the kinds person, song, album, and place, and read the recorded requests. Every question calls the text "a news document". The detection question lists person, organization, place, nationality, event, product, and creative work. The request never says "song" or "album" in that list. The answers still came back right on the user's sentence.

## What the code and records do

- Both question texts are constants. `DETECTION_WORDS` and `KIND_WORDS` begin "The snippet shows five consecutive words from a news document" (`crates/thinkthen/src/core/recognize.rs:7-8`).
- The detection question asks IN or OUT against that fixed category list. It also says dates and numbers are not names (`core/recognize.rs:7`, `:128-143`).
- The kind question offers the user's own kinds as its options, with any `--kind KIND=DESCRIPTION` text (`core/recognize.rs:145-162`). It never offers PER, ORG, LOC, or MISC. A bare positional kind such as `song` goes out as a label with no description.
- Ticket 0080 line 52 orders "the canonical lineage-B words in `words/` byte for byte". Those words come from experiment 225's harvest package, which chose lineage B because it was the measured one (CoNLL04 three-class F1 0.7636, experiment 221). That package's `words/kind.md` says caller kinds "are new words and a new measurement".
- `specification/recognize.md` and `recognize --help` never mention the fixed wording, the news-document framing, or the category list.

## Why it matters

A user who reads the request sees a question about a news document and a category list that may not match the kinds they asked for. With no page explaining it, that looks like a bug. There is also a real risk, found by reading and not yet measured. The detection question decides whether a word is part of a name before any user kind is consulted. A user kind that falls outside the fixed list, or one the list excludes such as a date, may be marked OUT and never reach the kind question.

## Fix options

1. Document it. Add a short section to `specification/recognize.md` that says the detection and kind wording is fixed, cites ticket 0080 and the measured lineage, and states that kinds outside the listed categories are unmeasured. Add one sentence to the help.
2. Measure a generic wording. Run an experiment that compares the current words with "a text" and a detection list built from the user's kinds, on the recorded cases and on kinds outside the fixed list. Change the words only if the new form holds the measured accuracy.

Recommendation: option 1 now. Take option 2 only if a user needs kinds outside the fixed categories. It changes request bytes and cache digests for every recognize run, so it needs Ian's authorization for a paid run and a ticket. Ian can overturn this choice.
