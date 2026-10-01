with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Sample is
   Client  : Engine;
   Fitting : Unbounded_String;
   Error   : Failure;

   Which_Labels : constant String :=
     "{""tag"": ""Which labels fit this message?"", "
     & """labels"": [""praise"", ""bug"", ""billing""], "
     & """evidence"": ""Love the new dashboard, but "
     & "export crashes the app,\nand I was charged "
     & "twice.\n""}";
begin
   Call (Client, Which_Labels, Fitting, Error);
   pragma Assert (Error.Kind = None);
   declare
      Labels : constant String :=
        Member (To_String (Fitting), "value");
   begin
      pragma Assert
        (Labels = "[""praise"",""bug"",""billing""]");
   end;
end Sample;
