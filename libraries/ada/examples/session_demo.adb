with Ada.Text_IO;
with Thinkthen.Inputs;
with Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
with Thinkthen.Answers;
procedure Session_Demo is
   use Thinkthen.Inputs;
   use Thinkthen.Sessions;
   use Thinkthen.Answers;
   Owner : Session;
   Completed : Completion;
begin
   Thinkthen.Sessions.Calls.Decide (Owner, Text_Question ("Does this ask for a refund?"),
     Records_Input ([Text_Item ("Please return my payment.")]));
   Finish (Owner);
   declare
      Answer : constant Decision := Decide_Answer (Owner, Completed);
   begin
      if Answer.Known then Ada.Text_IO.Put_Line (Boolean'Image (Answer.Value));
      else Ada.Text_IO.Put_Line ("uncertain"); end if;
   end;
end Session_Demo;
