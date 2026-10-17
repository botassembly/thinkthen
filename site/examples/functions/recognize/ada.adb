with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Inputs; use Thinkthen.Inputs;
with Thinkthen.Inputs.Calls;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Answers; use Thinkthen.Answers;
with Thinkthen.Values;

procedure Sample is
   package V renames Thinkthen.Values;
   Evidence : constant String :=
     "Sam Patel says his Kindle arrived broken "
     & "in Denver.";
begin
   declare
      Names_Call : constant Recognize_Result :=
        Thinkthen.Inputs.Calls.Recognize (
     File_Question ("kinds.json"),
     Make_Input_text (Evidence));
      Names : Unbounded_String;
   begin
      declare
            Spans : constant V.Value :=
              V.Field_Value (Names_Call.Value, "entities");
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
      pragma Assert (Names = "Sam Patel/person "
        & "Kindle/product " & "Denver/place ");
   end;
end Sample;
