with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Sample is
   Client    : Engine;
   Is_Refund : Decision;
   Facts     : Unbounded_String;
   Error     : Failure;

   Question : constant String :=
     "Does the customer ask for a refund?";
begin
   Decide
     (Client,
      Question,
      "Please refund my order. It arrived broken.",
      Is_Refund,
      Facts,
      Error);
   pragma Assert (Error.Kind = None);
   pragma Assert (Is_Refund.Value = Yes);
end Sample;
