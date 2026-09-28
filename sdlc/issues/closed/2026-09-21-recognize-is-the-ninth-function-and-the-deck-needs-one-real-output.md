# `recognize` is the ninth function, and the deck needs one real output

Status: Closed on 2026-09-25 after a check against main. recognize built (sdlc/records/0080-build-recognize.md); the demo output landed. Earlier status: Open

Ian ruled on 2026-09-21: "We're going to need a new function, `recognize`." This reverses his earlier word the same day that kept it out of the first release. The marketing side records what that ruling asks of the build team and of the recognize experiment. This page authorizes no build. The product shape is in `experiments/RECOGNIZE-PRODUCT-SPEC.md`.

## What was checked on 2026-09-21

- The binary has no `recognize`. `crates/` holds `thinkthen` and `thinkthen-core`, and no source file names the word.
- No script prints the specification's final object, `{"entities": [...], "relations": [...]}`. `experiments/216-conll04-relations/chain5.py` replays recorded replies and prints plain text lines. `experiments/213-thinkthen-recognize/proto.py` assembles entities and stops before relations.
- The recorded answers are real and strong on news sentences: a person to an organization at 0.99, an organization to a city at 0.98.

## What the deck needs

The deck build lets a function slide show only what the tool printed. The recognize slide therefore waits for one recorded output in the final shape, on a sentence we wrote ourselves. A benchmark sentence is someone else's text and stays off a slide. The sentence:

    Maria Chen joined Northwind Freight in Chicago last spring.

The ask of the recognize experiment: run that sentence at depth `relations` with a two-row relation table (`works_for: person -> organization`, `based_in: organization -> place`), save the recording, and print the final object. One sentence costs a fraction of a cent. The run goes through `sdlc/scripts/live` under a token cap like every paid run.

Until that lands, the deck stays at eight functions everywhere. It changes to nine in one commit, with the slide, the overview row, and every footer together.

## What the ruling asks of the build team

1. **The output is an object with offsets.** Every other function prints a value, a record, or an annotated record. `recognize` prints one object per text. Under `--lines` or `--jsonl` it should print one object per record, the way `annotate` does.
2. **Three thresholds want three names.** The specification has `span_threshold`, `word_threshold`, and `relation_threshold`. The eight functions have one `--threshold`. The suggestion: `--threshold` stays the entity bar, since it is the one a first user reaches for, and the other two are long options.
3. **The relation table is a file, like a question set.** It should load through the same `@file` form and the same validation path as a question file, and it needs `"version": 1`.
4. **The public words.** "Entity" becomes "name" in the first-week copy, "kind" stays, and "confidence" is "probability" everywhere per the vocabulary. The specification's `confidence` field is a computed number and no raw probability. The manual has to say so once, or the field has to be renamed.
5. **The request count.** The specification sends all of a sentence's questions in one request. The cost slide's rule, "requests are the bill", holds only if a long text does not turn into one request per sentence without the user knowing. The plan printed by `--dry-run` should state the request count.
6. **Relations are beta.** The help and the manual say so in the first line about relations.
7. **The nine surfaces.** `surfaces.md` in the deck is the acceptance test for the libraries. It gains a `recognize` line on one language slide and one database slide once the shape above is settled.

## The question file on the overview slide

Ian asked the same day whether the `@` form is a special function. The marketing ruling, his to overturn: it is not counted as a function, because it gives no answer. It gets a row of its own on the overview slide, named `@question`. The record is the marketing repository's `products/thinkthen/vocabulary.md`.

## What Ian can overturn

All of it.
2026-09-21: the recognize demo landed. Final object: `experiments/222-recognize-demo/output.json` in the workspace (pretty version and verification in report.md beside it). Recording: `experiments/222-recognize-demo/arms/demo/cache/` in the workspace. Real spend 0.021 cents, run through sdlc/scripts/live under caps 5000+3000.
