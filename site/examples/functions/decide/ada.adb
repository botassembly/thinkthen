with Thinkthen.Values;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen.Answers; use Thinkthen.Answers;

procedure Sample is
   Question : constant String :=
     "Does the customer ask for a refund?";
   Broken : constant String :=
     "Please refund my order. It arrived broken.";
   Request : T_RequestCall;
   Owner : Session;
   Completed : Completion;
   package V renames Thinkthen.Values;

begin
   Request.T_question.V_0.T_text :=
     To_Unbounded_String (Question);
   Request.T_input.V_0.T_text :=
     To_Unbounded_String (Broken);
   Decide (Owner, Request);
   Finish (Owner);
   declare
      Is_Refund : constant Decision :=
        Decide_Answer (Owner, Completed);
   begin
      pragma Assert
        (Is_Refund.Known and then Is_Refund.Value);
   end;
end Sample;
