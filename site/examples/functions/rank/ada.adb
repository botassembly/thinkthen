with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Inputs; use Thinkthen.Inputs;
with Thinkthen.Inputs.Calls;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Answers; use Thinkthen.Answers;
with Thinkthen.Values;

procedure Sample is
   package V renames Thinkthen.Values;
   use type V.Value;
   Records : constant Item_List :=
     [Text_Item ("Newsletter: our autumn catalog is here. "
        & "No reply needed."),
      Text_Item (
        "Our checkout page is down "
        & "and customers cannot pay"),
      Text_Item (
        "Reminder: your invoice is due in 30 days"),
      Text_Item (
        "Please send the signed quote by 5 pm today")];
begin
   declare
      Result : constant Rank_Result :=
        Thinkthen.Inputs.Calls.Rank (
     Text_Question ("Is this urgent?"),
     Records_Input (Records));
      By_Urgency : constant V.Value_List := Result.Value;
   begin
      pragma Assert (V.Array_Item (By_Urgency) = V.Array_Item
        ([V.Number_Item ("1"), V.Number_Item ("3"),
          V.Number_Item ("2"), V.Number_Item ("0")]));
   end;
end Sample;
