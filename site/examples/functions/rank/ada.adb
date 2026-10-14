with Thinkthen.Values;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen.Answers; use Thinkthen.Answers;

procedure Sample is
   Question : constant String := "Is this urgent?";
   function Message (Text : String) return T_RequestItem is
     (T_original =>
        (True,
         (T_RequestOriginal_arm_text,
          (T_kind => <>,
           T_text => To_Unbounded_String (Text)))),
      others => <>);
   Inbox : T_RequestItem_List;
   Request : T_RequestCall;
   Owner : Session;
   Completed : Completion;
   package V renames Thinkthen.Values;

begin
   Inbox.Append
     (Message ("Newsletter: our autumn catalog is here. "
               & "No reply needed."));
   Inbox.Append
     (Message
        ("Our checkout page is down and customers "
         & "cannot pay"));
   Inbox.Append
     (Message ("Reminder: your invoice is due in 30 days"));
   Inbox.Append
     (Message
        ("Please send the signed quote by 5 pm today"));
   Request.T_question.V_0.T_text :=
     To_Unbounded_String (Question);
   Request.T_input :=
     (T_RequestInput_arm_records,
      (T_items => Inbox, T_kind => <>));
   Rank (Owner, Request);
   Finish (Owner);
   declare
      By_Urgency : constant Rank_List :=
        Rank_Answer (Owner, Completed);
   begin
      pragma Assert (By_Urgency = [1, 3, 2, 0]);
   end;
end Sample;
