with Thinkthen.Values;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen.Answers; use Thinkthen.Answers;

procedure Sample is
   Text_In : constant String :=
     "Sam Patel says his Kindle arrived broken "
     & "in Denver.";
   Request : T_RequestCall;
   Owner : Session;
   Completed : Completion;
   package V renames Thinkthen.Values;

begin
   Request.T_question :=
     (T_RequestQuestion_arm_file,
      (T_kind => <>,
       T_path => To_Unbounded_String ("kinds.json")));
   Request.T_input.V_0.T_text :=
     To_Unbounded_String (Text_In);
   Recognize (Owner, Request);
   Finish (Owner);
   declare
      Rows : constant Recognition_List :=
        Recognize_Answer (Owner, Completed);
      Names : Unbounded_String;
   begin
      for Row of Rows loop
         declare
            Spans : constant V.Value := Entities (Row);
         begin
            for I in 1 .. V.Length (Spans) loop
               declare
                  One : constant V.Value :=
                    V.Element (Spans, I);
               begin
                  Append (Names,
                    V.As_String
                      (V.Field_Value (One, "text"))
                    & "/" &
                    V.As_String
                      (V.Field_Value (One, "kind"))
                    & " ");
               end;
            end loop;
         end;
      end loop;
      pragma Assert (Names = "Sam Patel/person "
        & "Kindle/product " & "Denver/place ");
   end;
end Sample;
