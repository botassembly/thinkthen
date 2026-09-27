# The recognize keys

Two answer keys for `recognize`, and one `jq` filter. Ticket 0164 put them here, so any `recognize` run can be graded with one `thinkthen audit --match strict` command. `sdlc/scripts/recognize-keys` checks every offset on the lint rung, and the spec rung runs the blocks on this page.

| File | What it holds |
| --- | --- |
| `names.jsonl` | 200 sentences with 372 names, one per line, in the source's order. Kinds: `place`, `person`, `organisation`, `work`, `thing`, `event`, `language`, `law`, `award` and `nationality`. `organisation` is spelled as the key spells it |
| `relations.jsonl` | 30 sentences with 73 names, 27 stated edges and 5 unstated edges. Kinds: `person`, `song`, `album`, `place` and `organisation` |
| `kinds.jq` | Keeps each key line, and keeps only the names and edges whose kinds a run asked for |

Each line is an `audit` key line and a `recognize` input record at once. `value.entities` takes the command's own value shape, `name`, `kind`, `start` and `end`, in text order. Offsets count Unicode code points. A run reads the text with `--field /text`, and `audit` finds the record by `id`. `audit` ignores `text`, `category`, `note` and `types`. A line with no names has `"entities":[]`, which `audit` counts as labeled.

`relations.jsonl` adds `relations`, the stated edges, in `relate`'s edge shape. `optional_relations`, on c10, c11 and c12, holds the edges local experiment 265 neither credited nor penalised. `unstated`, on c01, c02, c05, c06 and c14, holds one edge each that is true in the world and that its sentence does not state. `alt_kinds` on c17 says `Indica Gallery` may be a `place`.

## Sources and license

`names.jsonl` was written by hand for this repository in local experiments 267 and 277. It converts local experiment 277's `cases.jsonl`, SHA-256 `c361329c6d5b16f816f9d3059f42f010a15d74c47130f7fff474c597227abdc8`, on 2026-09-26. Lines n001 to n100 are local experiment 267's key unchanged, SHA-256 `87b49d13e01d6f641aaf9e5aeb1521fc859210f155b27a7e27ce677c3011ded5`. The conversion renames `label` to `kind` and `text` to `name`, and writes "Local experiment 265" for "Experiment 265" in the notes of n001, n017, n019, n024 and n034.

`relations.jsonl` converts local experiment 265's `cases/cases.jsonl`, SHA-256 `3b63bb99b5f6c0cf11198e9b4c34c80c01634d57da2e743c6aca0a67e2fc1a6e`, at that experiment's commit `2aea0a7`. Offsets come from a forward search that starts at the previous name's end. The five unstated edges come from that experiment's README, misses row 11.

```sh
sdlc/scripts/recognize-keys convert names SOURCE > specification/fixtures/recognize/names.jsonl
sdlc/scripts/recognize-keys convert relations SOURCE > specification/fixtures/recognize/relations.jsonl
```

Every sentence was written for this project, and the files are under the repository's MIT license. No sentence comes from a dataset. Local experiment 267 kept a 25-group WNUT-17 sample, which is CC-BY 4.0, and these files copy none of it. Note n066 names WNUT-17 as the source of a known confusion and copies no WNUT-17 text. The convention below cites Universal NER, MUC-7, OntoNotes, ACE, WNUT-17, CheckList and RockNER, and copies no text from any of them.

## Counts

The name key has 33 categories: possessive 20, quotes-brackets 10, punctuation 9, extra-types 9, titles-honorifics 8, side-by-side 7, initials-abbrev 7, small-words 6, case 6, list-and 6, sentence-edge 6, numbers 6, non-english 6, regnal-sequel 6, the-article 6, metonymy 6, ambiguous 6, hyphen 5, nested 5, no-names 5, brand-case 5, handles-hashtags 5, shared-names 5, descriptions 5, abbrev-defined 4, nicknames 4, long-titles 4, url-email 4, dates-numbers 4, emoji 4, informal-lowercase 4, transliteration 4, line-break 3. The five core kinds hold 347 names: place 96, person 90, organisation 82, work 57 and thing 22. The five extra kinds hold 25: event 10, language 6, law 4, award 3 and nationality 2.

```bash
set -euo pipefail
jq -s -c '{lines: length, names: ([.[].value.entities[]] | length), categories: ([.[].category] | unique | length)}' names.jsonl \
  | mustmatch '{"lines":200,"names":372,"categories":33}'
jq -s -c '[.[].value.entities[].kind] | group_by(.) | map({(.[0]): length}) | add' names.jsonl \
  | mustmatch '{"award":3,"event":10,"language":6,"law":4,"nationality":2,"organisation":82,"person":90,"place":96,"thing":22,"work":57}'
jq -s -c '{lines: length, names: ([.[].value.entities[]] | length), stated: ([.[].relations[]] | length), unstated: ([.[].unstated[]?] | length)}' relations.jsonl \
  | mustmatch '{"lines":30,"names":73,"stated":27,"unstated":5}'
```

