# systemone fixtures

Most cases are pairs: `NAME.request.json` is the body `encode` must produce, and `NAME.response.json` is a body `decode` must read. A `refused-` file is a response `decode` must refuse. Those fixtures compare as JSON values.

`decide-urgent` is the judging case: one yes/no question over one evidence.

`find-two.request.json` pins the exact request bytes for two generated ids and evidence containing a quote and a backslash. It has no response fixture because `find` reuses the settled choice decoder.

The `batch-` fixtures pin the exact bytes of batches, by ADR 0048 item 1. Each question quotes its record: `The text is `, the record's compact JSON, `. `, then the question. For example, the record `Come Together` asks `The text is "Come Together". The text is the title of a song by the Beatles.` Without a context the evidence is `{"records":[…]}`.

- `batch-three`: three plain records. The second holds a quote mark and a newline, so its quote carries JSON escapes, escaped again inside the instructions string.
- `batch-context`: two records beside a context. The context is the evidence, and the records appear only in their questions.
- `batch-choose`: two records under one pick-one question. The options stay as they are.
- `batch-csv`: two whole CSV rows, quoted and listed as JSON objects. Each question keeps its `--true` and `--false` meanings.
- `batch-duplicate`: the records `Come Together`, `Because` and `Come Together`. The copy adds no question and no evidence.
