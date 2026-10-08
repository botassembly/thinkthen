# 0471: Isolate spec and demo configuration

Source `58546daf828038fd318d3d0fe362bd0e03a40b0e` gives spec and demo children an owned empty configuration base after environment composition. The demo runner's existing self-test now plants a conflicting Ollama setting and malformed configuration, then checks the real command's default address and model. Usage state, explicit toolchain caches, replay assertions and owned cleanup remain in place.

Fresh read-only code review accepted the source with no findings. Policy, shell syntax, and full lint passed. The spec rung passed twice with owned conflicting and malformed caller configuration; each run passed 13 runner self-test cases and 24 green demos. The first spec attempt failed in the new self-test because a plan includes a summary row after its request row; the assertion now selects the request row. No runtime change was needed.

An additional full test run passed 1,766 workspace tests, one doctest and 338 library-only tests. It passed 22 external consumer tests, then was intentionally interrupted while the last consumer's unrelated native fixture remained active. The gate exited 100 from that interrupt, with no reported assertion failure before it. The coordinator owns any further broad test gate. Rust, C Rust, C and Go source totals stayed at 164133, 15470, 2740 and 4950. No source size warning was introduced; the existing warnings concern unchanged files. Count-only private-name lint found zero names in public text. No paid or release action ran.

## What the build taught us

The plan has a request row followed by a summary row. Selecting the request row lets the retained test prove that both hostile caller settings leave the demo's actual route at its default. Both runners must set the configuration base after their environment is composed.
