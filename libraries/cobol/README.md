# ThinkThen COBOL package

The development 0.2 Linux x86_64 GnuCOBOL 4 archive contains generated copybooks, the source bridge, a complete example and its matching native header, shared library and static library under `native/`. Unpack the archive into your project and compile its bridge with your program. The published 0.1.2 package remains a source wrapper with a separate C archive.

## Build an installed caller

```sh
cobc -x -free -fstatic-call -fno-gen-c-decl-static-call \
  -I copybooks -I native/include \
  -A '-include src/tt_session.h -Wno-incompatible-pointer-types' \
  -o session examples/session.cob src/tt_session.c src/tt_requests_generated.c \
  -L native/lib -lthinkthen -Q '-Wl,-rpath,$ORIGIN/native/lib'
```

Supply `TT_SESSION_QUESTION_FILE`, `TT_SESSION_EVIDENCE` and `TT_SESSION_EXPECT_FAILURE=N` for the example, with normal native backend settings. The executable uses the native library packaged beside it. Use `RETURNING OMITTED` for native void functions, including cancellation and frees.

## Blocking typed answers

The generated `TT_DECIDE`, `TT_CHOOSE`, `TT_TAG`, `TT_SCORE`, `TT_FILTER`, `TT_RANK`, `TT_ANNOTATE`, `TT_FIND`, `TT_RECOGNIZE` and `TT_RELATE` calls accept the engine, question selector, input selector and optional controls separately. Each blocks through the native packet wait and returns its function-specific generated answer record and `tt-n-cobol-AnswerStatus`. Each record embeds its first answer and retains the full owned row table. The rows contain generated decision, choice, score, tag, rank-index, original-record, find-selection, annotation-field, recognition or relation records. Nullable answers keep presence indicators. Filter retains selected original records. Recognition retains entities, boundary proposals and a separate typed edge table with source and target names, spans and probabilities. Relate returns source and target names directly.

Retain input records and their counted bytes until the call returns. The native layer copies input during construction. Answer rows, tag tables, original values and diagnostic records then belong to the status owner. Read them after releasing the engine or input records. Call `TT_ANSWER_FREE` once with `v-owner` of the status, including after a failure when that owner is nonnull. The table owner aliases it on success. Original values are borrowed from this owner; do not free them separately. `examples/blocking.cob` demonstrates two calls over three records.

The blocking calls close caller intake and wait for finite input to finish. Use the session interface below for a producer feed or cancellation during a call. Its existing cancellation function stops native waits without a host polling loop. Blocking calls preserve a cancelled terminal failure if the native operation reports one. They add no cancellation callback or session registry.

`TT_ENGINE_NEW` takes generated `tt-n-cobol-Settings` and an engine output pointer by reference. The settings graph comes from the canonical engine schema and delegates to the typed native constructor. Backend keys remain in the backend environment variable. `TT_PREVIEW` takes a function's counted bytes, its binary-double unsigned length by reference, the question, input and controls, then generated `tt-n-cobol-AnswerPlan` and status records. It copies all canonical Plan fields, including the token band and nullable first body. Native preview supports the existing six atomic functions and refuses unsupported functions without sending.

Status facts retain every canonical field, including attempts, identifiers, token observations, model, cost estimates, persistence observations and typed extension values. Status details retain failure kinds, stopped facts and send/input denials. Generated presence records distinguish missing, null and value. Every counted text copy either fits the scalar capacity or returns the explicit representation overflow; it never truncates.

## Generated native session calls

`copybooks/tt-native-generated.cpy` exposes the complete native packet graph and the canonical Request graph. The result declarations come from the measured generated C header. Request records and their counted native adapters come from Rust's Request schema. Generate them with `python3 sdlc/generators/results/generate.py --target cobol`; use `--check` to check freshness. Rust admits the request and owns every question rule, cache decision and execution failure.

