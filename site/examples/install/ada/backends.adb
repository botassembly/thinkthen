with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Backends is
   Client           : Engine;
   Broken_Is_Refund : Decision;
   Thanks_Is_Refund : Decision;
   Facts            : Unbounded_String;
   Error            : Failure;

   Settings : constant array (1 .. 3) of Unbounded_String :=
     (To_Unbounded_String ("{""backend"":""typesafe""}"),
      To_Unbounded_String ("{""backend"":""liquid""}"),
      To_Unbounded_String
        ("{""backend"":""ollama"",""base_url"":" &
         """http://localhost:11535/v1""}"));

   Question : constant String :=
     "Does the customer ask for a refund?";
begin
   for Setting of Settings loop
      Configure (Client, To_String (Setting), Error);
      pragma Assert (Error.Kind = None);
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
   end loop;
end Backends;
