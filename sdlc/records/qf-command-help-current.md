# Quick Fix: current command help

Accepted candidate: `28fc9e4b`, from clean main `00d0b37b`. Fresh independent Medium review accepted the six-file help change, and the coordinator merged the exact reviewed code.

## Behavior and evidence

`decide --help` now shows two executable `printf | thinkthen decide` commands before the answer, threshold, `set -e`, exit-code and script-control explanation. The first is an ordinary yes/no call; the second shows a threshold band. `--details` remains in the help and reference, outside these first examples. `decide -h` still opens with its one-sentence answer summary and everyday options. The commands change no option or runtime rule. `filter --help` now says records share requests by default and `--batch 1` asks one record per request. This follows [ADR 0055](../planning/adr/0055-decide-filter-and-rank-batches-send-each-record-once.md) and the [filter contract](../../specification/filter.md); it promises no fixed count for default packing.

The existing compiled-help case in `decide_edge.rs` now requires the explanation marker and compares the two complete, distinct `printf` example lines before it. It retains its short-help, threshold, `set -e`, exit and option checks. A separate compiled `filter --help` case checks the default-batch sentence and excludes the former one-paid-request-per-record claim. The old binary failed the first new example assertion at exit 101 before the help change. Existing checks did not assert example position or filter's request statement; neither new assertion uses a test-only hook. The new filter case is separate because adding it to the already long decide case exceeded Clippy's cognitive-complexity limit. No scaffold test was deleted or consolidated. The prior parser, secrecy, cancellation, cache, invalid-input and conflict cases remain.

Focused commands and outcomes:

| Check | Outcome |
| --- | --- |
| `cargo test --locked -p thinkthen --test decide_edge the_short_help_shows_the_everyday_options_and_the_long_help_adds_the_rest -- --exact` | Initial red: missing first example, exit 101; corrected case passed in the six-case selection |
| `cargo test --locked -p thinkthen --test decide_edge filter_help_names_default_batching -- --exact` | Passed individually and in the six-case selection |
| `cargo test --locked -p thinkthen --test decide_edge help -- --nocapture` | Final candidate: 6 passed, 0 failed, 15 filtered out |
| `PATH="$PWD/target/debug:$PATH" mustmatch test spec/decide.md` | 24 passed |
| `cargo clippy --locked -p thinkthen --test decide_edge -- -D warnings` | Passed after splitting the help cases; the first combined form failed the 22/20 cognitive-complexity check |

Cargo calls used `CARGO_NET_OFFLINE=true`, `CARGO_BUILD_JOBS=2`, cleared inherited Rust compiler wrappers, and the lane's `/run/user/1000/thinkthen-codex-2.lock`. Load was 1.57 with 20 GiB available memory before initial compilation and 1.65 with 21 GiB before the final six-case check. The warm lane target compiled the initial focused test in 10.41 seconds; the final six-case check compiled in 9.28 seconds after the correction. No SDK, toolchain, provider, backend, container or Actions command ran. The tests invoke compiled `--help`, and the page uses help or `--dry-run`; they do not start a backend or count a loopback listener. No paid call occurred.

The first six-case help selection returned exit 101: five passed and `shared_help_defers_order_and_document_rules_to_each_command` failed. It expected `most likely yes first`, but [the rank contract](../../specification/rank.md) allows either yes probability or a saved score question's weighted level position. The unchanged rank help already said `The printed order puts the highest value first.` The test now pins that exact shared wording; rank runtime and help text did not change. The corrected six-case selection passed 6/6.

Rust files are 428 and 499 nonblank lines, under the 500-line cap. The measured source total moved from 99,335 to 99,353 (+18): six new help-comment lines and twelve net test lines. `sdlc/ratchet.json` matches 99,353. The existing `run` helper and help case were reused; no copied harness was introduced. The extra test lines earn their place by pinning previously unchecked public wording and position. The rank correction changes only one expected sentence; formatting contributes to the measured test-line growth.

## What the build taught us

- The original `filter` sentence had survived the move to default batching. ADR 0055 and `specification/filter.md` already settle the replacement wording, so no new design was needed.
- Putting the filter assertion into the large decide help test tripped strict Clippy at 22/20 cognitive complexity. A separate focused compiled-help case kept the public boundary check and the file under its cap.
- The six-case help selection found a stale rank assertion. The settled graded-rank contract and existing help agree on highest value, including saved score questions, so the same claimed test file could correct its one-string expectation. No second ticket or rank runtime change was needed.
- A split that defaults to the full string when its marker is missing cannot prove that examples precede an explanation. The final case requires the marker and compares both complete example lines before it.
- The short summary, `set -e`, threshold and exit promises remained visible in the final compiled help. The executable page and selected test pin the new example order. New-user stumble row 8 is fixed; the register's other rows remain open.

## Independent review

Fresh Medium review accepted `28fc9e4b` after running the six help cases, all 24 executable decide-page assertions, strict Clippy, policy, measured ratchet and diff checks. It confirmed both complete examples precede the required marker, retain short-help and failure guidance, and introduce no runtime or option change. File counts were 428 and 499 nonblank lines; the measured total was 99,353. The coordinator verified an empty code-path diff against the accepted candidate and retained these checks. No full gate or external call ran.
