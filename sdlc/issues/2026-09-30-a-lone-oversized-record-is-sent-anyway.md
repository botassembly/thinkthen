# A lone record over the size setting is sent, and one backend refusal stops the file

Status: open. Asked by the release QA team on 2026-09-30. Owner: the queue owner, after 0.1.
Kind: idea
When: after the 0.1 release

Refusing such a record locally would save a paid call that the backend is sure to refuse, and would let the other records still answer.

1. **One oversized line stops the whole file.** A three-line file whose middle line holds 1,000,000 characters answers line 1, gets a 400 `max_tokens_exceeded` on line 2, exits 4, and never asks line 3. This is specified: `specification/records.md:124` stops a run at the first failed record. Only `annotate --details --batch 1 --on-error continue` keeps going, and only for a missing `on` pointer.
2. **The run sends a request its own plan says is too big.** `--plan` estimated 516,095 to 908,168 input tokens, and the run sent it. This is also specified: `specification/backends.md:24` says one question or record alone passes the size setting and goes as one request. No code checks a backend token limit.

A change reverses those two sentences. The coordinator's default, which Ian can overturn: keep both rules for 0.1. After 0.1, consider a local refusal when one record alone exceeds the request byte setting, with the record failing as a row error.
