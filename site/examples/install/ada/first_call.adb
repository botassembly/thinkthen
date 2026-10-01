with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure First_Call is
   Client    : Engine;
   Is_Refund : Decision;
   Facts     : Unbounded_String;
   Error     : Failure;

   Question : constant String :=
     "Does the customer ask for a refund?";
   Refund   : constant String :=
     "{""decide"": """ & Question & """, "
     & """threshold"": ""0.2:0.8""}";
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

   Decide
     (Client,
      Refund,
      "I want to send this back.",
      Is_Refund,
      Facts,
      Error);
   pragma Assert (Error.Kind = None);
   pragma Assert (Is_Refund.Value = Not_Sure);
end First_Call;
