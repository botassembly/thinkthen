Status: open. Filed 2026-09-26 by the marketing lead from a fresh architect review.

# Architect review 10: `recognize` and `relate`

A fresh reviewer tested `recognize` and `relate` as an architect who builds a name and relation graph from documents. The review ran against main `9d652bed` and the release binary. It rated the topic fair and found 2 severity 1, 4 severity 2 and 8 severity 3 issues. The full detail sits in the architect review report 273, 10. Work file names in that report refer to its local work folder, which stays unpushed.

Severity 1 means a wrong answer, data loss, a security problem or a hang. Severity 2 means a broken guarantee or a misleading document. Severity 3 means a sharp edge or a missing feature an integrator needs.

`2026-09-26-recognize-design.md` fixes several nearby problems: word rules, `confirm`, stated relations and long texts. It does not cover items 1 and 2 below. Its known-gaps list names only touching names of one kind.

## 1. One kind gives that kind to every name the model detects (severity 1)

Settled by ticket 0147, landed 2026-09-27. Every run with kinds asks the kind question with `none of these`, so a name no listed kind covers is dropped. Item 2 stays open for standalone `relate`.

Evidence. Live, `recognize person` labeled `Acme Corp` (0.99) and `Paris` (0.79) as `person`. Replayed real Jev detection answers label a song, an island, a studio and an album as `person` at 0.98 to 0.99. `facade/recognize.rs:219-231` sets the kind probability to 1.0, and the detection question at `recognize.rs:7` asks about every kind of named entity.

What an integrator hits. "Find the people in this text" is the most natural first call. It returns every organization and place as a person, with a strength that looks certain. Nothing in the output marks the mistake.

Direction. With one kind, ask the kind question against an implicit "something else" option, or ask a yes/no "is this a KIND?" per name. Until then, state on `recognize.md` that one kind means "every name", and refuse or warn.

## 2. A cross-kind relation keeps at most one edge per asking name, and the asking side depends on unrelated names (severity 1)

Evidence. Live, "John Lennon wrote Help!, Girl and In My Life" gave 1 of 3 edges (0.81, 0.12, 0.02). In replay, the same facts gave 0 edges with 3 songs and 3 edges after adding an unrelated fourth song. Code: `relation.rs:188-257` and `:194`. ADR 0019 makes the options total one. The bench states the same effect on real duets.

What an integrator hits. True edges vanish silently for any one-to-many fact, such as the authors of a paper or the members of a band. Adding or removing an unrelated name changes which edges come back, so two runs over overlapping texts disagree.

Direction. Give a relation a declared cardinality, or ask a many-option relation as an "every applicable" question in the `tag` shape. At minimum, state the limit in `relate.md` and `recognize.md`, and show the chosen method and asking side in the plain output.

## 3. Kinds and their descriptions never steer detection (severity 2)

Evidence. Live, `job_title` and `building`, bare and described, found nothing in a sentence full of both. The detection text is fixed at `recognize.rs:7` and names "a news document".

What an integrator hits. `--kind KIND=DESCRIPTION` reads like "find things of this kind". It only picks a label for words the fixed news-style question already calls names. Domain terms that are common nouns or lowercase never come back.

Direction. Say plainly on `recognize.md` that detection finds proper names in the news sense and kinds only label them. Longer term, let the described kinds shape the detection question, and measure it. `2026-09-25-recognize-and-relate-scale-and-shape.md` item 8 asks to show the fixed wording. This item adds the live evidence that kinds do not steer it.

## 4. The published recognize accuracy overstates the shipped build (severity 2)

Evidence. The site's "The best is `recognize` at 0.96" is song precision on 48 easy template sentences with a lenient scorer (Beatles Bench `reports/results.md:66-81`). The shipped build scores 70.8% recall and 76.3% precision on the 100-sentence edge-case key (experiment 270). The 87.5% figure comes from a simulation of an unbuilt design. The bench's relate F1 of 0.72 is closed-book, so it measures model knowledge, not extraction.

What an integrator hits. Capacity and review plans sized for 96% accuracy meet about 70% recall on ordinary prose with possessives, quotes and adjacent names.

Direction. Publish the shipped build's key score beside the bench number, name the measure (precision, one kind of name) in the claim, and label the relate figure as knowledge rather than extraction. Marketing owns the site and will correct its copy. Ticket R1 in the recognize design puts the key in the repo so `audit` can grade each build.

## Carried in other files

- A one-page text fails at the default address, and the plan does not warn (issue 4, severity 2). The architect review 06 file carries the ceiling for every verb, and recognize design ticket R4 carries the recognize fix.
- A `recognize` relation reports world knowledge as if the text stated it (issue 5, severity 2). Recognize design section 5 and ticket R5 carry it, and the closed issue `2026-09-26-recognize-relations-report-edges-the-text-does-not-state.md` holds the measurement. This review adds live evidence: `wrote(Ringo Starr, Octopus's Garden)` at 0.79 from "Ringo Starr sang Octopus's Garden." Standalone `relate` still needs one sentence saying its answers come from the model's knowledge.

## Severity 3 titles

- Lowering the name threshold never adds a word (`facade/recognize.rs:245` fixes membership at `IN > OUT`).
- Possessives and quote marks stay in names, and touching names merge. The recognize design sections 2 and 3 cover it.
- Mentions, not entities: repeats multiply relation cost and output, and SQL edge rows cannot tell them apart.
- `recognize --relation` has no entity cap and no useful cost bound: the dry-run bound for a 1,000-word text is 1,998,000.
- `recognize` sends its relation rules one after another: 0.87 s for 1 + 4 requests against 0.25 s for the same rules in `relate`.
- `recognize --details` hides relation probabilities.
- Offset units and field names change by surface.
- Split texts repeat the whole text in every request: 51% of bytes at 10,000 words. The recognize design section 6 covers word questions.
