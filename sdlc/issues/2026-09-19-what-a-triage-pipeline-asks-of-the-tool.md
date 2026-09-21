# What a triage pipeline asks of the tool

Status: Reference. The question is answered: no new verb. The flagship how-to carries the rest.

Found 2026-09-19. Ian described a document triage pipeline and asked whether the tool as designed would serve it. The pipeline finds candidate items in a document with local code, sends one small packet per candidate, asks about twenty narrow questions of each packet across all three question types, and lets plain code turn the answers into one of three actions: draft, block, or send to a person for review. Every action keeps an audit row. Thresholds are tuned later from the reviewers' decisions.

The design already covers the judging and the audit. One packet is one JSONL record. The question set is one `annotate` file. `on` with several pointers names exactly what leaves the machine. `--details` rows carry every probability, the model version, the digest of the question file, and a request digest. A band on a yes/no question turns a middling answer into `null`, and `null` is the review queue. No new verb or option is needed.

The pipeline adds five things to the work ahead. None widens the command surface.

1. **A flagship demo for slice 9.** One demo shows the whole shape on a plain subject such as support tickets: candidates in, `annotate` with all three question types, a policy in `jq`, three output files, and the audit rows. Demo 07 or 08 can grow into it.
2. **A policy recipe.** The step after `annotate` is always a policy: several answers in, one action and one reason out. The recipe is a `.jq` file with the explicit three-way test for yes, no, and unresolved, a block rule that any single safety answer can fire, and a `reason` field that names the rule that fired. A reviewer reads the reason.
3. **Two more recipe families.** Monitors count actions, and they count how often a reviewer overturned one. A grouped sweep fits one cut per group of records, such as one per document type. With the metric recipes that makes three families, which bears on Ian's fifth outcome for `report` in ADR 0010: a way to carry recipes inside the tool enters only when several exist.
4. **A pattern worth a page in the help.** The same question asked over two scopes of evidence, a sentence and its wider window, costs two requests per record and needs no feature. Disagreement between the two answers sends the record to review. Agreement is never averaged into one number.
5. **Two checks for the live probe.** Does the order of the options move a `choose` answer on our own cases, and does an added irrelevant option move it? A saved clipping reports that both do. If so, the help for `choose` says to keep option order fixed once a cut is tuned.
