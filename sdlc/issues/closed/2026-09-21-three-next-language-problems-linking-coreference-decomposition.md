# Three next language problems: linking, coreference, decomposition

Status: Closed on 2026-09-25 as reference. No work is owed. Backlog that authorizes nothing. Earlier status: Backlog. Written 2026-09-21 by the product side from Ian's discussion with the recognize team. Nothing here holds up `recognize` or `relate`, and nothing here is authorized work. The team's own shelf with worked examples is the local experiment note `RECOGNIZE-BACKLOG.md`. This page is the product's record of what each idea is, what shape it would take, and whether it would be a new function. Ian's order: linking first, coreference second, decomposition last.

## The pattern all three share

Code proposes and the model chooses. Code splits the text, builds the list of candidates, decides which questions ride in one request, and assembles the answers. The model answers small bounded questions, and a probability rides on every answer. The caller owns the threshold. No word list, dictionary, or template enters the pipeline, and "none of these" is always an option.

## 1. Linking a name to the caller's own list

**What it is.** `recognize` finds "Herceptin". The caller wants to know which entry in their own vocabulary that is.

**The shape.** This is `choose`, used only where there is doubt. An exact match against the caller's list settles most names with no request at all. The model is asked in two cases only. One written form can mean two things: "ER" is a receptor in one sentence and an emergency room in another, and the sentence decides. Or no exact match exists and code finds a few close entries by whatever means the caller owns (spelling overlap, a vector index), and the model picks among them or answers "none of these". "None of these" is useful output: it flags a name the list does not have yet.

**New function?** No. It is a how-to: `recognize`, then the caller's own lookup, then `choose` with `--none` for the doubtful ones. The product stays out of the lookup business. The tool ships no index and no similarity search.

**Open questions.** How a caller hands over candidates with descriptions cleanly (a question file with options already does most of it). How much of the surrounding sentence rides with the name. Which labeled set would measure it.

## 2. Coreference: tying words to the names already found

**What it is.** "Maria Chen joined Northwind Freight. She leads its Chicago office." `recognize` finds Maria Chen and Northwind Freight. It does not find "she" or "its", because they are no names. Coreference ties those words to the names they point at, so everything said about "she" lands on Maria Chen.

**The shape.** Two steps. First a new detection question over the words: does this phrase point at someone or something already mentioned? That catches "she", "the company", "the 54-year-old", which `recognize` cannot see. Then one pick-one question per reference: which earlier mention does it point at, with the earlier mentions as the options and "something new" always present. Code builds the groups from the answers, each group anchored by its named member. The second step is the `same_as` relation that `relate` already carries as an either-way rule.

**New function?** Yes, this is the one candidate for an eleventh function, because its input and output differ from every current function: a text and its found names go in, and groups of mentions come out. The eleventh-function sweep (`2026-09-21-is-there-an-eleventh-function-a-sweep-of-the-three-primitives.md`) found none among today's shapes, and this does not contradict it. This shape only exists once `recognize` does. A possible spelling is `recognize --references`, which adds the pointing words to each entity's record and keeps the function count at ten. The product side leans that way until a how-to shows it cannot work.

**Open questions.** A long document has more earlier mentions than the 100-option limit allows, so code must narrow the candidates by nearness before asking. "Her tumor" is related to the patient and is not the same as the patient, so the question must separate "same thing" from "belongs to". The detection wording is new and unmeasured.

## 3. Decomposition: one long phrase that is really several things

**What it is.** "Senior vice president of European sales at Northwind Freight" reads as one span and holds a rank, a region, a department, and an employer. Some names are bundles.

**The shape, undecided.** Two roads are recorded on equal footing. Decompose after recognition: ask whether a found span is one thing or a bundle, then ask one small typed question per part. Or, Ian's alternative, instruct detection itself to take the leftmost-longest phrase including composites, and split afterward. Nobody has measured whether the model draws the line between a simple name and a bundle the way a human curator does. That measurement is the entry ticket.

**New function?** Probably not. It looks like `annotate` run over each found span with a small form. It is the trickiest of the three and goes last.

## What the product does with this now

Nothing in the first release. After `recognize` and `relate` land, linking becomes a how-to on the site, since it needs no new code. Coreference gets an experiment brief when Ian asks for one. Decomposition waits for the measurement above.
