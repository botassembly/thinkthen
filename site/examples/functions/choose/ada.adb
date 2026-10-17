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
     "My parcel went to the wrong address.";
begin
   declare
      Team_Judgment : constant Choose_Result :=
        Thinkthen.Inputs.Calls.Choose (
     File_Question ("teams.json"),
     Make_Input_text (Evidence));
      Team : constant Choice := Team_Judgment.Value;
   begin
      pragma Assert
        (Team.Known and then
         To_String (Team.Value) = "shipping");
   end;
end Sample;
