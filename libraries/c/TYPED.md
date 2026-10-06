# Counted C interfaces (0426 slice A)

The installed `thinkthen.h` adds immutable owned question/source/result handles, counted UTF-8 descriptors, and ten named `thinkthen_FUNCTION_current` helpers. They use the existing native text APIs and expose their current result/1 values. These helpers are an additive functional slice of 0426. They do not establish result/2 adoption or complete SDK parity. Every prior symbol, signature, layout, error code, bare call and generic JSON door remains compatible.

The reviewed result/2 `thinkthen_FUNCTION_complete`, complete result accessors, failure snapshots and image execution exports are **not published**. Native 0443/0444/0445/0450 adoption must supply real transport/answer/observation IDs, origins, reported-model provenance and final call IDs. C supplies none of these by guessing from a cache flag, request digest, historical model or nonempty result. Native 0447/0448 own image validation, execution and reader admission. Reserved image descriptors have no executable constructor in this slice; image-backed records and IMAGE_FILE selections refuse with Usage. There are no stub exports.

## Inputs

`thinkthen_question_new(engine, spec, out)` copies the descriptors into the existing native question grammar, preserving named-member and option order. The native parsers decide admission, defaults and conflicts. `kind` selects one of the ten `FUNCTION_*_V1` constants. Decide/filter use a native decide question; choose/tag/score use `choices` as their options/labels/levels. Annotate uses ordered member names and question handles, with zero `text`; the constructor clones the member values before returning. Recognize/relate use `kinds`/`relations` and zero `text`. Plain rank/find use native text constructors; find also accepts `none`. Rank sets use ordered decide members and the native rank-set parser.

`thinkthen_question_load` reads a user-named, bounded UTF-8 file through `read_question_file`, then invokes the native single-question, question-set, recognize or relate parser. A loaded decide set can also reach `rank_current`, which applies native rank admission before reading evidence. Single saved decide/score rank criteria remain unavailable through these helpers because the current public native rank API cannot preserve their complete settings. The existing user-facing calls remain available. No second question parser or rank scheduler is added.

DEFAULT readings leave native defaults in force. NULL readings are admitted only where the current native API has no cut; required decide/tag/filter/entity cuts and recognition's relation cut refuse NULL rather than replacing it with a default.

Content TEXT is counted literal UTF-8; Content JSON is one caller-authored JSON value. Neither requires a terminator. Sources clone original records and explicit paths. TEXT remains text when serialized; JSON retains its authored value in native record admission. This slice refuses record-specific context, option overrides, images and authored choice weights because their required native typed doors are not available. It does not silently discard them. Non-root `on` and custom relate pointers follow the current native API's explicit refusals.

`thinkthen_source_files` validates LINE/WINDOW/FILE with native `ReaderOptions`. WINDOW requires a positive `window`; other units require zero. Execution calls `read_files` and consumes its existing iterator; there is no second enumeration, reader or traversal. Paths are provenance, never evidence. Every execution reads the selected files again. Native Unicode/CRLF handling, physical line coordinates and later-read errors remain authoritative. The old generic JSON source module remains shared with R until its native owner extracts it.

NULL controls means no deadline, no cancellation, native batch/context defaults and attempts disabled. A supplied controls descriptor must initialize `deadline_ms`; zero is an expired deadline. `batch` and `batch_max` conflict. Context/batching follows native admission. Attempts are opt-in. A nonempty `surface` refuses until native 0443 can carry it. Native scheduling, deadlines, cancellation, caches and recording/replay are reused.

## Results

Every successful helper returns an owned result, including null or empty success. Nonzero returns leave the output slot unchanged and use the existing per-thread engine error table. Current accessors return Usage on NULL, wrong kind or invalid index without changing any output or saved error. Current failures still use the prior error accessors; there is no incomplete complete-failure snapshot.

| Accessor | Current native fields |
| --- | --- |
| `current_summary` | Function, published row count, question-event count, native final facts, attempts presence/count |
| `current_atomic` | Decide/choose/tag/score values, original input/location/index, ordered probabilities, yes probability, score level, optional confidence, native detail metadata |
| `current_filter` | Accepted original records and locations; rejected decisions remain in question events |
| `current_rank` | Native ordered originals, original indices and probabilities; set rank also names the selecting member |
| `current_find` | Nullable selected original/index/probability and every ordered candidate, including an optional synthetic none with absent original/index |
| `current_annotate` | Ordered member names, success values or one of the six native member failure causes; authored decide meanings retain Content |
| `current_recognize` | Native entities with Unicode scalar offsets, kind/strength, and optional typed relation edges |
| `current_relate` | Native typed edges/endpoints and all original input occurrences/locations |
| `current_question` | Native question events: input/member/stage/position, request metadata, value or failure, ordered probabilities/confidence, failed count |
| `current_attempt` | Native ordinal, request digest, wall time, outcome, optional status/server time/provider request ID |

Decide distinguishes successful NULL, ordinary BOOLEAN and AUTHORED Content. Choose distinguishes absent selection from an empty string. Optional confidence distinguishes absence from zero. Absent arrays use NULL/zero; recognition distinguishes absent relations from a present empty list. Native question events expose their native judgments; row values additionally project authored decide meanings. Flat inactive value payloads are zero. Counts describe published rows, not accepted edges or fabricated observations; find and relate each have one aggregate row, including empty success.

Rank keeps current native result/1 probability ordering; it does not pretend to have adopted 0436's result/2 final-rank values. Recognition's pieces/name/pair distributions and relate's complete logical-question endpoint/method/direction carriers are not accessible through the current public native API. Native question events expose the available probabilities and failures; C does not reconstruct missing details from result JSON. Full canonical resolved question views, result/2 aggregate metadata, identities, stopped descriptors, image views and missing detailed metadata remain open. Native `cached` means cache **or replay**, not a result/2 origin. Recognition over files uses native scalar subcalls with a shared deadline and native `Tally`; attempt ordinals keep each native subcall's scope.

## Ownership and ABI

All concrete `_v1` layouts are frozen; fields/discriminators cannot be appended or renumbered. The `current_*_v1` layouts are distinct from the reviewed complete result/2 layouts. That target can land additively after its native owners; these helpers need no layout change.

Counted storage reads exactly its length, checks addressable byte extents before slicing, and requires real initialized readable memory from C. NULL data is valid only at zero length. Flags/presence are exactly 0/1. The boundary cannot validate arbitrary addresses. Forged, stale and concurrently freed nonnull handles remain outside the existing contract; no handle registry is added.

Constructors clone every admitted buffer and child question. Calls borrow immutable handles/controls only until return. Results own all nested list/string backing allocations; views survive engine/question/source destruction until `thinkthen_result_free`. Reading immutable handles/views concurrently is allowed while their owners remain live. Free each owner once, after all borrowers finish; freeing NULL is harmless. Rust alone allocates/frees handles. Reserved image views will borrow image memory until its future image free, and result image views will borrow result memory; this slice returns neither.

Tests compile real C consumers through the existing sanitizer harness and retain the existing symbol inventories, secrecy, cancellation, threading, error and source suites. Result/2/image parity and final qualification remain open; root owns fresh code review, full landing gates and the single landing record. Ian can overturn the current helper spellings before publication.
