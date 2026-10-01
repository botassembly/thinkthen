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
     & """records"": ["
     & """Please refund the extra fee on my invoice."", "
     & """My parcel went to the wrong address."", "
     & """I cannot reset my password.""]}";
begin
   Call (Client, Which_Team, Owners, Error);
   pragma Assert (Error.Kind = None);
   declare
      Teams : constant String :=
        Member (To_String (Owners), "value");
   begin
      pragma Assert
        (Teams = "[""billing"",""shipping"",""account""]");
   end;
end Sample;
