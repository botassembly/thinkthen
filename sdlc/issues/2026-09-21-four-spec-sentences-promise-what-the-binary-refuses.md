# Four spec sentences promise what the binary refuses

Status: Closed on 2026-09-22. Merged into 2026-09-22-command-wording-and-help-fixes-for-0-1.md.

Four sentences in `specification/` describe behavior the binary does not have. Each row gives both quotes and the probe.

## 1. `annotate` empty evidence

`specification/annotate.md:88`: "`--dry-run` validates the file and sends nothing. It needs no key. Empty evidence succeeds and prints nothing." `specification/records.md:99` says the opposite: "An empty document is a usage error, because a judgment about nothing is a mistake in the pipeline."

    $ printf '{"version":1,"questions":{"a":{"decide":"Is this a complaint?"}}}\n' > set.json
    $ thinkthen annotate set.json --dry-run < /dev/null
    thinkthen: evidence is text, not white space
    (exit 2)
    $ thinkthen annotate set.json < /dev/null
    thinkthen: evidence is text, not white space
    (exit 2)

The page likely meant an empty record stream, which does succeed. Fix: the sentence says the empty document is a usage error and the empty stream succeeds.

## 2. Empty CSV input

`specification/filter.md:49` and `specification/rank.md:47`: "An empty record stream exits 0 with no output and no request." `specification/records.md:32`: "CSV uses a comma and TSV uses a tab. Both require a header. Empty input is exit 2."

    $ thinkthen filter 'q?' --csv < /dev/null
    thinkthen: the CSV header is missing because the input is empty
    (exit 2)
    $ thinkthen filter 'q?' --jsonl < /dev/null
    (exit 0)

Fix: the two function pages qualify the sentence to line and JSONL streams, or point at `records.md`'s CSV rule.

## 3. A `choose` tie without a threshold

`specification/threshold.md:59`: "An exact tie for first place in `choose` is unresolved with or without a threshold. With no threshold `choose` returns the winning label." `specification/choose.md:76`: "An exact tie is unresolved with or without a threshold, because alphabetical order is no evidence."

    $ echo '{"mode":"prob","prob":0.5}' > ctl/mode.json
    $ thinkthen choose 'Which?' a b --url http://127.0.0.1:8931 < m.txt
    null
    (exit 3)
    $ thinkthen choose 'Which?' a b --threshold 0.8 --url http://127.0.0.1:8931 < m.txt
    null
    (exit 3)

The binary sides with `choose.md`; the last sentence of `threshold.md:59` reads as the opposite. Fix: drop or rewrite the trailing sentence.

## 4. Two illustrative examples

`specification/result.md:99` shows a `rank --details` example with `"value":true`, and `specification/rank.md:43` fixes "Every ranked row carries `threshold: null` and `value: null`". `specification/tag.md:25` shows a `--details` example missing the `schema` and `meta` fields that `result.md:22-26` fixes as always present. Fix: the examples gain the missing fields or a sentence saying they are abbreviated.

## How bad it is for a user

Minor. Each row sends a script author or a page reader against a sentence the tool breaks, and no wrong answer results.

Found by experiment 218, wave 1.5, the spec cross-reader seat.
