# The third and fourth outcomes have two names

Status: Closed on 2026-09-22. Merged into 2026-09-22-command-wording-and-help-fixes-for-0-1.md.

The deck says "not sure." The help and the specification say "unresolved." The README says "an error" where the vocabulary says "broken." A reader who meets the deck and then the manual meets two words for exit 3 and two for the broken outcome.

## Reproduction

The deck, slide 03-decide: "Exit code 0 for yes, 1 for no, 3 for not sure."

The binary, `decide --help`: "the exit code is 0 for yes, 1 for no, and 3 for unresolved." `choose --help` says "an unresolved pick" five lines in.

The README, line 21: "A yes, a no, an unresolved answer, and an error stay four different outcomes."

The vocabulary fixes both words: "not sure | The answer when the probability falls inside the band. Prints `null`, exits 3" and "broken | The tool could not get an answer. Exit 2, 4, 5, or 70." Ten specification pages carry "unresolved."

## Expected

One spelling per concept across help, README, specification, and slides. The vocabulary owns the public words, and it fixes "not sure" and "broken." The specification change is the wide one, because "unresolved" sits in ten pages and in the result schema's own sentences. The ruling that picks one spelling therefore touches either the vocabulary or the specification, and this issue names both owners.

## How bad it is for a user

Major. The deck hands the user one word for exit 3 and the manual hands another on the same day, and the trust story rests on the four outcomes reading as one thing.

Found by experiment 218, wave 1, area 12.
