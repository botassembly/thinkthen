# ThinkThen Ada

The development package supports Linux x86-64 with GNAT 13.3, gprbuild and Ada 2022. Its generated sources and exported `thinkthen.gpr` travel with `native/include/thinkthen.h` and `native/lib/libthinkthen.a`. The project supplies package-relative static linking and platform dependencies. An installed caller needs GNAT and gprbuild. Final platform distribution assembly belongs to the release process; the published 0.1.2 package retains its existing contract.

Extract the development package into a `thinkthen/` directory beside your application. Import its project:

```ada
with "thinkthen/thinkthen.gpr";
project Application is
   for Main use ("session_demo.adb");
   for Object_Dir use "obj";
   package Compiler is
      for Default_Switches ("Ada") use ("-gnat2022");
   end Compiler;
end Application;
```

Copy `thinkthen/examples/session_demo.adb` beside this project, then build with `gprbuild -p -P application.gpr -j2`. Configure a backend before executing a judgment. The installed focused check uses a synthetic loopback backend and the package's bundled static engine.

`Thinkthen.Sessions.Calls` provides named typed calls for all ten functions. Each accepts its generated `Thinkthen.Requests.T_RequestCall_*` record. Callers populate records, discriminated alternatives and vectors; the generated transport serializes them internally. Absent optional records omit their member. A present false remains false. Schema-declared arbitrary JSON content uses `JSON_Value`; native Rust owns its parsing and admission. The binding copies no label grammar, semantic validator or cache policy into this family.

A controlled `Session` owns cancellation and closes without waiting for provider work. `Push` and `Finish` accept generated descriptors and reader failures. `Try_Read` transfers a controlled independent `Packet`; `View` borrows the complete generated graph while that packet lives, including failure facts and extension JSON. Every nested pointer ends at packet finalization. Join concurrent Ada tasks before closing their owners. Immediate failures raise typed exceptions; execution failures retain a declared `Status` and the typed terminal failure. A terminal before execution can have missing facts. Check presence before dereferencing.

`Thinkthen_Session_C` is generated from the canonical C header through GNAT. C macros use a `K_` prefix because Ada identifiers ignore case and some C constants share names with types. Its fixed records and discriminated unchecked unions preserve the C layout. Check native kind and presence tags before selecting union arms. `State` distinguishes missing, null and present values. Native counts retain their full unsigned width. `Text` and `Index` refuse extents above `Natural'Last` with `Representation_Overflow` before allocation or conversion.

| Retained call | Generated Session call |
| --- | --- |
| `Thinkthen.Decide` or `Thinkthen.Typed.Complete.Decide` | `Thinkthen.Sessions.Calls.Decide` with `T_RequestCall_decide` |
| Other `Thinkthen.Typed.Complete` named calls | Same function under `Thinkthen.Sessions.Calls` with its generated request record |
| JSON `Thinkthen.Call` | Named generated request records and typed packet views |

The retained APIs remain reachable until complete installed migration parity permits removal. The following sections describe their compatibility contracts.

The four retained compatibility procedures `Decide`, `Decide_Many`, `Recognize`, and `Relate` return their value plus the run facts from the same native operation. Facts, recognize and relate values, and plans are the engine's JSON text, as `specification/result.schema.json` describes; Ada has no standard JSON value, so the package does not copy them into records. `Member (Text, Name)` returns one object member's JSON text and `Element (Text, Index)` one array element's, counting from 1; each returns `""` when absent and ignores members it is not asked for. A reported model can exist without usage. `Decide_Many` still refuses an empty input before sending. Every sending procedure takes `Deadline_Ms` (`-1` for none) and an optional `Cancel_Token`.

`Plan (Client, Verb, Question, Input, Result, Error, Settings_JSON)` previews a `decide`, `choose`, `score` or `tag` call through `thinkthen_plan_json` without a key, a cache read or a send. A question starting with `{` is a question object and is sent as written; other text is the bare question. `Result` is the result schema's `plan` object as JSON text. A usage failure, such as settings `{"batch":0}`, arrives in `Error` with kind `Usage`. `Configure` passes engine settings such as `{"max_requests_total":0}` to `thinkthen_engine_new_with` unchanged.

