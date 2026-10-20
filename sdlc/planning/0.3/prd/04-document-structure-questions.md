---
date: 2026-10-13
status: draft PRD
source: Ian's rulings in `functions.md` and the README's notes of 2026-10-12; Ian's notes on segment, compact and chunk, chunking strategies, hierarchical retrieval and table extraction by classification; Ian's notes on the data model and on fixed and tunable settings; the ThinkThen ideal state; PRD 00 and PRD 03; ThinkThen 0.2's `specification/` and `thinkthen help`; the held `specification/segment.md` at commit 62b163e81; experiments 0010, 0013 and 0021 in the experiment repo; the clippings on structure recovery, DocJev and long-document search with PageIndex; the outside sources listed at the end
---

# PRD 04 working file: segment, chunk and compact

This file backs [04-document-structure.md](04-document-structure.md). It works through the questions where a wrong answer costs a rename, a migration, a confused user or a failed launch. Claims about 0.2 cite its specification page. **Ian** marks the questions that need his product direction.

## What 0.2 and the experiments already say

- **0.2 reads files by line, window or whole file**, and every located answer names its file and one-based inclusive lines ([files.md](../../../../specification/files.md)). File names and positions never enter evidence or cache keys.
- **`segment` existed once and was held.** It cut one document at the gaps a yes/no question found, sent the whole document in one request with one question per gap, and printed a flat list of segments with no parent, level or role (`specification/segment.md` at commit 62b163e81). ADR 0010 held it because it was the least general verb and because the whole-document form refused large documents, "and the long documents are the ones that most need cutting". A form that "reads a long document in parts and joins the answers would bring it in" ([roadmap.md](../../../../specification/roadmap.md)).
- **`recognize` already does windowed boundary questions.** Step 1 asks where each piece stands in a name (BEGIN, INSIDE, END, SINGLE or OUT), with six pieces of context on each side and at most 40 pieces a request; step 3 asks relation questions ([recognize.md](../../../../specification/recognize.md), [backends.md](../../../../specification/backends.md)). "The same path serves a sentence and a book."
- **`rank --around N`** already prints a view of neighboring lines from a fixed snapshot of the source ([rank.md](../../../../specification/rank.md)). A compacted document is the same kind of view.
- **Experiment 0013** rebuilt Markdown from stripped text with a join question per line pair and a label question per block. On clean web text its joins scored 0.873 against 0.953 for always joining (validated in 0021). Block labels needed neighbors: PDF list items rose from 0.22 to 0.69 with 60 characters of context. PDF tables failed because extraction scrambled them first. Cost was about $0.001 a page.
- **Experiment 0010** found that flat ranking of every paragraph matched or beat a document-then-paragraph walk, and that the document step "loses silently and irreversibly". It did not test putting the section heading into each paragraph record.

## The example document

The public record of trial NCT02844816 on ClinicalTrials.gov, sponsored by the US National Cancer Institute. The registry's API returns its text fields as Markdown (checked 2026-10-13: nested `*` lists and backslash escapes such as `\<`). Its eligibility text is 13,529 bytes with 38 list items and 39 semicolons: several items hold more than one requirement joined by semicolons. A parser finds the items for free; only a model can say where one requirement ends inside an item. That is the boundary question in its plainest form, and it needs no biomedical judgment, only a count of where people would cut. The record also links a protocol PDF of about 2.9 MB, which shows the PDF case that 0.3 leaves out. The CommonMark specification serves as the second, non-clinical example: long, public and full of code blocks and lists.

## Part one: what the thing is

