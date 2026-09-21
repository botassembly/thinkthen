# systemone fixtures

Most cases are pairs: `NAME.request.json` is the body `encode` must produce, and `NAME.response.json` is a body `decode` must read. A `refused-` file is a response `decode` must refuse. Those fixtures compare as JSON values.

`decide-urgent` is the judging case: one yes/no question over one evidence.

`find-two.request.json` pins the exact request bytes for two generated ids and evidence containing a quote and a backslash. It has no response fixture because `find` reuses the settled choice decoder.
