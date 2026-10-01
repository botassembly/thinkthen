with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Backends is
   Client           : Engine;
   Broken_Is_Refund : Decision;
   Thanks_Is_Refund : Decision;
   Facts            : Unbounded_String;
   Error            : Failure;

   Question : constant String :=
     "Does the customer ask for a refund?";
begin
   Decide
     (Client,
      Question,
      "Please refund my order. It arrived broken.",
      Broken_Is_Refund,
      Facts,
      Error);
   pragma Assert (Error.Kind = None);
   pragma Assert (Broken_Is_Refund.Value = Yes);

   Decide
     (Client,
      Question,
      "Thanks for the quick help yesterday!",
      Thanks_Is_Refund,
      Facts,
      Error);
   pragma Assert (Error.Kind = None);
   pragma Assert (Thanks_Is_Refund.Value = No);
end Backends;