**Q1. Are segment, chunk and compact three functions, one, or none? (Ian)**
Options. (a) Three new functions. (b) One function, `segment`, that builds the graph; chunk is a reader unit every function reads; compact is a `rank` option. (c) No new word: a Markdown reader plus `recognize --unit sentence` for model cuts, with chunk and compact as tool recipes. (d) A job word, `document`, holding `segment`, `chunk` and `compact`.
Reasoning. Three functions break the ideal state's rule that a new function needs "a real job that no existing function or option can do"; chunking is packing, and compaction is ranking, as Ian said. Option (c) is cheapest in words, but `recognize` returns a flat list of named spans, and the hierarchy and links would have nowhere to live; users would learn the graph from a recipe. Option (d) adds a top-level word anyway and hides an Ask action under a job word, which PRD 00 reserves for the jobs around asking. Option (b) adds one word whose result shape is new (a covering tree), reuses `recognize`'s windowed engine, and matches PRD 00's Q32.
Recommendation. (b). `segment` becomes the twelfth flat word after the ten functions and `run`. It needs Ian under PRD 00's principle 3.

**Q2. Does segmenting need a classifier at all?**
For clean Markdown, the parser finds every heading, list, table and code block exactly; CommonMark defines them. Sentence splitting has strong rule systems (pySBD, the Unicode rules, `sentencex`). So for the structure the author wrote, no classifier is needed, and that is the finding Ian anticipated. Three cuts remain that rules cannot make: (1) topic shifts inside a long section with no subheadings, which TextTiling, C99 and the neural segmenters exist to find; (2) several units packed into one block, such as the trial example's semicolon-joined requirements; (3) attachments the syntax does not mark, such as a caption beside a table, a sentence that opens a list, or a rule and its exception.
Recommendation. Say it plainly in the docs: the parser does the author's structure, and the model does the three cuts above. The experiment in Q22 measures whether the model beats embeddings and rules on them.

**Q3. Ship the free parse even if prediction loses? (Ian)**
The parse alone gives agents the map, gives every function `--unit section`, and costs nothing. Prediction is the uncertain part. Tying both to one experiment would hold back the sure half.
Recommendation. Ship the Markdown reader units and parser-only `segment` in 0.3. Admit `--split` and the link questions only if the experiment wins. If they lose, publish the finding; "you don't need a model to chunk Markdown, and here are the numbers" is a strong article.

**Q4. Is the graph a tree or a general graph?**
mdast and Pandoc give a syntax tree in which a heading and the paragraphs after it are siblings. A section tree, built with a heading stack, makes the heading own its content. Cross-references make a general graph. A general graph is harder to print, to walk and to explain, and Ian worried people may not grasp a graph at first.
Recommendation. A tree of containment, with a closed set of typed links riding on nodes: `caption_of`, `introduces` (an opening sentence to its list or table), `continues` (a table or code block split across chunks) and `note` (a footnote reference, which the parser finds). Sections own their content. Next and previous are implied by document order. `refers_to` waits for an experiment. The docs draw the trial record's tree first, then the links, which shows the graph without assuming it.

**Q5. What does the graph JSON look like?**
Options: one nested JSON document (as mdast or PageIndex print), a flat node list with parent pointers, or the JSON Graph Format with separate node and edge arrays. Nested JSON cannot stream and is awkward in `jq` and SQL. Separate edge arrays split one fact across two places. A flat list matches every other ThinkThen output, which is JSON lines.
Recommendation. JSON lines, one node per line, in document order. Line one is the document node:

```json
{"id":"doc","kind":"document","schema":"thinkthen.document-graph/1","source":{"path":"NCT02844816.md","sha256":"…","bytes":24812},"parser":"commonmark-0.31.2+gfm-tables","lines":[1,402]}
{"id":"k3f9q2","kind":"section","parent":"doc","level":2,"span":[1840,15502],"lines":[38,96]}
{"id":"m8x1c4","kind":"heading","parent":"k3f9q2","level":2,"span":[1840,1863],"lines":[38,38]}
{"id":"a2r7d0","kind":"paragraph","parent":"k3f9q2","span":[1865,1885],"lines":[40,40],"links":[{"rel":"introduces","to":"p0w5h1","p":0.97}]}
{"id":"p0w5h1","kind":"list","parent":"k3f9q2","span":[1887,9902],"lines":[42,61]}
{"id":"t6n4b8","kind":"item","parent":"p0w5h1","span":[1887,2210],"lines":[42,42]}
{"id":"v9e2s5","kind":"requirement","parent":"t6n4b8","span":[1889,2030],"lines":[42,42],"by":"model","p":0.91}
```

