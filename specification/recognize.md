# Recognize names

Status: **Settled** for the command in ticket 0147, under ADR 0056. Relations are beta.

`thinkthen recognize [OPTIONS] [KIND]...` finds every name in one text and gives each one a caller-named kind. `thinkthen recognize [OPTIONS] @FILE` reads the same question from a file. With no kinds, every name has the kind `ENTITY`.

Recognize runs in three steps. Step 1 splits the text into pieces and asks where each piece stands in a name. Step 2 asks each found name's kind and checks its edges. Step 3 asks only the relation pairs a rule allows. The same path serves a sentence and a book.

## Step 1: boundaries

White space separates pieces. Each character of Unicode general category P or S is a piece of its own. A run of characters of category Mn, Me or Cf joins the piece that ends right before it. A run after white space or at the text's start begins a piece. Nothing else joins or splits. `Ada met Acme.` is four pieces: `Ada`, `met`, `Acme` and `.`.

Each piece gets one pick-one question: `BEGIN`, `INSIDE`, `END`, `SINGLE` or `OUT`. The question names the caller's kinds without their descriptions. With no kinds it asks about a name of any kind. Each question shows a snippet of six pieces on each side, with the piece wrapped in `[[ ]]`.

A step-1 request holds at most 40 consecutive pieces. Its evidence runs from six pieces before its first piece to six pieces after its last. A text of 40 pieces or fewer sends its whole text once. A longer text never sends its whole text in one step-1 request.

After every step-1 request returns, a Viterbi decode picks the most likely valid tag sequence over the whole text. `BEGIN` and `INSIDE` must be followed by `INSIDE` or `END`. Each probability is floored at one in a million. On a tie the earlier tag in the order above wins. A `SINGLE` piece is a name, and so is a `BEGIN` through its `END`.

## Step 2: kinds and edges

Step 2 sends one request for each step-1 request that holds a found name's first piece. Its evidence runs from six pieces before its first name to six pieces after its last. A request with no questions is not sent.

- **The kind question.** One per found name when the run has kinds. The options are the caller's kinds in order, each with its description when given, then `none of these`. A name whose answer is `none of these` is dropped.
- **The edge question.** One per found name that has two or more stretches to choose from. The options are the name as found, the name plus a touching mark at either end, and the name less a mark at either end. A one-piece name gets no removal option. A name with no edge question keeps its span.

With no kinds, only edge questions go out. When the edge pick leaves two names with the same start, end and kind, the one with the higher strength prints, and on equal strength the first.

## Names

One document prints one object:

```json
{"entities":[{"text":"Maria Chen","start":0,"end":10,"length":10,"kind":"person","strength":0.9987}]}
```

`start` is inclusive and `end` is exclusive. Both count Unicode scalar values, and `length` is `end - start`. Names print in order of `start`, then `end`. Equal names at different offsets remain separate. No name is a successful result: `{"entities":[]}`.

`strength` is P(kind) times P(span), rounded to four decimal places. P(kind) is step 2's probability of the chosen kind. With no kinds, P(kind) is 1. P(span) is the share of the valid tag paths, weighted by their floored probabilities, that tag exactly the stretch step 1 found as one name. One forward-backward pass computes it from the probabilities the decode already holds. A widened name keeps its step-1 P(span). `strength` ranks names. It is not itself a probability.

`--threshold` keeps a name whose printed strength is at or above the cut, so `audit` rescoring a saved line matches a live run. The default is `0.5`, and the cut stays above 0. A name under the cut leaves before step 3.

## Kinds

Bare kinds keep the caller's exact names. `--kind KIND=DESCRIPTION` gives a description, which reaches only the step-2 kind option. Bare and described kinds do not mix. A run takes 0 to 20 distinct nonblank kinds: `thinkthen: recognize takes 0 to 20 distinct, nonblank kinds`. A `--kind` with no `=` is refused at exit 2: ``--kind is KIND=DESCRIPTION, and this one holds no `=`; give a bare kind without --kind``.

