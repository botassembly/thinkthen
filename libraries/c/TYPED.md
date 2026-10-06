# Counted C carriers (0426, unpublished work)

Ticket 0426 is in progress. The reviewed target is ten `thinkthen_FUNCTION_complete` calls with result/2 views, observations and owned failure snapshots. Their exact signatures are retained privately in `src/complete-signatures.h`; they are not installed declarations or exported symbols yet. No temporary `*_current` ABI is exported. Every released symbol, signature, layout, bare call and JSON compatibility door is preserved.

`include/thinkthen.h` contains the canonical reviewed descriptors and views. They remain unpublished until the whole ticket is integrated and reviewed. The unpublished `record_v1` includes optional original content and context, ordered candidate options and ordered images; no invented `record_v2` is needed.

## Constructors and ownership

`thinkthen_question_new` clones counted descriptors and child questions, then uses the native question grammars for admission. Options and members retain their authored order. `thinkthen_question_load` uses the bounded native question-file reader and parsers. Native refusals remain explicit, including unsupported authored question weights, non-root `on`, custom relate pointers and unavailable saved rank criteria.

`thinkthen_image_clone` validates JPEG/PNG media, dimensions and pixels through native `ImageInput`, clones the original compressed bytes and optional filename, and never resizes or re-encodes. `thinkthen_image_view` borrows those bytes, dimensions, media and filename until `thinkthen_image_free`. The owner survives engine destruction. Media is explicit; ordinary text/bytes never imply an image.

`thinkthen_source_records` clones original content, context, candidate names/descriptions/weights and every referenced immutable image. It preserves order and duplicates. Text records require original content; image records may omit it. Native image-evidence count/byte/text bounds apply during construction. Candidate cloning does not claim function-specific execution admission.

`thinkthen_source_files` clones an explicit LINE/WINDOW/FILE/IMAGE_FILE selection and validates it with native reader options. Construction selects paths without opening evidence. The private integration adapter uses `read_inputs`; it adds no traversal or parser. Image files retain their filename with absent text line coordinates. Text sources retain native Unicode/CRLF and physical line coordinates. Filenames are provenance, never model evidence.

Counted storage must be initialized, readable, UTF-8 where specified, and have its declared extent. NULL data is valid at zero length only. Extent multiplication is checked before slices are formed. Flags/presence are exactly 0/1. Invalid required pointers/descriptors return Usage without writing output. The boundary cannot validate arbitrary addresses.

Constructors own their cloned values. Image and eventual result accessors borrow immutable owner memory; their nested pointers expire when that owner is freed. Reading immutable handles/views concurrently is permitted while owners remain live. Rust alone allocates/frees handles. Free each nonnull owner once after all borrowers finish; NULL frees are harmless. Forged, stale and concurrently freed handles violate the lifetime contract; no registry is added.

## Private integration still required

The canonical struct/union layouts, zeroed inactive union storage, owned backing allocations, authored question snapshots, typed judgment/probability/entity/edge conversions and image/location views are implemented. The C error table retains owned native final facts and private sticky snapshots without re-decoding JSON. Result/1 native conversion regressions remain private Rust tests; they do not expose an alternative ABI or claim result/2. Their legacy admission refuses context/options/images before execution rather than discarding them.

Native result/2 execution and metadata remain lane0 ownership. No `CompleteDecision`, `CompleteChoice`, `CompleteTags`, `CompleteScore`, `CompleteFilter`, `CompleteRank`, `CompleteFound`, `CompleteAnnotated`, `CompleteRecognized`, `CompleteRelated` or `CompleteFacts` is available on the merged native main used here. The lane0 sources are inspected read-only and their active WIP is not merged.

After those types land, the complete bridge still needs native typed access to row/aggregate Meta and normalized question/readings, raw choose/find answer picks, and question-observer answer/observation/failure identities and aligned sources. `QuestionDetail` currently exposes only legacy event fields; `Error` exposes no structured stop at/cause/status. The present `QuestionInput` carries text/images but no per-record context/candidate options. Those execution inputs and the existing saved rank/field-selection gaps remain native integration work. `Facts::complete`/attempt send IDs must supply actual final identity/attempt values before complete error snapshots can be exported. Historical model/cache flags, probability-derived picks and parsed result JSON cannot supply missing native fields. The C lane has not exported complete calls, result accessors or `thinkthen_error_complete` against unavailable APIs.

The existing sanitizer harness tests public constructors and image lifetime/refusals. Private native tests retain all ten value projections, source locations, null/authored/empty states, member failures, cancellation/deadline refusals, threading, final failure facts and zero-send replay. Root owns the fresh High whole-change review, final integration, full landing gates and the one landing record. Nothing is published or landed as complete parity.
