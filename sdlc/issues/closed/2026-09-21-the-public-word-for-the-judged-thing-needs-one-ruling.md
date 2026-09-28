# The public word for the judged thing needs one ruling

Status: Closed on 2026-09-21 by Ian's ruling. The public word is "evidence": it is precise, already shipped in the help, README, and specification, and the marketing vocabulary adopted it the same day (marketing commit 3bffb5c). The shipped help and pages need no sweep.

The vocabulary fixes "text" for what a question is asked about. The help, the README, and the specification say "evidence," in over a hundred places. The vocabulary was written to stop two spellings of one concept, and this is the largest one left.

## Reproduction

    $ grep -c evidence scratch/help/decide.txt
    1    # line 1: "about the evidence"

`decide --help` line 1 says "about the evidence." The README line 3 says "reads the evidence on standard input." The specification carries it throughout. The vocabulary's fixed-words table says "text | What the question is asked about | context, document, payload." "Evidence" is not in a never-say column, so this is a conflict between two fixed spellings, not a banned word.

## The recommendation

Adopt "evidence" as the public word and amend the vocabulary. "Evidence" is precise, already shipped in every surface, and the specification is built on it. Changing the public copy to "text" touches the help, the README, ten specification pages, and the demos, and buys nothing the reader does not already have. The recommendation exists to be overturned, and one word flips it the other way.

## How bad it is for a user

Minor. Both words are plain English, and the reader bridges them. The cost is the standing rule that one concept has one spelling.

Found by experiment 218, wave 1, area 12.
