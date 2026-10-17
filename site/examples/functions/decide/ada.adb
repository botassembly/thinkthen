with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Inputs; use Thinkthen.Inputs;
with Thinkthen.Inputs.Calls;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Answers; use Thinkthen.Answers;
with Thinkthen.Values;

procedure Sample is
   package V renames Thinkthen.Values;
   Evidence : constant String :=
     "Please refund my order. It arrived broken.";
begin
   declare
      Refund_Judgment : constant Decide_Result :=
        Thinkthen.Inputs.Calls.Decide (
     Text_Question ("Does the customer ask for a refund?"),
     Make_Input_text (Evidence));
      Is_Refund : constant V.Value := Refund_Judgment.Value;
   begin
      pragma Assert
        (V.As_Boolean (Is_Refund));
   end;
end Sample;
