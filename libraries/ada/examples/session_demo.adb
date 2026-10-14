with Ada.Text_IO;
with Thinkthen.Inputs;
with Thinkthen.Inputs.Calls;
with Thinkthen.Answers;
with Thinkthen.Values;
procedure Session_Demo is
   use Thinkthen.Inputs;
   use Thinkthen.Answers;
   use type Thinkthen.Values.Value_Kind;
   Answer : constant Decide_Result := Thinkthen.Inputs.Calls.Decide
     (Text_Question ("Does this ask for a refund?"),
      Make_Input_text ("Please return my payment."));
begin
   if Thinkthen.Values.Kind (Answer.Value) = Thinkthen.Values.Null_Value then
      Ada.Text_IO.Put_Line ("uncertain");
   else
      Ada.Text_IO.Put_Line (Boolean'Image (Thinkthen.Values.As_Boolean (Answer.Value)));
   end if;
end Session_Demo;
