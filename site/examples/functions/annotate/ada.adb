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
     "Steps: click Log in. Nobody gets in.";
begin
   declare
      Result : constant Annotate_Result :=
        Thinkthen.Inputs.Calls.Annotate (
     File_Question ("form.json"),
     Make_Input_text (Evidence));
      Triage : constant V.Value := Result.Value;
   begin
      pragma Assert
        (V.As_Boolean (V.Field_Value (Triage, "steps")));
      pragma Assert
        (V.As_String (V.Field_Value (Triage, "area")) =
           "login");
      pragma Assert
        (Long_Float'Value
           (V.As_Number (V.Field_Value (Triage, "impact")))
           = 1.98);
   end;
end Sample;
