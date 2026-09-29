with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Interfaces; use Interfaces;
with Interfaces.C; use Interfaces.C;
with Thinkthen; use Thinkthen;
procedure Facts_Boundary is
   procedure Select_Mode (Mode : int) with Import => True, Convention => C,
      External_Name => "facts_boundary_select";
   function Native_Calls return int with Import => True, Convention => C,
      External_Name => "facts_boundary_calls";
   function Native_Frees return int with Import => True, Convention => C,
      External_Name => "facts_boundary_frees";
   procedure Require (Good : Boolean; Note : String) is
   begin
      if not Good then raise Program_Error with Note; end if;
   end Require;
   Client : Engine;
   Answer : Decision;
   Facts : Run_Facts;
   Error : Failure;
   Start_Frees : constant int := Native_Frees;
   Rows : constant Evidence_Array := (To_Unbounded_String ("one"), To_Unbounded_String ("two"));
   Answers : Decision_Array (1 .. 2);
   Output : JSON_Result;
   Pair : constant Evidence_Array :=
      (To_Unbounded_String ("{""name"":""First"",""kind"":""alert""}"),
       To_Unbounded_String ("{""name"":""Second"",""kind"":""alert""}"));
   Refused : Boolean := False;
 begin
   Select_Mode (1);
   Decide (Client, "Is it?", "evidence", Answer, Facts, Error);
   Require (Error.Kind = None and Answer.Value = Yes and Answer.Probability = 0.9 and
            Facts.Records = 1 and Facts.Requests_Sent = 1 and Facts.Cache_Answers = 0 and
            Facts.Seconds = 0.125 and Facts.Has_Model and
            To_String (Facts.Model) = "synthetic-model" and
            not Facts.Has_Input_Tokens and not Facts.Has_Output_Tokens,
            "model without usage and fractional seconds survive one typed call");
   Require (Native_Calls = 1 and Native_Frees = Start_Frees + 1,
            "scalar owns and releases its native facts");

   Select_Mode (2);
   Decide (Client, "Is it?", "evidence", Answer, Facts, Error);
   Require (Error.Kind = Defect and not Error.Retryable and Failure_Facts (Error) = "" and
            Answer.Value = Not_Sure and Facts.Records = 0 and not Facts.Has_Model,
            "fractional required count is a typed defect without partial success");
   Require (Native_Calls = 2 and Native_Frees = Start_Frees + 2,
            "invalid scalar facts were released");

   Select_Mode (3);
   Decide_Many (Client, "Is it?", Rows, Answers, Facts, Error);
   Require (Error.Kind = Defect and Answers (1).Value = Not_Sure and
            Answers (2).Value = Not_Sure and Facts.Records = 0,
            "present-null optional tokens are a bulk defect without partial values");
   Require (Native_Calls = 3 and Native_Frees = Start_Frees + 3,
            "invalid bulk facts were released");

   Select_Mode (4);
   Recognize (Client, "{""version"":1,""recognize"":{}}", "text", Output, Facts, Error);
   Require (Error.Kind = Defect and To_String (Output.JSON) = "" and Facts.Records = 0,
            "missing required count is a recognize defect without partial JSON");
   Require (Native_Calls = 4 and Native_Frees = Start_Frees + 5,
            "recognize value and facts were released independently");

   Select_Mode (5);
   Relate (Client, "{""version"":1,""relate"":{""relations"":[{""name"":""linked"",""source"":""alert"",""target"":""alert""}]}}",
           Pair, Output, Facts, Error);
   Require (Error.Kind = Defect and To_String (Output.JSON) = "" and Facts.Records = 0,
            "unknown facts member is a relate defect under the closed result schema");
   Require (Native_Calls = 5 and Native_Frees = Start_Frees + 7,
            "relate value and facts were released independently");

   Select_Mode (6);
   Decide (Client, "Is it?", "evidence", Answer, Facts, Error);
   Require (Error.Kind = Defect and Answer.Value = Not_Sure and Facts.Records = 0 and
            Native_Calls = 6 and Native_Frees = Start_Frees + 8,
            "negative elapsed seconds are a defect with one released facts pointer");

   Select_Mode (7);
   Decide (Client, "Is it?", "evidence", Answer, Facts, Error);
   Require (Error.Kind = Defect and Answer.Value = Not_Sure and Facts.Records = 0 and
            Native_Calls = 7 and Native_Frees = Start_Frees + 9,
            "unsigned count overflow is a defect with one released facts pointer");

   begin
      Decide (Client, "Is" & Character'Val (0) & " it?", "evidence", Answer, Facts, Error);
   exception
      when Constraint_Error => Refused := True;
   end;
   Require (Refused and Native_Calls = 7 and Native_Frees = Start_Frees + 9,
            "caller C-string refusal remains pre-start and is not remapped");
   Put_Line ("ADA_FACTS_BOUNDARY_PASS");
end Facts_Boundary;
