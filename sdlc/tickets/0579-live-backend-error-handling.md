# 0579: Report live backend failures plainly

Status: OPEN.

Milestone: 0.2

Reviews: revision 5d81001, reject

Reviews: revision 782454e, reject

Reviews: revision 8c2f689, reject

Reviews: revision 5ab0cd6, reject

## Outcome

When a live backend fails, ThinkThen stops or retries correctly and tells the user why in the provider's own words. A quota error stops at once and says the account is out of credit. `backends check` names the provider it actually calls. A connection that never opened gets one retry.

## Evidence

- Starts from: the marketing lead's 2026-10-13 live runs of `backends check` and Beatles Bench through 0.2.0 built from main at 65ce9d5af on the Beelink. Jev, Liquid d1, pplx-decider and Jev through OpenRouter passed. Curl reproductions are in botassembly/awesome-thinkthen branch 0002-per-model-guides, under models/gpt-6-luna and models/jev-openrouter.
  - OpenAI returned status 429 with type `insufficient_quota` and the text "You have no credits remaining". ThinkThen treated it as a rate limit, retried 42 times over about three minutes, and printed "rate limit was reached ... try again later".
  - `backends check --backend openai` prints "provider systemone", although it sends the OpenAI Decisions shape.
  - Failures never show the provider's own error text, so neither failure could be explained without a hand-made request.
  - A single 30-second connection timeout stops a record run, because nothing resends after a connection failure.
  - For Jev, the check's `--plan` estimated about 4,200 input tokens, and the live reply reported 5,131.
- Keeps: the retry rules for real rate limits, and ADR 0111's rule that a request whose bytes were sent is never resent after a transport failure.
- Changes:
  - Read a 429's error type. On `insufficient_quota` or the provider's equivalent, stop at once with a sentence saying the account is out of credit, and name the backend.
  - Make `backends check` print the provider whose request shape it sends.
  - Show the provider's own error message in every backend failure, trimmed to a sensible length. Never include a key, a header or a derived key value.
  - Retry once when the connection never opened, since no bytes reached the server. Never resend after request bytes were sent.
  - Find why `--plan` undercounts Jev's input tokens by about a fifth, and correct the estimate or state its margin in the plan output.
- Proof:
  - Outside-in tests against a recorded or local fake server show the quota stop with zero retries, the provider's message in the failure, the right provider name, and one retry after a refused connection with none after a sent request.
  - Routine `test` and `lint` pass.
- Defers: nothing.

## Progress

PM amendment, 2026-10-13. Add the marketing lead's Ollama finding. On the M5 with Ollama 0.40.3, a plain `--url` to Ollama with no named backend fails `backends check` with three status 400 errors, because Ollama still refuses description objects, and the message says to "check --model and the request size". A status 400 shows the provider's own message. When the address looks like Ollama, the message also suggests `--backend ollama`. Drop the advice about `--model` and request size unless the provider's message points there. Keep the ollama built-in's text workaround.

PM amendment, 2026-10-13. Remaining work. `ticket/0579-backend-errors` holds 18 unmerged commits covering the quota stop (d996fce7d), the provider's own message with key and header screening (d5ee893fa to 782454ea7), the public provider name (f17eb57a2), one retry for an unopened connection (77d6b2ac7, 1872a07b3), and separate reported and planned token counts (3141e3695, 03ce8eb0b). A fresh review rejected 5d81001; 782454ea7 answers it and needs a fresh review. Remaining: implement the Ollama amendment, treating port 11434 or a host named `ollama` as Ollama-shaped, and remove the fixed text in `crates/thinkthen/src/cli/failure/status.rs` unless the provider's message points there, with an outside-in test against a local fake server. For Jev's plan estimate, state the margin in the plan output and name the measured cause in the record; do not tune the estimator. Rebase on `ticket/0543-final`, run routine `test` and `lint`, get a fresh accept on the head, and merge. No live or paid call is needed.

PM amendment, 2026-10-13. Yes: replace only the evidence inputs in the `status_reason` fixtures whose evidence is a short word such as `model` or `a` with unrelated evidence. Keep every diagnostic, kind, facts and send assertion, and keep the separate short-reflection refusal tests.
- 2026-10-13 started
- 2026-10-13 next: Qualify the reviewed backend repairs with the landed staging fixes and corrected generated outputs before merge; the combined Rust ceiling gates the candidate under the approved ruling.
- 2026-10-13 next: Fresh review accepts only the socket-isolation and unopened-connection retry test repairs at 5ab0cd66f with 187 CLI and 14 HTTP passes; rebased 26ed9f3b measures 188575 Rust lines, 969 above main, so finish routine gates and combine safe cleanup before merging.
- 2026-10-14 next: Merged the accepted backend repair into the combined branch at e363ee504 with all 28 patches preserved; the combined Rust ceiling remains a candidate gate and final qualification remains pending.