## Grading a run

`audit` refuses a key kind the run did not ask for. Filter the key to the run's kinds first, with `kinds.jq`. A line whose names all drop stays, with an empty list, so a name printed there still counts as extra. The `person` filter leaves 124 such lines, and the five core kinds leave 13.

```sh
jq -c --argjson kinds '["person"]' -f kinds.jq names.jsonl > person-key.jsonl
thinkthen recognize person --jsonl --field /text --threshold 0.01 --details < names.jsonl \
  | thinkthen audit - person-key.jsonl --match strict --table
```

The block below builds answers from the key with `jq`, each name said at strength 1, and grades them. Perfect answers under all ten kinds match every name. One moved offset costs one match. Answers under `person` alone fail against the whole key, and grade against the key filtered to `person`. Then a place said as a person counts as extra.

```bash
set -euo pipefail
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
answers() {
  jq -c --argjson kinds "$1" '{input: {id}, value: {entities: [.value.entities[] | select(.kind | IN($kinds[])) | .strength = 1]},
    question: {verb: "recognize", kinds: ($kinds | map({key: ., value: .}) | from_entries), threshold: 0.5}}' names.jsonl
}
grade() { thinkthen audit - "$1" --match strict --table | sed -n 2p; }

answers "$(jq -s -c '[.[].value.entities[].kind] | unique' names.jsonl)" > "$work/all.jsonl"
grade names.jsonl < "$work/all.jsonl" \
  | mustmatch "  matched 372, extra 0, missed 0: precision 1.000   recall 1.000   f1 1.000"
jq -c 'if .input.id == "n001" then .value.entities[0].start += 1 else . end' "$work/all.jsonl" | grade names.jsonl \
  | mustmatch "  matched 371, extra 1, missed 1: precision 0.997   recall 0.997   f1 0.997"

answers '["person"]' > "$work/person.jsonl"
set +e
thinkthen audit "$work/person.jsonl" names.jsonl --table > /dev/null 2> "$work/error"
code=$?
set -e
test "$code" -eq 2
mustmatch "thinkthen: audit: key line 1 names a level, label, or unit the question does not have" < "$work/error"

jq -c --argjson kinds '["person"]' -f kinds.jq names.jsonl > "$work/person-key.jsonl"
grade "$work/person-key.jsonl" < "$work/person.jsonl" \
  | mustmatch "  matched 90, extra 0, missed 0: precision 1.000   recall 1.000   f1 1.000"
jq -c 'if .input.id == "n035" then .value.entities += [{name: "Rome", kind: "person", start: 17, end: 21, strength: 1}] else . end' \
  "$work/person.jsonl" | grade "$work/person-key.jsonl" \
  | mustmatch "  matched 90, extra 1, missed 0: precision 0.989   recall 1.000   f1 0.994"
```

A run under the five core kinds, graded against the key filtered to them, never meets an extra-kind name in the key, so it misses none. It still loses precision when it says a core kind at an extra-kind span, such as `Christmas` in n173, an `event`, said as a `thing`. Local experiment 277 counts such an answer neither way, following MUC-7's optional strings. `audit` cannot express that rule, and it affects at most 25 of the 372 names.

```bash
set -euo pipefail
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
core='["place","person","organisation","work","thing"]'
jq -c --argjson kinds "$core" -f kinds.jq names.jsonl > "$work/core-key.jsonl"
jq -c --argjson kinds "$core" '{input: {id}, value: {entities: [.value.entities[] | .strength = 1]},
  question: {verb: "recognize", kinds: ($kinds | map({key: ., value: .}) | from_entries), threshold: 0.5}}' \
  "$work/core-key.jsonl" > "$work/core.jsonl"
thinkthen audit "$work/core.jsonl" "$work/core-key.jsonl" --table | sed -n 2p \
  | mustmatch "  matched 347, extra 0, missed 0: precision 1.000   recall 1.000   f1 1.000"
jq -c 'if .input.id == "n173" then .value.entities += [{name: "Christmas", kind: "thing", start: 12, end: 21, strength: 1}] else . end' \
  "$work/core.jsonl" | thinkthen audit - "$work/core-key.jsonl" --table | sed -n 2p \
  | mustmatch "  matched 347, extra 1, missed 0: precision 0.997   recall 1.000   f1 0.999"
```

`audit` does not grade `recognize`'s relations. The edges take `relate`'s shape, so a run reshaped with `jq` into `relate` lines grades against `{id, value: .relations}`. The stated edges match all 27. The five unstated edges, said as well, count as extra.

