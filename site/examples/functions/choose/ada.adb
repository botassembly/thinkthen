with Thinkthen.Values;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen.Answers; use Thinkthen.Answers;

procedure Sample is
   Question : constant String := "Which team owns this?";
   Parcel : constant String :=
     "My parcel went to the wrong address.";
   function Option (Name, Text : String)
     return T_Authored_description_Map_Entry is
     (Key => To_Unbounded_String (Name),
      Value =>
        (T_Authored_description_arm_0,
         To_Unbounded_String (Text)));
   Teams : T_Authored_description_Map;
   Which : T_RequestDefinition_Authored_choose;
   Request : T_RequestCall;
   Owner : Session;
   Completed : Completion;
   package V renames Thinkthen.Values;
   Choose_Arm : constant T_RequestDefinition_Kind :=
    T_RequestDefinition_arm_Authored_choose;

begin
   Teams.Append
     (Option ("billing", "Invoices, fees, and refunds."));
   Teams.Append
     (Option ("shipping", "Parcels and delivery."));
   Teams.Append
     (Option ("account", "Logins and passwords."));
   Which.T_choose :=
     (T_Authored_questionText_arm_0,
      To_Unbounded_String (Question));
   Which.T_options :=
     (True, (T_Authored_options_arm_1, Teams));
   Request.T_question :=
     (T_RequestQuestion_arm_definition,
      (T_kind => <>, T_value => (Choose_Arm, Which)));
   Request.T_input.V_0.T_text :=
     To_Unbounded_String (Parcel);
   Choose (Owner, Request);
   Finish (Owner);
   declare
      Team : constant Choice :=
        Choose_Answer (Owner, Completed);
   begin
      pragma Assert
        (Team.Known and then Team.Value = "shipping");
   end;
end Sample;
