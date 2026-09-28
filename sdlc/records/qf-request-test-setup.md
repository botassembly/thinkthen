# Quick Fix: reuse the request test's question setup

Status: fresh independent Medium review accepted `a4decd3d855c2c61a3d036619cd69309bf28681c`; the coordinator lands the unchanged test extraction. The coordinator claimed `engine/request/tests.rs` on main `86f021f2`. Graded-ranking verification exposed an inherited Clippy warning in `a_writer_seen_after_an_early_key_refusal_keeps_hit_or_mismatch`: its 95 counted lines exceeded the accepted 90-line function budget. The graded-ranking change did not introduce that function growth.

Extract the identical question-plan construction already used by two setup paths into `question_plan(model)`. Both callers retain their literal model, evidence and decide question. Every test row, triggering sequence, request, assertion and cleanup remains. This reduces the measured Rust source total from 96,250 to 96,244 nonblank lines. No production behavior, dependency, test count or lint rule changes.

## Focused verification

On Linux with the existing worktree build cache, the eight `engine::request::tests` cases passed. Cargo reported 11.16 seconds for the incremental test build and 0.15 seconds for execution. Strict `cargo clippy --locked --offline -p thinkthen --lib --tests -j 4 -- -D warnings` passed in 9.60 seconds. These observations describe this warmed local run; they are not a benchmark or a cross-machine comparison. Formatting, offline policy (189 resolved packages), ratchet 96,244/96,244 and diff checks passed. No full suite, provider call or stress campaign ran.

No test was deleted or consolidated. The cleanup removes duplicated fixture construction while retaining every distinct regression. No new permanent test is needed for this behavior-preserving extraction.

## What the build taught us

Focused target lint for an unrelated CLI change did not cover this existing library unit's size warning. The broader check correctly exposed it. Reusing identical setup fixes the warning without adding a suppression or changing the function budget. Preserve the CLI's valid checks and rerun only this affected test module and strict lint.