```bash
set -euo pipefail
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
jq -c '{id, value: .relations}' relations.jsonl > "$work/key.jsonl"
plan="$(jq -s -c '[.[].relations[] | {name: .relation, source: .source.kind, target: .target.kind, reads: .relation, either: false}] | unique' relations.jsonl)"
jq -c --argjson plan "$plan" '{input: {id}, value: [.relations[] | .probability = 1], question: {verb: "relate", relations: $plan, threshold: 0.5}, unstated}' \
  relations.jsonl > "$work/stated.jsonl"
thinkthen audit "$work/stated.jsonl" "$work/key.jsonl" --table | sed -n 2p \
  | mustmatch "  matched 27, extra 0, missed 0: precision 1.000   recall 1.000   f1 1.000"
jq -c '.value += [.unstated[]? | .probability = 1]' "$work/stated.jsonl" | thinkthen audit - "$work/key.jsonl" --table | sed -n 2p \
  | mustmatch "  matched 27, extra 5, missed 0: precision 0.844   recall 1.000   f1 0.915"
```

## Known key limits

- A core-kind answer at an extra-kind span counts as extra, as the section above shows.
- c17 records `Indica Gallery` as `organisation`, with `place` as an alternative. `audit --match strict` does not read `alt_kinds`, so a `place` answer there counts as a miss.
- The optional edges of c10, c11 and c12 are recorded, and `audit` counts one said as extra.
- n152, `Mr. and Mrs. Smith`, names two people, and the key gives one span, `Smith` at 13 to 18.

## Agreement

A blind second annotator agreed with the name key on 59 of 60 sampled cases after one key fix, with span F1 and labelled F1 both 0.995 (local experiment 277). The second annotator is the same model family as the key's author, so the figure is likely high.

## The convention

The name key follows local experiment 277's 29 rules, copied here. Rules 1 to 11 are local experiment 267's rules, and 277 widened rules 1, 3 and 5.

### Edges

1. A possessive `'s`, or a bare apostrophe after s, stays outside the name. The apostrophe can be straight `'`, curly `’`, a modifier letter `ʼ`, a backtick `` ` ``, an acute `´` or a prime `′`. `George Harrison's` gives `George Harrison`. `the Williamses' house` gives `Williamses`. The contraction `'s` for "is" or "has" also stays outside: `Adele's back` gives `Adele`. A group possessive closes the whole name: `The Bank of England's governor` gives `Bank of England`.
2. An apostrophe in the name's own spelling stays inside: `O'Brien`, `Octopus's Garden`, `Rock 'n' Roll Music`, `McDonald's`, `Don’t Pass Me By`.
3. Quote marks and brackets around a name stay outside. Punctuation that belongs to the name stays inside: `Help!`, `Yahoo!`, `U.S.`, `Inc.`, `Jr.`, `Washington, D.C.`, `2001: A Space Odyssey`. Rule 21 gives the one exception, a quoted nickname inside a person's name.
4. When one period ends both an abbreviation and the sentence, the name keeps it: `He moved to the U.S.` gives `U.S.`
5. The article rule:
   - A lowercase `the` before a name stays outside: `the Beatles` gives `Beatles`.
   - A capital `The` in mid-sentence stays inside, because the writer treats it as part of the name: `The Who`, `The Hague`, `The Economist`.
   - A capital `The` or `A` at the start of a sentence stays inside only when it starts a work title: `The Lord of the Rings was filmed`. Before any other name it stays outside: `The Beatles played` gives `Beatles`, and `The Kremlin denied` gives `Kremlin`.
6. Titles such as `Dr.` stay outside a person's name. Rule 12 gives the full rule.
7. Names side by side are separate names, even when both have the same kind.
8. A name is tagged whole. A name inside a longer name gets no second tag: `University of Liverpool`, `Harry Potter and the Philosopher's Stone`, `Detroit Auto Show`.
9. Names joined by `and` are separate unless `and` is part of one name (`Pride and Prejudice`, `Simon and Garfunkel`) or the names share an elided part (rule 19). A duo or group that performs under a joined name is an `organisation`, like a band.
10. Only the name part of a hyphenated word is the name: `London` in `London-based`. Universal NER would include `-based`.
11. Case changes nothing: `paul mccartney`, `PAUL MCCARTNEY`, `coldplay` and `folklore` are names.

### People

12. Titles and honorifics stay outside a person's name. These include `Mr.`, `Mrs.`, `Ms.`, `Dr.`, `Sir`, `Dame`, `Professor`, `Captain`, `President`, `Pope`, `King`, `Queen` and `Prince`. `Queen Elizabeth II` gives `Elizabeth II`. `Mr. Holmes` gives `Holmes`. A title with no name after it is not a name: `the Queen spoke`, `the King would visit`.
    - A title word that is part of a work, place or organisation name stays inside: the films `Captain Marvel` and `Mr. & Mrs. Smith`, the album `Sgt. Pepper's Lonely Hearts Club Band`, `St. Mary's Church`.
    - A peerage title that holds a place and serves as the person's only name is the whole name: `Duke of Wellington`.