`Engine` and `Cancel_Token` are controlled owners. Do not finalize an owner while another Ada task is using it: a joined task scope around the call is required. C strings are copied and released even on exceptions. Failure kind, retryability and message are captured on the calling Ada task; raw C codes stay private. `Null_Answer` is distinct from `Failed_Answer`. `Bare_Labels` accepts an ordered list and `Label_Descriptions` serializes it to a bare JSON list; ordered label/description pairs serialize to a map. A set cannot mix bare and described entries: the usage exception names the offending label. The pinned question-file schema accepts those two forms, not per-label optional descriptions in a mixed set. Structured descriptions are kept as the original JSON object, not flattened. The compatibility methods retain JSON for open-shaped results. The complete API below exposes known native fields through typed records. `Decode_Field` and `Annotation` read one annotate member as `Null_Answer` (unresolved), `Failed_Answer` with its `Error_Kind` and cause, or an answered value, and raise `Constraint_Error` for any other object. Recognize entity start/end/length offsets count **Unicode code points, zero-based with exclusive end** (not UTF-8 bytes or UTF-16 units). The Ada layer validates JSON syntax; the pinned native Rust parser authoritatively enforces the question grammar. The gate proves representative schema parity with `specification/question-file.schema.json` (positive and negative cases). There is no Ada-side Draft 2020-12 JSON Schema validator and no new runtime dependency. The product check runs the current J1 corpus through the public Ada JSON door. `Configure` accepts JSON settings; `Failure_Facts` returns a caller-owned snapshot after a started failure.

`Configure(Client, Settings_JSON, Error)` accepts `{"backend":"local"}` to select the `local` entry in the read-only ThinkThen configuration. Use `{"base_url":"http://localhost:11434/v1"}` for a direct address instead. A named backend supplies its address, model, wire settings and key environment variable; explicit constructor settings take precedence. Omitting `backend` preserves ordinary environment/default selection. A missing or invalid name fails before sending.

Explicit files and folders use the [library reader contract](../files.md), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments.

## Native typed API

These APIs are built from this 0.2 source checkout and require its matching C header and library.

`Thinkthen.Typed` owns native `Question`, `Source` and `Image` handles. `New_Question` takes counted `Question_Spec_V1` fields, including descriptions, ordered choices, thresholds, pointer projections, rank/annotation members, entity kinds and relation specifications. Its authored overload accepts optional name, wording version and item/context declarations. `Parse_Question` imports saved question grammar with an explicit loader role. `Load_Question` takes an explicit file; `Load_Named` and `Load_Reference` delegate names/references and role admission to the native loader. `Author` returns typed resolved metadata. No Ada parser, path search, cache or scheduler enters these calls.

`Records` clones typed records with arbitrary authored original/context values, ordered choices and image handles. `Files` loads native line, window, whole-file, image-file or JSONL sources. `Image_Files` admits image mode before opening any path. `Clone_Image` preserves counted PNG/JPEG bytes and optional filenames; `Image_View` exposes bytes, media and dimensions. Decide, choose and score admit ordered single/multiple images. The other seven functions return Usage before a send. Native route limits govern all images; filenames and ordinary strings never imply image input.

`Thinkthen.Typed.Complete` exposes `Decide`, `Choose`, `Tag`, `Score`, `Filter`, `Rank`, `Find`, `Annotate`, `Recognize` and `Relate` over the same engine, question, source and controls. Each writes an independently owned controlled `Result` on success. Overloads read typed rows. `Row` reads a final output position and preserves its original input index, including equal inputs and rank reordering. `Summary`, `Observation`, `Details`, `Observation_Details`, author accessors, located recognition/relation accessors and rank-member accessors expose result/2 IDs, probabilities, final facts, attempts, provenance, question inputs, member states, spans and endpoints. `Error_Snapshot` owns the calling task's failed-build or failed-call snapshot. Pre-start failures have no invented facts; started failures preserve completed work and final counts.