The kinds are closed: document, section, heading, paragraph, sentence, list, item, table, row, code, quote, html, image, footnote, front_matter, plus one caller-named kind for model-cut parts (`--as NAME`, default `part`). A kind name follows the identifier grammar of PRD 00's Q16. `--outline` prints the sections as an indented outline with ids and lines, for people and for agents choosing where to read.

**Q6. How are node ids made stable?**
Options: sequence numbers (change after any insertion), path numbers such as `2.3.p1` (change when a sibling moves), byte offsets (change with any upstream edit), or a hash of content. The W3C Web Annotation model and Hypothesis anchor by quoted text for the same reason: content survives edits that move positions.
Recommendation. The id is the first 6 base-32 characters of SHA-256 over the kind, the node's exact text, and the count of earlier nodes with the same kind and text. A seventh character is added on a collision inside one document. An edit elsewhere keeps every untouched node's id; offsets are recomputed each time. The document node's id is always `doc`. Ids are names, not keys: the answer store keys on the exact text sent, as 0.2 does ([cache.md](../../../../specification/cache.md)), so an unchanged node reuses its stored answers whatever its id.

**Q7. What do offsets count?**
Rust slices UTF-8 bytes, Python indexes code points and JavaScript indexes UTF-16 units. The Language Server Protocol fought the same problem and added a negotiated position encoding in version 3.17. ThinkThen already prints lines for every located answer, and `recognize` returns host-native offsets in each language.
Recommendation. The wire and the command line use half-open UTF-8 byte ranges plus one-based inclusive lines. Each library adds host-native offsets beside them, generated once in Rust as `recognize` does. One shared fixture with accents, emoji and CRLF checks every language.

**Q8. How do chunks stay views over a fixed source?**
A chunk that copies text drifts from its source. Experiment 0010 showed an agent needs file, line range, heading and probability for each passage.
Recommendation. The graph stores no text; `--text` adds it on output. A chunk record lists its node ids, its span or spans, its lines, its context node ids and the rendered text. `--graph FILE` checks the source's SHA-256 and refuses a mismatch, naming the command to segment again. Re-segmenting an edited file costs only the questions whose windows changed, because every other answer is stored.

## Part two: what the model is asked, and the cost

**Q9. What exactly is a boundary question?**
Options. (a) The held 0.2 form: the whole document in one request, one question per gap. (b) One yes/no question per gap with a window of neighbors on each side, many packed into a request, as Lukasik and others' cross-segment model reads left and right context around a candidate break. (c) A begin, inside, out tag per unit with a sequence pass, as `recognize` step 1 does. Form (a) fails past one request, the reason ADR 0010 held it. Form (c) adds nothing for a binary cut, since any set of cuts is a valid cover.
Recommendation. (b). The question is the caller's own sentence, such as "A new eligibility requirement begins here", consistent with Ian's ruling that ranking questions are always the caller's. ThinkThen marks the gap in the window and adds no wording of its own except the fixed gap marker, shown by `--plan`. Each gap is one decision, stored like any other.

**Q10. Where are the candidate gaps?**
Asking at every character is wasteful; asking only at block gaps misses the trial case. The candidate step limits what any model can find.
Recommendation. `--at block|sentence|clause`, default `block`. `clause` adds semicolon and colon breaks inside sentences, the trial case. `--within KIND` limits questions to gaps inside one kind of node, such as `item` or `section`. Code never asks across a heading, a table row or a code block's interior, so the model is never asked what the parser already knows.

**Q11. What are the link questions?**
GitHub tables have no caption syntax. A caption is a nearby paragraph, often starting "Table 2" or set in emphasis. A list's opening sentence usually ends in a colon. Experiment 0009's lesson applies: the candidate step caps recall.
Recommendation. Code proposes every adjacent paragraph-table, paragraph-image and paragraph-list pair, and the model confirms with a fixed yes/no wording per link kind, like `recognize`'s fixed step-1 wording. The link wording is ThinkThen's contract, versioned and shown in `--plan`. Rules propose, the model confirms: the 0.2 recipe pattern.

