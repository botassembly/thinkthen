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
     "My parcel went to the wrong address.";
begin
   Thinkthen.Inputs.Calls.Choose (Owner,
     File_Question ("teams.json"),
     Text_Input (Evidence));
   Finish (Owner);
   declare
      Team : constant Choice :=
        Choose_Answer (Owner, Completed);
   begin
      pragma Assert
        (Team.Known and then Team.Value = "shipping");
   end;
end Sample;
