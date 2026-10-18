---
date: 2026-10-13
status: draft PRD
release: 0.5
source: Ian's rulings in `functions.md` and the README's notes of 2026-10-12; Ian's idea note on segment, compact and chunk and its two architecture notes; PRD 00 (U39 to U41, Q32) and PRD 03; ThinkThen 0.2's `specification/files.md`, `recognize.md`, `rank.md`, `backends.md`, `roadmap.md` and the held `segment.md` in git history; ThinkThen experiments 0010, 0013 and 0021; published work on Markdown trees, text segmentation, chunking and context compression. The 24 worked questions and every source are in [04-document-structure-questions.md](04-document-structure-questions.md)
---

# PRD 04: Document structure: segment, chunk and compact

## Who and what's in it for them

- **The agent reading a long document** (U39) gets a map: sections, paragraphs, lists, tables and the parts a model cut, each with an id, lines and a byte range. Proof: maps of a public trial record and of the CommonMark specification whose node spans rebuild the source byte for byte.
- **The builder feeding a search index** (U40) gets chunks cut where the meaning changes, each with its heading path, none splitting a table from its caption. Proof: the segment experiment's boundary scores and retrieval recall at equal token budget, against parser-only and embedding baselines.
- **The agent short on context** (U41) gets the document cut to what matters for its question, in source order, every kept sentence verbatim. Proof: the share of QASPER's annotated evidence kept at 10 and 25 percent budgets, against lead-k and an embedding baseline.
- **Everyone who pays** (U19, U20) sees the requests first. Proof: `--plan` bounds that hold on every experiment run.

## The design

**One structure, three uses.** A document becomes a tree of nodes with a few typed links. `segment` builds it, chunking reads it under a size budget, and compaction is `rank` over its sentences.

**Code first, the model where code cannot decide.** A CommonMark parser with GitHub tables finds headings, paragraphs, lists, tables and code, and rules find sentences, at no cost. The model answers two kinds of yes or no question, each with a window of neighbors, packed many to a request as `recognize` packs its step-one questions. A boundary question asks the caller's own question at each candidate gap, such as "A new eligibility requirement begins here." A link question confirms a pairing code proposes, such as a caption beside a table or a sentence opening a list. A 50-page document costs about 30 requests at block gaps and 70 at sentence gaps, under two cents at the rate experiment 0013 recorded.

**The graph is JSON lines, one node per line, in document order.** The first line is the document node, with the source's SHA-256 and the schema name. Each node has an `id`, a `kind`, a `parent`, a half-open UTF-8 byte `span` and one-based inclusive `lines`, as 0.2's reader prints. Model-made nodes carry their probability. Links (`caption_of`, `introduces`, `continues`) ride on their node. The id hashes the node's kind, text and occurrence, so an edit elsewhere keeps it. The graph holds no text; output reads it from the source, so every chunk is a view over a fixed file.

```text
thinkthen segment --input record.md                    # parser only, sends nothing
thinkthen segment --input record.md --split "A new eligibility requirement begins here" --at clause --as requirement
thinkthen segment --input record.md --unit chunk --chunk-bytes 4000
thinkthen filter "Mentions prior therapy" --input record.md --unit section
thinkthen rank "Says who may enroll" --input record.md --compact 3000
```

**Every function reads Markdown by structure.** 0.2's `--unit line|file` gains `block`, `section`, `sentence` and `chunk`. These use the free parse unless `--graph FILE` names a saved `segment`, so no function spends money on structure nobody asked for. `--graph` refuses a source whose hash changed.

**Chunking is code over predictions.** The packer fills a budget, never cuts inside a row, code block or sentence, prefers the highest-scored gaps, and attaches the heading path, a table's header row and a list's opening sentence, copied from the source and marked as context. Boundary answers are stored raw, so a new budget or cut costs no call.

**Compaction is a `rank` option.** `--compact BYTES` ranks every sentence with its heading path, keeps a paragraph whole when half its sentences make the cut, keeps the headings above kept lines, and replaces each dropped stretch with one marker naming its lines. PRD 03's `--compare` dial applies. No character comes from a model.

**Every surface uses the same words.** `thinkthen.segment(text, split=..., at="clause")` returns a typed graph with `nodes`, `units(kind)` and `chunks(max_bytes)`. SQL offers `thinkthen_segment` as a table function. Through a proxy, `segment` is one call on the function door, like `recognize`.

## What must be true

| Risk | What retires it |
| --- | --- |
| Prediction adds nothing over the parser and rules | The segment experiment: parser only, plus embedding breaks, plus model gap questions, and a fixed-size splitter, scored by boundary F1, WindowDiff and retrieval recall at equal budget. If the model arm loses, the parser ships alone and the finding is published |
| Windowed gap questions miss what whole-document reading sees | Window sizes of 2, 4 and 8 units against one whole-document request on documents that fit |
| Compaction drops the evidence | QASPER evidence kept at fixed budgets, flat ranking against section-first ranking, since experiment 0010 found hard routing loses evidence |
| Byte spans confuse Python and JavaScript callers | Each library returns host-native offsets beside the bytes, as `recognize` does, checked by one shared fixture |
| Ids churn on small edits | An edit fixture: change one paragraph, count changed ids |

## Out of 0.3

PDF, HTML, Word and images as input; convert them to Markdown first. Repairing broken Markdown with a model, which experiment 0013 found loses to always joining lines on clean text. Cross-references, coreference and generated summaries. Embeddings and late chunking. Right answers for boundaries. Token-level compression.

## Questions for Ian

Ian's open questions for every PRD are in one list, ordered by importance, at the start of the PRD PDF. This PRD's are numbers 26, 30, 31 and 32.
