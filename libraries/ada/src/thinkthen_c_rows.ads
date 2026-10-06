-- Unpublished 0430 typed C carriers. Counted data never uses a JSON buffer.
with Interfaces;
with Interfaces.C;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
with Thinkthen_C_Answers; use Thinkthen_C_Answers;
with Thinkthen_C_Entities; use Thinkthen_C_Entities;
with Thinkthen_C_Metadata; use Thinkthen_C_Metadata;
package Thinkthen_C_Rows is
   C_STAGE_BOUNDARY_V1 : constant Interfaces.Unsigned_32 := 1;
   C_STAGE_KIND_V1 : constant Interfaces.Unsigned_32 := 2;
   C_STAGE_EDGE_V1 : constant Interfaces.Unsigned_32 := 3;
   C_STAGE_RELATION_V1 : constant Interfaces.Unsigned_32 := 4;
   C_PROBABILITIES_YES_V1 : constant Interfaces.Unsigned_32 := 1;
   C_PROBABILITIES_NAMED_V1 : constant Interfaces.Unsigned_32 := 2;
   type Row_V1 is record
      Answer_Id : Byte_String_V1 := (others => <>);
      Input : Optional_Content_V1 := (others => <>);
      Question : Optional_Question_V1 := (others => <>);
      Answer : Optional_Answer_V1 := (others => <>);
      Threshold : Optional_Rule_V1 := (others => <>);
      Position : Optional_Location_V1 := (others => <>);
      Input_File : Optional_String_V1 := (others => <>);
      Meta : Meta_V1 := (others => <>);
      Images : Optional_Image_Views_V1 := (others => <>);
   end record with Convention => C;
   for Row_V1 use record
      Answer_Id at 0 range 0 .. 127;
      Input at 16 range 0 .. 255;
      Question at 48 range 0 .. 2815;
      Answer at 400 range 0 .. 511;
      Threshold at 464 range 0 .. 255;
      Position at 496 range 0 .. 511;
      Input_File at 560 range 0 .. 191;
      Meta at 584 range 0 .. 3007;
      Images at 960 range 0 .. 191;
   end record;
   for Row_V1'Size use 7872;
   for Row_V1'Alignment use 8;
   type Decide_View_V1 is record
      Common : Row_V1 := (others => <>);
      Value : Decide_Value_V1 := (others => <>);
   end record with Convention => C;
   for Decide_View_V1 use record
      Common at 0 range 0 .. 7871;
      Value at 984 range 0 .. 255;
   end record;
   for Decide_View_V1'Size use 8128;
   for Decide_View_V1'Alignment use 8;
   type Choose_View_V1 is record
      Common : Row_V1 := (others => <>);
      Value : Optional_String_V1 := (others => <>);
   end record with Convention => C;
   for Choose_View_V1 use record
      Common at 0 range 0 .. 7871;
      Value at 984 range 0 .. 191;
   end record;
   for Choose_View_V1'Size use 8064;
   for Choose_View_V1'Alignment use 8;
   type Tag_View_V1 is record
      Common : Row_V1 := (others => <>);
      Value : Strings_V1 := (others => <>);
   end record with Convention => C;
   for Tag_View_V1 use record
      Common at 0 range 0 .. 7871;
      Value at 984 range 0 .. 127;
   end record;
   for Tag_View_V1'Size use 8000;
   for Tag_View_V1'Alignment use 8;
   type Score_View_V1 is record
      Common : Row_V1 := (others => <>);
      Value : Interfaces.C.double := 0.0;
   end record with Convention => C;
   for Score_View_V1 use record
      Common at 0 range 0 .. 7871;
      Value at 984 range 0 .. 63;
   end record;
   for Score_View_V1'Size use 7936;
   for Score_View_V1'Alignment use 8;
   type Filter_View_V1 is record
      Common : Row_V1 := (others => <>);
      Value : Interfaces.C.int := 0;
   end record with Convention => C;
   for Filter_View_V1 use record
      Common at 0 range 0 .. 7871;
      Value at 984 range 0 .. 31;
   end record;
   for Filter_View_V1'Size use 7936;
   for Filter_View_V1'Alignment use 8;
   type Rank_View_V1 is record
      Common : Row_V1 := (others => <>);
      Value : Optional_Size_V1 := (others => <>);
      Question_Name : Optional_String_V1 := (others => <>);
   end record with Convention => C;
   for Rank_View_V1 use record
      Common at 0 range 0 .. 7871;
      Value at 984 range 0 .. 127;
      Question_Name at 1000 range 0 .. 191;
   end record;
   for Rank_View_V1'Size use 8192;
   for Rank_View_V1'Alignment use 8;
   type Find_View_V1 is record
      Common : Row_V1 := (others => <>);
      Value : Optional_Content_V1 := (others => <>);
      Index : Optional_Size_V1 := (others => <>);
   end record with Convention => C;
   for Find_View_V1 use record
      Common at 0 range 0 .. 7871;
      Value at 984 range 0 .. 255;
      Index at 1016 range 0 .. 127;
   end record;
   for Find_View_V1'Size use 8256;
   for Find_View_V1'Alignment use 8;
   type Annotate_View_V1 is record
      Common : Row_V1 := (others => <>);
      Answers : Members_V1 := (others => <>);
   end record with Convention => C;
   for Annotate_View_V1 use record
      Common at 0 range 0 .. 7871;
      Answers at 984 range 0 .. 127;
   end record;
   for Annotate_View_V1'Size use 8000;
   for Annotate_View_V1'Alignment use 8;
   type Recognize_View_V1 is record
      Common : Row_V1 := (others => <>);
      Value : Recognize_Value_V1 := (others => <>);
      Answer : Recognize_Answer_V1 := (others => <>);
   end record with Convention => C;
   for Recognize_View_V1 use record
      Common at 0 range 0 .. 7871;
      Value at 984 range 0 .. 319;
      Answer at 1024 range 0 .. 383;
   end record;
   for Recognize_View_V1'Size use 8576;
   for Recognize_View_V1'Alignment use 8;
   type Relate_View_V1 is record
      Common : Row_V1 := (others => <>);
      Value : Edges_V1 := (others => <>);
      Questions : Relation_Answers_V1 := (others => <>);
   end record with Convention => C;
   for Relate_View_V1 use record
      Common at 0 range 0 .. 7871;
      Value at 984 range 0 .. 127;
      Questions at 1000 range 0 .. 127;
   end record;
   for Relate_View_V1'Size use 8128;
   for Relate_View_V1'Alignment use 8;
end Thinkthen_C_Rows;