Streaming callers fill the `tt-n-cobol-RequestCall` record and call its `TT_SESSION_*` function. The family covers decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Generated selector constants end in `-kind-k`; callers select a generated question, input or original arm and point `v-value` at that arm's record. Optional member pointers omit absent members. A generated null arm or authored JSON `null` supplies an explicit null where the Request schema permits it. Arrays contain a count and a pointer to consecutive element pointers. Rust clones admitted input, so callers can release their request records after a successful call. `examples/session.cob` demonstrates ordinary records, native status codes, retained values and facts, failure kinds and explicit cleanup without constructing or decoding transport JSON.

`TT_QUESTION_TEXT`, `TT_QUESTION_FILE`, `TT_QUESTION_NAME` and `TT_QUESTION_REFERENCE` fill existing caller-owned selectors. `TT_QUESTION_RECOGNIZE` and `TT_QUESTION_RELATE` accept typed definitions. `TT_TEXT_ITEM`, `TT_JSON_ITEM` and `TT_FILE_INPUT` fill existing item or source records. These builders perform no admission or file reads. Retain their records and bytes until the named session call returns. `TT_JSON_ITEM` takes an opaque value owner built with `TT_VALUE_NULL`, `TT_VALUE_BOOL`, `TT_VALUE_NUMBER`, `TT_VALUE_STRING`, `TT_VALUE_ARRAY` or `TT_VALUE_OBJECT`. Containers copy their children, preserve field order and decimal spelling, and own their storage. `TT_VALUE_FIELD` and `TT_VALUE_ELEMENT` return independent owners; false, explicit null and missing fields remain distinct. Counted text accessors refuse overflow without changing outputs.

`TT_PLAN` accepts a counted function name and the same typed call record. Rust returns an owned native preview value without sending. Read fields with `TT_VALUE_FIELD` and typed value accessors, then release each returned owner with `TT_VALUE_FREE`. Record inputs and the engine can be released after return. The preview supports exactly the native plan functions and refusals.

`TT_SESSION_NEXT` is the synchronous convenience over native blocking reads. It returns the native read status and transfers an independent packet owner. `thinkthen_session_result_view` borrows that packet's complete typed graph. Nested field names include their native path, such as `v-facts-presence`, to avoid ambiguous COBOL names. Presence constants distinguish missing, null and value. Read the selected union arm only after checking its kind. Every nested pointer and array remains valid while its packet owner lives, including after `thinkthen_session_cancel`, `thinkthen_session_free` and `thinkthen_engine_free`. Free each transferred packet with `thinkthen_session_result_free`. Cancelling a session requests native cancellation; it does not fabricate final facts. A feed caller uses generated `RequestSessionDescriptor` records with `TT_SESSION_PUSH`, checks native capacity statuses, and explicitly calls `thinkthen_session_finish` when input ends.

Each request text or authored JSON scalar uses a counted readable byte buffer and has an 8192-byte COBOL representation limit. Larger values return `tt-n-cobol-overflow-k` before creating a session or sending a request. This limit applies per scalar, rather than to the whole serialized request. A caption larger than this representation limit cannot reach the native body or packing limits through COBOL. Shared image body and packing cases therefore require the COBOL overflow code before session creation and zero backend sends; the other consumers exercise the native limits. File and image selectors retain native behavior within their generated records. `TT_SESSION_TEXT` copies a borrowed native UTF-8 value to an 8192-byte, space-padded COBOL field, returns its byte count, and refuses overflow without changing either output. Complete native views retain longer strings through their counted borrowed pointers. Counts use bytes, not Unicode characters. Callers must supply readable, correctly sized, acyclic record graphs and keep borrowed buffers alive during conversion.

The generated declarations support the Linux x86_64 layout measured by its C compiler. Native ABI declarations must be regenerated and consumer-qualified for another platform. The development package includes its matching native library. Final platform and release qualification run at the candidate. `TT_SESSION_DECIDE` replaces `TT-DECIDE`, `TT-CALL` and descriptor-based decisions. The other nine `TT_SESSION_*` functions replace the corresponding generic JSON or descriptor calls. Generated question and input records replace hand-written copybooks and label validation. Rust owns label admission; callers retain no separate label grammar. The frozen 0.1 C declarations and symbols remain in the matching C library.

