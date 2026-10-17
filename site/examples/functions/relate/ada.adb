with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Inputs; use Thinkthen.Inputs;
with Thinkthen.Inputs.Calls;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Answers; use Thinkthen.Answers;
with Thinkthen.Values;

procedure Sample is
   package V renames Thinkthen.Values;
   function Entity (Name, Kind : String) return Item is
     (With_Original (Make_Item,
       Original_From_json
         (Make_Original_json (V.Object_Item
           ([V.Named ("name", V.String_Item (Name)),
             V.Named ("kind", V.String_Item (Kind))])))));
   Records : constant Item_List :=
     [Entity ("Apple", "company"),
      Entity ("Amazon", "company"),
      Entity ("iPhone", "product"),
      Entity ("Kindle", "product")];
begin
   declare
      Links_Call : constant Relate_Result :=
        Thinkthen.Inputs.Calls.Relate (
     File_Question ("makes.json"),
     Entities_Input (Records));
      Connections : constant V.Value := Links_Call.Value;
      Who_Makes : Unbounded_String;
   begin
      for I in 1 .. V.Length (Connections) loop
         declare
            One : constant V.Value :=
              V.Element (Connections, I);
         begin
            Append (Who_Makes, Source_Name (One)
              & " makes " & Target_Name (One) & ". ");
         end;
      end loop;
      pragma Assert (Who_Makes = "Apple makes iPhone. "
        & "Amazon makes Kindle. ");
   end;
end Sample;
