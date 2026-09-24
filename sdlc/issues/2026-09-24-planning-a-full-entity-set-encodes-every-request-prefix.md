# Planning a full entity set encodes every request prefix

Status: Open

The shared request splitter in `crates/thinkthen/src/engine/prepared_request.rs` (lines 44 to 50 at ticket 0088's landing) tries every prefix length from 1 to the number of remaining questions. It encodes and preflights each prefix. With no backend-profile limit, one plan costs a number of encodings that grows with the square of its question count. A same-kind relation over n entities asks about n(n-1) questions, so planning grows with the fourth power of n.

No request goes out during this work, so it costs no money. It delays dry-run output and the first send. It also turns a regressed 256th-entity refusal into a test that hangs instead of failing.

## Reproduction

Found by the final review of ticket 0088 (`sdlc/records/0088-review-final.md`, finding F1). A dry run sends nothing:

    $ seq 1 80 | sed 's/^/entity-/' \
        | thinkthen relate linked --lines --dry-run --url http://127.0.0.1:9/v1 --model local-1 --no-cache

The reviewer measured a release build at 0.47 s for 40 entities and 7.65 s for 80. A debug build took 5 s for 40 and 81 s for 80. The 255-entity limit extrapolates to about 13 minutes of release CPU.

## Smallest fix

Try the whole remainder first and take it when it passes. When it fails on a limit that permits a split, binary-search the largest passing count. The limits only tighten as a chunk grows.
