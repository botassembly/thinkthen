with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Sample is
   function "+" (Text : String) return Unbounded_String
     renames To_Unbounded_String;

   Client : Engine;
   Sings  : Unbounded_String;
   Facts  : Unbounded_String;
   Error  : Failure;

   Singer_Song : constant String :=
     "{""version"": 1, ""relate"": {""relations"": ["
     & "{""name"": ""sings"", ""source"": ""singer"", "
     & """target"": ""song""}]}}";
   Beatles : constant Evidence_Array :=
     (+"{""name"": ""Paul McCartney"", "
      & """kind"": ""singer""}",
      +"{""name"": ""Ringo Starr"", ""kind"": ""singer""}",
      +"{""name"": ""Yesterday"", ""kind"": ""song""}",
      +"{""name"": ""Octopus's Garden"", "
      & """kind"": ""song""}");
   Pairs : constant array (1 .. 2) of Unbounded_String :=
     (+"""Paul McCartney"" ""Yesterday""",
      +"""Ringo Starr"" ""Octopus's Garden""");
begin
   Relate
     (Client, Singer_Song, Beatles, Sings, Facts, Error);
   pragma Assert (Error.Kind = None);
   declare
      Edges : constant String :=
        Member (To_String (Sings), "edges");
   begin
      for Row in Pairs'Range loop
         declare
            Edge   : constant String :=
              Element (Edges, Row);
            Source : constant String :=
              Member (Member (Edge, "source"), "name");
            Target : constant String :=
              Member (Member (Edge, "target"), "name");
         begin
            pragma Assert
              (Source & " " & Target
               = To_String (Pairs (Row)));
         end;
      end loop;
   end;
end Sample;
