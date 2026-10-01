with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Sample is
   Client       : Engine;
   Is_Complaint : Unbounded_String;
   Error        : Failure;

   Reviews : constant String :=
     "{""filter"": ""Is this a complaint?"", "
     & """records"": ["
     & """Arrived a day early. Thank you!"", "
     & """The zipper broke the first time I used it."", "
     & """Does this come in blue?"", "
     & """The strap snapped on day two.""]}";
begin
   Call (Client, Reviews, Is_Complaint, Error);
   pragma Assert (Error.Kind = None);
   declare
      Complaints : constant String :=
        Member (To_String (Is_Complaint), "value");
   begin
      pragma Assert
        (Element (Complaints, 1)
         = """The zipper broke the first time "
           & "I used it.""");
      pragma Assert
        (Element (Complaints, 2)
         = """The strap snapped on day two.""");
      pragma Assert (Element (Complaints, 3) = "");
   end;
end Sample;
