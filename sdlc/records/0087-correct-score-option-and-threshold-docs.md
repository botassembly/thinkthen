# 0087: Correct score, option-limit, and threshold documentation

Status: Accepted and landed on the main build line.

## Result

The score specification now reports the measured difference between the tool's computed score and the vendor field. The architect handoff replaces the stale 100-option claim with the successful 101- and 255-option probes and keeps the product ceiling separate from the backend request-size evidence.

The threshold how-to now maps all ten functions to the rows, signals, and boundaries needed for an honest labeled sweep. It distinguishes complete saved details from filtered outputs that require complete candidate collection or recording-backed reruns. The public trust page summarizes that boundary without linking to a private repository.

## Review and verification

Independent design review corrected the evidence claims, the ten-function mapping, the filtered-output boundary, the public-link requirement, and the documentation budgets. Independent code review rejected a private-repository link and an incomplete site-build record. Remediation removed the link and recorded the Node 22.22.3 build. The same reviewer accepted the final change.

The coordinator ran `sdlc/scripts/install`, `lint`, `test`, and `spec` sequentially with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset. All Rust tests and two doctests passed. Replay checks and all nineteen executable how-tos passed. The Astro build produced 44 pages, 44 Markdown twins, and `llms.txt`; every internal link landed. The four owning files total 268 nonblank lines. How-to 13 has 88 lines and 830 words. `git diff --check` passes. No paid or external request ran.

This ticket changes documentation only. Request splitting, recognize, relate, command completion, process controls, the public Rust API, packaging, publication, and deployment remain in their authorized tickets.
