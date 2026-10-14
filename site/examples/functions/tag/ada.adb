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
     "Which labels fit this message?";
   Message : constant String :=
     "Love the new dashboard, but export crashes the app,"
     & ASCII.LF & "and I was charged twice." & ASCII.LF;
   Labels : T_Authored_name_List;
   Which : T_RequestDefinition_Authored_tag;
   Request : T_RequestCall;
   Owner : Session;
   Completed : Completion;
   package V renames Thinkthen.Values;
   Tag_Arm : constant T_RequestDefinition_Kind :=
     T_RequestDefinition_arm_Authored_tag;

begin
   Labels.Append
     (To_Unbounded_String ("praise"));
   Labels.Append
     (To_Unbounded_String ("bug"));
   Labels.Append
     (To_Unbounded_String ("billing"));
   Which.T_tag :=
     (T_Authored_questionText_arm_0,
      To_Unbounded_String (Question));
   Which.T_labels :=
     (True, (T_Authored_labels_arm_0, Labels));
   Request.T_question :=
     (T_RequestQuestion_arm_definition,
      (T_kind => <>, T_value => (Tag_Arm, Which)));
   Request.T_input.V_0.T_text :=
     To_Unbounded_String (Message);
   Tag (Owner, Request);
   Finish (Owner);
   declare
      Fitting_Labels : constant Text_List :=
        Tag_Answer (Owner, Completed);
   begin
      pragma Assert (Fitting_Labels =
        [To_Unbounded_String ("praise"),
         To_Unbounded_String ("bug"),
         To_Unbounded_String ("billing")]);
   end;
end Sample;
