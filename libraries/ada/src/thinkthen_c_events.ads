-- Counted native carriers. Typed data never uses a JSON buffer.
with Interfaces;
with Interfaces.C;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
with Thinkthen_C_Answers; use Thinkthen_C_Answers;
with Thinkthen_C_Metadata; use Thinkthen_C_Metadata;
with Thinkthen_C_Rows; use Thinkthen_C_Rows;
package Thinkthen_C_Events is
   C_EVENT_QUESTION_V1 : constant Interfaces.Unsigned_32 := 1;
   C_EVENT_ROW_V1 : constant Interfaces.Unsigned_32 := 2;
   C_RESULT_SUCCESS_V1 : constant Interfaces.Unsigned_32 := 1;
   C_RESULT_FAILURE_V1 : constant Interfaces.Unsigned_32 := 2;
   type Observed_Probabilities_V1_Data (Arm : Positive := 1) is record
      case Arm is
         when 1 => Yes : Interfaces.C.double := 0.0;
         when 2 => Named : Probabilities_V1 := (others => <>);
         when others => null;
      end case;
   end record with Convention => C, Unchecked_Union;
   for Observed_Probabilities_V1_Data use record
      Yes at 0 range 0 .. 63;
      Named at 0 range 0 .. 127;
   end record;
   for Observed_Probabilities_V1_Data'Size use 128;
   for Observed_Probabilities_V1_Data'Alignment use 8;
   type Observed_Probabilities_V1 is record
      Kind : Interfaces.Unsigned_32 := 0;
      Data : Observed_Probabilities_V1_Data := (others => <>);
   end record with Convention => C;
   for Observed_Probabilities_V1 use record
      Kind at 0 range 0 .. 31;
      Data at 8 range 0 .. 127;
   end record;
   for Observed_Probabilities_V1'Size use 192;
   for Observed_Probabilities_V1'Alignment use 8;
   type Observation_Success_V1 is record
      Answer_Id : Byte_String_V1 := (others => <>);
      Observation_Id : Byte_String_V1 := (others => <>);
      Value : Member_Value_V1 := (others => <>);
      Probabilities : Observed_Probabilities_V1 := (others => <>);
      Confidence : Optional_Double_V1 := (others => <>);
   end record with Convention => C;
   for Observation_Success_V1 use record
      Answer_Id at 0 range 0 .. 127;
      Observation_Id at 16 range 0 .. 127;
      Value at 32 range 0 .. 319;
      Probabilities at 72 range 0 .. 191;
      Confidence at 96 range 0 .. 127;
   end record;
   for Observation_Success_V1'Size use 896;
   for Observation_Success_V1'Alignment use 8;
   type Question_Observation_V1_Data (Arm : Positive := 1) is record
      case Arm is
         when 1 => Success : Observation_Success_V1 := (others => <>);
         when 2 => Failure : Member_Failure_V1 := (others => <>);
         when others => null;
      end case;
   end record with Convention => C, Unchecked_Union;
   for Question_Observation_V1_Data use record
      Success at 0 range 0 .. 895;
      Failure at 0 range 0 .. 191;
   end record;
   for Question_Observation_V1_Data'Size use 896;
   for Question_Observation_V1_Data'Alignment use 8;
   type Question_Observation_V1 is record
      Index : Interfaces.C.size_t := 0;
      Member : Optional_String_V1 := (others => <>);
      Stage : Optional_Discriminator_V1 := (others => <>);
      Position : Interfaces.C.size_t := 0;
      Question_Sha256 : Byte_String_V1 := (others => <>);
      Model : Byte_String_V1 := (others => <>);
      Url : Byte_String_V1 := (others => <>);
      Requests : Strings_V1 := (others => <>);
      Requests_Sent : Interfaces.Unsigned_64 := 0;
      Cached : Interfaces.C.int := 0;
      Failed_Questions : Interfaces.C.size_t := 0;
      Usage : Optional_Usage_V1 := (others => <>);
      Question_Sources : Question_Sources_V1 := (others => <>);
      State : Interfaces.Unsigned_32 := 0;
      Data : Question_Observation_V1_Data := (others => <>);
   end record with Convention => C;
   for Question_Observation_V1 use record
      Index at 0 range 0 .. 63;
      Member at 8 range 0 .. 191;
      Stage at 32 range 0 .. 63;
      Position at 40 range 0 .. 63;
      Question_Sha256 at 48 range 0 .. 127;
      Model at 64 range 0 .. 127;
      Url at 80 range 0 .. 127;
      Requests at 96 range 0 .. 127;
      Requests_Sent at 112 range 0 .. 63;
      Cached at 120 range 0 .. 31;
      Failed_Questions at 128 range 0 .. 63;
      Usage at 136 range 0 .. 191;
      Question_Sources at 160 range 0 .. 127;
      State at 176 range 0 .. 31;
      Data at 184 range 0 .. 895;
   end record;
   for Question_Observation_V1'Size use 2368;
   for Question_Observation_V1'Alignment use 8;
   type Row_Observation_V1_Data (Arm : Positive := 1) is record
      case Arm is
         when 1 => Decide : Decide_View_V1 := (others => <>);
         when 2 => Choose : Choose_View_V1 := (others => <>);
         when 3 => Tag : Tag_View_V1 := (others => <>);
         when 4 => Score : Score_View_V1 := (others => <>);
         when 5 => Filter : Filter_View_V1 := (others => <>);
         when 6 => Rank : Rank_View_V1 := (others => <>);
         when 7 => Find : Find_View_V1 := (others => <>);
         when 8 => Annotate : Annotate_View_V1 := (others => <>);
         when 9 => Recognize : Recognize_View_V1 := (others => <>);
         when 10 => Relate : Relate_View_V1 := (others => <>);
         when others => null;
      end case;
   end record with Convention => C, Unchecked_Union;
   for Row_Observation_V1_Data use record
      Decide at 0 range 0 .. 8127;
      Choose at 0 range 0 .. 8063;
      Tag at 0 range 0 .. 7999;
      Score at 0 range 0 .. 7935;
      Filter at 0 range 0 .. 7935;
      Rank at 0 range 0 .. 8191;
      Find at 0 range 0 .. 8255;
      Annotate at 0 range 0 .. 7999;
      Recognize at 0 range 0 .. 8575;
      Relate at 0 range 0 .. 8127;
   end record;
   for Row_Observation_V1_Data'Size use 8576;
   for Row_Observation_V1_Data'Alignment use 8;
   type Row_Observation_V1 is record
      Index : Interfaces.C.size_t := 0;
      Function_Code : Interfaces.Unsigned_32 := 0;
      Data : Row_Observation_V1_Data := (others => <>);
   end record with Convention => C;
   for Row_Observation_V1 use record
      Index at 0 range 0 .. 63;
      Function_Code at 8 range 0 .. 31;
      Data at 16 range 0 .. 8575;
   end record;
   for Row_Observation_V1'Size use 8704;
   for Row_Observation_V1'Alignment use 8;
   type Observation_V1_Data (Arm : Positive := 1) is record
      case Arm is
         when 1 => Question : Question_Observation_V1 := (others => <>);
         when 2 => Row : Row_Observation_V1 := (others => <>);
         when others => null;
      end case;
   end record with Convention => C, Unchecked_Union;
   for Observation_V1_Data use record
      Question at 0 range 0 .. 2367;
      Row at 0 range 0 .. 8703;
   end record;
   for Observation_V1_Data'Size use 8704;
   for Observation_V1_Data'Alignment use 8;
   type Observation_V1 is record
      Kind : Interfaces.Unsigned_32 := 0;
      Data : Observation_V1_Data := (others => <>);
   end record with Convention => C;
   for Observation_V1 use record
      Kind at 0 range 0 .. 31;
      Data at 8 range 0 .. 8703;
   end record;
   for Observation_V1'Size use 8768;
   for Observation_V1'Alignment use 8;
   type Summary_V1 is record
      State : Interfaces.Unsigned_32 := 0;
      Schema : Byte_String_V1 := (others => <>);
      Answer_Id : Optional_String_V1 := (others => <>);
      Function_Code : Optional_Discriminator_V1 := (others => <>);
      Count : Interfaces.C.size_t := 0;
      Observation_Count : Interfaces.C.size_t := 0;
      Meta : Optional_Meta_V1 := (others => <>);
      Facts : Optional_Facts_V1 := (others => <>);
      Attempts : Optional_Attempts_V1 := (others => <>);
      Error : Optional_Error_V1 := (others => <>);
   end record with Convention => C;
   for Summary_V1 use record
      State at 0 range 0 .. 31;
      Schema at 8 range 0 .. 127;
      Answer_Id at 24 range 0 .. 191;
      Function_Code at 48 range 0 .. 63;
      Count at 56 range 0 .. 63;
      Observation_Count at 64 range 0 .. 63;
      Meta at 72 range 0 .. 3071;
      Facts at 456 range 0 .. 1215;
      Attempts at 608 range 0 .. 191;
      Error at 632 range 0 .. 639;
   end record;
   for Summary_V1'Size use 5696;
   for Summary_V1'Alignment use 8;
end Thinkthen_C_Events;
