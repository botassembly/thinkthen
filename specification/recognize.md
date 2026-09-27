# Recognize names

Status: **Settled** for the command in ticket 0080. Relations are beta.

`thinkthen recognize [OPTIONS] [KIND]...` finds every name in one text and assigns one caller-named kind. `thinkthen recognize [OPTIONS] @FILE` reads the same question from a file. With no kinds, the ordered defaults are `person`, `organization`, and `place`.

Every token gets one detection question. With two or more kinds, every token also gets one kind question. One kind is assigned locally with probability one. Long text keeps its complete source context while ticket 0079's request splitter divides only the questions. A name may cross a request boundary. Offsets always refer to the original text.

## Names

One document prints one object:

```json
{"entities":[{"name":"Maria Chen","kind":"person","start":0,"end":10,"strength":0.98}]}
```

`start` is inclusive and `end` is exclusive. Both count Unicode scalar values. Equal names at different offsets remain separate.

`strength` is computed. It is the lowest detection probability in the name multiplied by the mean probability of the winning kind, rounded to four decimal places. It is not a model probability. `--threshold` keeps a name when its strength is at or above the cut. The default is `0.5`.

Whitespace separates tokens. `.`, `!`, `?`, `,`, `:`, and `;` peel from the end of a token. Internal punctuation stays. Lowercase connector words receive their own detection decision and may split a name. A trailing separate possessive is removed. One maximal contiguous run of `IN` words is one candidate. Runs are disjoint, and no overlap resolver runs. Ruled 2026-09-23, this baseline explicitly overturns the earlier overlap promise.

No name is a successful result: `{"entities":[]}`.

## Kinds

Bare kinds keep the caller's exact names. `--kind KIND=DESCRIPTION` gives descriptions instead. Bare and described kinds do not mix. A run takes 1 through 20 distinct nonblank kinds. A `--kind` with no `=` is refused at exit 2: ``--kind is KIND=DESCRIPTION, and this one holds no `=`; give a bare kind without --kind``.

## Relations

Relations are beta. `--relation NAME=SOURCE:TARGET` adds a directed rule and makes `relations` present. `*` explicitly means any kind. `--relation-threshold` keeps an edge when its model probability is at or above the cut. The default is `0.5`.

An edge repeats both complete names. A reader never follows an entity number:

```json
{"relation":"works_for","source":{"name":"Maria Chen","kind":"person","start":0,"end":10,"strength":0.98},"target":{"name":"Northwind Freight","kind":"organization","start":18,"end":35,"strength":1.0},"probability":1.0}
```

Wildcard sides expand to concrete kinds in first-seen name order. Different-kind concrete relations use one choice per member of the larger side, with the smaller side plus `none` as options. Every option at or above the cut becomes an edge. Same-kind concrete relations use yes/no pair questions. A name is never related to itself. A choice over 255 options or one choice that cannot fit the selected backend profile falls back to yes/no pairs for that concrete relation only. Sibling concrete relations choose their methods independently.

`relations` is absent when no rule was supplied. It is an empty list when rules were supplied and no edge passed.

## Records and details

Record modes print `{"input":INPUT,"value":OBJECT}` in input order. `--details` prints `thinkthen.result/1`, keeps the same object under `value`, lists every logical request digest in construction order, sums successful send counts and usage, and carries each token's detection and kind probabilities under `answer.tokens`.

A failed detection, kind, or relation question fails that input. It prints no partial name or edge object for that input. Earlier completed record rows remain printed.

## Dry run and cost

`--dry-run` needs no key and sends nothing. It prints one compact `thinkthen.recognize-plan/1` object for the first record. Keys appear in this order: `schema`, `url`, `model`, `key_env`, optional `from`, `words`, `detection_questions`, `kind_questions`, `request_count`, the optional relation bounds, and `requests`.

`words` counts words as the tokenizer above splits them. `Ada met Acme.` is four words: `Ada`, `met`, `Acme`, and `.`. It is not a model token count. `request_count` is the exact recognition request count. `requests` lists each of those requests in send order, with its recording `digest`, UTF-8 `bytes`, and exact `body_utf8`, as `relate --dry-run` lists its own. An empty or blank text prints no plan. It exits 2 with `thinkthen: the evidence is empty or blank`, as a live run does.

```json
{"schema":"thinkthen.recognize-plan/1","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY","words":4,"detection_questions":4,"kind_questions":4,"request_count":1,"requests":[{"digest":"…","bytes":1234,"body_utf8":"…"}]}
```

With relations, numeric `relation_pairs_upper_bound` and `relation_requests_upper_bound` sum safe per-rule bounds because names and kinds do not exist yet. A file-backed report carries `{"from":{"question":"file"}}`.

This specification makes no public price claim.