13. Generational numbers (`Jr.`, `Sr.`, `III`), regnal numbers (`Henry VIII`, `Elizabeth II`) and sequel numbers (`Rocky II`, `Toy Story 3`) stay inside the name.

### Kinds

14. Label a name by what it refers to in its sentence. This is metonymy.
    - A capital city or a seat of government that acts for a government is an `organisation`: `Washington imposed tariffs`, `Downing Street said nothing`, `The Kremlin denied`.
    - A country or city used for its team is an `organisation`: `Brazil beat Argentina`.
    - A building or institution named as the venue of an event (a talk, a concert, a match, a stay) is a `place`: `spoke at the Royal Institution`. The same name acting, employing or teaching is an `organisation`: `The White House is white` against `the White House vetoed`, and `works at New York University`.
    - A place name that stands for a whole industry or sector keeps the literal label `place`: `Hollywood`, `Wall Street`.
    - A person's name that stands for their works keeps the label `person`: `read all of Dickens`, `owns a Hockney`.
    - A periodical that acts (reports, reviews, employs) is an `organisation`. One that is read or watched is a `work`.
15. When a spelling fits several kinds, context decides: `Jordan scored` is a person, `a trip to Jordan` is a place. When the context does not decide, choose the literal meaning, then the most common one. A place where an event happened stays a `place`: `won at Waterloo`. Only an event's own name is an `event`: `Battle of Hastings`.
16. A company as a business is an `organisation`. A brand used for goods is a `thing`: `adidas trainers`, `a starbucks` meaning a coffee. Software is a `thing`. Games, TV shows and speeches are `work`. A product name that holds its maker's name is one `thing`: `MacBook Air`, `Model S`.
17. Names keep their own spelling and case: `iPad`, `eBay`, `easyJet`, `adidas`, `IKEA`.

### Social media and addresses

18. The `@` of a handle and the `#` of a hashtag stay outside the name. Tag a handle or hashtag only when it names something, and label it by what it names: `@andymurray` gives person `andymurray`, `#Wimbledon` gives event `Wimbledon`. A hashtag that is an ordinary word or phrase is not a name: `#tbt`, `#blessed`.
19. When names share an elided part, the whole joined phrase is one name: `North and South Korea`, `Venus and Serena Williams`. Titles still stay outside: `Mr. and Mrs. Smith` gives `Smith`.
20. An abbreviation defined in brackets gives two names with the same label, the long form and the short form. The brackets stay outside: `European Space Agency (ESA)`, `MIT (Massachusetts Institute of Technology)`.
21. A nickname is a name with the label of what it names: `Satchmo`, `Fab Four`, `Big Apple`, `Red Devils`. A quoted nickname inside a person's full name stays inside with its quote marks: `Dwayne "The Rock" Johnson`. Rule 5 still applies, so `the Big Apple` gives `Big Apple`.
22. A long title is one name, with its internal punctuation and small words: `Dr. Strangelove or: How I Learned to Stop Worrying and Love the Bomb`.
23. A web address or email address is not a name, and names inside it are not tagged. An address that acts in place of a name is tagged whole, with the label of what it stands for: `press@bbc.co.uk confirmed` gives organisation `press@bbc.co.uk`.

### Text features

24. White space inside a name stays inside, including a line break: `Pink\nFloyd`. A hyphen added at a line wrap stays inside with the break: `Rijks-\nmuseum`. Names on separate lines of a list are separate names.
25. Emoji stay outside names, even when they touch: `Lisbon❤️` gives `Lisbon`.
26. Short and informal forms are names when they name something: `spurs`, `villa`. Chat words such as `omg`, `b4` and `lol` are not names.
27. A transliterated or variant spelling is a name with the same label as the usual spelling: `Peking`, `Rachmaninov`, `Moskva`. Name particles stay inside: `Ibn Battuta`, `da Vinci`, `van Beethoven`.

### Not names

28. Pronouns, descriptions and roles are not names: `she`, `the singer`, `the author of Emma`, `the prime minister`, `our president`. A name inside such a phrase is still a name: `Emma`, `Leeds` in `the mayor of Leeds`.
29. Dates, weekdays, months, years, decades, times, money, counts and percentages are not names: `14 July`, `May`, `1990s`, `£45`, `12%`. A named holiday or festival is an `event`: `Christmas`, `Woodstock`. A number inside a name stays inside: `Studio 54`, `2012 Summer Olympics`, `Copyright, Designs and Patents Act 1988`.
