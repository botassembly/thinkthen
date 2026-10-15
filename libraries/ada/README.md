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

`Thinkthen.Sessions.Calls` supplies the named typed calls. Pass a question and an input directly: `Decide (Owner, Text_Question ("Does it pass?"), Items_Input ([Text_Item ("Evidence.")]))`. Construction helpers also provide file, name and reference questions, file inputs, and typed recognize and relate definitions. Generated builders supply detailed options and settings. Private adapters lower those values into native counted carriers. Missing optional fields stay missing, and present false stays false. Schema-declared arbitrary values and nullable decision booleans use `Thinkthen.Values.Value`. Rust owns request admission, question grammar, files, images, cache and replay.

The named functions in `Thinkthen.Inputs.Calls` return one owned result with `Value`, `Probability`, `Results` and `Facts`. For example, `Refund : constant Decide_Result := Thinkthen.Inputs.Calls.Decide (Text_Question ("Does it pass?"), Text_Input ("Evidence."));` blocks until completion. Read `Refund.Value` for the answer, `Refund.Probability` for its probability, `Refund.Results` for the per-record receipts and `Refund.Facts` for the call facts. Every returned field survives session finalization.

Text and JSON inputs return scalar results for decide, choose, tag, score, annotate and recognize. Records, units, entities, sources and feeds return the corresponding list results; they judge each input separately. Choose uses `Choice` or `Choice_List`, tag uses `Text_List` or `Text_Lists.Vector`, and score uses `Long_Float` or `Score_List`. A `Choice` exposes its text only when `Known` is true. Decide uses nullable boolean values in `Thinkthen.Values.Value` or `Value_List`. Arbitrary JSON answers and original records also use `Thinkthen.Values.Value`; concrete labels, numbers and indexes keep their concrete Ada types.

Filter, rank, find and relate judge the whole input set. Filter returns selected original values in `Value_List`; rank returns zero-based original indexes in `Rank_List`. Find returns its selected unit or null, and relate returns the relation graph. Annotation and recognition preserve their structured answers. `Source_Name`, `Target_Name` and `Relation_Name` read relation edge names directly. `Results` retains the owned per-record receipts in every result.

A probability uses `Number_Field`, which preserves missing, null and present states. Test `Refund.Probability.State = Present_Field` before reading `Refund.Probability.Value`. Decide, choose and find use one such field for scalar answers; decide and choose lists, filter and rank use `Number_Field_List`. Recognition and relation use `Probability_Lists.Vector`, and recognition lists use `Nested_Probability_Lists.Vector`. Tag, score and annotation have null probability. An unresolved decide or find answer has a null `Value` and null probability. An unresolved choose answer has `Value.Known = False` and null probability. A resolved authored null choice also has `Value.Known = False` but retains its numeric probability, so `Known = False` alone does not mean not sure.

Named functions raise `Session_Failure` for a failed call and `Answer_Cardinality_Error` for malformed completion. For packet handling, `Thinkthen.Answers.Collect` drains a session through its terminal and retains the successful prefix, typed failure and facts. The named projections in `Thinkthen.Answers` read that completion. `Retained_Values` reads its owned prefix. `Requests_Sent`, `Records`, `Call_Id` and the other fact accessors return native values; optional fact fields preserve missing and null independently.

`Thinkthen.Inputs` provides private typed questions, inputs, items and options. Its `Make_*`, `With_*` and `Get_*` functions construct and inspect native values; ordered maps and collections use ordinary Ada arrays. `Text_Question`, `Records_Input`, `Text_Item`, `Recognize_Question` and `Relate_Question` feed the friendly named overloads in `Thinkthen.Sessions.Calls`. `No_Options` preserves omission and `Some_Options` supplies an explicit options value. The private lowering child stays inside the implementation.

A controlled `Session` owns cancellation and closes without waiting for provider work. `Thinkthen.Inputs.Calls.Push` accepts a typed `Feed_Item`. Push waits for input room and parks unread answers in session order; it no longer waits on unread answers. Pushing every record before reading consumes memory like collecting results. `Full` remains in `Push_Status`. Its `Finish` overload accepts a typed reader failure. `Thinkthen.Sessions.Finish` finishes a successful producer. `Next_Packet` blocks and transfers an independent controlled `Packet`; typed completion accessors retain the answer and its facts. Join tasks that share a session before closing or finalizing it. Request carriers, C layouts and borrowed views stay in private Ada child packages.

The generated `Thinkthen.Inputs.Calls.Plan (Client, Function_Name, Question, Input, Options)` uses native typed admission and the existing Rust preview serializer. Its returned `Plan_Estimate` owns the native preview tree and remains valid after request and engine finalization. Read all nine fields through typed accessors. Planning sends no judgment request.

`Thinkthen.Values` supplies owned native value trees. Construct strings, booleans, nulls, exact decimal spellings, arrays and ordered objects directly. Copying a value owns an independent nested tree. `Has_Field` distinguishes a missing object member from a member holding `Null_Item`; `Field_Name` preserves object order. `As_String` preserves counted characters, including embedded zero bytes. No document parser is provided.

The old host judging APIs, handwritten C carriers, JSON validator and label grammar are removed. The generated header package declares the current native session interface. The routine archive consumer exercises the shared named functions, presence, failure facts, cache and replay, files, ordered images, cancellation, cleanup, explicit overflow refusal and compiler-measured layouts. Full parity and platform checks belong to the release suite.

Configure an engine with `Thinkthen.Inputs.Calls.Configure (Client, Settings, Error)`. `Make_Engine_Settings` creates the settings container; the generated `With_` builders preserve unset fields. Failed configuration preserves the current engine. Request carriers and C layouts are Ada private child packages.
