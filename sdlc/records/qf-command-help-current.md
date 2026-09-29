# Quick Fix: current command help

Candidate: `ticket/qf-command-help-current` from clean main `00d0b37bdbba013f868c7b6ebe6ad4bed0f6e31a`, with only `crates/thinkthen/src/cli/args/command.rs`, `crates/thinkthen/tests/decide_edge.rs`, `spec/decide.md`, the measured `sdlc/ratchet.json`, and new-user stumble row 8 changed. This record is the sixth file. Root will arrange fresh read-only code review and push after this local commit.

## Behavior and evidence

`decide --help` now shows two executable `printf | thinkthen decide` commands before the answer, threshold, `set -e`, exit-code and script-control explanation. `decide -h` still opens with its one-sentence answer summary and everyday options. The commands change no option or runtime rule. `filter --help` now says records share requests by default and `--batch 1` asks one record per request. This follows [ADR 0055](../planning/adr/0055-decide-filter-and-rank-batches-send-each-record-once.md) and the [filter contract](../../specification/filter.md); it promises no fixed count for default packing.

The existing compiled-help case in `decide_edge.rs` now checks both examples before the long explanation while retaining its short-help, threshold, `set -e`, exit and option checks. A separate compiled `filter --help` case checks the default-batch sentence and excludes the former one-paid-request-per-record claim. The old binary failed the first new example assertion at exit 101 before the help change. Both selected cases later passed individually with one selected test each. Existing checks did not assert example position or filter's request statement; neither new assertion uses a test-only hook. The new filter case is separate because adding it to the already long decide case exceeded Clippy's cognitive-complexity limit. No scaffold test was deleted or consolidated. The prior parser, secrecy, cancellation, cache, invalid-input and conflict cases remain.

Focused commands and outcomes:

| Check | Outcome |
| --- | --- |
| `cargo test --locked -p thinkthen --test decide_edge the_short_help_shows_the_everyday_options_and_the_long_help_adds_the_rest -- --exact` | Initial red: missing first example, exit 101; final green: 1 passed, 0 failed |
| `cargo test --locked -p thinkthen --test decide_edge filter_help_names_default_batching -- --exact` | 1 passed, 0 failed |
| `PATH="$PWD/target/debug:$PATH" mustmatch test spec/decide.md` | 24 passed |
| `cargo clippy --locked -p thinkthen --test decide_edge -- -D warnings` | Passed after splitting the help cases; the first combined form failed the 22/20 cognitive-complexity check |

Cargo calls used `CARGO_NET_OFFLINE=true`, `CARGO_BUILD_JOBS=2`, cleared inherited Rust compiler wrappers, and the lane's `/run/user/1000/thinkthen-codex-2.lock`. Load was 1.57 with 20 GiB available memory before compilation. The warm lane target compiled the initial focused test in 10.41 seconds; later checks reused it. No SDK, toolchain, provider, backend, container or Actions command ran. The tests invoke compiled `--help`, and the page uses help or `--dry-run`; they do not start a backend or count a loopback listener. No paid call occurred.

An exploratory `cargo test --test decide_edge help` selected six cases and returned exit 101: five passed; the unchanged `shared_help_defers_order_and_document_rules_to_each_command` test expects `most likely yes first` in `rank --help`, while unchanged main `00d0b37b` already says `highest value first`. Both sides were confirmed with `git show HEAD`. This is outside the claimed help correction, not a passing broad test result. The two changed exact cases and the executable page pass.

Rust files are 428 and 497 nonblank lines, under the 500-line cap. The measured source total moved from 99,335 to 99,351 (+16): six new help-comment lines and ten net test lines. `sdlc/ratchet.json` matches 99,351. The existing `run` helper and help case were reused; no copied harness was introduced. The extra test lines earn their place by pinning previously unchecked public wording and position.

## What the build taught us

- The original `filter` sentence had survived the move to default batching. ADR 0055 and `specification/filter.md` already settle the replacement wording, so no new design was needed.
- Putting the filter assertion into the large decide help test tripped strict Clippy at 22/20 cognitive complexity. A separate focused compiled-help case kept the public boundary check and the file under its cap.
- A broad name filter in `decide_edge` includes an unrelated stale rank-help expectation. Future help changes should select the exact affected cases and report that mismatch separately; do not change rank wording or its test under this claim.
- The short summary, `set -e`, threshold and exit promises remained visible in the final compiled help. The executable page and selected test pin the new example order. New-user stumble row 8 is fixed; the register's other rows remain open.
