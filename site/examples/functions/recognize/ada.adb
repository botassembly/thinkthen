with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Sample is
   Client : Engine;
   Names  : Unbounded_String;
   Facts  : Unbounded_String;
   Error  : Failure;

   Kinds : constant String :=
     "{""version"": 1, ""recognize"": {""kinds"": {"
     & """person"": null, ""organization"": null, "
     & """place"": null}}}";
   Text : constant String :=
     "Maria Chen joined Northwind Freight, "
     & "a company in Chicago.";
   Spans : constant array (1 .. 3) of Unbounded_String :=
     (To_Unbounded_String
        ("""Maria Chen"" ""person"""),
      To_Unbounded_String
        ("""Northwind Freight"" ""organization"""),
      To_Unbounded_String
        ("""Chicago"" ""place"""));
begin
   Recognize (Client, Kinds, Text, Names, Facts, Error);
   pragma Assert (Error.Kind = None);
   declare
      Entities : constant String :=
        Member (To_String (Names), "entities");
   begin
      for Row in Spans'Range loop
         declare
            One  : constant String :=
              Element (Entities, Row);
            Span : constant String :=
              Member (One, "text") & " "
              & Member (One, "kind");
         begin
            pragma Assert (Span = To_String (Spans (Row)));
         end;
      end loop;
   end;
end Sample;
