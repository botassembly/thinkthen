-- Counted native carriers. Typed data never uses a JSON buffer.
with Interfaces;
with Interfaces.C;
with System;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
package Thinkthen_C_Answers is
   C_ANSWER_YES_NO_V1 : constant Interfaces.Unsigned_32 := 1;
   C_ANSWER_CHOICE_V1 : constant Interfaces.Unsigned_32 := 2;
   C_ANSWER_TAG_V1 : constant Interfaces.Unsigned_32 := 3;
   C_ANSWER_SCORE_V1 : constant Interfaces.Unsigned_32 := 4;
   C_ANSWER_FIND_V1 : constant Interfaces.Unsigned_32 := 5;
   C_MEMBER_SUCCESS_V1 : constant Interfaces.Unsigned_32 := 1;
   C_MEMBER_FAILURE_V1 : constant Interfaces.Unsigned_32 := 2;
   C_MEMBER_MISSING_ANSWER_V1 : constant Interfaces.Unsigned_32 := 1;
   C_MEMBER_WRONG_KIND_V1 : constant Interfaces.Unsigned_32 := 2;
   C_MEMBER_MISSING_PROBABILITY_V1 : constant Interfaces.Unsigned_32 := 3;
   C_MEMBER_INVALID_PROBABILITY_V1 : constant Interfaces.Unsigned_32 := 4;
   C_MEMBER_INVALID_DISTRIBUTION_V1 : constant Interfaces.Unsigned_32 := 5;
   C_MEMBER_UNEXPECTED_PROBABILITY_V1 : constant Interfaces.Unsigned_32 := 6;
   type Decide_Value_V1_Data (Arm : Positive := 1) is record
      case Arm is
         when 1 => Boolean : Interfaces.C.int := 0;
         when 2 => Authored : Content_V1 := (others => <>);
         when others => null;
      end case;
   end record with Convention => C, Unchecked_Union;
   for Decide_Value_V1_Data use record
      Boolean at 0 range 0 .. 31;
      Authored at 0 range 0 .. 191;
   end record;
   for Decide_Value_V1_Data'Size use 192;
   for Decide_Value_V1_Data'Alignment use 8;
   type Decide_Value_V1 is record
      Kind : Interfaces.Unsigned_32 := 0;
      Data : Decide_Value_V1_Data := (others => <>);
   end record with Convention => C;
   for Decide_Value_V1 use record
      Kind at 0 range 0 .. 31;
      Data at 8 range 0 .. 191;
   end record;
   for Decide_Value_V1'Size use 256;
   for Decide_Value_V1'Alignment use 8;
   type Probability_V1 is record
      Name : Byte_String_V1 := (others => <>);
      Probability : Interfaces.C.double := 0.0;
   end record with Convention => C;
   for Probability_V1 use record
      Name at 0 range 0 .. 127;
      Probability at 16 range 0 .. 63;
   end record;
   for Probability_V1'Size use 192;
   for Probability_V1'Alignment use 8;
   type Probabilities_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Probabilities_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Probabilities_V1'Size use 128;
   for Probabilities_V1'Alignment use 8;
   type Optional_Probabilities_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Probabilities_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Probabilities_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 127;
   end record;
   for Optional_Probabilities_V1'Size use 192;
   for Optional_Probabilities_V1'Alignment use 8;
   type Named_Answer_V1 is record
      Pick : Byte_String_V1 := (others => <>);
      Probabilities : Probabilities_V1 := (others => <>);
      Confidence : Optional_Double_V1 := (others => <>);
   end record with Convention => C;
   for Named_Answer_V1 use record
      Pick at 0 range 0 .. 127;
      Probabilities at 16 range 0 .. 127;
      Confidence at 32 range 0 .. 127;
   end record;
   for Named_Answer_V1'Size use 384;
   for Named_Answer_V1'Alignment use 8;
   type Score_Answer_V1 is record
      Level : Byte_String_V1 := (others => <>);
      Probabilities : Probabilities_V1 := (others => <>);
      Confidence : Optional_Double_V1 := (others => <>);
   end record with Convention => C;
   for Score_Answer_V1 use record
      Level at 0 range 0 .. 127;
      Probabilities at 16 range 0 .. 127;
      Confidence at 32 range 0 .. 127;
   end record;
   for Score_Answer_V1'Size use 384;
   for Score_Answer_V1'Alignment use 8;
   type Answer_V1_Data (Arm : Positive := 1) is record
      case Arm is
         when 1 => Probability : Interfaces.C.double := 0.0;
         when 2 => Choice : Named_Answer_V1 := (others => <>);
         when 3 => Tag : Probabilities_V1 := (others => <>);
         when 4 => Score : Score_Answer_V1 := (others => <>);
         when 5 => Find : Named_Answer_V1 := (others => <>);
         when others => null;
      end case;
   end record with Convention => C, Unchecked_Union;
   for Answer_V1_Data use record
      Probability at 0 range 0 .. 63;
      Choice at 0 range 0 .. 383;
      Tag at 0 range 0 .. 127;
      Score at 0 range 0 .. 383;
      Find at 0 range 0 .. 383;
   end record;
   for Answer_V1_Data'Size use 384;
   for Answer_V1_Data'Alignment use 8;
   type Answer_V1 is record
      Kind : Interfaces.Unsigned_32 := 0;
      Data : Answer_V1_Data := (others => <>);
   end record with Convention => C;
   for Answer_V1 use record
      Kind at 0 range 0 .. 31;
      Data at 8 range 0 .. 383;
   end record;
   for Answer_V1'Size use 448;
   for Answer_V1'Alignment use 8;
   type Optional_Answer_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Answer_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Answer_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 447;
   end record;
   for Optional_Answer_V1'Size use 512;
   for Optional_Answer_V1'Alignment use 8;
   type Location_V1 is record
      File : Optional_String_V1 := (others => <>);
      First_Line : Optional_Size_V1 := (others => <>);
      Last_Line : Optional_Size_V1 := (others => <>);
   end record with Convention => C;
   for Location_V1 use record
      File at 0 range 0 .. 191;
      First_Line at 24 range 0 .. 127;
      Last_Line at 40 range 0 .. 127;
   end record;
   for Location_V1'Size use 448;
   for Location_V1'Alignment use 8;
   type Optional_Location_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Location_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Location_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 447;
   end record;
   for Optional_Location_V1'Size use 512;
   for Optional_Location_V1'Alignment use 8;
   type Member_Value_V1_Data (Arm : Positive := 1) is record
      case Arm is
         when 1 => Decide : Decide_Value_V1 := (others => <>);
         when 2 => Choose : Optional_String_V1 := (others => <>);
         when 3 => Tag : Strings_V1 := (others => <>);
         when 4 => Score : Interfaces.C.double := 0.0;
         when others => null;
      end case;
   end record with Convention => C, Unchecked_Union;
   for Member_Value_V1_Data use record
      Decide at 0 range 0 .. 255;
      Choose at 0 range 0 .. 191;
      Tag at 0 range 0 .. 127;
      Score at 0 range 0 .. 63;
   end record;
   for Member_Value_V1_Data'Size use 256;
   for Member_Value_V1_Data'Alignment use 8;
   type Member_Value_V1 is record
      Kind : Interfaces.Unsigned_32 := 0;
      Data : Member_Value_V1_Data := (others => <>);
   end record with Convention => C;
   for Member_Value_V1 use record
      Kind at 0 range 0 .. 31;
      Data at 8 range 0 .. 255;
   end record;
   for Member_Value_V1'Size use 320;
   for Member_Value_V1'Alignment use 8;
   type Member_Failure_V1 is record
      Failure_Id : Byte_String_V1 := (others => <>);
      Cause : Interfaces.Unsigned_32 := 0;
   end record with Convention => C;
   for Member_Failure_V1 use record
      Failure_Id at 0 range 0 .. 127;
      Cause at 16 range 0 .. 31;
   end record;
   for Member_Failure_V1'Size use 192;
   for Member_Failure_V1'Alignment use 8;
   type Member_Success_V1 is record
      Answer_Id : Byte_String_V1 := (others => <>);
      Value : Member_Value_V1 := (others => <>);
      Answer : Answer_V1 := (others => <>);
      Threshold : Rule_V1 := (others => <>);
   end record with Convention => C;
   for Member_Success_V1 use record
      Answer_Id at 0 range 0 .. 127;
      Value at 16 range 0 .. 319;
      Answer at 56 range 0 .. 447;
      Threshold at 112 range 0 .. 191;
   end record;
   for Member_Success_V1'Size use 1088;
   for Member_Success_V1'Alignment use 8;
   type Member_V1_Data (Arm : Positive := 1) is record
      case Arm is
         when 1 => Success : Member_Success_V1 := (others => <>);
         when 2 => Failure : Member_Failure_V1 := (others => <>);
         when others => null;
      end case;
   end record with Convention => C, Unchecked_Union;
   for Member_V1_Data use record
      Success at 0 range 0 .. 1087;
      Failure at 0 range 0 .. 191;
   end record;
   for Member_V1_Data'Size use 1088;
   for Member_V1_Data'Alignment use 8;
   type Member_V1 is record
      Name : Byte_String_V1 := (others => <>);
      Request : Byte_String_V1 := (others => <>);
      Question : Question_View_V1 := (others => <>);
      State : Interfaces.Unsigned_32 := 0;
      Data : Member_V1_Data := (others => <>);
   end record with Convention => C;
   for Member_V1 use record
      Name at 0 range 0 .. 127;
      Request at 16 range 0 .. 127;
      Question at 32 range 0 .. 2751;
      State at 376 range 0 .. 31;
      Data at 384 range 0 .. 1087;
   end record;
   for Member_V1'Size use 4160;
   for Member_V1'Alignment use 8;
   type Members_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Members_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Members_V1'Size use 128;
   for Members_V1'Alignment use 8;
end Thinkthen_C_Answers;
