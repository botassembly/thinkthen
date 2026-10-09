# PostgreSQL request conversion

The adapter starts from `ee9b233f8`. PostgreSQL reads saved questions on the backend thread through its existing role, descriptor, regular-file, link, size and confinement checks. A typed Request receives the admitted definition and caller-owned evidence. PostgreSQL keeps its SQL key, row-id and NULL projections. Evidence paths remain the native client-reader workaround.

## Shared legacy result conversion

Complete atomic results retain `core::result::complete::Atomic::legacy`, a native `DecisionResult`. That carrier already owns the question, threshold, value, probabilities, nearest level and metadata. Its existing result/1 serializer supplies the exact legacy details envelope. A narrow native `CompleteDecision`, `CompleteChoice`, `CompleteTags` and `CompleteScore` conversion to `Details` should reuse that carrier and serializer. PostgreSQL must not construct a second result/1 envelope. The conversion must retain authored readings, successful null, confidence, unknown usage, profile warnings, context identity, sources and observations. Primitive SQL results still use typed value accessors.

`CompleteAnnotated::canonical.legacy` owns the `AnnotateResult`. A native `value_json` projection should reuse the existing annotation value serializer, including named failures. Recognition and relation complete carriers already expose their native legacy actionable values.

Equivalence cases compare the existing native `details_with` and `details_many_with` result/1 documents with Request results converted by the shared seam. Cases cover scalar decide/choose/tag/score, described and banded decisions, shared context, authored `on`, missing usage, profile warnings and successful null. Annotation cases compare existing `annotate_with().value_json()` with the shared projection for successful members and failed named members. Installed SQL cases retain their ordinary scalar, table and details outputs, and count loopback sends.

## Diagnostic planning

The coordinator ruled that `thinkthen_plan` retains the existing shared `Engine::plan_with` diagnostic implementation. Planning makes no judgment and sends nothing. Its existing admission, saved-file authority and estimate output remain the contract. The adapter adds no planner or copied estimate logic. Existing planning checks count loopback requests to retain zero sends.

## Lessons

Complete find originals are `QuestionInput`, which implements `InputEvidence` rather than `Evidence`. Typed composition retains supplied literal text in `RecordEvidence::original()`. The installed duplicate/tie find cases caught a projection that assumed the original remained the bare Text variant. The SQL text-array projection must read the retained literal and use the typed selection index; it must not restore a separate index-bearing evidence wrapper.

The installed rank refusal case caught an error-wording difference before sending: shared Request admission says `the evidence is empty or blank`, while the SQL legacy contract says `evidence is text, not white space`. The coordinator ruled that PostgreSQL adopts the shared Request sentence as the intentional 0.2 migration. The SQL assertion retains Usage classification, zero sends and secrecy. PostgreSQL adds no whitespace validator.

PostgreSQL imports SQLite source through `#[path]`, so Cargo formatting follows those shared modules. Formatting a PostgreSQL file can dirty shared SQLite source, and restoring that incidental diff exposes baseline formatting drift in the PostgreSQL gate. The reviewed SQLite dependency owns that correction. Keep the existing formatter and rerun the affected installed cases after integration; changing a running shell script invalidates that run.
