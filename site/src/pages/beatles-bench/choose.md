---
layout: "../../layouts/BenchPage.astro"
title: "How to pick one option with choose"
tagline: "choose selects one option."
slide: "/beatles-bench/img/choose.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/examples/02-choose/README.md"
runsIn: "examples/02-choose"
---

Use `choose` when a text needs exactly one answer from a list you give. It returns the pick and a probability for every option. The probabilities add up to 1.

Here Jev names the lead singer of five songs. The options are John, Paul, George, Ringo, and a John and Paul duet.

## The files

- `recording/` holds the saved requests and answers, so the command needs no key.
- `choose-cold.jsonl` holds the cases, one song each, with the right answer taken from `data/songs.tsv`.
- `choose-context.jsonl` holds the same five songs with each song's catalog entry.

The question and the options sit on the command line, so this example needs no question file. The last line of `choose-cold.jsonl` holds the case for She Loves You:

```sh
sed -n 5p choose-cold.jsonl
```

```text
{"id": "choose-cold-05", "function": "choose", "args": ["The text is the title of a song by the Beatles. Who sings the lead vocal on it?", "John", "Paul", "George", "Ringo", "John and Paul duet", "--jsonl", "--field", "/input"], "records": [{"id": "choose-cold-05", "input": "She Loves You"}], "title": "She Loves You", "truth": "John and Paul duet", "fields": [["songs.tsv", "She Loves You", "lead_vocals", "Lennon+McCartney"], ["songs.tsv", "She Loves You", "article_lead", "Lennon+McCartney"]]}
```

`truth` is the right answer. `fields` names the rows and columns of `data/songs.tsv` behind it. `lead_vocals` comes from the list of songs, and `article_lead` from the song's own page. Both say Lennon and McCartney. [How to run the bench for free](/beatles-bench/run-it-for-free/) explains every field of a case line.

Everything in the folder:

- `choose-cold.jsonl`, `choose-context.jsonl`: the cases, one song each, written by `scripts/generate/examples.py`.
- `recording/`, `outputs.jsonl`, `timing.tsv`: the requests, the committed answers, and the wall times.
- `run.sh live OUT | replay [OUT]`: asks the cases through `scripts/run/functions.sh`.

## Run it

Run this in `examples/02-choose`:

```sh
printf '%s\n' 'Come Together' 'Yesterday' 'Something' "Octopus's Garden" 'She Loves You' |
  thinkthen choose 'The text is the title of a song by the Beatles. Who sings the lead vocal on it?' \
    John Paul George Ringo 'John and Paul duet' --lines --details --replay recording |
  jq -c '{song: .input, value, p: .answer.probabilities}'
```

```json
{"song":"Come Together","value":"John","p":{"John":0.98,"Paul":0.01,"George":0.0,"Ringo":0.0,"John and Paul duet":0.01}}
{"song":"Yesterday","value":"Paul","p":{"John":0.02,"Paul":0.98,"George":0.0,"Ringo":0.0,"John and Paul duet":0.0}}
{"song":"Something","value":"George","p":{"John":0.09,"Paul":0.02,"George":0.86,"Ringo":0.02,"John and Paul duet":0.01}}
{"song":"Octopus's Garden","value":"Ringo","p":{"John":0.01,"Paul":0.02,"George":0.13,"Ringo":0.84,"John and Paul duet":0.0}}
{"song":"She Loves You","value":"John","p":{"John":0.33,"Paul":0.25,"George":0.13,"Ringo":0.01,"John and Paul duet":0.28}}
```

- The words after the question are the options, in order.
- `--lines` takes each line of input as one record.
- `--details` prints the whole result, with every probability. Without it, each line gives the song and its pick.
- `--replay recording` answers from the saved recording. No request leaves the machine.
- `--threshold 0.5` would return no pick when the top option falls under 0.5.