`none of these`, `ENTITY` and `ANY` are reserved in any ASCII case. The refusal echoes no text: `thinkthen: recognize reserves the kind names none of these, ENTITY and ANY in any ASCII case`. It exits 2 on the command line and 5 from a question file.

## Relations

Relations are beta. `--relation NAME=SOURCE:TARGET` adds a directed rule and makes `relations` present. `--relation NAME` means `NAME=*:*`. Either side may be `*` or `ANY`, and the canonical question writes `*`. `NAME=KIND` is malformed. A concrete side naming a kind the run lacks exits 2.

Each ordered pair of kept names whose kinds match a rule's sides gets one yes/no question: `Does the text itself state that i1 READS i2?`. Under `either` it reads `Does the text itself state that i1 READS i2, or that i2 READS i1?`, and the pair is asked once. `*` expands to the kinds of kept names in first-seen order. Names with equal text and kind are asked once. A name is never related to itself. The pair questions share requests that carry the whole text, split at ADR 0040's ceiling and at 400 questions. `--relation-threshold` keeps an edge when its probability is at or above the cut. The default is `0.5`.

An edge repeats both complete names:

```json
{"relation":"works_for","source":{"text":"Maria Chen","start":0,"end":10,"length":10,"kind":"person","strength":0.9987},"target":{"text":"Northwind Freight","start":18,"end":35,"length":17,"kind":"organization","strength":0.997},"probability":1.0}
```

`relations` is absent when no rule was supplied. It is an empty list when rules were supplied and no edge passed.

## The guard

A text over 600,000 UTF-8 bytes exits 2 before any request, at every address: `thinkthen: recognize: the text is N bytes, over the limit of M; raise it with --max-text-bytes`. `--max-text-bytes N`, from 1 to 2^53 - 1, sets the limit. It enters no digest. In record mode an oversize record fails that record.

## Records and details

Record modes print `{"input":INPUT,"value":OBJECT}` in input order. `--details` prints `thinkthen.result/1`, keeps the same object under `value`, lists every request digest in send order, and sums send counts and usage. `answer.pieces` lists each piece's offsets and its five tag probabilities. `answer.names` lists each found name's span as found, its kind probabilities, and its edge option probabilities or null. `answer.pairs` lists each pair's probability.

A failed step-1, step-2 or relation request fails that input. It prints no partial name or edge object for that input. Earlier completed record rows remain printed.

## Dry run

`--dry-run` needs no key and sends nothing. It prints one compact `thinkthen.recognize-plan/2` object for the first record. Keys appear in this order: `schema`, `url`, `model`, `key_env`, optional `from`, `pieces`, `request_count`, `name_requests_upper_bound`, the optional relation bounds, and `requests`.

`pieces` counts pieces as step 1 splits them. `request_count` counts the step-1 requests. `name_requests_upper_bound` equals it, because each step-1 request leads to at most one step-2 request. `requests` lists each step-1 request in send order, with its recording `digest`, UTF-8 `bytes`, and exact `body_utf8`. An empty or blank text prints no plan. It exits 2 with `thinkthen: the evidence is empty or blank`, as a live run does.

```json
{"schema":"thinkthen.recognize-plan/2","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY","pieces":4,"request_count":1,"name_requests_upper_bound":1,"requests":[{"digest":"…","bytes":1234,"body_utf8":"…"}]}
```

With relations, numeric `relation_pairs_upper_bound` and `relation_requests_upper_bound` sum safe per-rule bounds, because names and kinds do not exist yet. A file-backed report carries `{"from":{"question":"file"}}`.

## Measured quality

`specification/fixtures/recognize/README.md` replays ticket 0147's recorded runs on the repository's own keys. At the five core kinds the run scored F1 0.865. With no kinds it scored 0.884, at `person` alone 0.847, and on a 1,018-word text 0.960. It found 20 of 27 stated relation edges. Local experiment 288 measured 80.1 F1 on a full public split and 59.1 on WNUT-17 with the same wording and window. Each figure comes from one run.

This specification makes no public price claim.
