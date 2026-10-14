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

`Thinkthen.Sessions.Calls` supplies the named typed calls. Pass a question and an input directly: `Decide (Owner, Text_Question ("Does it pass?"), Items_Input ([Text_Item ("Evidence.")]))`. Construction helpers also provide file, name and reference questions, file inputs, and typed recognize and relate definitions. Generated builders supply detailed options and settings. Private adapters lower those values into native counted carriers. Missing optional fields stay missing, and present false stays false. Only schema-declared arbitrary values use `Thinkthen.Values.Value`. Rust owns request admission, question grammar, files, images, cache and replay.

`Thinkthen.Answers` supplies one blocking answer accessor for every function. For example, `Answer := Decide_Answer (Owner, Completed)` drains through terminal and stores the owned completion before returning a nullable `Decision`. Choose returns a nullable `Choice`, score a number, tag a text list, and rank returns zero-based original indexes and filter returns the original selected values in plain arrays. Find, annotation, recognition and relation use short result types. `Source_Name`, `Target_Name` and `Relation_Name` read relation edge names directly. The shipped example uses this API.

`Completed` retains typed failure information, facts and its completed prefix when an answer accessor raises `Session_Failure` or `Answer_Cardinality_Error`. `Retained_Values` returns owned native values from that prefix. A successful accessor requires one valid final terminal and the scalar or aggregate cardinality prescribed by the shared answer contract. `Collect` also supports reading several projections from the same completion. `Requests_Sent`, `Records`, `Call_Id` and the other fact accessors return native values; optional fact fields preserve missing and null independently. All returned values survive packet and session finalization.

`Thinkthen.Inputs` provides private typed questions, inputs, items and options. Its `Make_*`, `With_*` and `Get_*` functions construct and inspect native values; ordered maps and collections use ordinary Ada arrays. `Text_Question`, `Records_Input`, `Text_Item`, `Recognize_Question` and `Relate_Question` feed the friendly named overloads in `Thinkthen.Sessions.Calls`. `No_Options` preserves omission and `Some_Options` supplies an explicit options value. The private lowering child stays inside the implementation.

A controlled `Session` owns cancellation and closes without waiting for provider work. `Thinkthen.Inputs.Calls.Push` accepts a typed `Feed_Item`, and its `Finish` overload accepts a typed reader failure. `Thinkthen.Sessions.Finish` finishes a successful producer. `Next_Packet` blocks and transfers an independent controlled `Packet`; typed completion accessors retain the answer and its facts. Join tasks that share a session before closing or finalizing it. Request carriers, C layouts and borrowed views stay in private Ada child packages.

The generated `Thinkthen.Inputs.Calls.Plan (Client, Function_Name, Question, Input, Options)` uses native typed admission and the existing Rust preview serializer. Its returned `Plan_Estimate` owns the native preview tree and remains valid after request and engine finalization. Read all nine fields through typed accessors. Planning sends no judgment request.

`Thinkthen.Values` supplies owned native value trees. Construct strings, booleans, nulls, exact decimal spellings, arrays and ordered objects directly. Copying a value owns an independent nested tree. `Has_Field` distinguishes a missing object member from a member holding `Null_Item`; `Field_Name` preserves object order. `As_String` preserves counted characters, including embedded zero bytes. No document parser is provided.

The old host judging APIs, handwritten C carriers, JSON validator and label grammar are removed. The generated header package declares the current native session interface. The routine archive consumer exercises the shared named functions, presence, failure facts, cache and replay, files, ordered images, cancellation, cleanup, explicit overflow refusal and compiler-measured layouts. Full parity and platform checks belong to the release suite.

Configure an engine with `Thinkthen.Inputs.Calls.Configure (Client, Settings, Error)`. `Make_Engine_Settings` creates the settings container; the generated `With_` builders preserve unset fields. Failed configuration preserves the current engine. Request carriers and C layouts are Ada private child packages.
