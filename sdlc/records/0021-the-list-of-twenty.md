# 0021: The list of twenty

Branch `ticket/0021-list-of-twenty`. Built 2026-09-19.

## What landed

`demos/README.md`, `sdlc/planning/documentation-plan.md`, and the README's front window carry ADR 0018: twenty how-tos in five groups, twelve of them green, eight marked coming with the ticket or the slice that writes them. The front window is 01, 02, 15, 43, 06, 14, 16, in that order, each line a title, a one-line made-up scenario, and a link, with a red page marked coming. No behavior of the binary changed, and nothing under `crates/` was touched.

Two green pages left with their folders, and two lists and one script now hold the rest together.

## Pages deleted, and where each lesson lives

**How-to 20, "tell not stated from false", into how-to 40.** The scenario is the same three maintenance notices, so 40 already taught the whole of 20's first step: a yes/no question answers a denial and a silence alike, and the two probabilities put no usable distance between them. Three lessons were not on 40 and are now traps on it:

- A pick with `supported`, `contradicted`, and `not_stated` keeps silence as its own label, and the label has to be typed because nothing adds it.
- `--details` on a pick prints a probability for every option in the order they were sent, so a margin over the runner-up is a rule a desk writes in `jq`.
- An `ambiguous` label is the model hedging, and exit 3 is the tool saying no option cleared the mark.

20's warning that a confident answer says nothing about facts the model never saw joined 40's bullet on reading 0.81 as a measurement. 40's closing list gained a link to how-to 02, which is the pick that names silence as its own label. How-to 02 lost its two links to 20: the inline one became the plain sentence that `--details` prints a probability for every option in the order they were sent, and the closing one now points at 40. 40 measures 98 lines and 889 words against the limits of 120 and 900.

**How-to 24, "compare two runs", into how-to 41.** 41 already ran `compare.jq` over the two tuning runs and already asserted `paired`, `same`, `flips`, and `mismatched_label`. Three of 24's lessons were not on 41 and are now traps on it: `changed` names the model and the cut as well as the question, and two moving at once means rerunning with one held still; `paired`, `only_in_before`, `only_in_after`, `repeated_ids`, and the two mismatch lists say the runs measured the same cases, so a run that stopped early is listed rather than passed off as a smaller run; and folding unresolved into no makes a case that moved into the band read as a regression. The header of `transforms/compare/compare.jq` states every one of those rules in full, and 41's trap names it as the fuller home.

24 was the only page that ran `transforms/compare/example.sh`. That line moved into 41's step 3 block, so the file is still run by a rung. `transforms/README.md` points the `compare/` row at 41.

24's doctored-file block, which changed one label and dropped one case to prove the mismatch lists are not vacuous, did **not** move. 41 had no room for a sixth asserting block inside 120 lines, and the lesson it carried is the trap above. This is the one thing a reader of 24 saw and a reader of 41 does not.

Neither deleted recording is used by another page. 20's six exchanges went with its folder, and 24 held no recording.

## Pages kept against the ADR, and why

- **04, act only when the answer is sure.** Green, and its absorbing pages 19, 16, and 13 are not all green. It keeps its folder and its row in the list of folders that stay and then leave, and the row says it leaves when 16 is green.
- **07, judged columns.** Red. ADR 0018 drops it into 14 and 16. Ticket 0015 owns the folder, so it stays, and its first lines now say which pages absorb it, the way 08 and 09 already did.
- **08 and 09.** Red, held for tickets 0015 and 0014, already marked.
- **03, 06, 14, and 15** keep their red folders. They are in the list of 20, and tickets 0014, 0015, and slice 11 turn them green.

No other red folder sits outside the twenty, so nothing else was deleted.

## The check that keeps them in step

`sdlc/scripts/pages` reads the tables of `demos/README.md` and `documentation-plan.md` whose header is `| # | How to |` and refuses:

- a number in one list and not in the other,
- a title or a state that differs between them,
- a state that is neither green nor coming,
- a green row whose folder is missing, whose page does not say `Status: green`, or whose own title is not "How to " and the row's title,
- a coming row whose folder exists and does not say `Status: red`,
- a folder under `demos/` that `demos/README.md` never names,
- any relative link in the README, either list, or any demo page that resolves to nothing.

`sdlc/scripts/pages-self-test` builds eleven small trees under `target/`, each breaking one check, runs the real script over each, and pins the exit code and the whole sentence. Both run from the `lint` rung, the cheapest one, before the ratchet and cargo.

## Choices made where ADR 0018 was silent

Each of these is Ian's to overturn.

- **How-to 21 keeps its own title.** ADR 0018's table shortens it to "Pick the next action from a list that changes". The green page is titled "How to choose the next action from a list that changes at every step", and the lists now say exactly that. Changing a green page's title means changing the page, its links, and its row, which is not this ticket's work. The lever is a one-line rename in a later ticket.
- **The word in the state column is "coming", not "red".** A folder's own page still says `Status: red`, which is what `sdlc/scripts/demos` reads. The lists face a reader, and a reader wants to know the page is on its way. `pages` maps the two.
- **The closing links on 13, 25, and 28 read "How to tune a question file", not the page's whole title.** Those three pages sit within two words of the 900-word limit, and the full title of 41 costs nine words. The short text is honest and it fits.
- **`sdlc/scripts/pages` and its self-test are Python.** `lint` already runs `policy.py`, the check is all text handling, and the self-test builds small trees, which is half the length in Python that it is in `sh`.
- **The check runs from `lint` and not from `spec`.** It reads only Markdown, so it belongs on the cheapest rung that will catch it, and `spec` already carries the demo runner and its self-test.
- **`documentation-plan.md` keeps a slice column for pages that are already green.** The column now says which slice or ticket turned each one green, which is the history a reader of the plan wants.
- **The README's sentence about the evals section was rewritten.** It named comparing two runs and keeping a traceable run as separate pages, and both left the list. It now names the four eval pages that exist. The edit sits inside the front-window section, as the ticket asks.

## What proved wrong in the ticket or in ADR 0018

- The ticket says "any other red folder outside the 20 goes now". There is none. Every red folder on disk is either in the twenty or held for ticket 0014 or 0015.
- ADR 0018's merge table sends 07 into 14 and 16 and 42 into 12, but ticket 0021's scope names only 20 and 24 as folders that leave. 07 has a folder and 42 never did, so 07 is marked and kept and 42 needs nothing.
- ADR 0016's rule 6 forbids two pages sharing a scenario unless one is the sequel of the other. 40 and 20 shared the maintenance notices, which ticket 0017's record raised. Deleting 20 settles it.
