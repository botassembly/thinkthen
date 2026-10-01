with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Sample is
   Client   : Engine;
   Deadline : Unbounded_String;
   Error    : Failure;

   Which_Line : constant String :=
     "{""find"": ""Which line gives the refund "
     & "deadline?"", "
     & """units"": ["
     & """Returns need the original receipt."", "
     & """Refunds are issued within 30 days "
     & "of purchase."", "
     & """Shipping is free on orders over $50."", "
     & """Gift cards cannot be exchanged for cash.""]}";
begin
   Call (Client, Which_Line, Deadline, Error);
   pragma Assert (Error.Kind = None);
   declare
      Found : constant String :=
        Member (To_String (Deadline), "value");
   begin
      pragma Assert
        (Member (Found, "unit")
         = """Refunds are issued within 30 days "
           & "of purchase.""");
   end;
end Sample;
