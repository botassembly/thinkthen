# 0059: Add named backend profiles and local size preflight

Date: 2026-09-22

Status: landed

## Result

An explicitly selected JSON profile now gives a backend a stable calibration name and local evidence-byte, encoded-request-byte, and expanded-question limits. Every logical request is checked before replay, cache, key lookup, or network access. The file cannot select an address, model, adapter, credential, retry policy, cache, or worker width.

A saved question or question set may name the profile used to tune its threshold. A different selected profile prints one fixed warning at the first ordered successful judgment and adds the same fact to detailed metadata. An early preflight refusal prints no warning. A successful `filter` run still warns when it removes every row.

Request bytes and recording identities did not change. Unprofiled question digests remain unchanged. The implementation adds no default profile, discovery, tokenizer, dependency, or paid call.

The Rust ceiling rose from 26,545 to 28,112 nonblank lines. The production parser, shared conformance cases, exact request-boundary checks, command coverage, ordered-warning proof, secrecy sweep, and necessary module splits account for the increase.

## Review and proof

Independent design review rejected two drafts for ambiguous question-set calibration, incomplete count and ordering rules, broad conformance wording, and warning timing under streamed failure. The final design fixes one top-level calibration identity, exact production counts and messages, preflight before stored or live answers, digest compatibility, and ordered warning behavior. The same reviewer accepted it.

Independent code review rejected the first implementation because workers could print warnings out of logical order, two specification pages were stale, and new failures lacked complete secrecy coverage. Re-review then found that an all-rejected `filter` run could hide its calibration mismatch. The final repair emits at the ordered successful-judgment boundary before output selection. The same reviewer accepted it.

The final gates passed with the key and base-address variables unset. The test rung passed 184 library tests, 231 backend tests, every compiled edge and transform suite, local live-script cases, and two documentation tests. The specification rung passed 27 page checks, seven transform checks, every committed replay, and all 19 green how-tos. The focused profile suite passed six unit and conformance tests and 15 compiled tests. `git diff --check`, the 500-line ceiling, and the exact 28,112-line ratchet passed. No outside network or paid call ran.
