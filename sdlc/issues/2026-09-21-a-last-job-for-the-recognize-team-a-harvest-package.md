# A last job for the recognize team: a harvest package

Status: Open

The recognize program is closed and its method is final. Ian asked on 2026-09-21 whether that team should break out `relate`, or move its work into a worktree, or whether the build team should harvest from the experiments as they are.

The product side's answer: no worktree, and one last job. The experiment code is bench code in another language, and the core is built test-first under strict gates by the team that owns it. Code does not move. Three things move, and today they are spread across sixteen folders. The recognize team knows where they are, and the build team would spend days finding them.

## The package, one folder

1. **The exact words.** Every instruction and option text the model sees in each of the three passes, as files, with the rule that fills each blank. These words are the product. A port that rewords them is a new, unmeasured method.
2. **The rules in plain statements, each with a test case.** How a text is split into words, the window, the edge markers, how yes-words join into a name, the connector list, the kind vote, the confidence formula, the span gate, how legal pairs are listed, and how direction rides in the options.
3. **Cases with recordings.** Thirty to fifty short made-up texts with the expected final object in the shape of `sdlc/planning/recognize-design.md`, each with its saved recording, so the build team's tests replay with no key. Include the hard cases the record already knows: a capitalized ordinary word, an unusual title, a lowercase name, the same name twice, no names, a name at the end of a long text.
4. **`relate` as its own set.** The same three things for pairs over records, in the shape of `sdlc/planning/relate-design.md`, plus the two measurements that page leaves open: pick-one against a yes-or-no question per relation on pairs that hold two relations, and pairs against one question per subject at 10, 50, and 200 records.
5. **One page that maps the package to the sixteen folders**, so a doubt can be traced to its evidence.

No benchmark text goes in the package. The cases are made up, because the repository will be public.

## What it is not

It is not a stand-in command, a library, or a change to the thinkthen repository. The surfaces rehearsal builds the stand-in from this package, and the build team builds the core from it.
