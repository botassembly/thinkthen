with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Sample is
   Client : Engine;
   Names  : Unbounded_String;
   Facts  : Unbounded_String;
   Error  : Failure;

   Kinds : constant String :=
     "{""version"": 1, ""recognize"": {""kinds"": {"
     & """PER"": ""Part of a person's name."", "
     & """ORG"": ""Part of the name of an organization: "
     & "a company, band, team, agency, government "
     & "body, or media outlet."", "
     & """LOC"": ""Part of the name of a place: "
     & "a country, region, city, or geographic "
     & "feature."", "
     & """MISC"": ""Part of another named entity: a "
     & "nationality, an event, a product, or the "
     & "name of a creative work.""}}}";
   Spans : constant array (1 .. 3) of Unbounded_String :=
     (To_Unbounded_String ("""Maria Chen"" ""PER"""),
      To_Unbounded_String ("""Northwind Freight"" ""ORG"""),
      To_Unbounded_String ("""Chicago"" ""LOC"""));
begin
   Recognize
     (Client,
      Kinds,
      "Maria Chen joined Northwind Freight in Chicago "
      & "last spring.",
      Names,
      Facts,
      Error);
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
