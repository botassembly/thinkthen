with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
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
   Facts : Unbounded_String;
   Error : Failure;
   Start_Frees : constant int := Native_Frees;
   Rows : constant Evidence_Array := (To_Unbounded_String ("one"), To_Unbounded_String ("two"));
   Answers : Decision_Array (1 .. 2);
   Output : Unbounded_String;
   Pair : constant Evidence_Array :=
      (To_Unbounded_String ("{""name"":""First"",""kind"":""alert""}"),
       To_Unbounded_String ("{""name"":""Second"",""kind"":""alert""}"));
   Refused : Boolean := False;
begin
   Select_Mode (1);
   Decide (Client, "Is it?", "evidence", Answer, Facts, Error);
   Require (Error.Kind = None and Answer.Value = Yes and Answer.Probability = 0.9 and
            Facts = "{""records"":1,""requests_sent"":1,""cache_answers"":0,""seconds"":0.125,""model"":""synthetic-model""}" and
            Member (To_String (Facts), "input_tokens") = "",
            "facts pass through as the native JSON text");
   Require (Native_Calls = 1 and Native_Frees = Start_Frees + 1,
            "scalar owns and releases its native facts");

   Select_Mode (3);
   Decide_Many (Client, "Is it?", Rows, Answers, Facts, Error);
   Require (Error.Kind = None and Answers (1).Value = Yes and Answers (2).Value = No and
            Member (To_String (Facts), "input_tokens") = "null",
            "a null optional member reads as JSON null");
   Require (Native_Calls = 2 and Native_Frees = Start_Frees + 2,
            "bulk facts were released");

   Select_Mode (4);
   Recognize (Client, "{""version"":1,""recognize"":{}}", "text", Output, Facts, Error);
   Require (Error.Kind = Defect and Output = "" and Facts = "",
            "success without facts is a recognize defect without partial JSON");
   Require (Native_Calls = 3 and Native_Frees = Start_Frees + 3,
            "recognize value was released");

   Select_Mode (5);
   Relate (Client, "{""version"":1,""relate"":{""relations"":[{""name"":""linked"",""source"":""alert"",""target"":""alert""}]}}",
           Pair, Output, Facts, Error);
   Require (Error.Kind = None and Output = "{""edges"":[]}" and
            Member (To_String (Facts), "surprise") = "1" and Member (To_String (Facts), "records") = "1",
            "an unknown facts member reads without error");
   Require (Native_Calls = 4 and Native_Frees = Start_Frees + 5,
            "relate value and facts were released independently");

   begin
      Decide (Client, "Is" & Character'Val (0) & " it?", "evidence", Answer, Facts, Error);
   exception
      when Constraint_Error => Refused := True;
   end;
   Require (Refused and Native_Calls = 4 and Native_Frees = Start_Frees + 5,
            "caller C-string refusal remains pre-start and is not remapped");
   Put_Line ("ADA_FACTS_BOUNDARY_PASS");
end Facts_Boundary;
