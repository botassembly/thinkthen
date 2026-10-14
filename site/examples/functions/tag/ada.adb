with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Inputs; use Thinkthen.Inputs;
with Thinkthen.Inputs.Calls;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Answers; use Thinkthen.Answers;
with Thinkthen.Values;

procedure Sample is
   package V renames Thinkthen.Values;
   Owner : Session;
   Completed : Completion;
   Evidence : constant String :=
     "Love the new dashboard, "
     & "but export crashes the app," & ASCII.LF
     & "and I was charged twice." & ASCII.LF;
begin
   Thinkthen.Inputs.Calls.Tag (Owner,
     File_Question ("labels.json"),
     Text_Input (Evidence));
   Finish (Owner);
   declare
      Fitting_Labels : constant Text_List :=
        Tag_Answer (Owner, Completed);
   begin
      pragma Assert (Fitting_Labels =
        [To_Unbounded_String ("praise"),
         To_Unbounded_String ("bug"),
         To_Unbounded_String ("billing")]);
   end;
end Sample;