**Q12. How many requests does a 50-page document cost?**
Assume 50 pages is about 150 KB and 41,000 tokens (the PageIndex example put a 93-page report at about 76,000 tokens), about 1,200 blocks and 2,500 sentences. Requests hold 40 gap questions sharing one window, as `recognize` caps step 1; the request ceiling stays 96,000 bytes ([settings.md](../../../../specification/settings.md)). Tokens use 0.2's measured 0.516 tokens per byte. Price uses the $0.042 per million input tokens that experiment 0013 recorded.

| Step | Questions | Requests | Input tokens | Cost |
| --- | --- | --- | --- | --- |
| Parse and sentences | 0 | 0 | 0 | $0 |
| Boundaries at block gaps | about 1,000 | about 25 | about 100,000 | about $0.004 |
| Boundaries at sentence gaps | about 2,500 | about 63 | about 250,000 | about $0.011 |
| Link questions | about 100 | about 3 | about 10,000 | under $0.001 |
| Compact: rank every sentence with its heading path | about 2,500 | about 6 | about 270,000 | about $0.011 |

A full block-level segment and one compaction cost about 35 requests and under two cents. Experiment 0013 measured full Markdown repair at about $0.001 a page, so these figures sit within the measured range. `--plan` prints the exact count before anything is sent, and the 0.2 spend caps apply.

**Q13. Does the window replace whole-document reading?**
The vendor's recipe sends the whole document once with all questions, so each gap sees everything. Windows see only neighbors. Long-range cues, such as a list that resumes after an aside, may need more.
Recommendation. Windows by default, with a measured size. The experiment compares windows of 2, 4 and 8 units against one whole-document request on documents that fit, and picks the smallest window within one point of boundary F1. A document that fits one request may use the whole-document form when it measures better.

**Q14. Where does the cut threshold live?**
Store raw, read by rule (PRD 00 principle 4). A gap's probability is a raw answer; whether it cuts is a reading.
Recommendation. `--threshold` reads the stored probabilities, default 0.5, with `--min-bytes` and `--max-bytes` as reading rules that merge the weakest cut or add the strongest. Changing any of them re-reads stored answers and sends nothing.

## Part three: elements and inputs

**Q15. How are tables, lists, code blocks and captions handled?**
The notes' element table and Docling's chunker agree on the policies, and none needs a model.
Recommendation. A table is one node with row children; a chunk never splits a row, and a split table repeats its header row as marked context. A code block is atomic; an oversized one splits at blank lines with a `continues` link. A list owns its items and nests; its opening sentence travels with any chunk of its items. A caption travels with its table or image. HTML blocks, front matter and images are opaque nodes; image alt text is their only text. A unit larger than the budget alone becomes its own chunk with a warning, never silently cut.

**Q16. Are PDF and HTML in scope? (Ian)**
Experiment 0013 found PDF text extraction scrambles tables and equations before any pass runs, and 0009 rebuilt 0 of 150 tables from words. Layout models trained on DocLayNet or PubLayNet, and converters such as Docling, Marker and Nougat, do this job with images and page geometry, which decision models do not yet read in ThinkThen. HTML converts to Markdown well through pandoc, but offsets then point into the Markdown.
Recommendation. Markdown and plain text only; plain text is valid CommonMark. The docs show a converter step before `segment` and say that spans refer to the converted file. A later version can accept a converter's page map.

**Q17. Is broken Markdown repaired?**
Ian said cleaning Markdown with a model does not work well yet. Experiment 0013 agrees on clean text, where always joining won.
Recommendation. Out of 0.3. `segment` reads the text as written. The experiment keeps a repair arm for PDF-derived text, where 0013's joins beat always joining, so the idea can return with numbers.

**Q18. What sizes are admitted?**
The whole-document limit sank the old `segment`. Windows remove it.
Recommendation. Admit up to 16 MiB, the 0.2 record bound, refused before sending past that. Requests stay under the 96,000-byte ceiling.

## Part four: chunk and compact

