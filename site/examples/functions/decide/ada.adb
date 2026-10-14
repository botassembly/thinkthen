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
     "Please refund my order. It arrived broken.";
begin
   Thinkthen.Inputs.Calls.Decide (Owner,
     Text_Question ("Does the customer ask for a refund?"),
     Text_Input (Evidence));
   Finish (Owner);
   declare
      Is_Refund : constant Decision :=
        Decide_Answer (Owner, Completed);
   begin
      pragma Assert
        (Is_Refund.Known and then Is_Refund.Value);
   end;
end Sample;
