-- Unpublished 0430 typed C carriers. Counted data never uses a JSON buffer.
with Interfaces;
with Interfaces.C;
with System;
package Thinkthen_C_Inputs is
   C_CONTENT_TEXT_V1 : constant Interfaces.Unsigned_32 := 1;
   C_CONTENT_JSON_V1 : constant Interfaces.Unsigned_32 := 2;
   C_RULE_DEFAULT_V1 : constant Interfaces.Unsigned_32 := 0;
   C_RULE_NULL_V1 : constant Interfaces.Unsigned_32 := 1;
   C_RULE_CUT_V1 : constant Interfaces.Unsigned_32 := 2;
   C_RULE_BAND_V1 : constant Interfaces.Unsigned_32 := 3;
   C_FUNCTION_DECIDE_V1 : constant Interfaces.Unsigned_32 := 1;
   C_FUNCTION_CHOOSE_V1 : constant Interfaces.Unsigned_32 := 2;
   C_FUNCTION_TAG_V1 : constant Interfaces.Unsigned_32 := 3;
   C_FUNCTION_SCORE_V1 : constant Interfaces.Unsigned_32 := 4;
   C_FUNCTION_FILTER_V1 : constant Interfaces.Unsigned_32 := 5;
   C_FUNCTION_RANK_V1 : constant Interfaces.Unsigned_32 := 6;
   C_FUNCTION_FIND_V1 : constant Interfaces.Unsigned_32 := 7;
   C_FUNCTION_ANNOTATE_V1 : constant Interfaces.Unsigned_32 := 8;
   C_FUNCTION_RECOGNIZE_V1 : constant Interfaces.Unsigned_32 := 9;
   C_FUNCTION_RELATE_V1 : constant Interfaces.Unsigned_32 := 10;
   C_IMAGE_JPEG_V1 : constant Interfaces.Unsigned_32 := 1;
   C_IMAGE_PNG_V1 : constant Interfaces.Unsigned_32 := 2;
   C_SOURCE_LINE_V1 : constant Interfaces.Unsigned_32 := 1;
   C_SOURCE_WINDOW_V1 : constant Interfaces.Unsigned_32 := 2;
   C_SOURCE_FILE_V1 : constant Interfaces.Unsigned_32 := 3;
   C_SOURCE_JSONL_V1 : constant Interfaces.Unsigned_32 := 5;
   C_SOURCE_IMAGE_FILE_V1 : constant Interfaces.Unsigned_32 := 4;
   C_DECIDE_NULL_V1 : constant Interfaces.Unsigned_32 := 0;
   C_DECIDE_BOOLEAN_V1 : constant Interfaces.Unsigned_32 := 1;
   C_DECIDE_AUTHORED_V1 : constant Interfaces.Unsigned_32 := 2;
   type Byte_String_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C_Pass_By_Copy;
   for Byte_String_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Byte_String_V1'Size use 128;
   for Byte_String_V1'Alignment use 8;
   type Strings_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Strings_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Strings_V1'Size use 128;
   for Strings_V1'Alignment use 8;
   type Optional_String_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Byte_String_V1 := (others => <>);
   end record with Convention => C_Pass_By_Copy;
   for Optional_String_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 127;
   end record;
   for Optional_String_V1'Size use 192;
   for Optional_String_V1'Alignment use 8;
   type Optional_Size_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Optional_Size_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 63;
   end record;
   for Optional_Size_V1'Size use 128;
   for Optional_Size_V1'Alignment use 8;
   type Optional_U64_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Interfaces.Unsigned_64 := 0;
   end record with Convention => C;
   for Optional_U64_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 63;
   end record;
   for Optional_U64_V1'Size use 128;
   for Optional_U64_V1'Alignment use 8;
   type Optional_U16_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Interfaces.Unsigned_16 := 0;
   end record with Convention => C;
   for Optional_U16_V1 use record
      Present at 0 range 0 .. 31;
      Value at 4 range 0 .. 15;
   end record;
   for Optional_U16_V1'Size use 64;
   for Optional_U16_V1'Alignment use 4;
   type Optional_Double_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Interfaces.C.double := 0.0;
   end record with Convention => C;
   for Optional_Double_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 63;
   end record;
   for Optional_Double_V1'Size use 128;
   for Optional_Double_V1'Alignment use 8;
   type Optional_Discriminator_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Interfaces.Unsigned_32 := 0;
   end record with Convention => C;
   for Optional_Discriminator_V1 use record
      Present at 0 range 0 .. 31;
      Value at 4 range 0 .. 31;
   end record;
   for Optional_Discriminator_V1'Size use 64;
   for Optional_Discriminator_V1'Alignment use 4;
   type Content_V1 is record
      Kind : Interfaces.Unsigned_32 := 0;
      Data : Byte_String_V1 := (others => <>);
   end record with Convention => C;
   for Content_V1 use record
      Kind at 0 range 0 .. 31;
      Data at 8 range 0 .. 127;
   end record;
   for Content_V1'Size use 192;
   for Content_V1'Alignment use 8;
   type Optional_Content_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Content_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Content_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 191;
   end record;
   for Optional_Content_V1'Size use 256;
   for Optional_Content_V1'Alignment use 8;
   type Rule_V1 is record
      Kind : Interfaces.Unsigned_32 := 0;
      Low : Interfaces.C.double := 0.0;
      High : Interfaces.C.double := 0.0;
   end record with Convention => C;
   for Rule_V1 use record
      Kind at 0 range 0 .. 31;
      Low at 8 range 0 .. 63;
      High at 16 range 0 .. 63;
   end record;
   for Rule_V1'Size use 192;
   for Rule_V1'Alignment use 8;
   type Optional_Rule_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Rule_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Rule_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 191;
   end record;
   for Optional_Rule_V1'Size use 256;
   for Optional_Rule_V1'Alignment use 8;
   type Choice_V1 is record
      Name : Byte_String_V1 := (others => <>);
      Description : Optional_Content_V1 := (others => <>);
      Weight : Optional_Double_V1 := (others => <>);
   end record with Convention => C;
   for Choice_V1 use record
      Name at 0 range 0 .. 127;
      Description at 16 range 0 .. 255;
      Weight at 48 range 0 .. 127;
   end record;
   for Choice_V1'Size use 512;
   for Choice_V1'Alignment use 8;
   type Choices_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Choices_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Choices_V1'Size use 128;
   for Choices_V1'Alignment use 8;
   type Relation_V1 is record
      Name : Byte_String_V1 := (others => <>);
      Source : Byte_String_V1 := (others => <>);
      Target : Byte_String_V1 := (others => <>);
      Reads : Optional_String_V1 := (others => <>);
      Either : Interfaces.C.int := 0;
      Single : Interfaces.C.int := 0;
   end record with Convention => C;
   for Relation_V1 use record
      Name at 0 range 0 .. 127;
      Source at 16 range 0 .. 127;
      Target at 32 range 0 .. 127;
      Reads at 48 range 0 .. 191;
      Either at 72 range 0 .. 31;
      Single at 76 range 0 .. 31;
   end record;
   for Relation_V1'Size use 640;
   for Relation_V1'Alignment use 8;
   type Relations_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Relations_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Relations_V1'Size use 128;
   for Relations_V1'Alignment use 8;
   type Member_Spec_V1 is record
      Name : Byte_String_V1 := (others => <>);
      Question : System.Address := System.Null_Address;
   end record with Convention => C;
   for Member_Spec_V1 use record
      Name at 0 range 0 .. 127;
      Question at 16 range 0 .. 63;
   end record;
   for Member_Spec_V1'Size use 192;
   for Member_Spec_V1'Alignment use 8;
   type Member_Specs_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Member_Specs_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Member_Specs_V1'Size use 128;
   for Member_Specs_V1'Alignment use 8;
   type Question_Spec_V1 is record
      Kind : Interfaces.Unsigned_32 := 0;
      Text : Content_V1 := (others => <>);
      Yes : Optional_Content_V1 := (others => <>);
      No : Optional_Content_V1 := (others => <>);
      Choices : Choices_V1 := (others => <>);
      Threshold : Rule_V1 := (others => <>);
      Relation_Threshold : Rule_V1 := (others => <>);
      Model : Optional_String_V1 := (others => <>);
      Profile : Optional_String_V1 := (others => <>);
      Batch : Optional_Size_V1 := (others => <>);
      Batch_Max : Interfaces.C.int := 0;
      None : Interfaces.C.int := 0;
      On : Strings_V1 := (others => <>);
      Members : Member_Specs_V1 := (others => <>);
      Kinds : Choices_V1 := (others => <>);
      Relations : Relations_V1 := (others => <>);
      Name_Pointer : Optional_String_V1 := (others => <>);
      Kind_Pointer : Optional_String_V1 := (others => <>);
   end record with Convention => C;
   for Question_Spec_V1 use record
      Kind at 0 range 0 .. 31;
      Text at 8 range 0 .. 191;
      Yes at 32 range 0 .. 255;
      No at 64 range 0 .. 255;
      Choices at 96 range 0 .. 127;
      Threshold at 112 range 0 .. 191;
      Relation_Threshold at 136 range 0 .. 191;
      Model at 160 range 0 .. 191;
      Profile at 184 range 0 .. 191;
      Batch at 208 range 0 .. 127;
      Batch_Max at 224 range 0 .. 31;
      None at 228 range 0 .. 31;
      On at 232 range 0 .. 127;
      Members at 248 range 0 .. 127;
      Kinds at 264 range 0 .. 127;
      Relations at 280 range 0 .. 127;
      Name_Pointer at 296 range 0 .. 191;
      Kind_Pointer at 320 range 0 .. 191;
   end record;
   for Question_Spec_V1'Size use 2752;
   for Question_Spec_V1'Alignment use 8;
   type Question_Member_V1 is record
      Name : Byte_String_V1 := (others => <>);
      Question : System.Address := System.Null_Address;
   end record with Convention => C;
   for Question_Member_V1 use record
      Name at 0 range 0 .. 127;
      Question at 16 range 0 .. 63;
   end record;
   for Question_Member_V1'Size use 192;
   for Question_Member_V1'Alignment use 8;
   type Question_Members_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Question_Members_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Question_Members_V1'Size use 128;
   for Question_Members_V1'Alignment use 8;
   type Question_View_V1 is record
      Kind : Interfaces.Unsigned_32 := 0;
      Text : Content_V1 := (others => <>);
      Yes : Optional_Content_V1 := (others => <>);
      No : Optional_Content_V1 := (others => <>);
      Choices : Choices_V1 := (others => <>);
      Threshold : Rule_V1 := (others => <>);
      Relation_Threshold : Rule_V1 := (others => <>);
      Model : Optional_String_V1 := (others => <>);
      Profile : Optional_String_V1 := (others => <>);
      Batch : Optional_Size_V1 := (others => <>);
      Batch_Max : Interfaces.C.int := 0;
      None : Interfaces.C.int := 0;
      On : Strings_V1 := (others => <>);
      Members : Question_Members_V1 := (others => <>);
      Kinds : Choices_V1 := (others => <>);
      Relations : Relations_V1 := (others => <>);
      Name_Pointer : Optional_String_V1 := (others => <>);
      Kind_Pointer : Optional_String_V1 := (others => <>);
   end record with Convention => C;
   for Question_View_V1 use record
      Kind at 0 range 0 .. 31;
      Text at 8 range 0 .. 191;
      Yes at 32 range 0 .. 255;
      No at 64 range 0 .. 255;
      Choices at 96 range 0 .. 127;
      Threshold at 112 range 0 .. 191;
      Relation_Threshold at 136 range 0 .. 191;
      Model at 160 range 0 .. 191;
      Profile at 184 range 0 .. 191;
      Batch at 208 range 0 .. 127;
      Batch_Max at 224 range 0 .. 31;
      None at 228 range 0 .. 31;
      On at 232 range 0 .. 127;
      Members at 248 range 0 .. 127;
      Kinds at 264 range 0 .. 127;
      Relations at 280 range 0 .. 127;
      Name_Pointer at 296 range 0 .. 191;
      Kind_Pointer at 320 range 0 .. 191;
   end record;
   for Question_View_V1'Size use 2752;
   for Question_View_V1'Alignment use 8;
   type Optional_Question_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Question_View_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Question_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 2751;
   end record;
   for Optional_Question_V1'Size use 2816;
   for Optional_Question_V1'Alignment use 8;
   type Images_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Images_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Images_V1'Size use 128;
   for Images_V1'Alignment use 8;
   type Image_View_V1 is record
      Media : Interfaces.Unsigned_32 := 0;
      Bytes : System.Address := System.Null_Address;
      Bytes_Len : Interfaces.C.size_t := 0;
      Width : Interfaces.Unsigned_32 := 0;
      Height : Interfaces.Unsigned_32 := 0;
      Filename : Optional_String_V1 := (others => <>);
   end record with Convention => C;
   for Image_View_V1 use record
      Media at 0 range 0 .. 31;
      Bytes at 8 range 0 .. 63;
      Bytes_Len at 16 range 0 .. 63;
      Width at 24 range 0 .. 31;
      Height at 28 range 0 .. 31;
      Filename at 32 range 0 .. 191;
   end record;
   for Image_View_V1'Size use 448;
   for Image_View_V1'Alignment use 8;
   type Image_Views_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Image_Views_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Image_Views_V1'Size use 128;
   for Image_Views_V1'Alignment use 8;
   type Optional_Image_Views_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Image_Views_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Image_Views_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 127;
   end record;
   for Optional_Image_Views_V1'Size use 192;
   for Optional_Image_Views_V1'Alignment use 8;
   type Record_Data_V1 is record
      Original : Optional_Content_V1 := (others => <>);
      Context : Optional_Content_V1 := (others => <>);
      Options : Choices_V1 := (others => <>);
      Images : Images_V1 := (others => <>);
   end record with Convention => C;
   for Record_Data_V1 use record
      Original at 0 range 0 .. 255;
      Context at 32 range 0 .. 255;
      Options at 64 range 0 .. 127;
      Images at 80 range 0 .. 127;
   end record;
   for Record_Data_V1'Size use 768;
   for Record_Data_V1'Alignment use 8;
   type Source_Spec_V1 is record
      Paths : Strings_V1 := (others => <>);
      Unit : Interfaces.Unsigned_32 := 0;
      Window : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Source_Spec_V1 use record
      Paths at 0 range 0 .. 127;
      Unit at 16 range 0 .. 31;
      Window at 24 range 0 .. 63;
   end record;
   for Source_Spec_V1'Size use 256;
   for Source_Spec_V1'Alignment use 8;
   type Controls_V1 is record
      Deadline_Ms : Interfaces.Integer_64 := 0;
      Cancel : System.Address := System.Null_Address;
      Context : Optional_Content_V1 := (others => <>);
      Batch : Optional_Size_V1 := (others => <>);
      Batch_Max : Interfaces.C.int := 0;
      Attempts : Interfaces.C.int := 0;
      Surface : Byte_String_V1 := (others => <>);
   end record with Convention => C;
   for Controls_V1 use record
      Deadline_Ms at 0 range 0 .. 63;
      Cancel at 8 range 0 .. 63;
      Context at 16 range 0 .. 255;
      Batch at 48 range 0 .. 127;
      Batch_Max at 64 range 0 .. 31;
      Attempts at 68 range 0 .. 31;
      Surface at 72 range 0 .. 127;
   end record;
   for Controls_V1'Size use 704;
   for Controls_V1'Alignment use 8;
end Thinkthen_C_Inputs;