**Q19. Does chunking need prediction?**
Ian asked whether each step needs prediction. Chunking has three decisions: where units end, which units go together, and what context travels. With a graph in hand, the first is segment's predicted cuts, the second is packing under a budget, and the third follows the links. Semantic chunkers use embedding similarity for the second; Qu, Tu and Bao found that semantic chunking's gains over fixed size were inconsistent and often reversed on realistic documents.
Recommendation. Chunking is code over segment's predictions: no model call of its own. The packer fills a budget inside one section, prefers higher-probability cuts, and attaches heading path, header row and list opening. The docs say where the prediction lives.

**Q20. What context travels with a chunk, and is any of it generated?**
Anthropic's contextual retrieval prepends a model-written summary to each chunk and cut failed retrievals by 49 percent, 67 percent with reranking. ThinkThen never generates text.
Recommendation. Context is copied from the source: the heading path, the table header row, the list's opening sentence and the caption. It is marked as context in the chunk record. A user who wants generated context can add it downstream; ThinkThen's breadcrumb is the deterministic form of the same idea.

**Q21. Is compaction its own function, a recipe, or a `rank` option?**
Ian called compaction a kind of ranking. PRD 03 lists "a built-in compaction command" as out of 0.3 and says compaction reuses its dial through `rank`. A recipe leaves the roll-up and rendering to every caller, and the roll-up is where the quality lives.
Recommendation. `rank --compact BYTES`, which reads sentences by itself. It ranks every sentence with its heading path in the record, keeps the best until the budget is full, keeps a paragraph whole when at least half its sentences are kept (Ian's rule), keeps every heading above a kept line, and prints the result in document order with each dropped stretch replaced by one marker naming its lines. `--compare` from PRD 03 applies. This bends U41's "`rank` recipe" in PRD 00 into a `rank` option; the option is still `rank`.

**Q22. Flat or top-down compaction, and token or sentence pruning?**
LLMLingua and its successors drop tokens, and the text becomes unreadable to people and unquotable. Selective Context drops low-information phrases. RECOMP's extractive compressor and Provence keep or drop sentences for a question, which keeps text verbatim. Experiment 0010 found that pruning by document first loses evidence that no later step recovers.
Recommendation. Sentence granularity, flat by default: every sentence is ranked, nothing is pruned by section first. `--compact` never drops part of a sentence. Section-first compaction is an experiment arm, not a default.

## Part five: fit and proof

**Q23. How is the segment experiment designed?**
Arms: a fixed-size recursive splitter, the parser and rules only, the parser plus embedding-similarity breaks (TextTiling-style), and the parser plus model gap questions, on two decision models. Data: Wiki-727K for topic boundaries, the HiChunk benchmark for multi-level boundaries, QASPER rendered to Markdown for retrieval and compaction, and the trial record set for clause cuts, with a person marking where requirements begin and no clinical judgment asked. Measures: boundary F1 within one unit, Pk and WindowDiff, retrieval recall at equal token budget (fixed top-k misleads when chunk sizes differ), and cost per document. Stop rule: the model arm ships if it beats the better of the rules and embedding arms by a stated margin on boundary F1 and retrieval recall on two of three sets. A zero-cost first pass compares the parser with the fixed-size splitter.
Recommendation. Run it before any `--split` ticket. Publish the result either way.

**Q24. Where does `segment` sit on every surface, and in how many languages? (Ian)**
PRD 00 puts the Ask functions in all 24 languages and the data jobs in four. A graph is a larger type than any 0.2 result, and hand-writing it in COBOL or Ada would be costly. 0.2's result types are already generated from the Rust schema, so the cost is in the generator, not per language.
Recommendation. All languages, generated from the Rust schema, because `segment` is an Ask function. If the generator cannot carry the graph type cheaply, the fallback is Rust, Python, R and TypeScript first, with the graph as JSON text elsewhere. The command line as above. SQL offers a table function, one row per node, so `SELECT … FROM thinkthen_segment(doc)` joins nodes to anything. MCP takes the command as a string. Through a proxy, `segment` is one call on the function door; the client reads the file and sends text. A tool can chain `segment`, `filter` and `rank` (U14). The answer store keeps each gap question as an ordinary decision. Right answers for boundaries, a future curation use, key on the question and the window text, and wait for a later version.

## Sources

- CommonMark Spec 0.31.2, 2024. https://spec.commonmark.org/0.31.2/
- GitHub Flavored Markdown Spec, tables extension. https://github.github.com/gfm/
- mdast, the Markdown abstract syntax tree, and unist positions. https://github.com/syntax-tree/mdast
- pulldown-cmark offset iterator and markdown-rs, Rust parsers with source positions. https://github.com/pulldown-cmark/pulldown-cmark, https://github.com/wooorm/markdown-rs
- W3C Web Annotation Data Model, text quote and position selectors, 2017. https://www.w3.org/TR/annotation-model/
- Language Server Protocol 3.17, position encodings. https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/
- Unicode Standard Annex 29, text segmentation. https://www.unicode.org/reports/tr29/
- Hearst, TextTiling, Computational Linguistics 23(1), 1997.
- Choi, Advances in domain independent linear text segmentation (C99), NAACL 2000.
- Beeferman, Berger and Lafferty, Statistical models for text segmentation (Pk), Machine Learning, 1999. Pevzner and Hearst, WindowDiff, Computational Linguistics 28(1), 2002.
- Koshorek and others, Text Segmentation as a Supervised Learning Task (Wiki-727K), NAACL 2018. arXiv:1803.09337
- Lukasik, Dadachev, Simões and Papineni, Text Segmentation by Cross Segment Attention, EMNLP 2020. arXiv:2004.14535
- Frohmann and others, Segment Any Text, EMNLP 2024. arXiv:2406.16678
- Lu and others, HiChunk, ACL 2026 (as cited in Ian's notes on hierarchical retrieval).
- Zhong and others, PubLayNet, 2019; Pfitzmann and others, DocLayNet, KDD 2022, arXiv:2206.01062; Auer and others, Docling Technical Report, arXiv:2408.09869; Blecher and others, Nougat, arXiv:2308.13418.
- LangChain text splitters: RecursiveCharacterTextSplitter and MarkdownHeaderTextSplitter. LlamaIndex node parsers: SemanticSplitterNodeParser, MarkdownNodeParser, HierarchicalNodeParser.
- Günther and others, Late Chunking, 2024. arXiv:2409.04701
- Anthropic, Introducing Contextual Retrieval, September 2024. https://www.anthropic.com/engineering/contextual-retrieval
- Qu, Tu and Bao, Is Semantic Chunking Worth the Computational Cost?, Findings of NAACL 2025. arXiv:2410.13070
- Smith and Troynikov, Evaluating Chunking Strategies for Retrieval, Chroma technical report, 2024.
- Chen and others, Dense X Retrieval, 2023, arXiv:2312.06648; Sarthi and others, RAPTOR, ICLR 2024, arXiv:2401.18059.
- Jiang and others, LLMLingua, EMNLP 2023, arXiv:2310.05736; LongLLMLingua, ACL 2024, arXiv:2310.06839; Pan and others, LLMLingua-2, 2024, arXiv:2403.12968.
- Li and others, Compressing Context to Enhance Inference Efficiency (Selective Context), EMNLP 2023, arXiv:2310.06201.
- Xu, Shi and Choi, RECOMP, 2023, arXiv:2310.04408.
- Chirkova, Formal, Nikoulina and Clinchant, Provence: efficient and robust context pruning for retrieval-augmented generation, ICLR 2025. arXiv:2501.16214
- Liu and others, Lost in the Middle, TACL 2024. arXiv:2307.03172
- Dasigi and others, QASPER, NAACL 2021. arXiv:2105.03011
- ClinicalTrials.gov API v2, study NCT02844816, read 2026-10-13. https://clinicaltrials.gov/api/v2/studies/NCT02844816
- TypeSafe structure recovery cookbook; DocJev; long-document search with PageIndex (clippings in Ian's notes).
