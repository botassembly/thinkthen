# Cut Rust test code back toward product size: delete each of the 514 tests inside crates/thinkthen/src that an outside-in test already covers, and turn the repeated Rust Polars typed refusal tests into one edge-case table

Status: open.

Kind: idea
When: after 0.2.0 ships

## Problem

Cut Rust test code back toward product size: delete each of the 514 tests inside crates/thinkthen/src that an outside-in test already covers, and turn the repeated Rust Polars typed refusal tests into one edge-case table. a smaller, faster routine suite; findings in sdlc/records/2026-10-11-0-2-closure-review.md.
