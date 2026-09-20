# `tag`: a fourth question type, for zero or more labels from a list

Status: Open. Proposed for an ADR. No code is authorized by this page.

Ian asked for it on 2026-09-20. His words: "I'd like to add tagging if that actually makes sense, and that would be a workable demo." He gave two jobs. A file of emails gets several tags at once: spam or not, important or not, and a bucket. A blog inventory has twenty topic tags, and each post gets every topic that applies.

An earlier issue the same day recommended against a `tag` verb, on the grounds that `annotate` with one `decide` per label does the same. This page reverses that recommendation. Two facts changed it.

## The two facts

1. **The vendor does many labels as one request.** The vendor's guardrail cookbook splits "out of bounds" into four yes or no questions and sends them as one battery in one call. A sibling tuning tool from the same ecosystem builds one yes or no question per label and sends every label in one `questions` map. The vendor measured thirteen questions over one long article: the same answers, 12.2 times cheaper and 10.0 times faster than thirteen calls. `specification/annotate.md` already plans to send every question over the same evidence in one request. Twenty tags on a post therefore cost one request and bill the post once.
2. **Twenty hand-written entries is real friction.** The `annotate` route makes a user write one named `decide` question per label, then one `jq` line to fold twenty true or false fields into a list. A person who wants to tag a blog will not get that far. "Tag" is also the word that person already uses.

## What it looks like

The four question types then read as a set: one yes or no (`decide`), one of many (`choose`), any of many (`tag`), and a place on a scale (`score`).

As a verb, with the same grammar as `choose`:

    thinkthen tag 'Which topics does this post cover?' rust python devops career < post.md

One document prints one JSON list on one line, such as `["rust","devops"]`, and `[]` when no label passes. A record stream prints one JSON list per record, in input order. `--details` carries the probability of every label. An earlier draft of this page printed one label per line for one document. That was wrong, and the update at the end says why.

As a question file:

    {"tag": "Which topics does this post cover?", "labels": ["rust", "python", "devops", "career"]}

`labels` takes the two shapes `options` already takes: a list, or a map from each label to one sentence saying when it applies.

Inside an `annotate` file, which covers Ian's email job whole:

    {
      "spam":      {"decide": "The email is unsolicited bulk mail."},
      "important": {"decide": "The email needs a reply from me this week."},
      "folder":    {"choose": "Which folder fits this email?", "options": ["family", "work", "bills", "newsletters", "other"]},
      "topics":    {"tag": "Which topics does the email touch?", "labels": ["travel", "money", "health", "school"]}
    }

The first three lines work under `annotate` as it is specified today. Only the fourth needs this proposal. The record comes back with four new fields, and `topics` holds a list.

## On the wire

One yes or no question per label, all in one `questions` map, one request per record. The label's description rides in `criteria.true`, the field `--true` already fills. The wording that joins the question to a label has to be measured. The sibling tool appends a sentence of the form "Determine whether the label NAME applies."

`choose` cannot do this job. It returns one distribution that sums to one, so three true topics would each score near a third and none would pass a cut.

## What to measure before an ADR accepts it

- How many yes or no questions one request holds, and whether answers hold steady at 5, 10, 20, and 40 labels against one request per label. The plan already queues this probe behind `annotate`.
- Whether a label's answer moves when its neighbors change. A tag that flips because an unrelated label joined the list is a defect users will find.
- The joining sentence, and whether a bare label with no description is good enough to be the default.

No live call was made for this page when it was written. A live run later on 2026-09-20 measured it, and `2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md` holds the numbers. Twenty labels in one request worked on twenty made-up posts. Bare labels matched the whole set on 10 of 20 posts. A description per label, sent as `criteria.true`, raised that to 16 of 20 and cut false positives from 10 to 3. Two added labels and a shuffled order each flipped 3 answers of 400.

## To decide

- The exit code of `tag` on one document when no label passes. `filter` sets no exit code from an answer, and `grep` exits 1 on no match.
- Whether `tag` takes a band. A single cut is simpler, and `--details` already shows the middle.
- Whether the verb lands with `annotate` or right after it. It shares the request builder for many questions.

## The demos it opens

1. Tag a blog inventory with twenty topics, one request per post.
2. Triage a mailbox export into spam, important, a folder, and topics, then split it into one file per folder.
3. Both start from a CSV file for most people. See the stumble register for CSV.

## What Ian can overturn

Nothing here is decided. The recommendation is to add `tag` as the fourth question type and the eighth verb, after `annotate`, if the probe holds at twenty labels. The change widens the public surface, so a second agent reviews it and an ADR records it.

## Update, later on 2026-09-20: three public choices

**Superseded on 2026-09-20 by ADR 0028.** Ian ruled that CSV and DSV are input only and every output remains JSONL. Points 2 and 3 below preserve the earlier recommendation; they no longer steer implementation.

Ian relayed three questions from the builder and asked for the marketing side's view. These are recommendations. The builder's ADR decides.

1. **`tag` prints one JSON list per line.** Ian assumed so, and he is right. The tool already prints JSON values: `choose` prints `"billing"` and `--raw` prints `billing`. A list keeps one line out for each record in, so line N still belongs to record N and `paste` and `jq` keep working. `[]` says "no label passed", which one label per line cannot say, because an empty output also looks like a run that failed before it printed. `--raw` gives the shell form: one label per line for one document, and one tab-joined line per record in a stream. A label already refuses control characters, so a tab is a safe separator. In a library the same answer is a plain list of strings.
2. **`--csv` and `--tsv` say how to read, and nothing more.** The tool gets no CSV writer and no output mode. Everything the tool produces stays JSON: a judgment is a JSON value, a `--details` row is a JSON object, and `annotate` builds a JSON object with the columns in their order and the judged fields after them. A list of tags, an unresolved `null`, and a nested `--details` row have no honest spelling in a CSV cell, and each spelling would be a public choice that is awkward to reverse. One nuance follows from a promise the tool already makes. `filter` and `rank` produce nothing. They hand back the user's own records as they arrived. On CSV input that means the header line and then the kept rows, byte for byte. A semantic grep over a spreadsheet then returns a spreadsheet, with no writer in the tool. If that nuance costs too much, the fallback is JSON objects from every verb, and the how-to ends with one `jq` line.
3. **The page that writes a CSV back is a transform.** A spreadsheet user wants a spreadsheet at the end of `annotate`. A `jq` file under `transforms/` that writes a header, joins a list with a semicolon, and ends in `@csv` covers it and keeps the rule that transforms come before any new command.
4. **Keep twenty how-to pages.** Every page is a tested demo, so a smaller set that is all green beats a larger one. Two conditions from the marketing side. Each title names the job in the user's words, because the site builds its navigation from these pages. The rewritten pages have to cover the four lessons the launch leans on: split one file into piles by a `choose` label, a tool-call guard with the exit code mapping, the long-lived loop, and text split into paragraphs before a verb reads it. CSV rules also need a home on the records reference page, so a reader who only wants to read a CSV does not have to find the tagging page.
