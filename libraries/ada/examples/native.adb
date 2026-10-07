with Ada.Environment_Variables;
with Ada.Text_IO;
with Interfaces; use type Interfaces.Unsigned_32;
with Interfaces.C; use Interfaces.C;
with Thinkthen; use Thinkthen;
with Thinkthen.Typed; use Thinkthen.Typed;
with Thinkthen.Typed.Complete; use Thinkthen.Typed.Complete;
with Thinkthen.Views;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
with Thinkthen_C_Rows; use Thinkthen_C_Rows;
with Thinkthen_C_Extensions; use Thinkthen_C_Extensions;
procedure Native is
   Output : Result;
   Row : Decide_View_V1;
   Detail : Details_V1;
   Code : int;
   procedure Require (Condition : Boolean) is
   begin
      if not Condition then raise Program_Error with "native ownership"; end if;
   end Require;
begin
   declare
      Client : Engine;
      Error : Failure;
      Q : Question;
      Input : Source;
      Question_Text : aliased String := "Refund?";
      Evidence : aliased String (1 .. 9_000) := (others => 'x');
      Spec : Question_Spec_V1 :=
        (Kind => C_FUNCTION_DECIDE_V1,
         Text => (C_CONTENT_TEXT_V1, (Question_Text'Address, Question_Text'Length)), others => <>);
      Records_Input : Record_Array (1 .. 1);
   begin
      Configure (Client, Ada.Environment_Variables.Value ("TT_NATIVE_SETTINGS"), Error);
      Require (Error.Kind = Thinkthen.None);
      New_Question (Client, Spec, Q, Code); Require (Code = 0);
      Records_Input (1).Original :=
        (1, (C_CONTENT_TEXT_V1, (Evidence'Address, Evidence'Length)));
      Records (Client, Records_Input, Input, Code); Require (Code = 0);
      -- Constructors own independent snapshots before these buffers change.
      Question_Text := (others => 'z');
      Evidence := (others => 'z');
      Decide (Client, Q, Input, Controls, Output, Code); Require (Code = 0);
   end;
   -- The immutable result survives engine, question and source destruction.
   Decide (Output, 0, Row, Code); Require (Code = 0);
   Require (Row.Value.Kind = C_DECIDE_BOOLEAN_V1 and then Row.Value.Data.Boolean = 1);
   Require (Thinkthen.Views.Value (Row.Common.Answer_Id)'Length = 64);
   Details (Output, 0, Detail, Code); Require (Code = 0);
   Require (Thinkthen.Views.Element (Detail.Inputs, 0).Original.Present = 1);
   Require (Thinkthen.Views.Value
     (Thinkthen.Views.Element (Detail.Inputs, 0).Original.Value.Data) = String'(1 .. 9_000 => 'x'));
   Ada.Text_IO.Put_Line ("ADA_NATIVE_PASS");
end Native;
