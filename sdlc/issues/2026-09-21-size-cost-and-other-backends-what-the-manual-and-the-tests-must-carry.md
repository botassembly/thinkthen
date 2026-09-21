# Size, cost, and other backends: what the manual and the tests must carry

Status: Open

Ian asked on 2026-09-21 that the facts about text size, cost, and context length be captured, and said other models will be supported later "as long as they support the system one API, or we can adapt to whatever their API is". This page gathers the facts that already have a record, lists what a user is never told today, and says what a second backend must state. The windowing design question is the experiment team's to file, and this page does not repeat it.

## What is measured, with its record

| Fact | Value | Record |
| --- | --- | --- |
| The vendor's price for input tokens | 0.042 US dollars a million | `sdlc/records/0011-the-live-probe.md` |
| What one short record bills | 290.4 input tokens on average, from 277 to 301, over 3,000 records | `sdlc/issues/2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md` |
| The fixed part of every request | near 256 input tokens | the same page |
| What a thousand short judgments cost | about 1.2 US cents (3,000 records cost 3.6 cents) | the same page |
| The text in one request | about 32,000 tokens | `specification/records.md` |
| One whole request, text plus every question | about 64,000 tokens, confirmed by one live request of 33,663 tokens | `specification/records.md`, `sdlc/records/0017-every-question-option-has-two-homes.md` |
| The local size limit for one record | 16 MiB, refused before any request with exit code 2 | `crates/thinkthen-core/src/records.rs` |
| `find` in one request | 2 to 255 lines or records | `probes/find-0040/README.md` |

Three rules of thumb follow, and each is safe to print:

1. **Requests are the bill.** A short text pays almost only for the fixed part. Many questions about one text pay the fixed part once, which is what `annotate` does. Many texts pay it once each.
2. **The tool never splits a text.** The text crosses whole. No word, sentence, or punctuation handling exists. A text over the vendor's limit is refused by the backend with exit code 4.
3. **The limit is two numbers.** One question and its text fit in about 32,000 tokens. The text and all the questions together fit in about 64,000. A 30,000-token document with ten short questions fits. A 40,000-token document does not.

One number is not safe to print: experiment 208 declared 1,301,000 tokens and was billed 645,840. The first number is the sum of the caps reserved for the runs, and the second is what the vendor billed. The gap is unused reservation. It does not measure a saving from packing.

## What a user is never told today

- No help page or manual page states the 32,000 and 64,000 limits, or that the measure is the vendor's tokens and not words or bytes.
- Nothing tells a user before a run that a record is too large for the backend. The 16 MiB check is far above the real limit, so the first news is an exit code 4 from the backend, after the request was sent.
- No page states what a judgment costs, or that the fixed part dominates.
- `rank` says an endless stream is cut into windows upstream, and no how-to shows a user how.

The documentation plan should carry one page, "Size and cost", with the table and the three rules above. The trust list already asks for the request count before a big run.

## The rule for other models, stated once

Ian's direction on 2026-09-21: ThinkThen supports any service that speaks the System One request and reply format, and it adapts to a service whose format differs, provided the service answers the three kinds of question and returns probabilities.

The first half works today and is already decided. ADR 0004 makes a backend a URL and an adapter, and the first adapter is `systemone`. `specification/backends.md` has the tool post to `BASE/systemone`, with the base taken from `THINKTHEN_BASE_URL` and the model from `--model`. A compatible server needs one environment variable and no code. The second half is ADR 0004's planned `chat-logprobs` adapter and its later subprocess adapter. No new decision is needed. What is missing is any test against a second server.

## Context windows: what is known and what is not

| Question | State | Where |
| --- | --- | --- |
| The first backend's limit for one text with one question | Known: about 32,000 tokens, from the vendor's table | `specification/records.md` |
| The first backend's limit for a whole request | Known and checked live once: about 64,000 tokens, with a 33,663-token request accepted | `sdlc/records/0017-every-question-option-has-two-homes.md` |
| Whether the two limits hold exactly at the edge | Not known. Nobody has sent a text just under and just over 32,000, or a request just under and just over 64,000 | |
| What the service replies when a request is too large | Not known by test. The specification says exit code 4. No recording of that reply exists | |
| Whether the limit differs by route, since a reseller may advertise a smaller one | Not known. Ian's note reports one reseller listing 32,000. Unverified | `Jev context length.md` in Ian's notes |
| How many questions one request may carry | Not known. No count limit was found. Twenty-two `tag` labels worked. Only the 64,000-token total is documented | `2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md` |
| Whether accuracy falls as a text grows toward the limit | Not known. Every accuracy run used short texts | |
| Whether many questions in one request change each other's answers | Partly known. The packing and tagging probes found agreement on short texts. Not measured on long ones | the same page |
| How a user learns a text is too large before paying for the request | Not built. The tool counts no tokens. The only local check is 16 MiB | `crates/thinkthen-core/src/records.rs` |
| Any other model's limits, price, or format | Not known first-hand. Ian's note holds a web survey, unverified. Its lesson: limits run from about 512 tokens to about 32,000 per question, and the underlying model's window is often far larger than the decision layer's | `Jev context length.md` |
| Whether any other server accepts our exact request bytes | Not known. No request has been sent to one | |

