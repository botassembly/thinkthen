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
     "Our checkout page is down and customers cannot pay."
     & ASCII.LF;
begin
   Thinkthen.Inputs.Calls.Score (Owner,
     File_Question ("levels.json"),
     Text_Input (Evidence));
   Finish (Owner);
   declare
      Urgency : constant Long_Float :=
        Score_Answer (Owner, Completed);
   begin
      pragma Assert (Urgency = 2.0);
   end;
end Sample;
