# batching fixtures

ADR 0111 removed the content cut, and ticket 0304 slice 3a removed the last batcher that used it. The request files stay the byte oracle for quoted questions. The cuts, memberships, close reasons and digests below are history of ADR 0048's batcher.

`grouping.txt` holds 25 lines, one record each. It proved where batches closed, by ADR 0048 item 2.

## Portable cross-surface corpus

`portable-records.json` pins five selected text values, their exact compact JSON byte spellings, first-eight-byte SHA-256 heads, cut decisions, ordered membership and close reasons. `portable-1.request.json` through `portable-3.request.json` hold complete literal System One bodies. Each fixture file has one final line feed for ordinary text-file handling; that line feed is **not** part of the selected record or transmitted body. The corpus fixes `Is it relevant?`, model `jev-1.13.0`, Max batching, no context, and `http://127.0.0.1:9/systemone` for the three independently calculated exchange digests. A listener's ephemeral URL changes only the digest prefix; its body must still equal the literal file byte for byte. The first two requests hold records 1–2 and 3–4 and close on content; the third holds record 5 and closes at input end. The fifth is a batch of one, and it takes the same quoted form as the others, by ADR 0111 section 1.

A one-question frame annotation over those same five text cells sends exactly `portable-1.request.json` through `portable-3.request.json`, because every function sends the same quoted form. The Python pandas/Polars frame and Rust Polars frame proofs read those files.

The second text contains literal UTF-8 `c3 a9` for `é`. Its compact spelling has hash head `3db0138d194cf000`, a content cut. Hashing the alternative escaped spelling `"caf\u00e9-5544"` would give `72afc19814d82dd9` and move the cut; parsing that escaped JSONL input must instead recover the same selected string and the same three request bodies. `portable-structured.jsonl` gives a separate CLI-only input. Its oracle in `portable-structured.json` and two request files pins nested member order, Unicode, controls, `1` versus `1.0`, `-0.0`, an exponent, and the integer object's content cut. Text-only hosts do not receive those objects as structured values.

The core test reads both oracles. The compiled CLI and public C JSON door independently reach counted loopback listeners and compare actual bodies and returned request identities. These first two host paths are a checkpoint; the applicable remaining host runners and C-wrapper forwarding proofs are still required before register 64 closes. No answer accuracy or token-cost claim follows from this corpus.

## The content hash

A record's content hash is the SHA-256 of its compact JSON. A line's compact JSON is the line in double quotes. A record is a content cut when the first 8 bytes of its hash, read big-endian, are 0 mod 4,096. So a cut is a line whose first 16 hex digits end in `000`.

Work out each line's hash by hand:

```sh
printf '"%s"' 'line 2907' | sha256sum | cut -c1-16
```

The `printf` rule holds only for lines that need no JSON escapes. Every line here is plain ASCII with no quote mark or backslash.

| Position | Line | First 16 hex digits |
| --- | --- | --- |
| 1 | `line 1` | `68a9812806f89e5a` |
| 2 | `line 2` | `f470b97919ad563a` |
| 3 | `line 3` | `70b5941880230566` |
| 4 | `line 4` | `11bc694bf7db5e4d` |
| 5 | `line 5` | `9c083b7e08c9c242` |
| 6 | `line 6` | `085f33cc0a0fbb93` |
| 7 | `line 7` | `adc708608e6b9dfd` |
| 8 | `line 2907` | `ee04192375591000` |
| 9 | `line 8` | `59d1e69785b988ee` |
| 10 | `line 9` | `8d589aa624418f9b` |
| 11 | `line 10` | `6c75b11cb8875332` |
| 12 | `line 11` | `47fc315da055f030` |
| 13 | `line 12` | `9bec8b080ed2cceb` |
| 14 | `line 13` | `6f7d6bf5c7bc80e4` |
| 15 | `line 14` | `d2fbf12d42ddd4e5` |
| 16 | `line 15` | `7a85c5e8b5f0675f` |
| 17 | `line 7527` | `86b4d826fcfa2000` |
| 18 | `line 16` | `98e031560a35cc9b` |
| 19 | `line 17` | `63c8981e16c77a13` |
| 20 | `line 18` | `43b540402ce096fe` |
| 21 | `line 19` | `dfbebc77951bfe40` |
| 22 | `line 20` | `13a718f11918aac4` |
| 23 | `line 21` | `dda43e64e4ac355d` |
| 24 | `line 22` | `a82b1da554e8e004` |
| 25 | `line 23` | `92ad11487456f17c` |

Positions 8 and 17 are the two cuts. Two lines outside the file serve the inserts: `line 24` hashes to `95aa5f406e20fcd9` and is not a cut, and `line 10633` hashes to `b77bcfe021cb2000` and is one.

## The batches

Each batch is written as its positions and the reason it closed.

**Max, under a profile whose `max_questions` is 8.** A batch closes after a cut, or before the record that would make 9 questions.

1. 1 to 8, content. Position 8 is a cut and the batch holds 8.
2. 9 to 16, limit. Position 17 would make 9 questions.
3. 17 alone, content.
4. 18 to 25, end.

**`--batch 5`.** A batch closes after a cut or after 5 records.

1. 1 to 5, size.
2. 6 to 8, content.
3. 9 to 13, size.
4. 14 to 17, content.
5. 18 to 22, size.
6. 23 to 25, end.

**Max, `max_questions` 8, with `line 24` inserted after position 3.** The first stretch now holds 9 records: 1 to 3, the insert, then 4 to 8.

1. 1 to 3, the insert, and 4 to 7, limit. The next record would make 9 questions. This batch changes.
2. 8 alone, content. This batch changes.
3. 9 to 16, 17 and 18 to 25 keep their batches and digests, because they lie after the next cut.

**Max, `max_questions` 8, with `line 10633` inserted after position 12.** The insert is a cut, so it splits the second stretch.

1. 1 to 8 keeps its batch and digest.
2. 9 to 12 and the insert, content. This batch changes.
3. 13 to 17, content. This batch changes.
4. 18 to 25 keeps its batch and digest.
