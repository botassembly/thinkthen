with Thinkthen.Values;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen.Answers; use Thinkthen.Answers;

procedure Sample is
   Question : constant String := "Is this a complaint?";
   function Review (Text : String) return T_RequestItem is
     (T_original =>
        (True,
         (T_RequestOriginal_arm_text,
          (T_kind => <>,
           T_text => To_Unbounded_String (Text)))),
      others => <>);
   Reviews : T_RequestItem_List;
   Request : T_RequestCall;
   Owner : Session;
   Completed : Completion;
   package V renames Thinkthen.Values;

begin
   Reviews.Append
     (Review ("Arrived a day early. Thank you!"));
   Reviews.Append
     (Review
        ("The zipper broke the first time I used it."));
   Reviews.Append
     (Review ("Does this come in blue?"));
   Reviews.Append
     (Review ("The strap snapped on day two."));
   Request.T_question.V_0.T_text :=
     To_Unbounded_String (Question);
   Request.T_input :=
     (T_RequestInput_arm_records,
      (T_items => Reviews, T_kind => <>));
   Filter (Owner, Request);
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