## Upgrade from the old API

| Removed call or carrier | Replacement |
| --- | --- |
| `TT-ENGINE-NEW` | `TT_ENGINE_NEW` with generated settings |
| `TT-DECIDE` and `TT-CALL` | `TT_SESSION_DECIDE` with generated question and input records |
| Generic JSON judging calls | The corresponding generated blocking call or named session |
| `TT_QUESTION_NEW`, `TT_QUESTION_NEW_AUTHORED`, `TT_QUESTION_PARSE`, `TT_QUESTION_LOAD`, `TT_QUESTION_LOAD_NAMED`, `TT_QUESTION_LOAD_REFERENCE` | Generated question records with text, authored definition, file, name or reference selectors |
| `TT-FILES`, `TT_SOURCE_FILES`, `TT_SOURCE_IMAGE_FILES`, `TT_SOURCE_RECORDS`, `TT_IMAGE_CLONE` | Generated record, file and image input selectors in the named request record |
| `TT_*_BATCH_START` and copied batch carriers | Generated session feed descriptors, `TT_SESSION_PUSH`, `thinkthen_session_finish` and `TT_SESSION_NEXT` |
| `TT-CAPTURE-ERROR`, `TT-PARSE-FIELD`, `TT-JSON-MEMBER` and copied JSON result buffers | Generated admission diagnostics, terminal failures and complete graphs borrowed through `thinkthen_session_result_view`; `TT_SESSION_TEXT` copies counted strings |
| `TT-PLAN` | `TT_PREVIEW` with the generated Plan record |
| `thinkthen.cpy`, `thinkthen-typed.cpy` and hand-written `tt-*.cpy` carriers | Generated `tt-native-generated.cpy`, `tt-n-cobol-RequestCall-*` records and selector constants |
| `TT-VALIDATE-LABELS` and hand-written label validation | Native Rust admission of generated question records |


Compile the new caller with the matching installed C header and library, plus `src/tt_session.c`, `src/tt_requests_generated.c` and `src/tt_session.h`. Supply `-I copybooks` and the native include directory, and use `-fstatic-call -fno-gen-c-decl-static-call -A '-include src/tt_session.h -Wno-incompatible-pointer-types'`. Install the executable beside its packaged `native/lib` directory. The focused consumer stages these package files and proves all ten named calls, primitive false and unresolved null, retained values and failure facts, feed cancellation and counted zero-send refusals. Routine checks unpack the native-bearing archive and run generated consumers against a local backend. Full shared selectors remain release-only.

`TT_ENGINE_USAGE_PERSISTENCE` observes the count-only writer on the existing native engine. `TT_ENGINE_FINISH_USAGE_STATUS` drains this engine's current deltas. Both take the engine by value, followed by the generated `tt-n-complete-usage-persistence-v1` record, an 8192-byte advice field and a binary-double unsigned advice length by reference. Allocate the generated record before calling and free it when finished. Read `v-kind` through the generated disabled, pending, written and failed constants. A zero advice length means no advice; otherwise the space-padded field owns exactly that many copied bytes. The record and advice remain readable after engine destruction.

```cobol
call "TT_ENGINE_FINISH_USAGE_STATUS" using by value engine
   by reference tt-n-complete-usage-persistence-v1 advice-text advice-length
   returning status-code
```

A nonzero return leaves all outputs unchanged. Native operation failures retain their exact return code and report through `thinkthen_session_error_message`, rather than the engine's saved judgment diagnostic. A representation overflow returns `tt-n-cobol-overflow-k` without truncation. A successful failed persistence observation leaves answers and historical call facts intact and exposes only fixed safe advice. These calls send no judgment request. Written covers current deltas only; Failed remains latched. Only usage-lock acquisition has a deadline; other filesystem work may take longer. The shared focused installed consumer is `../ada/checks/usage_installed.py`, with `THINKTHEN_C_LIBRARY` selecting the matching native library. Final platform qualification remains separate.
