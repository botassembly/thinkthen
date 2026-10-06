with Interfaces; use Interfaces;
with Interfaces.C; use Interfaces.C;
with System;
with Thinkthen.Buffers;
with Thinkthen.Views; use Thinkthen.Views;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
with Thinkthen_C_Answers; use Thinkthen_C_Answers;
with Thinkthen_C_Metadata; use Thinkthen_C_Metadata;
with Thinkthen_C_Rows; use Thinkthen_C_Rows;
with Thinkthen_C_Events; use Thinkthen_C_Events;
procedure Carrier_Bounds is
   Text, Description, Label, Pointer : Thinkthen.Buffers.Buffer;
   Choices : aliased array (0 .. 1) of Choice_V1;
   Pointers : aliased array (0 .. 1) of Byte_String_V1;
   Spec : aliased Question_Spec_V1;
   Record_Input : aliased Record_Data_V1;
   Summary_View : aliased Summary_V1;
   Row : aliased Decide_View_V1;
   Member : aliased Member_V1;
   Fixture_Record : aliased Record_Data_V1;
   function Fixture (S, R, M, I : System.Address) return int
     with Import, Convention => C, External_Name => "tt_carrier_case";
   function Request (S, I : System.Address; Kind : Unsigned_32) return int
     with Import, Convention => C, External_Name => "tt_carrier_request";
   procedure Require (Condition : Boolean) is
   begin
      if not Condition then raise Program_Error with "typed carrier boundary"; end if;
   end Require;
   procedure Refuses_Invalid_List is
      Invalid : Probabilities_V1 := (System.Null_Address, 1);
      Unused : Probability_V1;
   begin
      begin
         Unused := Element (Invalid, 0);
         raise Program_Error with "null nonempty list accepted";
      exception when Constraint_Error => null; end;
      Invalid := (Text'Address, size_t'Last);
      begin
         Unused := Element (Invalid, 0);
         raise Program_Error with "overflowing list accepted";
      exception when Constraint_Error => null; end;
      Invalid := (Choices'Address, 0);
      begin
         Unused := Element (Invalid, 0);
         raise Program_Error with "empty list indexed";
      exception when Constraint_Error => null; end;
   end Refuses_Invalid_List;
begin
   Require (Record_Data_V1'Size = 96 * 8 and Question_Spec_V1'Size = 344 * 8
            and Summary_V1'Size = 712 * 8);
   Thinkthen.Buffers.Set (Text, (1 .. 9000 => 'x'));
   Thinkthen.Buffers.Set (Description, "{""what"":""keep"",""examples"":[""a"",""a""]}");
   Thinkthen.Buffers.Set (Label, "same");
   Thinkthen.Buffers.Set (Pointer, "/field");
   for I in Choices'Range loop
      Choices (I) := (Name => Thinkthen.Buffers.View (Label),
         Description => (1, Thinkthen.Buffers.Authored_JSON (Description)), others => <>);
      Pointers (I) := Thinkthen.Buffers.View (Pointer);
   end loop;
   Spec.Choices := (Choices'Address, 2);
   Spec.On := (Pointers'Address, 2);
   Record_Input := (Original => (1, Thinkthen.Buffers.Text (Text)),
      Context => (1, (Kind => C_CONTENT_TEXT_V1, others => <>)),
      Options => Spec.Choices, others => <>);
   for Kind in Unsigned_32 range 1 .. 10 loop
      Spec.Kind := Kind;
      Require (Request (Spec'Address, Record_Input'Address, Kind) = 0);
   end loop;
   Require (Value (Element (Spec.Choices, 1).Name) = "same");
   Refuses_Invalid_List;
   begin
      declare
         Unused : constant String := Value ((System.Null_Address, size_t (Natural'Last) + 1));
      begin
         raise Program_Error with "Ada String bound accepted " & Unused;
      end;
   exception when Constraint_Error => null; end;
   Require (Fixture (Summary_View'Address, Row'Address, Member'Address, Fixture_Record'Address) = 0);
   Require (Value (Summary_View.Schema) = "thinkthen.result/2");
   Require (Summary_View.Facts.Value.Input_Tokens.Present = 1
            and Summary_View.Facts.Value.Input_Tokens.Value = Unsigned_64'Last
            and Summary_View.Facts.Value.Output_Tokens.Present = 0);
   Require (Value (Summary_View.Facts.Value.Estimated_Cost_Usd.Value) = "0.000123");
   Require (Summary_View.Attempts.Present = 1 and Summary_View.Attempts.Value.Len = 0);
   Require (Value (Element (Summary_View.Meta.Value.Requests, 0)) = "repeat"
            and Value (Element (Summary_View.Meta.Value.Requests, 1)) = "repeat");
   Require (Element (Summary_View.Meta.Value.Observations, 1).Kind = C_ID_FAILURE_V1);
   Require (Row.Value.Kind = C_DECIDE_BOOLEAN_V1 and Row.Value.Data.Boolean = 0);
   Require (Member.State = C_MEMBER_SUCCESS_V1
            and Member.Data.Success.Value.Data.Decide.Kind = C_DECIDE_NULL_V1);
   Require (Element (Member.Data.Success.Answer.Data.Tag, 0).Probability = 0.0
            and Element (Member.Data.Success.Answer.Data.Tag, 1).Probability = 1.0);
   Require (Value (Fixture_Record.Original.Value.Data) = "é" & ASCII.CR & ASCII.LF & ASCII.NUL & "z");
   Require (Fixture_Record.Context.Present = 1 and Fixture_Record.Context.Value.Data.Len = 0);
   Require (Row.Common.Position.Value.First_Line.Value = 2
            and Row.Common.Position.Value.Last_Line.Value = 4);
end Carrier_Bounds;
