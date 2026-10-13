# ThinkThen Ada

The development package supports Linux x86-64 with GNAT 13.3, gprbuild and Ada 2022. It carries generated sources, `thinkthen.gpr`, the canonical C header and a static native engine. An installed caller needs GNAT and gprbuild. Platform qualification and final distribution assembly belong to the release process. The published 0.1.2 package keeps its published contract.

Extract the development archive into a `thinkthen/` directory beside your application. Import its project:

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

Copy `thinkthen/examples/session_demo.adb` beside this project and run `gprbuild -p -P application.gpr -j2`. Configure a backend before executing a judgment. The installed routine uses a synthetic loopback backend and the archive's static engine.

`Thinkthen.Sessions.Calls` supplies the named typed calls. Pass a question and an input directly: `Decide (Owner, Text_Question ("Does it pass?"), Items_Input ([Text_Item ("Evidence.")]))`. Construction helpers also provide file, name and reference questions, file inputs, and typed recognize and relate definitions. Generated argument records remain available for detailed options. Generated adapters copy those records into native counted carriers. Missing optional fields stay missing, and present false stays false. Only schema-declared arbitrary JSON uses `JSON_Value`. Rust owns request admission, question grammar, files, images, cache and replay.

`Thinkthen.Answers` supplies one blocking answer accessor for every function. For example, `Answer := Decide_Answer (Owner, Completed)` drains through terminal and stores the owned completion before returning a nullable `Decision`. Choose returns a nullable `Choice`, score a number, tag a text list, and rank and filter plain arrays. Find, annotation, recognition and relation use short result types. `Source_Name`, `Target_Name` and `Relation_Name` read relation edge names directly. The shipped example uses this API.

`Completed` retains typed failure information, facts and its completed prefix when an answer accessor raises `Session_Failure` or `Answer_Cardinality_Error`. `Retained_Values` returns owned native values from that prefix. A successful accessor requires one valid final terminal and the scalar or aggregate cardinality prescribed by the shared answer contract. `Collect` also supports reading several projections from the same completion. `Requests_Sent`, `Records`, `Call_Id` and the other fact accessors return native values; optional fact fields preserve missing and null independently. All returned values survive packet and session finalization.

`Thinkthen.Inputs` provides private typed questions, inputs, items and options. Its `Make_*`, `With_*` and `Get_*` functions construct and inspect native values; ordered maps and collections use ordinary Ada arrays. `Text_Question`, `Records_Input`, `Text_Item`, `Recognize_Question` and `Relate_Question` feed the friendly named overloads in `Thinkthen.Sessions.Calls`. `No_Options` preserves omission and `Some_Options` supplies an explicit options value. The private lowering child stays inside the implementation.

A controlled `Session` owns cancellation and closes without waiting for provider work. `Push` and `Finish` accept generated descriptors and reader failures. `Next_Packet` blocks and transfers an independent controlled `Packet`. Recognition returns one aggregate packet followed by the terminal packet. Retain every packet needed by your application. `View` borrows the complete generated result graph while its packet lives, including typed answers, observations, facts, failures, provenance and extensions. Closing a session or engine leaves previously returned packets readable. Join concurrent Ada tasks before finalizing their owners.

Immediate native failures raise the declared exceptions in `Thinkthen.Sessions`. Execution failures appear in the terminal packet and its typed `Status`. A failure before execution can have missing facts. Check presence before dereferencing. `State` distinguishes missing, null and present. Check kind tags before selecting unchecked union arms.

The convenience named calls create an engine using the ordinary environment and read-only configuration. To observe usage from the same engine, declare `Client : Thinkthen.Engine` and pass it first: `Thinkthen.Sessions.Calls.Decide (Client, Owner, Request)`. `Configure` delegates constructor settings to Rust and preserves the existing engine if replacement fails. Its `Failure` carries a typed kind, retryability and owned message.

`Thinkthen.Persistence.Usage_Persistence (Client)` observes the count-only writer. `Finish_Usage_Status (Client)` drains its current deltas. Both return an owned `Observation` with `Disabled`, `Pending`, `Written` or latched `Failed` state and an owned advice string. The observation survives engine finalization. A persistence failure leaves successful packets and their historical facts intact. These calls send no judgment request. Written covers the current engine's deltas. Only usage-lock acquisition has a deadline; other filesystem work may take longer.

`Thinkthen_Session_C` is generated by GNAT from the canonical C header. It retains all frozen C declarations, including compatibility symbols. Macros use a `K_` prefix because Ada identifiers ignore case. Native counts, coordinates and token dimensions keep their full unsigned width. `Text` preserves counted UTF-8 and NUL bytes. `Text` and `Index` raise `Representation_Overflow` before allocating or converting an extent above `Natural'Last`. On this target, the Ada string bound is 2,147,483,647 bytes.

| Removed host call | Current call |
| --- | --- |
| Root `Decide`, `Decide_Many`, `Recognize`, `Relate` | The corresponding `Thinkthen.Sessions.Calls` call with text, records or entity inputs |
| `Thinkthen.Typed.Complete` and `.Batches` | Named session calls, `Push`, `Finish` and `Try_Read` |
| `Thinkthen.Native` handle operations | Controlled session and packet owners; generated C declarations remain available for direct ABI callers |
| `Call`, `Files`, label constructors and JSON readers | Generated request records, native source descriptors and typed packet fields |
| Root cancellation token | `Thinkthen.Sessions.Cancel` |
| Root `Plan` JSON builder | `Thinkthen.Sessions.Calls.Plan` using the same typed request records |

The generated `Thinkthen.Sessions.Calls.Plan (Client, Request)` uses native typed admission and the existing Rust preview serializer. Its returned `Thinkthen.Values.Value` owns the native preview tree and remains valid after request and engine finalization. Read its fields with the typed value accessors. Planning sends no judgment request.

`Thinkthen.Values` supplies owned native value trees. Construct strings, booleans, nulls, exact decimal spellings, arrays and ordered objects directly. Copying a value owns an independent nested tree. `Has_Field` distinguishes a missing object member from a member holding `Null_Item`; `Field_Name` preserves object order. `As_String` preserves counted characters, including embedded zero bytes. No document parser is provided.

The old host judging APIs, handwritten C carriers, JSON validator and label grammar are removed. The generated header package declares the current native session interface. The routine archive consumer exercises the shared named functions, presence, failure facts, cache and replay, files, ordered images, cancellation, cleanup, explicit overflow refusal and compiler-measured layouts. Full parity and platform checks belong to the release suite.