To ask your own server, leave out `--replay recording` and follow [Ask your own server](/beatles-bench/run-it-for-free/#ask-your-own-server). The answers can then differ by a few points from these.

## Read it

The bench's `run.sh` sent the same requests and saved the answers in `outputs.jsonl`. This command sets each pick beside the right answer and marks it:

```sh
jq -c --slurpfile out outputs.jsonl '
  ($out | map({(.id): .rows[0]}) | add) as $rows | $rows[.id] as $row
  | {song: .title, lead_vocals: .fields[0][3], truth, jev: $row.value,
     p_jev: $row.answer.probabilities[$row.value], p_truth: $row.answer.probabilities[.truth],
     mark: (if $row.value == null then "not sure" elif $row.value == .truth then "right" else "wrong" end)}
' choose-cold.jsonl
```

```json
{"song":"Come Together","lead_vocals":"Lennon","truth":"John","jev":"John","p_jev":0.98,"p_truth":0.98,"mark":"right"}
{"song":"Yesterday","lead_vocals":"McCartney","truth":"Paul","jev":"Paul","p_jev":0.98,"p_truth":0.98,"mark":"right"}
{"song":"Something","lead_vocals":"Harrison","truth":"George","jev":"George","p_jev":0.86,"p_truth":0.86,"mark":"right"}
{"song":"Octopus's Garden","lead_vocals":"Starr","truth":"Ringo","jev":"Ringo","p_jev":0.84,"p_truth":0.84,"mark":"right"}
{"song":"She Loves You","lead_vocals":"Lennon+McCartney","truth":"John and Paul duet","jev":"John","p_jev":0.33,"p_truth":0.28,"mark":"wrong"}
```

Four of five are right. Octopus's Garden goes to Ringo at 0.84. The rest of its weight goes mostly to George at 0.13.

She Loves You is wrong. It is a duet, and Jev picks John at 0.33. The duet gets 0.28 and Paul 0.25. Jev spreads its weight across John, Paul, and the duet, and no option stands out. The top pick wins with a third of the weight. A bar of 0.5 would turn this wrong pick into not sure. A low top probability is the sign to look for.

## With context

The context run sends each song's catalog entry before its title. The entry for She Loves You ends with these two lines:

```sh
jq -r '.records[0].input' choose-context.jsonl | tail -2
```

```text
She Loves You (lead: Lennon, McCartney; written: Lennon–McCartney; 2:21; released 1963-08-23; first album: Past Masters)
Text: She Loves You
```

The question opens "The text gives a catalog entry and then names a song by the Beatles."

```sh
jq -c '.records[]' choose-context.jsonl |
  thinkthen choose 'The text gives a catalog entry and then names a song by the Beatles. Who sings the lead vocal on it?' \
    John Paul George Ringo 'John and Paul duet' --jsonl --field /input --details --replay recording |
  jq -c '{song: (.input.input | split("\nText: ") | last), value, p: .answer.probabilities}'
```

```json
{"song":"Come Together","value":"John","p":{"John":1.0,"Paul":0.0,"George":0.0,"Ringo":0.0,"John and Paul duet":0.0}}
{"song":"Yesterday","value":"Paul","p":{"John":0.0,"Paul":1.0,"George":0.0,"Ringo":0.0,"John and Paul duet":0.0}}
{"song":"Something","value":"George","p":{"John":0.0,"Paul":0.0,"George":1.0,"Ringo":0.0,"John and Paul duet":0.0}}
{"song":"Octopus's Garden","value":"Ringo","p":{"John":0.0,"Paul":0.0,"George":0.0,"Ringo":1.0,"John and Paul duet":0.0}}
{"song":"She Loves You","value":"John and Paul duet","p":{"John":0.06,"Paul":0.04,"George":0.01,"Ringo":0.0,"John and Paul duet":0.89}}
```

- `--jsonl` takes each line as one JSON record.
- `--field /input` names the part of the record Jev reads.

The entry for She Loves You reads "lead: Lennon, McCartney". With it, Jev picks the duet at 0.89. The four solo songs go to 1.0. All five are right.

## What can go wrong

- **The shared traps.** A replay miss, a missing key, and a cache bound to another server can stop any command. [How to run the bench for free](/beatles-bench/run-it-for-free/#what-can-go-wrong) gives each exit code and its fix.
- **The pick always comes back.** Without `--threshold`, `choose` returns its top option even at 0.33. Set a bar when a weak pick should read as not sure.
- **The options must cover the answer.** Leave out the duet, and She Loves You has no right option. Jev still picks one.

## The slide

Every value is white. Green marks a pick over the 0.5 bar that matches `data/songs.tsv`. Such a pick gets a green ring and a ✓. A pick under the bar gets an amber ring and a ?. She Loves You's pick of John at 0.33 gets the amber ring. A dashed green ring marks the real answer when the pick missed it. Here it circles the duet at 0.28.

## Related

- [How to fill in a form with annotate](/beatles-bench/annotate/): several `choose` questions saved in one file.
- [How to name every label that fits with tag](/beatles-bench/tag/): any number of labels in place of one pick.
- [Retrieval-augmented decisions (RAD)](/beatles-bench/rad/): what the catalog entry fixes and what it costs.

## Image credits

The four faces are crops of one photo: United Press International (UPI Telephoto), 7 February 1964, via the Library of Congress (cph.3c11094), https://commons.wikimedia.org/wiki/File:The_Beatles_members_at_New_York_City_in_1964.jpg. The crops are John_Lennon_NY_1964.png, Paul_McCartney_NY_1964.png, George_Harrison_NY_1964.png, and Ringo_Starr_NY_1964.png on Wikimedia Commons, cropped by Zakke and retouched by Indopug and Misterweiss. The photo was published in the US in 1964 with no copyright notice. It is public domain in the US (PD-US-no notice). Other countries may still protect it.
