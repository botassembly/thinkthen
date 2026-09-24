# relate sends a request over Jev's input token limit

Status: Open

Filed by the marketing session on 2026-09-24, from the Beatles deck's relate run. ThinkThen main was at ea3ac7ca.

## What happened

`thinkthen relate @graph.json --jsonl --kind-field /kind --field /name < beatles.jsonl` runs over 184 songs, 13 albums, and 4 people. With no `--profile`, the dry run plans two requests:

| Request | Relation | Bytes | Questions | Options per question |
| --- | --- | --- | --- | --- |
| 1 | sung_by | 85,458 | 184 | 5 |
| 2 | appears_on | 161,252 | 184 | 14 |

Jev answers request 1. It refuses request 2 with status 400. The reply body reads `{"detail":{"error_type":"max_tokens_exceeded"}}`. ThinkThen prints only "the backend refused the request; check `--model` and the request size".

## What the limit is

Eight live calls separated the causes:

- 161,266 bytes passed, so the request size in bytes is not the limit.
- 184 questions passed, so the question count is not the limit.
- A request with 5 options per question failed once its input grew, so the option count is not the limit.
- 142 album questions passed at 65,423 input tokens. 143 failed.

The cap fits 65,536 input tokens. The step between 142 and 143 questions gives that inference. The backend never states the number. `2026-09-21-a-refused-request-hides-the-backends-reason.md` saw the same edge: 61,819 answered and about 66,000 refused.

The Jev profile in ticket 0059 (`max_request_bytes` 250,000, `max_evidence_bytes` 120,000, `max_questions` 64) would not stop request 2 on bytes. Only `max_questions` 64 splits it. No record gives the source of the two byte numbers.

## Asks

1. relate splits a request that would exceed the backend's input token limit, with no profile needed. The deck works around it today with a profile of `"max_questions":64`.
2. The status 400 message carries the backend's `error_type` when the body has one. "max_tokens_exceeded" tells the user what to change.
3. The Jev profile states the real input limit, with this issue as its source.

Ian can overturn all three.