The cheapest probes that turn "not known" into "known", each a handful of requests under `sdlc/scripts/live`: the two edges of each limit with the reply recorded, one long text with the twenty conformance questions asked alone and then together, and one accuracy check at 1,000, 10,000, and 30,000 tokens on a public set with long documents. The second-server check waits until a compatible server is chosen, and running one locally needs no paid call.

## A public leaderboard, read on 2026-09-21

Ian pointed to a public benchmark page that compares about forty models and projects on decision tasks. A grunt agent read it once. These are the page's statements, not ours, and nothing here is verified.

- **Most listed projects are marked compatible with the first vendor's request format,** several of them "inferred" and not confirmed. One family says it serves a `/v1/systemone` server. If true, those work with `THINKTHEN_BASE_URL` alone, and that is the first thing to test.
- **A few are marked not compatible,** and one needs its own Python client. Those would need an adapter under ADR 0004, or stay out.
- **Context limits are stated for only a handful, and they are small:** 512 tokens for two projects, 64 tokens of text for one, 8,192 for one. For every other entry the page says "not stated". The page also lists the questions a request may carry as unknown for most systems.
- **Prices are mostly estimates,** and the page calls its adjusted speed figures "an assumption, not a measurement".

What this changes: the adapter rules above stand, and rule 1 matters most. A user who points ThinkThen at a 512-token server and pipes in a contract must get a clear refusal that names the limit. Today the tool knows no backend's limit, so the refusal would be whatever that server sends back. A profile that carries its backend's stated limits, checked before the request leaves, is the smallest fix, and it also answers "how does a user learn a text is too large before paying" for the first backend.

## Other backends

ADR 0004 makes a backend a URL and an adapter. Ian's direction: a second model qualifies when it answers the three kinds of question the functions use, which are yes or no, pick one, and place on a scale, and returns probabilities. Ian's note `Jev context length.md` in his notes folder surveys several open implementations of that interface. Its numbers come from a chat assistant's web reading on 2026-09-21. Nobody here has verified them, and they are recorded only as a reason for the rules below. The survey's useful finding is that the context limit differs by more than a hundred times between implementations, from about 512 tokens to about 32,000 per question, and that the limit of the underlying model is often far above what the decision layer was trained or served for.

What an adapter page must state before its backend ships, each with the check that measured it:

1. The limit for one question with its text, and the limit for a whole request, in that backend's own tokens.
2. Whether many questions about one text ride in one request, and the most questions in one request.
3. Which of the three kinds of question it answers. A backend that lacks one makes the functions built on it refuse by name.
4. Whether answers to packed questions are independent of each other.
5. The price, the fixed part of a request, and the documented request rate.

What the conformance cases gain, so that one file tests every backend: a text just under and just over each stated limit, a question set at the stated maximum, and the same twenty cases run through each adapter with the measured agreement against the first backend reported beside the results.

## What Ian can overturn

All of it. The "Size and cost" page and the adapter rules are recommendations to the build team for the documentation and engine work already planned.

## The probes ran, 2026-09-21

Two paid probes under `sdlc/scripts/live` answered five of the rows above. The
folder is `experiments/219-thinkthen-limit-edges/` in Ian's workspace, and its
`RESULTS.md` holds the tables, the commands, and every saved reply. Eight jobs
charged 472,500 tokens of cap and billed 250,390 input tokens, about one US
cent.

| Question | New state |
| --- | --- |
| Whether the two limits hold exactly at the edge | **Known as a bracket, not a cut.** One text with one question: 31,826 input tokens answered, and the next step up, about 33,150, was refused. A whole request: 61,819 answered, and about 66,000 was refused. Both documented figures sit inside their bracket |
| What the service replies when a request is too large | **Known.** Exit code 4 and one line, `thinkthen: the backend answered with status 400`. The vendor's own message is not on record, because `--record` writes its folder only after a successful exchange. Nothing was written for either refusal |
| How many questions one request may carry | **Still only bounded by size.** Twelve mixed questions and ten padded ones both rode in one request. No count limit appeared |
| Whether many questions in one request change each other's answers | **Known on a long text.** Twelve questions of three kinds over a 10,763-token document, asked alone and then packed: no answer changed, and the largest probability difference was 0.01, below the 0.08 repeat noise that experiment 212 measured |
| How a user learns a text is too large before paying for the request | **Still not built,** and the refusal is now measured to carry no size and no limit, so the case for a profile that checks before the request leaves is stronger than this page assumed |

The 61,819-token request more than doubles the previous live high water mark of
33,663 tokens in `sdlc/records/0017-every-question-option-has-two-homes.md`.

Packing paid for itself again on a long text. Twelve questions asked alone
billed 121,335 input tokens. The same twelve packed billed 10,763, which is 11.3
times cheaper for the same twelve answers.

Two findings for the build team, neither fixed here:

1. **A refusal leaves no recording.** `--record DIR` creates `DIR` after the
   exchange succeeds, so the one reply a user most needs to read is the one
   reply never kept. The specification's exit code 4 is correct and the body
   behind it is unreachable.
2. **`--dry-run` does not validate `--jobs`.** `decide --jobs 1 --dry-run` over
   one document printed a plan and exited 0. The same command without
   `--dry-run` refused at exit 2 with `--jobs bounds the requests in flight, and
   one document sends one request`. A dry run that passes is no promise that the
   live run will.

What is still not known: whether the limit differs by route, whether accuracy
falls as a text grows toward the limit, and anything first-hand about a second
backend. None of those was in this probe's scope.
