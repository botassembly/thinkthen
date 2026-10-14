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
   Question : constant String := "How urgent is this?";
   Outage : constant String :=
     "Our checkout page is down and customers cannot pay."
     & ASCII.LF;
   Levels : T_Authored_name_List;
   Scale : T_RequestDefinition_Authored_score;
   Request : T_RequestCall;
   Owner : Session;
   Completed : Completion;
   package V renames Thinkthen.Values;
   Score_Arm : constant T_RequestDefinition_Kind :=
     T_RequestDefinition_arm_Authored_score;

begin
   Levels.Append
     (To_Unbounded_String ("Routine."));
   Levels.Append
     (To_Unbounded_String ("Soon."));
   Levels.Append
     (To_Unbounded_String ("Immediate."));
   Scale.T_score :=
     (T_Authored_questionText_arm_0,
      To_Unbounded_String (Question));
   Scale.T_levels :=
     (True, (T_Authored_levels_arm_0, Levels));
   Request.T_question :=
     (T_RequestQuestion_arm_definition,
      (T_kind => <>, T_value => (Score_Arm, Scale)));
   Request.T_input.V_0.T_text :=
     To_Unbounded_String (Outage);
   Score (Owner, Request);
   Finish (Owner);
   declare
      Urgency : constant Long_Float :=
        Score_Answer (Owner, Completed);
   begin
      pragma Assert (Urgency = 2.0);
   end;
end Sample;
