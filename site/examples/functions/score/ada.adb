with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Inputs; use Thinkthen.Inputs;
with Thinkthen.Inputs.Calls;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Answers; use Thinkthen.Answers;
with Thinkthen.Values;

procedure Sample is
   package V renames Thinkthen.Values;
   Evidence : constant String :=
     "Our checkout page is down and customers cannot pay."
     & ASCII.LF;
begin
   declare
      Grade_Judgment : constant Score_Result :=
        Thinkthen.Inputs.Calls.Score (
     File_Question ("levels.json"),
     Make_Input_text (Evidence));
      Urgency : constant Long_Float :=
        Grade_Judgment.Value;
   begin
      pragma Assert (Urgency = 2.0);
   end;
end Sample;