`Thinkthen.Typed.Complete.Batches` exposes six owned lazy record batches through named `Decide`, `Choose`, `Tag`, `Score`, `Filter` and `Annotate` starts. `Next` returns owned results until an absent result signals exhaustion. It preserves output on terminal error. `Facts` returns an owned final summary after termination. Start clones children and controls; children may finalize afterward. The engine remains live, and start, next, facts and finalization run exclusively on the creating Ada task. Finalization stops and joins native work. Aggregate rank/find/recognize/relate retain complete-call semantics.

`Thinkthen.Native`, `.Inputs`, `.Results` and `.Batches` expose the same counted operations for applications that manage native handles explicitly. Free each nonnull handle exactly once after all borrowers finish. The controlled API supplies that ownership automatically. Every complete/batch call fixes surface `ada`. `Thinkthen_C_Inputs`, `_Answers`, `_Entities`, `_Metadata`, `_Rows`, `_Events` and `_Extensions` define the native records. Check presence flags and discriminators before selecting optional/union arms. Result views borrow immutable storage and survive engine destruction while their result owner remains live. Constructors clone caller storage and nested handles; a failed replacement preserves the prior owner. Join callers before owner finalization.

`Thinkthen.Buffers` owns counted bytes. `Thinkthen.Views.Value` copies native strings to Ada `String`, and zero-based `Element` overloads copy list entries after checking range, alignment and address arithmetic. UTF-8, CRLF and NUL bytes remain counted. Native counts, coordinates and tokens retain 64-bit precision; costs remain decimal text. Ruling: copying into Ada `String` admits at most `Natural'Last` bytes, 2,147,483,647 on this Linux x86_64 GNAT target. Larger copies refuse before allocation. Lists keep native counts. There is no 8 KB typed-data cap. Existing generic JSON methods remain compatible.

`examples/native.adb` executes a 9,000-byte counted record, changes the caller's buffers after construction and reads a typed result after the engine and inputs finalize. Compile it with `gnatmake -gnat2022 -Isrc examples/native.adb -D obj -o obj/native -largs -L/path/to/native/lib -lthinkthen`; supply `TT_NATIVE_SETTINGS` with the engine's JSON settings. The source/package gate executes the shared named-function cases, native files/images, errors, cache/record/replay and lazy batches through compiled public consumers and counted loopback sends.

Rank-set rows retain every member in saved declaration order. Each member exposes its native positive rank position, probability, answer identity, author declarations and complete details. Details preserve independently reported token dimensions and source batch sizes. Parent and member metadata overlap; read final call facts for invocation usage.
`Rank_Member_Details` borrows the member’s own `Details_V1` from its retained `Result`; `Member_Author` reads that member’s authored declarations.

`Thinkthen.Persistence.Usage_Persistence (Client)` observes the live count-only writer on the existing `Thinkthen.Engine`. `Finish_Usage_Status (Client)` drains its current deltas and returns the same owned `Observation`, with a `Persistence_State` of `Disabled`, `Pending`, `Written` or latched `Failed`. `Advice` is an owned `Unbounded_String`; an empty value means no advice. The observation remains readable after the controlled engine finalizes. A persistence failure leaves successful answers and their historical call facts intact. These calls send no judgment request. Written covers only this engine's current deltas. Only usage-lock acquisition has a deadline; other filesystem work may take longer. Operation failures raise the existing `Thinkthen.Sessions` typed exceptions using the calling thread's session diagnostic.

```ada
with Thinkthen.Persistence;
-- After the existing typed judgment call:
Saved := Thinkthen.Persistence.Finish_Usage_Status (Client);
```

Declare `Saved : Thinkthen.Persistence.Observation`. The Linux focused installed consumer is `checks/usage_installed.py`; it also exercises the COBOL package with a matching native library selected through `THINKTHEN_C_LIBRARY`. It does not qualify a release archive or another platform.
