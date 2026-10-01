with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Sample is
   Client      : Engine;
   Most_Urgent : Unbounded_String;
   Error       : Failure;

   Is_Urgent : constant String :=
     "{""rank"": ""Is this urgent?"", ""records"": ["
     & """Newsletter: our autumn catalog is here. "
     & "No reply needed."", "
     & """Our checkout page is down and customers "
     & "cannot pay"", "
     & """Reminder: your invoice is due in 30 days"", "
     & """Please send the signed quote by 5 pm today""]}";
   Expected : constant array (1 .. 4) of String (1 .. 1) :=
     ("1", "3", "2", "0");
begin
   Call (Client, Is_Urgent, Most_Urgent, Error);
   pragma Assert (Error.Kind = None);
   declare
      Order : constant String :=
        Member (To_String (Most_Urgent), "value");
   begin
      for Row in Expected'Range loop
         pragma Assert
           (Member (Element (Order, Row), "index")
            = Expected (Row));
      end loop;
   end;
end Sample;
