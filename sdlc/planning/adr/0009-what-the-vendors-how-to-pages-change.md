# ADR 0009: What the vendor's how-to pages change

- Status: Accepted in part by Ian on 2026-09-19. Items 2, 4, and 6 are Accepted. Item 1 is held with `segment`, which left the plan. Item 3, `find`, stayed Draft until a live run compared it with `rank --top 1`. The run is ticket 0011, and ADR 0014 accepts `find` on its numbers, open to Ian's overturn. Item 5, structured questions, is struck from version one. Item 7 carries over to the `jq` recipes. ADR 0010 records the rulings
- Date: 2026-09-19

An agent proposal. Ian asked for the design to be checked against the first vendor's own how-to pages. Three agents read the saved documentation: the primitives, the patterns, and eighteen cookbooks. This record keeps what they found and what it changes. It becomes Accepted on Ian's word, and he can strike any line cheaply.

## What the pages confirm

- **`rank` is right as it stands.** The vendor's reranking recipe asks one yes/no question per candidate, runs the calls at once, and sorts by the probability. No recipe for pairwise ranking exists anywhere in the pages.
- **Many questions over one piece of evidence belong in one request.** The vendor measured thirteen questions over one long article: the same answers, 12 times cheaper, and 10 times faster than thirteen calls. The evidence is billed once per request. `annotate` already works this way.
- **One request per record is right for `filter`, `rank`, and `annotate`.** Packing many records into one request saves nothing on tokens, because each record is sent once either way. It saves round trips, and those already run in parallel. It costs isolation, and the pages warn that accuracy falls as the evidence fills with unrelated content.
- **A band over one answer is the vendor's only remedy for unstable answers.** The pages advise against asking a question together with its negation. The tool asks each question once.
- **The parallel setting.** The vendor's own code uses 4 to 12 workers and says the public endpoint limits above about eight. The published limits are 1,200 requests a minute, about 32,000 tokens of evidence, and 64,000 tokens a request. A default of 4 for `jobs` stands.

## Decision

1. **`segment` sends the whole document once.** Every unit carries an id, and one yes/no question per gap rides in a single request. This is the vendor's own measured recipe. `--window` is dropped. The tool adds the two unit ids to the user's question, and `--dry-run` shows the exact text. A document too large for one request is refused before any request.
2. **A detailed result keeps everything the backend returned.** `answer` carries the probability of every option or level and the vendor's `confidence` when one exists. Today the adapter discards `confidence`. The cut on `choose` stays on the winning probability, because that number exists on every backend and a reader can say what it means. Most of the vendor's pages cut on `confidence` instead, and its formula is unpublished. `report` sweeps both against labels, and the rule is looked at again once that has been measured.
3. **A new verb, `find`, is planned after `rank`.** `thinkthen find QUESTION` reads up to 255 lines or records, sends them together with an id on each, and asks which one best answers the question. It is one request where `filter` and `rank` make one per record. The answer is relative: `find` picks the best unit present, and `filter` judges each unit alone. The units see each other, and the user accepts that by choosing the verb. `find` prints the chosen unit as it arrived, the way `filter` prints a record. How `find` says that nothing fits is an open point: a second yes/no question, a "none" option, or a cut on confidence. Demo 15 argues for the "none" option, because it stays one request and behaves as `choose` does. A measurement settles it.
4. **`choose` can take its options from each record.** `--options POINTER` names a list of labels, or a map from label to description, inside the record. The vendor's extraction recipes and Ian's own notes both need a different candidate list for every record. On a single document the shell already does this with command substitution, so `--from FILE` stays held.
5. **An `annotate` file may hold a structured question.** The question text and each option or level description may be any JSON value, passed through unchanged. The tool never reads the keys inside such a value, so the rule against unknown keys applies only to the file's own structure. The vendor shows a real gain where two options are easily confused, and Ian's notes asked for the same. The command line keeps plain strings.
6. **`match` leaves the roadmap.** The vendor aligns two lists by putting both entities in one record and asking several questions of it. That is `annotate` over a file of pairs.
7. **`report` leads with accuracy at coverage:** the share of rows that resolved, the accuracy among them, and the accuracy among the rest. It keeps accuracy, precision, recall, and F1, because Ian asked for them. It gains no option for a held-out split. The page shows the procedure instead: pick the cut on one file, then report a second file at that cut.

## Considered and left out

- A threshold on `score`. The vendor does cut on scores. `jq -e '. >= 2'` after the command does the same in one line, and the help shows it.
- A flag that repeats a run for trials. A shell loop does it.
- Packing records into one request for `filter` and `rank`. See above.
- A two-pass `find` beyond 255 units. It waits for a job that needs it.

## Consequences

`segment.md`, `result.md`, `backends.md`, `choose.md`, `annotate.md`, `report.md`, and `roadmap.md` change, and `find.md` is added as Draft. The plan gains `find` after `rank`. The adapter ticket that touches `choose` keeps `confidence` and the full distribution.
