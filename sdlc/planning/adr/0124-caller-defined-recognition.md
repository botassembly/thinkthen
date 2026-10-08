# ADR 0124: Let callers define recognition entities

Status: accepted implementation contract under accepted ticket 0461.

Reviews: accept

Fresh read-only ABI design review accepted the exact additive interface before host adoption.

## Decision

Keep the existing bare-label default question bytes. Treat a nonblank, non-null label description, instructions or entity definition as a caller declaration. Null and blank label descriptions retain their existing absent meaning. Include the full declaration in token, kind, decline and boundary questions. Preserve supplied fields in the canonical description and question identity. Preserve punctuation through the existing BILOU and boundary algorithm. Apply no semantic suppression in custom mode.

Rust adds `RecognizeBuilder::instructions(&str)` and `entity_definition(&str)`. Both refuse blank values. `Recognize::reading()` and `RecognitionReading` expose borrowed authored fields. Saved JSON accepts optional nonblank string members under `recognize`. CLI flags override the corresponding saved members. All existing kind/relations file conflicts remain.

## Additive C interface

Keep every existing public V1 layout unchanged. Add `thinkthen_recognition_task_v1` containing two `thinkthen_optional_string_v1` fields in order: `instructions`, `entity_definition`.

Add `thinkthen_question_new_recognition_v1(engine, spec, author, task, out)`. Spec must be a recognition `thinkthen_question_spec_v1` with kind 9. Author may be NULL, task must be nonnull. The constructor validates flags, UTF-8 and nonblank active strings through the existing native grammar. It copies every active buffer before return. It publishes the owned question only on success. Failures preserve the caller output pointer. Absent task fields ignore their value storage. The existing author constructor and question constructor remain unchanged.

Add `thinkthen_question_recognition_task_v1(question, out)` and `thinkthen_result_recognition_task_v1(result, row, out)`. Readers return borrowed counted strings tied to the immutable owning question or result, never constructor buffers or an engine. They require live owners, writable full-sized output and no concurrent owner destruction. Non-recognition owners/rows, NULL pointers and invalid row ordinals return Usage and preserve output. Successful default declarations return both fields absent. Empty and malformed strings cannot reach these views.

Use the existing failure guard and status conventions. These sidecars avoid extending the settled question and result V1 structures. Host adoption follows fresh review of this exact interface.

## Evidence and limits

The public admission regression failed on the reviewed baseline because the closed saved grammar rejected the new members. The implementation changes native task wording and its carriers. Offline loopback responses prove admission, transport, exact spans, identity and compatibility. They do not establish model accuracy. The priced model evaluation remains with the coordinator and requires a finite allocation. Landing waits for v0.2.0.
