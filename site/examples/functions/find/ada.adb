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
     "Which line gives the refund deadline?";
   function Line (Text : String) return T_RequestItem is
     (T_original =>
        (True,
         (T_RequestOriginal_arm_text,
          (T_kind => <>,
           T_text => To_Unbounded_String (Text)))),
      others => <>);
   Policy : T_RequestItem_List;
   Request : T_RequestCall;
   Owner : Session;
   Completed : Completion;
   package V renames Thinkthen.Values;

begin
   Policy.Append
     (Line ("Returns need the original receipt."));
   Policy.Append
     (Line
        ("Refunds are issued within 30 days of purchase."));
   Policy.Append
     (Line ("Shipping is free on orders over $50."));
   Policy.Append
     (Line ("Gift cards cannot be exchanged for cash."));
   Request.T_question.V_0.T_text :=
     To_Unbounded_String (Question);
   Request.T_input :=
     (T_RequestInput_arm_records,
      (T_items => Policy, T_kind => <>));
   Find (Owner, Request);
   Finish (Owner);
   declare
      Found : constant Find_Result :=
        Find_Answer (Owner, Completed);
      Refund_Deadline : constant String :=
        V.As_String (Found_Value (Found));
   begin
      pragma Assert (Refund_Deadline =
        "Refunds are issued within 30 days of purchase.");
   end;
end Sample;
