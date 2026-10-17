with Ada.Environment_Variables;
with Ada.Text_IO;
with Thinkthen.Inputs;
with Thinkthen.Inputs.Calls;
with Thinkthen.Values;
with Thinkthen.Answers;
procedure Smoke is
   use type Thinkthen.Values.Value_Kind;
   Answer : constant Thinkthen.Answers.Decide_Result := Thinkthen.Inputs.Calls.Decide
     (Thinkthen.Inputs.Text_Question (Ada.Environment_Variables.Value ("THINKTHEN_TEST_SMOKE_QUESTION")),
      Thinkthen.Inputs.Make_Input_text (Ada.Environment_Variables.Value ("THINKTHEN_TEST_SMOKE_TEXT")));
begin
   Ada.Text_IO.Put_Line (if Thinkthen.Values.Kind (Answer.Value) = Thinkthen.Values.Null_Value then "smoke: null"
     elsif Thinkthen.Values.As_Boolean (Answer.Value) then "smoke: true" else "smoke: false");
end Smoke;
