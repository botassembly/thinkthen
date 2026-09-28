# `recognize --help` describes the old one-call design

Status: corrected by the CLI diagnostic Quick Fix candidate. Fresh independent code review and landing remain.

Filed 2026-09-28 by marketing, from the Beatles deck's recognize steps slide.

What happens. On main 3b96e720, `thinkthen recognize --help` says: "Each record makes paid requests: a detection question for every word, a kind question for every word when two or more kinds are given, and relation questions when rules are given." `crates/thinkthen/src/cli/args/command.rs` holds the sentence, and `crates/thinkthen/tests/version.rs` pins it.

What the command does. Ticket 0147 and `specification/recognize.md` describe three steps. Step 1 asks one BEGIN, INSIDE, END, SINGLE or OUT question per piece. Step 2 asks one kind question per found name, plus an edge question for some names. Step 3 asks the allowed relation pairs. A recorded run of `recognize person song album place` with three relation rules on a 28-piece sentence sent 28 step-1 questions, 7 step-2 questions (5 kinds, 2 edges), and 4 pair questions, in 3 requests. No request asked a kind question for every word.

Evidence. Private presentation branch `draft/recognize-steps`, `decks/2026-09-24-thinkthen-beatles/recordings/recognize-steps/`: the recording, `details.json`, and the `--dry-run` plan.

Why it matters. A user reading the help to estimate cost counts the wrong questions. The deck and site describe the three steps, so the help disagrees with them.

## Resolution

The help now describes one boundary question per text piece, a kind question per found name when kinds are supplied, a possible edge question, and only relation pairs allowed by rules. It also says `--dry-run` prints exact boundary requests and upper bounds for later requests. The exact CLI help assertion pins the new sentence. No request planner or paid call changed. The Quick Fix record names the focused checks and reviewer disposition.
