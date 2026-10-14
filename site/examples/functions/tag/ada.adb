with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Inputs; use Thinkthen.Inputs;
with Thinkthen.Inputs.Calls;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Answers; use Thinkthen.Answers;
with Thinkthen.Values;

procedure Sample is
   package V renames Thinkthen.Values;
   use type V.Value;
   Evidence : constant String :=
     "Love the new dashboard, "
     & "but export crashes the app," & ASCII.LF
     & "and I was charged twice." & ASCII.LF;
begin
   declare
      Result : constant Tag_Result :=
        Thinkthen.Inputs.Calls.Tag (
     File_Question ("labels.json"),
     Make_Input_text (Evidence));
      Fitting_Labels : constant Thinkthen.Answers.Text_List := Result.Value;
   begin
      pragma Assert (Fitting_Labels =
        [To_Unbounded_String ("praise"),
         To_Unbounded_String ("bug"), To_Unbounded_String ("billing")]);
   end;
end Sample;
