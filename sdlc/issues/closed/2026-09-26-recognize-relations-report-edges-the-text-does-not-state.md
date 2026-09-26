# recognize relations report edges the text does not state

Status: Closed on 2026-09-26. Replaced by `../2026-09-26-recognize-design.md`.

## What happens

`recognize --relation` sends the sentence as `state.evidence` with every relation question (`sdlc/planning/relate-design.md`, "Recognition state includes its original normalized source text exactly"). The edges it prints still include relations the sentence never states. They are relations Jev knows or guesses from the world.

Rules used: `sang=person:song`, `wrote=person:song`, default `--relation-threshold 0.5`.

| input | edge the text does not state | probability | Jev asked directly: "Does the text itself state that ...?" |
|---|---|---|---|
| `Ringo Starr sang Octopus's Garden.` | wrote(Ringo Starr, Octopus's Garden) | 0.78 | 0.02 |
| `John Lennon wrote All You Need Is Love.` | sang(John Lennon, All You Need Is Love) | 0.74 | 0.04 |
| `Paul McCartney wrote Let It Be in London.` | sang(Paul McCartney, Let It Be) | 0.55 | 0.02 |
| `Paul McCartney sang Help! at the show.` | wrote(Paul McCartney, Help!) | 0.60 | 0.03 |
| `John Lennon and Paul McCartney wrote Yesterday.` | sang(Paul McCartney, Yesterday) | 0.68 | 0.03 |

The same direct question says yes for the stated edges: sang(Ringo Starr, Octopus's Garden) 0.99 and wrote(John Lennon, All You Need Is Love) 0.99. It also says yes for stated counterfactuals such as `Yoko Ono wrote Something.` (0.97). Recognize found those too. Over the test, `recognize --relation` found 21 of 27 stated edges and printed 37 edges, a precision of 56.8%. Most of the other misses come from the name errors filed in `2026-09-26-recognize-loses-possessive-and-side-by-side-names.md`.

## Smallest reproduction

```sh
echo "Ringo Starr sang Octopus's Garden." | thinkthen recognize person song --relation sang=person:song --relation wrote=person:song
```

The output holds `wrote` Ringo Starr to Octopus's Garden at 0.78 in the recorded run. The `--dry-run` plan shows the question text, and its evidence holds only the sentence.

## Where the answer is lost

`crates/thinkthen/src/core/relation.rs:241` writes every cross-kind choice as:

```text
Which listed song fills the blank: Item 1 (person "Ringo Starr") wrote ___? Choose none if no listed song does.
```

The same-kind yes/no form at `:277` asks "Does the relation hold from i1 to i2?". Neither question says to answer from the evidence. The sentence rides along in `state.evidence` (`:304` to `:310`), but the question asks about the world. Jev answers from both. Asked about the text itself, Jev answers from the text.

## Related: recognize, then relate

Standalone `relate` sends no evidence by design (`relate-design.md`, "Standalone relate omits only `evidence`"). Recognize's names piped into `relate` therefore get world answers only. In the test that pipeline found 0 of 3 stated counterfactual edges. Examples are `Ringo Starr sang Yesterday.` and `Here Comes the Sun appears on Revolver.`. It found 17 of 27 stated edges at 44.7% precision. This follows the design and is not filed as a defect. A user who reads the relate page may still expect `relate` after `recognize` to relate what the text says.

## What is asked

Decide whether a recognize relation edge means "the text states it" or "it is true". Then make the question and `specification/recognize.md` say the same.

The cut alone does not settle it. The unstated edges scored 0.55 to 0.80. The stated edges with correct names scored 0.89 to 1.0. A saved sweep at `--relation-threshold 0.8` lifts precision from 56.8% to 72.4% with no loss of recall. Still, wrote(Ringo Starr, Octopus's Garden) scored 0.78 in the first run and exactly 0.80 in both repeat runs, so it passes a 0.8 cut. The direct question splits the same edges at 0.02 to 0.04 and 0.97 to 0.99. A wording change alters every relation request digest, so it needs a paid comparison. The design belongs to this repository. Ian can overturn this issue.
