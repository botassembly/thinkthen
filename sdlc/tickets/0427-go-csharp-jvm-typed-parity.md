# 0427: Complete Go, C# and JVM typed parity

Status: in progress, implementation complete; awaiting root whole-family review and landing. All ten typed native calls and six owned lazy batches are implemented for Go, C#, Java, Kotlin and Scala. All 1,235 required shared cells passed.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Go, C# and JVM expose all ten named functions and typed located file results with the shared request/result/error/storage behavior.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 2, 3 and 5.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Every admitted function/input combination is required; function-specific refusals remain explicit. Named calls are decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Known result fields must have typed accessors, including details, probabilities, final facts, annotation states, spans, relation endpoints and located file/first-line/last-line coordinates. Arbitrary caller payloads may remain JSON; known engine fields may not. Explicit question/file variants avoid guessing paths from text. Own libraries/go, libraries/csharp and libraries/jvm. Execute Java, Kotlin and Scala public consumers separately. Preserve Go native thread pinning and native error lifetimes.
- Proof: Use the existing canonical behavior/settings/types/files fixtures through actual named public consumers. Check admitted text, records, files, question files/sets, descriptions, contexts and options; six errors; complete typed results; cache, record and changed-reading replay with counted zero sends. Compile consumers where the language is static; inspect documented carrier accessors where dynamic. Retain duplicate identities, Unicode/CRLF locations, empty/null states and started-failure facts. No paid calls or new evidence system.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0426 supplies the C carrier foundation; 0442 defines shared metadata. Adopt semantic work from 0406–0414, 0418, 0300 and 0445 after their contracts settle. This family owns its host methods/types, source adapters, canonical adapters and README examples. 0432 owns the common case inventory and generated matrix.

## Design notes

A raw JSON return does not satisfy a typed cell. Keep generic JSON as compatibility. Document genuine language limits as explicit rulings with boundary tests; execute the shared cases with those declared expectations, never skip them.

## Vision and answer identity in 0.2

Second PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, asks 1–4, supersedes the image deferral. Adopt 0447 typed single/multiple-image and image-source inputs for decide/choose/score, 0448 route admission, and 0450 stable full-result identity/proxy reservations. Preserve image order/duplicates and absent text lines; ordinary strings/bytes do not imply images. Execute text-only refusals for the other seven functions, with zero sends. This ticket owns its public typed carriers/consumers; shared native image behavior stays in 0447 and SQL adaptation in 0452. Respect 0449’s single endpoint/key/API-type boundary. Known image/result/identity fields cannot remain raw JSON.

## Current implementation and remaining qualification

The existing Go Engine, C# Engine and Java Door now implement the complete typed interfaces over counted native constructors, all ten complete calls, result/detail/author/observation/error getters, and native file/image/JSONL sources. KotlinComplete and ScalaComplete execute their own public methods with their closed surface tokens. Known output fields are copied into typed owned values before native free; arbitrary caller originals remain JSON. Saved/named/reference questions take explicit native grammar roles. Generic JSON and existing bare calls retain their compatibility behavior.

Six lazy batch starts use the existing native scheduler. A pinned Go goroutine or dedicated C#/JVM creating thread retains its engine for start/next/facts/free; close joins the native work. Rows own their copied views and survive close. Typed failure snapshots preserve stop coordinates, request counts, final facts and attempts. No host parser, reader, cache or scheduler replaces the native implementation.

Actual all-ten record and CRLF line-source tests passed in Go and in separately compiled C#, Java, Kotlin and Scala consumers. The C#/JVM all-ten consumers counted exactly 22 loopback requests each. Existing family shared-case adapters now execute named typed methods and six native batches and emit the existing parity protocol; all required projections use the accepted native-backed C recipes without skips or fabricated passes. Image record/replay checks preserve image order/bytes and answer identity with counted zero replay sends. All backend routes use explicit owned loopback settings and fake keys.

The accepted native/cache and C prerequisites supply the storage/cancellation boundary recipes, all 24 image-admission categories, corrected files/relate and saved backend bodies. Hosts preserve explicit null context and typed find metadata. Atomic find answers decode the published native kind tag 5; function and result kind 7 are unchanged. Absent optional values do not read inactive union members. Image checks compare whole request bodies without imposing request scheduling order, while preserving candidate/image order within each body.

The existing `libraries/go/check.sh`, `libraries/csharp/check.sh` and `libraries/jvm/check.sh` gates all exited 0 on implementation commit `039acb0fa`. Go, C#, Java, Kotlin and Scala each passed 247 unique required shared cases, including all 24 image-admission categories: 1,235 passing cells, zero failures or skips. Each static consumer compiled against its public package; installed-consumer and all-ten checks passed. The gates retain settings, secrecy, cancellation, cache/replay, J1, package and planted-negative checks; Go tests and vet passed. All checks ran offline with owned loopback backends under the lane0 heavy lock and shared-cache read lock, one build job, MemoryMax 10G and MemorySwapMax 1G. No known host implementation gap remains in the required shared inventory.

Root owns the single fresh whole-family review, landing full tests/lint and the one landing record. No landing record, publication, platform qualification or paid call is created by this builder. Ian can overturn host API spellings and the fixture/native directions before landing.
