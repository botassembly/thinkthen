with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Sample is
   Client : Engine;
   Owners : Unbounded_String;
   Error  : Failure;

   Which_Team : constant String :=
     "{""choose"": ""Which team owns this?"", "
     & """options"": {"
     & """billing"": ""Invoices, fees, and refunds."", "
     & """shipping"": ""Parcels and delivery."", "
     & """account"": ""Logins and passwords.""}, "
     & """evidence"": "
     & """My parcel went to the wrong address.""}";
begin
   Call (Client, Which_Team, Owners, Error);
   pragma Assert (Error.Kind = None);
   declare
      Team : constant String :=
        Member (To_String (Owners), "value");
   begin
      pragma Assert (Team = """shipping""");
   end;
end Sample;
