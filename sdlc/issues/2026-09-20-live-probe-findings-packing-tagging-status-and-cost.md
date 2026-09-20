# Live probe findings: packing, tagging, status, and the cost of one file

Status: Open. Four measurements are done. Each one names a change for the builders below.

Ian gave the direct go-ahead on 2026-09-20 with a ceiling of 50 US cents. A builder ran the four measurements from `2026-09-20-a-live-probe-plan-for-tagging-many-questions-and-status.md` as a spike outside this repository. Every launch went through `sdlc/scripts/live`. Every case is made-up text, and every trusted answer was fixed and hashed before the first paid call. One model answered everything: `jev-1.13.0` behind the alias `jev-latest`.

The raw requests, answers, scoring scripts, and the full write-up sit in the workspace at `experiments/thinkthen-probes-2026-09-20/`. `FINDINGS.md` there is the long form and `ledger-notes.md` holds the hashes and the ledger line for each launch. That folder is a spike and can rot. This page is the record.

## The spend

| Figure | Value |
| --- | --- |
| Launches | 4 |
| Declared maximum, all launches | 1,520,000 tokens |
| Billed input tokens, from the responses | 1,080,200 |
| Cost at 0.042 dollars per million | 4.5 US cents |
| Ledger before and after | 429,118 and 1,949,118 charged tokens |
| Requests sent | 3,190, every one answered 200 |

The ledger charges the declared maximum with no refund, so it moved by 1,520,000 while the service billed 1,080,200. `live --status` was read by command after the last launch.

## A. Many yes or no questions in one request

Three made-up documents of about 400 words. Forty yes or no questions per document, half true and half false, 120 in all. Five arms sent the same 120 questions.

| Arm | Requests | Wrong against the trusted answer | Flips against the single arm | Largest move | Billed input tokens | Wall seconds |
| --- | --- | --- | --- | --- | --- | --- |
| single | 120 | 0 | | | 90,468 | 38.7 |
| 5 per request | 24 | 0 | 0 | 0.02 | 19,812 | 7.5 |
| 10 per request | 12 | 0 | 0 | 0.03 | 10,980 | 3.9 |
| 20 per request | 6 | 0 | 0 | 0.01 | 6,564 | 2.0 |
| 40 per request | 3 | 0 | 0 | 0.02 | 4,356 | 1.0 |

No answer crossed the cut in any arm. The bill fell by a factor of 20.8 and the wall clock by a factor of 37. The service bills the evidence once per request, so the saving is the evidence text.

**For `annotate`.** Pack every question about one document into one request. Split only when the published token limits force it: 64,000 tokens for the request and 32,000 for the evidence plus the longest question. The measurement found no lower ceiling.

**Limits.** All 120 questions were clear cases, and the single arm scored 100 percent. This shows that packing does no harm on easy questions. It says nothing about hard ones.

## B. Tagging, twenty posts by twenty labels

Each post is one request carrying one yes or no question per label. The posts truly carry 37 label assignments.

| Arm | Exact set match | Precision | Recall | Flips against arm 1 | Billed input tokens |
| --- | --- | --- | --- | --- | --- |
| 1, bare labels | 10 of 20 | 0.778 | 0.946 | | 20,528 |
| 2, a description in `criteria.true` | 16 of 20 | 0.921 | 0.946 | | 30,088 |
| 3, arm 1 plus two unrelated labels | 12 of 20 | 0.783 | 0.973 | 3 of 400 | 21,748 |
| 4, arm 1 with the order shuffled | 13 of 20 | 0.818 | 0.973 | 3 of 400 | 20,528 |

The description is the one change that clearly helped. False positives fell from 10 to 3 and recall held. It costs about 46 percent more input tokens. A bare label is read generously. Added labels and a shuffled order each flipped 3 answers of 400, and two of those three are the same pair, so those sit near the cut.

