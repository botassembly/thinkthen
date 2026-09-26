# recognize loses possessive and side-by-side names

Status: Held by Ian. Do not start. Ian wants a full edge-case set and a general redesign before any fix, not a one-off. The redesign lands in this issue first. Filed 2026-09-26 from workspace experiment 265, a hand-written test of 30 plain English sentences run three times against `jev-latest` (`jev-1.13.0`). The recognize code tested is byte-identical to main `0f255579`.

## Summary

On plain sentences, recognize finds most names with the exact text and the right kind. Every exact span in the test had the right kind. Three ordinary English forms still come back wrong:

1. A name with an attached possessive, such as `Ringo Starr's`.
2. Two names with no word between them, such as `On Revolver Paul McCartney sang`.
3. A name whose words Jev puts in two kinds, such as `Abbey Road` read as place and album.

Asked directly about the whole span, Jev gets every one of these right. The loss happens in recognize's tokenizer and span assembly. Each result below was the same in all three live runs.

## 1. Attached possessives

| input | recognize returns | Jev asked directly |
|---|---|---|
| `Octopus's Garden was Ringo Starr's song.` | `Ringo Starr's` person, 21 to 34 | picks `Ringo Starr` over `Ringo` and `Ringo Starr's`, 1.0 |
| `George Harrison's song Something appears on Abbey Road.` | `George` person | picks `George Harrison`, 0.97 |
| `Paul McCartney's band Wings played in Glasgow.` | `Paul` person | picks `Paul McCartney`, 0.99 |

Smallest reproduction, with no key:

```sh
echo "Octopus's Garden was Ringo Starr's song." | thinkthen recognize person song --dry-run | jq -r '.requests[0].body_utf8'
```

The detection snippet is `was Ringo [[Starr's]] song .`. Jev must judge `Starr's` as one word. It said IN at 0.73 for `Starr's`, so the name keeps the `'s`. For `Harrison's` it said IN at 0.31 and for `McCartney's` at 0.36, so those names lose their last word.

Where the answer is lost:

- `crates/thinkthen/src/core/recognize.rs:70` `tokenize` splits only on white space. `split_piece` peels only `.!?,:;` from a word's end (`:95`). An attached `'s` stays inside the word.
- `crates/thinkthen/src/core/recognize.rs:228` strips a final token only when the whole token equals `'s`. The tokenizer makes that token only from text already split as `Starr 's`. Ordinary text never has it, so the rule never fires.

What the rulings say. `sdlc/issues/closed/2026-09-21-four-small-closings-for-the-recognize-team.md` item 3 states the rule as "a trailing `'s` trims off and a middle one stays". Ticket 0080 keeps "the fixed trailing-possessive rule". `specification/recognize.md` says "A trailing separate possessive is removed". On raw text the shipped rule removes nothing.

## 2. Names side by side

| input | recognize returns | Jev asked directly |
|---|---|---|
| `On Revolver Paul McCartney sang Eleanor Rigby.` | `Revolver Paul McCartney` person, 3 to 26 | "Is Revolver Paul McCartney a single name?" 0.12. Revolver album 1.0, Paul McCartney person 1.0 |
| `In Liverpool John Lennon met Paul McCartney.` | `Liverpool John Lennon` person, 3 to 24 | single name? 0.27. Liverpool place 0.99 |
| `At Abbey Road Studios George Martin recorded Revolver.` | neither name | single name? 0.11. Abbey Road Studios place 0.98, George Martin person 1.0 |

Per-word answers for the third input (detection, then kind in the order person, song, album, place, organisation):

```text
Abbey    0.99  place 0.77
Road     1.00  place 0.82
Studios  1.00  place 0.51, organisation 0.49
George   0.97  person 0.95
Martin   0.98  person 0.97
```

Jev puts every word in a name and gives each name its own kind. Where the answer is lost:

- `crates/thinkthen/src/core/recognize.rs:221` `assemble` makes one candidate from every contiguous detected word. The five words become one name.
- `kind_vote` (`:279` to `:310`) picks place, three words to two.
- `candidate` averages the place probability over all five words (`:266`), so strength is 0.97 × 0.426 = 0.413. The cut at `:233` drops the whole run. Two names Jev found with 0.97 or better are both gone.

The same path turns `Revolver Paul McCartney` into one person. The relation step then reports `sang(Revolver Paul McCartney, Eleanor Rigby)`.

What the rulings say. The 2026-09-23 ruling in `sdlc/planning/recognize-design.md` ships "one maximal contiguous run of `IN` words" as one candidate. It overturned the earlier overlap rule. It does not mention two names that touch. This item may need Ian's decision because it touches that ruling.

## 3. A detected name dropped over its kind

| input | recognize returns | Jev asked directly |
|---|---|---|
| `Ringo Starr sang "Octopus's Garden" on Abbey Road.` | no `Abbey Road` | "Is Abbey Road a single name?" 0.83, and it is an album, 0.68 |

Per-word answers: `Abbey` IN 0.99 with place 0.78. `Road` IN 0.99 with album 0.83. The kind vote ties, one word each, and the first word's kind wins. `candidate` then averages the place probability over both words, (0.78 + 0.15) / 2 = 0.465. Strength is 0.99 × 0.465 = 0.46, under the 0.5 cut at `:233`. A name detected at 0.99 disappears because its words disagree on the kind. The specification defines strength this way (`specification/recognize.md`, "Names"). With the default cut, kind doubt removes the name itself. The user gets no name with a doubtful kind.

## What was not filed

- Quoted titles come back with their marks: `"Octopus's Garden"`, `"Help!"`, `'Boys'`. The tokenizer never separates a quote mark. Asked directly, Jev also picked the quoted form (0.72, 0.91, 0.61), so the experiment files no defect for it.
- Unquoted `Help!` comes back exact, `!` included, before a space and before a comma. A sentence-final `?` is left out correctly. Titles with small words inside, such as `Lucy in the Sky with Diamonds`, come back whole.

## Evidence

The experiment keeps its recordings, replay, and per-word answers locally (`experiments/265-recognize-simple-cases/`). Overall exact-span recall was 82.2% (60 of 73) and precision 88.2%. Possessive cases scored 70% and side-by-side cases 50%. Easy, small-word, and unquoted-punctuation cases scored 100%.

## What is asked

Recognize should return each of the names above with the text Jev reads directly. Whether that needs a tokenizer change, a span change, a strength change, or a specification change belongs to this repository's design. Items 2 and 3 touch the 2026-09-23 ruling and the specified strength formula. Ian can overturn this issue.
