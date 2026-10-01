with Ada.Command_Line;
with Ada.Directories;
with Ada.Environment_Variables;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Strings.Fixed;
with Ada.Text_IO; use Ada.Text_IO;
with Interfaces; use Interfaces;
with Interfaces.C; use Interfaces.C;
with Interfaces.C.Strings; use Interfaces.C.Strings;
with Thinkthen_C; use Thinkthen_C;
with Thinkthen;
with System; use System;
procedure Main is
   E : constant Handle := Engine_New;
   A : aliased Answer := (Outcome => 123, Probability => -1.0);
   F : Thinkthen.Failure;
   T : Handle := Null_Handle;
   Barrier : constant String := Ada.Environment_Variables.Value ("TT_BARRIER_DIR", "");
   procedure Require (Condition : Boolean; Note : String) is
   begin
      if not Condition then raise Program_Error with Note; end if;
   end Require;
   procedure Arrive (Name : String) is
   begin
      for I in 1 .. 2_000 loop
         exit when Ada.Directories.Exists (Barrier & "/arrived-" & Name);
         delay 0.005;
      end loop;
      Require (Ada.Directories.Exists (Barrier & "/arrived-" & Name), "no counted arrival: " & Name);
   end Arrive;
   procedure Release (Name : String) is
      File : File_Type;
   begin
      Create (File, Out_File, Barrier & "/release-" & Name); Close (File);
   end Release;
   procedure Expect (State : String; Expected : int; Deadline : Interfaces.Integer_64 := No_Deadline; Token : Handle := Null_Handle) is
   begin
      A := (Outcome => 123, Probability => -1.0);
      Thinkthen.Decide (E, Token, "Is it?", State, Deadline, A, F);
      Require (F.Code = Expected, State & ": code" & F.Code'Image & " expected" & Expected'Image & " " & To_String (F.Message));
      if Expected /= 0 then
         Require (A.Outcome = 123 and A.Probability = -1.0, State & ": result overwritten");
         Require (not F.Retryable, State & ": retryable");
      else
         Require (A.Outcome = (if State = "no" then 0 else 1), State & ": wrong outcome" & A.Outcome'Image & " probability" & A.Probability'Image);
      end if;
      Put_Line (State & " code=" & F.Code'Image & " outcome=" & A.Outcome'Image);
   end Expect;
   Result : Unbounded_String;
   JSON_Requests : constant array (Positive range 1 .. 10) of Unbounded_String :=
      (To_Unbounded_String ("{""decide"":""Is it?"",""evidence"":""json-decide"",""details"":true}"),
       To_Unbounded_String ("{""choose"":""Which team?"",""options"":[""first"",""second""],""evidence"":""choose""}"),
       To_Unbounded_String ("{""tag"":""Which labels?"",""labels"":[""first"",""second""],""evidence"":""tag""}"),
       To_Unbounded_String ("{""score"":""What level?"",""levels"":[""Low."",""High.""],""evidence"":""score""}"),
       To_Unbounded_String ("{""filter"":""Is it?"",""records"":[""filter-one"",""filter-two""]}"),
       To_Unbounded_String ("{""rank"":""Is it?"",""records"":[""rank-one"",""rank-two""]}"),
       To_Unbounded_String ("{""find"":""Which line?"",""units"":[""find-one"",""find-two""]}"),
       To_Unbounded_String ("{""annotate"":{""version"":1,""questions"":{""check"":{""decide"":""Is it?""}}},""records"":[""annotate-one""]}"),
       To_Unbounded_String ("{""recognize"":{""kinds"":{""person"":""A person's name.""}},""version"":1,""evidence"":""Maria Chen""}"),
       To_Unbounded_String ("{""relate"":{""relations"":[{""name"":""caused_by"",""source"":""alert"",""target"":""alert""}]},""version"":1,""records"":[{""name"":""First"",""kind"":""alert""},{""name"":""Second"",""kind"":""alert""}]}"));
