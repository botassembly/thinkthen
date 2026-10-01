with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Sample is
   Client  : Engine;
   Urgency : Unbounded_String;
   Error   : Failure;

   How_Urgent : constant String :=
     "{""score"": ""How urgent is this?"", "
     & """levels"": [""Routine."", ""Soon."", "
     & """Immediate.""], "
     & """evidence"": ""Our checkout page is down "
     & "and customers cannot pay.\n""}";
begin
   Call (Client, How_Urgent, Urgency, Error);
   pragma Assert (Error.Kind = None);
   pragma Assert
     (Member (To_String (Urgency), "value") = "2.0");
end Sample;
