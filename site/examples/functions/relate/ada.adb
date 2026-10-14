with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Values;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen.Answers; use Thinkthen.Answers;

procedure Sample is
   function Name (Text, Kind : String)
     return T_RequestItem is
     (T_original =>
        (True,
         (T_RequestOriginal_arm_json,
          (T_kind => <>,
           T_value => Thinkthen.Values.Object_Item
             ([Thinkthen.Values.Named
                ("name",
                 Thinkthen.Values.String_Item (Text)),
               Thinkthen.Values.Named
                ("kind",
                 Thinkthen.Values.String_Item (Kind))])))),
      others => <>);
   Names : T_RequestItem_List;
   Request : T_RequestCall;
   Owner : Session;
   Completed : Completion;
   package V renames Thinkthen.Values;

begin
   Names.Append (Name ("Apple", "company"));
   Names.Append (Name ("Amazon", "company"));
   Names.Append (Name ("iPhone", "product"));
   Names.Append (Name ("Kindle", "product"));
   Request.T_question :=
     (T_RequestQuestion_arm_file,
      (T_kind => <>,
       T_path => To_Unbounded_String ("makes.json")));
   Request.T_input :=
     (T_RequestInput_arm_entities,
      (T_items => Names, T_kind => <>));
   Relate (Owner, Request);
   Finish (Owner);
   declare
      Related : constant Relation :=
        Relate_Answer (Owner, Completed);
      Connections : constant V.Value := Edges (Related);
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
