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
   Evidence : constant String :=
     "Sam Patel says his Kindle arrived broken "
     & "in Denver.";
begin
   Thinkthen.Inputs.Calls.Recognize (Owner,
     File_Question ("kinds.json"),
     Text_Input (Evidence));
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
