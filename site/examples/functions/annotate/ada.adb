with Thinkthen.Values;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Interfaces.C; use Interfaces.C;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen.Answers; use Thinkthen.Answers;

procedure Sample is
   Report : constant String :=
     "Steps: click Log in. Nobody gets in.";
   Request : T_RequestCall;
   Owner : Session;
   Completed : Completion;
   package V renames Thinkthen.Values;

begin
   Request.T_question :=
     (T_RequestQuestion_arm_file,
      (T_kind => <>,
       T_path => To_Unbounded_String ("form.json")));
   Request.T_input.V_0.T_text :=
     To_Unbounded_String (Report);
   Annotate (Owner, Request);
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