**For `tag`.**
- Twenty labels in one request works, and twenty-two worked. The planned cap of twenty is safe on this evidence. Point 4 of the steering page is moot.
- `--label LABEL=DESCRIPTION` sends the description as `criteria.true`. The how-to teaches described labels first and shows bare labels as the quick form.
- The page says a tag near the cut can flip when the label list changes. A caller who needs a stable set holds the list fixed and keeps the probability under `--details`.

**Limits.** Twenty posts. One false positive moves precision by 0.02. The gap between arm 1 and arm 2 is worth trusting. The gap between arm 3 and arm 4 is noise.

## C. What status the service reports

The models listing returned two entries, `jev-latest` and `jev-preview`, each with a description and a release date of 2026-09-10. It named no versioned model, no credit, no quota, and no rate limit. One ordinary judgment carried six response header names: `content-length`, `content-type`, `date`, `server`, `x-envoy-upstream-service-time`, and `x-typesafe-request-id`. No header value was saved.

**For a `status` command.** It can report that the key reaches the address, which model names the key may send, and their release dates. It cannot report credit or quota, because the service publishes neither. Usage has to come from a local count. `--details` should carry the request id, since it is the one handle a support conversation can use. The response body names the versioned model, and `--details` should carry that too.

**Limits.** One listing and one judgment. A 429 may carry headers that a 200 does not. This spike saw no 429.

## D. The cost of one real file through the tool

The first 3,000 non-empty lines of Pride and Prejudice from Project Gutenberg. One run of the release binary asked `The line is dialogue spoken aloud by a character.` with `--lines --details` and sixteen requests in flight.

| Figure | Value |
| --- | --- |
| Records judged | 3,000 |
| Requests on the wire | 2,943, with 57 answered from the run's own cache |
| Billed input tokens | 854,804 |
| Cost | 3.6 US cents |
| Wall clock | 40.9 seconds |
| Mean input tokens per record | 290.4, from 277 to 301 |
| Records that failed | 0 |

A line of the novel is roughly 15 tokens and still bills about 290. Every request carries a fixed overhead near 256 input tokens. Short records pay almost entirely for overhead.

**The run went faster than the documented request limit, and nothing refused it.** 2,943 requests in 40.9 seconds is 72 a second, about 4,300 a minute. The vendor documents 1,200 a minute. The job was meant to stay under that and did not. No 429 came back. The limit may be counted another way, or it may not be enforced at this level. Nobody should plan on it.

**For the builders.**
- The tool does not pace itself, by design. `specification/records.md` says a run over the rate limit gets a 429, and `backends.md` fixes the retry. This run went 3.6 times over the documented limit at a width of sixteen and got no 429. The sentence in `records.md` is now unproven at this level, and it should say what was seen. I recommend no pacer. It is more code, it slows every honest run to two and a half minutes for a file like this one, and the 429 path already covers the day the vendor enforces the number. A test that replays a 429 in the middle of a wide run is the cheap guard.
- `filter` over short records is where packing would pay most, and the wire shape does not allow it. One request holds many questions about one piece of evidence. It does not hold one question about many pieces. Do not build a workaround that glues records together. Record 0017 kept evidence as one string for a reason.

**Limits.** One file, one question, one width, one day. The figure is for short records. A 400-word document bills roughly 700 tokens.

## What marketing may quote

| Claim | Number | Rule for quoting it |
| --- | --- | --- |
| Many questions in one request is cheaper | 20.8 times fewer billed tokens, forty questions against one, on our own 120 cases | Only once `annotate` ships and packs. Say "on clear questions" |
| The cost of a real file | 3,000 lines of a novel for 3.6 US cents | Quotable now. Name the book and the question |
| The cost of short records | About 1.2 US cents per thousand | Quotable now |
| Speed | 3,000 lines in 41 seconds | Hold. The run passed the documented rate limit. Measure again at a width that respects it |
| Tagging accuracy | 16 of 20 exact with described labels | Hold until `tag` ships. Twenty posts is a small set. Say the size |

## What Ian can overturn

Nothing here is a ruling. The recommendation against a pacer in section D is mine, and the builders or Ian can reverse it. The quoting rules are mine, and Ian can loosen any of them.
