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
     [Text_Item ("Arrived a day early. Thank you!"),
      Text_Item (
        "The zipper broke the first time I used it."),
      Text_Item ("Does this come in blue?"),
      Text_Item ("The strap snapped on day two.")];
begin
   Thinkthen.Inputs.Calls.Filter (Owner,
     Text_Question ("Is this a complaint?"),
     Records_Input (Records));
   Finish (Owner);
   declare
      Complaints : constant V.Value_List :=
        Filter_Answer (Owner, Completed);
   begin
      pragma Assert (Complaints'Length = 2);
      pragma Assert (V.As_String (Complaints (1)) =
        "The zipper broke the first time I used it.");
      pragma Assert (V.As_String (Complaints (2)) =
        "The strap snapped on day two.");
   end;
end Sample;
