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

## Generated native session calls

`copybooks/tt-native-generated.cpy` exposes the complete native packet graph and the canonical Request graph. The result declarations come from the measured generated C header. Request records and their mechanical serializer come from Rust's Request schema. Generate them with `python3 sdlc/generators/results/generate.py --target cobol`; use `--check` to check freshness. Rust admits the request and owns every question rule, cache decision and execution failure.

Ordinary callers fill the function's `tt-n-cobol-RequestCall-*` record and call its `TT_SESSION_*` function. The family covers decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Generated selector constants end in `-kind-constant`; callers select a generated question, input or original arm and point `v-value` at that arm's record. Optional member pointers omit absent members. A generated null arm or authored JSON `null` supplies an explicit null where the Request schema permits it. Arrays contain a count and a pointer to consecutive element pointers. The native serializer clones admitted input, so callers can release their request records after a successful call. `examples/session.cob` demonstrates ordinary records, native status codes, retained values and facts, failure kinds and explicit cleanup without constructing or decoding transport JSON.

`TT_SESSION_NEXT` is the synchronous convenience over native nonblocking reads. It returns the native read status and transfers an independent packet owner. `thinkthen_session_result_view` borrows that packet's complete typed graph. Nested field names include their native path, such as `v-facts-presence`, to avoid ambiguous COBOL names. Presence constants distinguish missing, null and value. Read the selected union arm only after checking its kind. Every nested pointer and array remains valid while its packet owner lives, including after `thinkthen_session_cancel`, `thinkthen_session_free` and `thinkthen_engine_free`. Free each transferred packet with `thinkthen_session_result_free`. Cancelling a session requests native cancellation; it does not fabricate final facts. A feed caller uses generated `RequestSessionDescriptor` records with `TT_SESSION_PUSH`, checks native capacity statuses, and explicitly calls `thinkthen_session_finish` when input ends.

Each request text or authored JSON scalar uses a counted readable byte buffer and has an 8192-byte COBOL representation limit. Larger values return `tt-n-cobol-overflow-constant` before creating a session or sending a request. This limit applies per scalar, rather than to the whole serialized request. A caption larger than this representation limit cannot reach the native body or packing limits through COBOL. Shared image body and packing cases therefore require the COBOL overflow code before session creation and zero backend sends; the other consumers exercise the native limits. File and image selectors retain native behavior within their generated records. `TT_SESSION_TEXT` copies a borrowed native UTF-8 value to an 8192-byte, space-padded COBOL field, returns its byte count, and refuses overflow without changing either output. Complete native views retain longer strings through their counted borrowed pointers. Counts use bytes, not Unicode characters. Callers must supply readable, correctly sized, acyclic record graphs and keep borrowed buffers alive during conversion.

The generated declarations support the Linux x86_64 layout measured by its C compiler. Native ABI declarations must be regenerated and consumer-qualified for another platform. The development package includes its matching native library. Final platform and release qualification run at the candidate. `TT_SESSION_DECIDE` replaces `TT-DECIDE`, `TT-CALL` and descriptor-based decisions. The other nine `TT_SESSION_*` functions replace the corresponding generic JSON or descriptor calls. Generated question and input records replace hand-written copybooks and label validation. Rust owns label admission; callers retain no separate label grammar. The frozen 0.1 C declarations and symbols remain in the matching C library.

## Upgrade from the old API

| Removed call or carrier | Replacement |
| --- | --- |
| `TT-ENGINE-NEW` | `thinkthen_engine_new_with` and the matching native engine declarations |
| `TT-DECIDE`, `TT-CALL` and `TT_DECIDE` | `TT_SESSION_DECIDE` with generated question and input records |
| `TT_CHOOSE`, `TT_TAG`, `TT_SCORE`, `TT_FILTER`, `TT_RANK`, `TT_FIND`, `TT_ANNOTATE`, `TT_RECOGNIZE`, `TT_RELATE` | The corresponding named `TT_SESSION_*` function |
| `TT_QUESTION_NEW`, `TT_QUESTION_NEW_AUTHORED`, `TT_QUESTION_PARSE`, `TT_QUESTION_LOAD`, `TT_QUESTION_LOAD_NAMED`, `TT_QUESTION_LOAD_REFERENCE` | Generated question records with text, authored definition, file, name or reference selectors |
| `TT-FILES`, `TT_SOURCE_FILES`, `TT_SOURCE_IMAGE_FILES`, `TT_SOURCE_RECORDS`, `TT_IMAGE_CLONE` | Generated record, file and image input selectors in the named request record |
| `TT_*_BATCH_START` and copied batch carriers | Generated session feed descriptors, `TT_SESSION_PUSH`, `thinkthen_session_finish` and `TT_SESSION_NEXT` |
| `TT-CAPTURE-ERROR`, `TT-PARSE-FIELD`, `TT-JSON-MEMBER` and copied JSON result buffers | Generated admission diagnostics, terminal failures and complete graphs borrowed through `thinkthen_session_result_view`; `TT_SESSION_TEXT` copies counted strings |
| `TT-PLAN` | Direct native C preview remains available; the COBOL package has no named typed plan wrapper |
| `thinkthen.cpy`, `thinkthen-typed.cpy` and hand-written `tt-*.cpy` carriers | Generated `tt-native-generated.cpy`, `tt-n-cobol-RequestCall-*` records and selector constants |
| `TT-VALIDATE-LABELS` and hand-written label validation | Native Rust admission of generated question records |


Compile the new caller with the matching installed C header and library, plus `src/tt_session.c`, `src/tt_requests_generated.c` and `src/tt_session.h`. Supply `-I copybooks` and the native include directory, and use `-fstatic-call -fno-gen-c-decl-static-call -A '-include src/tt_session.h -Wno-incompatible-pointer-types'`. Install the executable beside its packaged `native/lib` directory. The focused consumer stages these package files and proves all ten named calls, primitive false and unresolved null, retained values and failure facts, feed cancellation and counted zero-send refusals. Routine checks unpack the native-bearing archive and run generated consumers against a local backend. Full shared selectors remain release-only.

`TT_ENGINE_USAGE_PERSISTENCE` observes the count-only writer on the existing native engine. `TT_ENGINE_FINISH_USAGE_STATUS` drains this engine's current deltas. Both take the engine by value, followed by the generated `tt-n-complete-usage-persistence-v1` record, an 8192-byte advice field and a binary-double unsigned advice length by reference. Allocate the generated record before calling and free it when finished. Read `v-kind` through the generated disabled, pending, written and failed constants. A zero advice length means no advice; otherwise the space-padded field owns exactly that many copied bytes. The record and advice remain readable after engine destruction.

```cobol
call "TT_ENGINE_FINISH_USAGE_STATUS" using by value engine
   by reference tt-n-complete-usage-persistence-v1 advice-text advice-length
   returning status-code
```

A nonzero return leaves all outputs unchanged. Native operation failures retain their exact return code and report through `thinkthen_session_error_message`, rather than the engine's saved judgment diagnostic. A representation overflow returns `tt-n-cobol-overflow-constant` without truncation. A successful failed persistence observation leaves answers and historical call facts intact and exposes only fixed safe advice. These calls send no judgment request. Written covers current deltas only; Failed remains latched. Only usage-lock acquisition has a deadline; other filesystem work may take longer. The shared focused installed consumer is `../ada/checks/usage_installed.py`, with `THINKTHEN_C_LIBRARY` selecting the matching native library. Final platform qualification remains separate.
