with Ada.Environment_Variables;
with Ada.Text_IO;
with Thinkthen.Inputs;
with Thinkthen.Inputs.Calls;
with Thinkthen.Sessions;
with Thinkthen.Answers;
procedure Smoke is
   Owner : Thinkthen.Sessions.Session;
   Completed : Thinkthen.Answers.Completion;
begin
   Thinkthen.Inputs.Calls.Decide (Owner,
     Thinkthen.Inputs.Text_Question (Ada.Environment_Variables.Value ("THINKTHEN_TEST_SMOKE_QUESTION")),
     Thinkthen.Inputs.Text_Input (Ada.Environment_Variables.Value ("THINKTHEN_TEST_SMOKE_TEXT")));
   Thinkthen.Sessions.Finish (Owner);
   declare Answer : constant Thinkthen.Answers.Decision := Thinkthen.Answers.Decide_Answer (Owner, Completed);
   begin
      Ada.Text_IO.Put_Line (if not Answer.Known then "smoke: null"
        elsif Answer.Value then "smoke: true" else "smoke: false");
   end;
end Smoke;
