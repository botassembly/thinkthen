with Ada.Directories;
with Ada.Environment_Variables;
with Ada.Strings.Fixed;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Interfaces; use Interfaces;
with Thinkthen; use Thinkthen;
procedure Package_Bulk is
   Client : Engine;
   Barrier : constant String := Ada.Environment_Variables.Value ("TT_BARRIER_DIR");
   procedure Require (Good : Boolean; Note : String) is
   begin
      if not Good then raise Program_Error with Note; end if;
   end Require;
   procedure Arrive (Name : String) is
   begin
      for I in 1 .. 2_000 loop
         exit when Ada.Directories.Exists (Barrier & "/arrived-" & Name);
         delay 0.005;
      end loop;
      Require (Ada.Directories.Exists (Barrier & "/arrived-" & Name), "missing held arrival: " & Name);
   end Arrive;
   procedure Release (Name : String) is
      F : File_Type;
   begin
      Create (F, Out_File, Barrier & "/release-" & Name); Close (F);
   end Release;
   Basic : constant Evidence_Array := (To_Unbounded_String ("first"), To_Unbounded_String ("second"), To_Unbounded_String ("third"));
   Answers : Decision_Array (1 .. 3);
   Facts : Run_Facts;
   Error : Failure;
begin
   Decide_Many (Client, "Is it?", Basic, Answers, Facts, Error);
   Require (Error.Kind = None and Answers (1).Value = Yes and Answers (2).Value = No and
            Answers (3).Value = Yes and Answers (1).Probability = 0.9 and
            Answers (2).Probability = 0.1 and Answers (3).Probability = 0.6 and
            Facts.Records = 3 and Facts.Requests_Sent = 1 and Facts.Cache_Answers = 0 and
            Facts.Has_Model and Facts.Has_Input_Tokens and Facts.Input_Tokens = 1,
            "typed bulk result order");
   Decide_Many (Client, "Is it?", Basic, Answers, Facts, Error);
   Require (Error.Kind = None and Answers (3).Probability = 0.6 and Facts.Records = 3 and
            Facts.Requests_Sent = 0 and Facts.Cache_Answers = 1,
            "identical packed reply is one cache answer");
   declare
      Empty : Evidence_Array (1 .. 0);
      Empty_Answers : Decision_Array (1 .. 0);
      Refused : Boolean := False;
   begin
      begin
         Decide_Many (Client, "Is it?", Empty, Empty_Answers, Facts, Error);
      exception
         when Constraint_Error => Refused := True;
      end;
      Require (Refused, "empty Ada bulk must still refuse before send");
   end;
   declare
      Token : aliased Cancel_Token;
      Answer : Decision;
      Kind : Error_Kind := None;
      task Caller is entry Start; end Caller;
      task body Caller is
         Err : Failure;
      begin
         accept Start;
         Decide (Client, "Is it?", "hold-scalar", Answer, Facts, Err, Token => Token'Access);
         Kind := Err.Kind;
      end Caller;
   begin
      Caller.Start; Arrive ("hold-scalar"); Cancel (Token); Cancel (Token); Release ("hold-scalar");
      -- The task joins at this scope's end before its token is finalized.
      for I in 1 .. 2_000 loop exit when Caller'Terminated; delay 0.005; end loop;
      Require (Caller'Terminated and Kind = Cancelled, "typed held scalar");
   end;
   declare
      Token : aliased Cancel_Token;
      Evidence : constant Evidence_Array := (To_Unbounded_String ("hold-bulk-1"), To_Unbounded_String ("hold-bulk-2"));
      Results : Decision_Array (1 .. 2);
      Kind : Error_Kind := None;
      task Caller is entry Start; end Caller;
      task body Caller is
         Err : Failure;
      begin
         accept Start;
         Decide_Many (Client, "Is it?", Evidence, Results, Facts, Err, Token => Token'Access);
         Kind := Err.Kind;
      end Caller;
   begin
      Caller.Start; Arrive ("hold-bulk-1"); Cancel (Token); Cancel (Token);
      Release ("hold-bulk-1"); Release ("hold-bulk-2");
      for I in 1 .. 2_000 loop exit when Caller'Terminated; delay 0.005; end loop;
      Require (Caller'Terminated and Kind = Cancelled, "typed held bulk");
   end;
   declare
      Kind : Error_Kind := None;
      Answer : Decision;
      task Caller is entry Start; end Caller;
      task body Caller is
         Err : Failure;
      begin
         accept Start;
         Decide (Client, "Is it?", "hold-deadline", Answer, Facts, Err, Deadline_Ms => 80);
         Kind := Err.Kind;
      end Caller;
   begin
      Caller.Start; Arrive ("hold-deadline"); delay 0.11; Release ("hold-deadline");
      for I in 1 .. 2_000 loop exit when Caller'Terminated; delay 0.005; end loop;
      Require (Caller'Terminated and Kind = Deadline, "typed held deadline");
   end;
   Decide (Client, "Is it?", "recovery-package", Answers (1), Facts, Error);
   Require (Error.Kind = None and Answers (1).Value = Yes and Facts.Requests_Sent = 1,
            "fresh-token recovery");
   declare
      Structured : JSON_Result;
      Pair : constant Evidence_Array :=
        (To_Unbounded_String ("{""name"":""Third"",""kind"":""alert""}"),
         To_Unbounded_String ("{""name"":""Fourth"",""kind"":""alert""}"));
   begin
      Recognize (Client, "{""version"":1,""recognize"":{""kinds"":{""person"":""A person's name.""}}}",
                 "John Smith", Structured, Facts, Error);
      Require (Error.Kind = None and Facts.Requests_Sent = 2 and Facts.Has_Model and
               Ada.Strings.Fixed.Index (To_String (Structured.JSON), """entities""") > 0,
               "typed recognize facts and value");
      Relate (Client, "{""version"":1,""relate"":{""relations"":[{""name"":""caused_by"",""source"":""alert"",""target"":""alert""}]}}",
              Pair, Structured, Facts, Error);
      Require (Error.Kind = None and Facts.Requests_Sent = 1 and Facts.Records = 1 and
               Ada.Strings.Fixed.Index (To_String (Structured.JSON), """edges""") > 0,
               "typed relate facts and value");
   end;
   declare
      Saved : Failure;
      Snapshot : Unbounded_String;
   begin
      declare
         Another : Engine;
         Answer : Decision;
      begin
         Decide (Another, "Is it?", "status-401", Answer, Facts, Error);
         Require (Error.Kind = Backend and Failure_Facts (Error) /= "" and
                  Ada.Strings.Fixed.Index (Failure_Facts (Error), """records"":0") > 0 and
                  Ada.Strings.Fixed.Index (Failure_Facts (Error), """requests_sent"":1") > 0,
                  "typed first failed row has zero completed records and one send");
         Saved := Error; Snapshot := To_Unbounded_String (Failure_Facts (Error));
         Decide (Another, "Is it?", "after-error", Answer, Facts, Error);
         Require (Error.Kind = None and Answer.Value = Yes, "typed recovery after failure");
      end;
      Require (Failure_Facts (Saved) = To_String (Snapshot) and Message (Saved) /= "",
               "typed failure facts survive later call and engine close");
   end;
   Put_Line ("TYPED_ADA_MATRIX_PASS");
end Package_Bulk;