begin
   Require (E /= Null_Handle, "engine build failed: " & Value (Error_Message (E)));
   Require (Answer'Size = 128 and Answer'Alignment = 8, "Ada answer ABI mismatch");
   if Ada.Command_Line.Argument_Count > 0 and then Ada.Command_Line.Argument (1) = "strict" then
      declare
         Local : constant Handle := Token_New;
         Code : int := -1;
         Output : aliased Answer := (Outcome => 123, Probability => -1.0);
         task Caller is entry Start; end Caller;
         task body Caller is
            Err : Thinkthen.Failure;
         begin
            accept Start;
            Thinkthen.Decide (E, Local, "Is it?", "hold-scalar", No_Deadline, Output, Err);
            Code := Err.Code;
         end Caller;
      begin
         Caller.Start; Arrive ("hold-scalar"); Cancel (Local); Cancel (Local); Release ("hold-scalar");
         -- Wait for the calling task before freeing its token.
         for I in 1 .. 2_000 loop
            exit when Caller'Terminated;
            delay 0.005;
         end loop;
         Require (Caller'Terminated and Code = 5 and Output.Outcome = 123 and Output.Probability = -1.0,
                  "held scalar must return code 5 and preserve output");
         Token_Free (Local);
      end;
      Expect ("recovery-scalar", 0);
      Put_Line ("STRICT_ADA_CANCEL_PASS");
   elsif Ada.Command_Line.Argument_Count > 0 and then Ada.Command_Line.Argument (1) = "bulk" then
      declare
         Q : chars_ptr := New_String ("Is it?");
         Texts : aliased Text_Array (0 .. 2) := (New_String ("first"), New_String ("second"), New_String ("third"));
         Lens : aliased Length_Array (0 .. 2) := (5, 6, 5);
         Answers : aliased Answer_Array (0 .. 2) := (others => (Outcome => 123, Probability => -1.0));
         Code : int;
      begin
         Code := Decide_Many (E, Q, Texts'Address, Lens'Address, 3, Answers'Address);
         Require (Code = 0, "bulk code" & Code'Image & " " & Value (Error_Message (E)));
         Require (Answers (0).Probability = 0.9 and Answers (1).Probability = 0.1 and Answers (2).Probability = 0.6,
                  "bulk input order/ABI");
         for I in Texts'Range loop Free (Texts (I)); end loop;
         Free (Q);
         Put_Line ("ADA_BULK_PASS");
      end;
   elsif Ada.Command_Line.Argument_Count > 0 and then Ada.Command_Line.Argument (1) = "reverse" then
      declare
         Q : chars_ptr := New_String ("Is it?");
         Texts : aliased Text_Array (0 .. 1) := (New_String ("hold-reverse-1"), New_String ("hold-reverse-2"));
         Lens : aliased Length_Array (0 .. 1) := (14, 14);
         Answers : aliased Answer_Array (0 .. 1) := (others => (Outcome => 123, Probability => -1.0));
         Code : int := -1;
         task Caller is entry Start; end Caller;
         task body Caller is
         begin
            accept Start;
            Code := Decide_Many (E, Q, Texts'Address, Lens'Address, 2, Answers'Address);
         end Caller;
      begin
         -- Both rows now share one packed request (ADR 0055); the fixture
         -- verifies both quoted rows and the original ordered answers.
         Caller.Start; Arrive ("hold-reverse-1");
         Release ("hold-reverse-1");
         for I in 1 .. 2_000 loop
            exit when Caller'Terminated;
            delay 0.005;
         end loop;
         Require (Caller'Terminated and Code = 0 and Answers (0).Probability = 0.9 and
                  Answers (1).Probability = 0.1, "reverse completion reordered bulk results");
         for I in Texts'Range loop Free (Texts (I)); end loop;
         Free (Q);
         Put_Line ("ADA_REVERSE_BULK_PASS");
      end;
   elsif Ada.Command_Line.Argument_Count > 0 and then Ada.Command_Line.Argument (1) = "typed-json" then
      declare
         Spec : chars_ptr := New_String ("{""version"":1,""recognize"":{""kinds"":{""person"":""A person's name.""}}}");
         Text : chars_ptr := New_String ("John Smith");
         Output : aliased chars_ptr := Null_Ptr;
         Out_Len : aliased size_t := 777;
         Code : int;
      begin
         Code := Recognize (E, Spec, Text, 10, Output'Access, Out_Len'Access);
         Require (Code = 0 and Output /= Null_Ptr, "typed recognize: " & Value (Error_Message (E)));
         Require (size_t (Length (To_Unbounded_String (Value (Output)))) = Out_Len and Out_Len > 10,
                  "typed recognize byte length");
         Put_Line ("typed recognize bytes=" & Out_Len'Image);
         Free_String (Output); Free (Spec); Free (Text);
      end;
      declare
         Spec : chars_ptr := New_String ("{""version"":1,""relate"":{""relations"":[{""name"":""caused_by"",""source"":""alert"",""target"":""alert""}]}}");
         Records : aliased Text_Array (0 .. 1) := (New_String ("{""name"":""Third"",""kind"":""alert""}"),
                                                  New_String ("{""name"":""Fourth"",""kind"":""alert""}"));
         Lengths : aliased Length_Array (0 .. 1) := (31, 32);
         Output : aliased chars_ptr := Null_Ptr;
         Out_Len : aliased size_t := 777;
         Code : int;
      begin
         Lengths (0) := size_t (Length (To_Unbounded_String (Value (Records (0)))));
         Lengths (1) := size_t (Length (To_Unbounded_String (Value (Records (1)))));
         Code := Relate (E, Spec, Records'Address, Lengths'Address, 2, Output'Access, Out_Len'Access);
         Require (Code = 0 and Output /= Null_Ptr, "typed relate: " & Value (Error_Message (E)));
         Require (size_t (Length (To_Unbounded_String (Value (Output)))) = Out_Len and Out_Len > 10, "typed relate byte length");
         Put_Line ("typed relate bytes=" & Out_Len'Image);
         Free_String (Output);
         for I in Records'Range loop Free (Records (I)); end loop;
         Free (Spec);
      end;
      Put_Line ("ADA_TYPED_JSON_PASS");
   elsif Ada.Command_Line.Argument_Count > 0 and then Ada.Command_Line.Argument (1) = "boundaries" then
      declare
         Z : constant String := "a" & Character'Val (0) & "b";
         Q : chars_ptr := New_String ("Is it?");
         Text : chars_ptr := New_String (Z);
         Output : aliased Answer := (Outcome => 123, Probability => -1.0);
         Code : int;
         Caught : Boolean := False;
         Saved : Unbounded_String;
      begin
         Code := Decide (E, Q, Text, 3, Output'Access);
         Require (Code = 0 and Output.Outcome = 1, "interior NUL byte-counted evidence");
         Free (Text); Free (Q);
         begin
            Thinkthen.JSON (E, Null_Handle, "{""decide"":""Is""}" & Character'Val (0) & "junk", No_Deadline, Result, F);
         exception
            when Constraint_Error => Caught := True;
         end;
         Require (Caught, "interior NUL in C-string request must be rejected");
         Expect ("negative-budget", 1, -2);
         Expect ("status-401", 2);
         Saved := F.Message;
         Require (Ada.Strings.Fixed.Index (To_String (Saved), "401") > 0, "wrong failure message");
         Expect ("after-error", 0);
         Require (Error_Code (E) = 2 and Value (Error_Message (E)) = To_String (Saved),
                  "success cleared calling task error");
         Expect ("failure-two", 2);
         Require (Ada.Strings.Fixed.Index (To_String (F.Message), "403") > 0 and
                  Ada.Strings.Fixed.Index (To_String (Saved), "401") > 0,
                  "saved Ada copy changed after next failure");
         declare
            Other : constant Handle := Engine_New;
            Other_Output : aliased Answer := (Outcome => 123, Probability => -1.0);
            Other_Error : Thinkthen.Failure;
         begin
            Require (Other /= Null_Handle, "second engine build");
            Thinkthen.Decide (Other, Null_Handle, "Is it?", "other-engine-error", No_Deadline,
                             Other_Output, Other_Error);
            Require (Other_Error.Code = 2 and Other_Output.Outcome = 123,
                     "second engine failure");
            Engine_Free (Other);
            Require (Ada.Strings.Fixed.Index (To_String (Other_Error.Message), "401") > 0 and
                     Ada.Strings.Fixed.Index (To_String (Saved), "401") > 0,
                     "error snapshot invalid after engine teardown");
         end;
         Put_Line ("ADA_BOUNDARIES_PASS");
      end;
   elsif Ada.Command_Line.Argument_Count > 0 and then Ada.Command_Line.Argument (1) = "concurrent" then
      declare
         type Failure_Array is array (1 .. 3) of Thinkthen.Failure;
         Failures : Failure_Array;
         Results : array (1 .. 3) of Answer := (others => (Outcome => 123, Probability => -1.0));
         task type Caller (Index : Positive) is entry Start; end Caller;
         task body Caller is
            State : constant String := (if Index = 1 then "failure-one" elsif Index = 2 then "failure-two" else "success");
            Local : aliased Answer := (Outcome => 123, Probability => -1.0);
         begin
            accept Start;
            Thinkthen.Decide (E, Null_Handle, "Is it?", State, No_Deadline, Local, Failures (Index));
            Results (Index) := Local;
         end Caller;
         First : Caller (1);
         Second : Caller (2);
         Third : Caller (3);
      begin
         First.Start; Second.Start; Third.Start;
         for I in 1 .. 2_000 loop
            exit when First'Terminated and Second'Terminated and Third'Terminated;
            delay 0.005;
         end loop;
         Require (First'Terminated and Second'Terminated and Third'Terminated, "concurrent tasks stalled");
         Require (Failures (1).Code = 2 and Failures (2).Code = 2 and Failures (3).Code = 0,
                  "concurrent codes");
         Require (Ada.Strings.Fixed.Index (To_String (Failures (1).Message), "401") > 0 and
                  Ada.Strings.Fixed.Index (To_String (Failures (2).Message), "403") > 0,
                  "cross-task failure message");
         Require (Results (1).Outcome = 123 and Results (2).Outcome = 123 and
                  Results (3).Outcome = 1, "concurrent outputs");
         Put_Line ("ADA_CONCURRENT_ERRORS_PASS");
      end;
   elsif Ada.Command_Line.Argument_Count > 0 and then Ada.Command_Line.Argument (1) = "held" then
      declare
         Local : constant Handle := Token_New;
         Q : chars_ptr := New_String ("Is it?");
         Texts : aliased Text_Array (0 .. 1) := (New_String ("hold-bulk-1"), New_String ("hold-bulk-2"));
         Lens : aliased Length_Array (0 .. 1) := (11, 11);
         Answers : aliased Answer_Array (0 .. 1) := (others => (Outcome => 123, Probability => -1.0));
         Code : int := -1;
         task Caller is entry Start; end Caller;
         task body Caller is
         begin
            accept Start;
            Code := Decide_Many_Opts (E, Q, Texts'Address, Lens'Address, 2, No_Deadline, Local, Answers'Address);
         end Caller;
      begin
         Caller.Start; Arrive ("hold-bulk-1");
         Cancel (Local); Cancel (Local);
         -- The scheduler may have accepted either or both bulk rows before the fire.
         Release ("hold-bulk-1"); Release ("hold-bulk-2");
         for I in 1 .. 2_000 loop
            exit when Caller'Terminated;
            delay 0.005;
         end loop;
         Require (Caller'Terminated and Code = 5, "held bulk must cancel with code 5");
         for A of Answers loop
            Require (A.Outcome = 123 and A.Probability = -1.0, "partial bulk result");
         end loop;
         Token_Free (Local);
         for I in Texts'Range loop Free (Texts (I)); end loop;
         Free (Q);
         Put_Line ("ADA_HELD_BULK_PASS");
      end;
      -- A held deadline has no token; the budget alone must return code 3.
      declare
         Code : int := -1;
         Output : aliased Answer := (Outcome => 123, Probability => -1.0);
         task Caller is entry Start; end Caller;
         task body Caller is
            Err : Thinkthen.Failure;
         begin
            accept Start;
            Thinkthen.Decide (E, Null_Handle, "Is it?", "hold-deadline", 80, Output, Err);
            Code := Err.Code;
         end Caller;
      begin
         Caller.Start; Arrive ("hold-deadline");
         delay 0.11;
         Release ("hold-deadline");
         for I in 1 .. 2_000 loop
            exit when Caller'Terminated;
            delay 0.005;
         end loop;
         Require (Caller'Terminated and Code = 3 and Output.Outcome = 123 and Output.Probability = -1.0,
                  "held deadline must return 3 with untouched result");
         Put_Line ("ADA_HELD_DEADLINE_PASS");
      end;
      Expect ("recovery-held", 0);
   else
      Expect ("café", 0);
      Expect ("no", 0);
      Expect ("unsure", 0);
      Expect ("zero", 3, 0);
      T := Token_New; Cancel (T); Cancel (T);
      Expect ("prefired", 5, No_Deadline, T); Token_Free (T); T := Null_Handle;
      for I in JSON_Requests'Range loop
         Thinkthen.JSON (E, Null_Handle, To_String (JSON_Requests (I)), No_Deadline, Result, F);
         Require (F.Code = 0, "JSON verb" & I'Image & ": " & To_String (F.Message));
         Require (Length (Result) > 0, "empty JSON result" & I'Image);
         declare
            V : constant String := Thinkthen.Call_Value (Result);
            Match : constant Boolean :=
              (case I is
                 when 1 => Ada.Strings.Fixed.Index (V, "thinkthen.result/1") > 0 and Ada.Strings.Fixed.Index (V, "true") > 0,
                 when 2 => V = """first""",
                 when 3 => V = "[""first"",""second""]",
                 when 4 => V = "0.1",
                 when 5 => V = "[""filter-one"",""filter-two""]",
                 when 6 => V = "[{""index"":0,""record"":""rank-one"",""probability"":0.9},{""index"":1,""record"":""rank-two"",""probability"":0.9}]",
                 when 7 => V = "{""index"":0,""unit"":""find-one"",""probability"":0.9}",
                 when 8 => Ada.Strings.Fixed.Index (V, """check"":true") > 0,
                 when 9 => Ada.Strings.Fixed.Index (V, """entities""") > 0 and Ada.Strings.Fixed.Index (V, "Maria Chen") > 0,
                 when 10 => Ada.Strings.Fixed.Index (V, """edges""") > 0,
                 when others => False);
         begin
            Require (Match, "JSON verb" & I'Image & " wrong shape: " & V);
            if I in 1 .. 2 then Put_Line ("ADA_CALL_ENVELOPE=" & To_String (Result)); end if;
         end;
         Put_Line ("json" & I'Image & " bytes=" & Natural'Image (Length (Result)));
      end loop;
      Put_Line ("ADA_BASIC_PASS");
   end if;
   Engine_Free (E);
end Main;
