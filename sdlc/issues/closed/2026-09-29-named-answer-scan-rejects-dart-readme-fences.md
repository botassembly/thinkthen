# Named-answer scan rejects the shipped Dart README fences

Status: Closed by Quick Fix `ec00b80d9`, accepted by fresh Medium review. The executable Dart scan and exact configuration-data boundary pass focused negative cases and the actual README scan.

Original finding: The selected lint checkpoint on main `7053849e1` reports unsupported `yaml` and `dart` fences in `libraries/dart/README.md` at lines 20 and 28. The scanner self-test passes; the real scan fails.

Determine how the current named-answer rule applies to package data and Dart examples, then fix the supported syntax or example without hiding executable code as plain text. Preserve the rule and negative fixtures. Inspect the shared scanner owner before editing it: `sdlc/scripts/named-answers.mjs` imports the site scanner, and site ownership remains separate. Prefer an exact small change with focused scanner fixtures and the actual README check; no port build or provider call is needed.
