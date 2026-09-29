with Ada.Strings.Unbounded;
with Interfaces.C;
with Thinkthen_C;
package Thinkthen is
   type Failure is record
      Code : Interfaces.C.int := 0;
      Retryable : Boolean := False;
      Message : Ada.Strings.Unbounded.Unbounded_String;
   end record;
   -- Caller owns the engine and token. Never free either while another task calls.
   procedure Decide (Engine, Token : Thinkthen_C.Handle; Question, Evidence : String;
                     Deadline : Interfaces.Integer_64; Result : aliased in out Thinkthen_C.Answer;
                     Error : out Failure);
   function Call_Value (Envelope : Ada.Strings.Unbounded.Unbounded_String) return String;
   procedure JSON (Engine, Token : Thinkthen_C.Handle; Request : String;
                   Deadline : Interfaces.Integer_64; Result : out Ada.Strings.Unbounded.Unbounded_String;
                   Error : out Failure);
end Thinkthen;
