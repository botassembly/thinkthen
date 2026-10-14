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
     "Steps: click Log in. Nobody gets in.";
begin
   Thinkthen.Inputs.Calls.Annotate (Owner,
     File_Question ("form.json"),
     Text_Input (Evidence));
   Finish (Owner);
   declare
      Rows : constant Annotation_List :=
        Annotate_Answer (Owner, Completed);
      Triage : constant V.Value := Fields (Rows (1));
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
