# 0610 — The recognize page explains how it works

Status: OPEN.

Milestone: 0.2.1

Source: Ian asked for this on 2026-10-01. The site content team handed it to ThinkThen on 2026-10-17 with its rulings and evidence intact. It starts after 0.2 ships.
## Why

Ian asked that readers of the `recognize` function page understand at least the level of detail in a review the ThinkThen team sent him on 2026-10-01. Today the page shows calls and options. It does not say how `recognize` finds names, picks kinds, scores them or asks for relations.

## Prior evidence

The review, as Ian pasted it:

1. Each word or punctuation mark gets one tag: begin, inside, end, single or out. The code calls this scheme BILOU, which is another name for BIOES. Tags go at most 40 words per request, with 6 words of context on each side.
2. The code picks the best allowed tag sequence across the whole text (Viterbi), not the top tag for each word alone. Invalid sequences, such as a name that begins and never ends, are ruled out.
3. A pick-one question chooses among the reader's kinds plus "none of these". A name answered "none of these" is dropped. A second question checks whether the name should include a neighboring punctuation mark.
4. A name's strength is the label probability times how sure the tagging was of that exact span. It is computed over every allowed tag sequence, not only the best one.
5. Relations: each allowed pair gets one yes/no question, such as "Does the text itself state that Paul McCartney wrote Yesterday?"

Its live tests, 10 calls for under a cent:
- "Paul McCartney wrote Yesterday for the album Help! in 1965." found the person, the song and the album, each with the right kind.
- Three people and three songs with a "wrote" relation: all 9 pairs were asked, and the 2 the text states were kept. Ringo, who "sang" his song, was left out.
- A name across the 40-word boundary, at words 40 and 41, was found at the right position.
- Text with an accent and an emoji before the name gave the right position, 10 to 20, as in the specification's example.

Two results look odd but work as designed:
- "Help!" came back as "Help". The code offered both spans, and the model picked the one without "!", 0.87 against 0.13.
- The "!" on its own was tagged as a name, labeled album at 0.51 strength. It fell below the 0.5 strength cutoff and was dropped.

## Change

Add a "How it works" section to the `recognize` function page, below the language tabs, in plain sentences per `site/WRITING.md`. It covers the five steps, the 40-word window and its context, what strength means and how the cutoff uses it, and both odd-looking results as worked cases. Check every claim against `specification/recognize.md`, `specification/relate.md` and the code, and use the specification's words for options. Name BILOU once, and say it is the same as BIOES. Show the McCartney sentence as a replayed example with its output. Record it through ThinkThen `sdlc/scripts/live` under a $0.05 cap if no recording covers it. Link the section from the `relate` page's relation question.

## Retained behavior

The page's examples and options stay. `npm run build`, `npm run check` and both smoke runs pass.

## Proof

A fresh reviewer checks each sentence against the specification and code, and returns ACCEPT.
