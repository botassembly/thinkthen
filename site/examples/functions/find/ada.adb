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
   Records : constant Item_List :=
     [Text_Item ("Returns need the original receipt."),
      Text_Item (
        "Refunds are issued within 30 days of purchase."),
      Text_Item ("Shipping is free on orders over $50."),
      Text_Item (
        "Gift cards cannot be exchanged for cash.")];
begin
   Thinkthen.Inputs.Calls.Find (Owner,
     Text_Question (
       "Which line gives the refund deadline?"),
     Records_Input (Records));
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
