with Interfaces;
with Interfaces.C;
with System;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
with Thinkthen_C_Answers; use Thinkthen_C_Answers;
with Thinkthen_C_Entities; use Thinkthen_C_Entities;
with Thinkthen_C_Metadata; use Thinkthen_C_Metadata;
with Thinkthen_C_Rows; use Thinkthen_C_Rows;
with Thinkthen_C_Events; use Thinkthen_C_Events;
package Thinkthen_C_Extensions is
   C_Declaration_Absent_V1 : constant Interfaces.Unsigned_32 := 0;
   C_Declaration_String_V1 : constant Interfaces.Unsigned_32 := 1;
   C_Declaration_Object_V1 : constant Interfaces.Unsigned_32 := 2;
   C_Property_String_V1 : constant Interfaces.Unsigned_32 := 1;
   C_Property_Number_V1 : constant Interfaces.Unsigned_32 := 2;
   C_Property_Boolean_V1 : constant Interfaces.Unsigned_32 := 3;
   C_Property_String_List_V1 : constant Interfaces.Unsigned_32 := 4;

   C_Load_Atomic_V1 : constant Interfaces.Unsigned_32 := 1;
   C_Load_Set_V1 : constant Interfaces.Unsigned_32 := 2;
   C_Load_Dynamic_Choose_V1 : constant Interfaces.Unsigned_32 := 3;
   C_Load_Recognize_V1 : constant Interfaces.Unsigned_32 := 4;
   C_Load_Relate_V1 : constant Interfaces.Unsigned_32 := 5;
   C_Load_Rank_V1 : constant Interfaces.Unsigned_32 := 6;
   C_Load_Rank_Set_V1 : constant Interfaces.Unsigned_32 := 7;
   C_Load_Find_V1 : constant Interfaces.Unsigned_32 := 8;

   type Input_Property_V1 is record
      Name : Byte_String_V1 := (others => <>);
      Kind : Interfaces.Unsigned_32 := 0;
   end record with Convention => C;
   for Input_Property_V1 use record
      Name at 0 range 0 .. 127;
      Kind at 16 range 0 .. 31;
   end record;
   for Input_Property_V1'Size use 192;
   for Input_Property_V1'Alignment use 8;
   type Input_Properties_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Input_Properties_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Input_Properties_V1'Size use 128;
   for Input_Properties_V1'Alignment use 8;
   type Input_Declaration_V1 is record
      Kind : Interfaces.Unsigned_32 := 0;
      Properties : Input_Properties_V1 := (others => <>);
      Required : Strings_V1 := (others => <>);
   end record with Convention => C;
   for Input_Declaration_V1 use record
      Kind at 0 range 0 .. 31;
      Properties at 8 range 0 .. 127;
      Required at 24 range 0 .. 127;
   end record;
   for Input_Declaration_V1'Size use 320;
   for Input_Declaration_V1'Alignment use 8;
   type Question_Author_V1 is record
      Name : Optional_String_V1 := (others => <>);
      Wording_Version : Optional_U64_V1 := (others => <>);
      Item_Schema : Input_Declaration_V1 := (others => <>);
      Context_Schema : Input_Declaration_V1 := (others => <>);
   end record with Convention => C;
   for Question_Author_V1 use record
      Name at 0 range 0 .. 191;
      Wording_Version at 24 range 0 .. 127;
      Item_Schema at 40 range 0 .. 319;
      Context_Schema at 80 range 0 .. 319;
   end record;
   for Question_Author_V1'Size use 960;
   for Question_Author_V1'Alignment use 8;
   type Reported_Usage_V1 is record
      Present : Interfaces.C.int := 0;
      Input_Tokens : Optional_U64_V1 := (others => <>);
      Output_Tokens : Optional_U64_V1 := (others => <>);
   end record with Convention => C;
   for Reported_Usage_V1 use record
      Present at 0 range 0 .. 31;
      Input_Tokens at 8 range 0 .. 127;
      Output_Tokens at 24 range 0 .. 127;
   end record;
   for Reported_Usage_V1'Size use 320;
   for Reported_Usage_V1'Alignment use 8;
   type Source_Detail_V1 is record
      Origin : Interfaces.Unsigned_32 := 0;
      Answered_By : Byte_String_V1 := (others => <>);
      Batch_Size : Optional_Size_V1 := (others => <>);
   end record with Convention => C;
   for Source_Detail_V1 use record
      Origin at 0 range 0 .. 31;
      Answered_By at 8 range 0 .. 127;
      Batch_Size at 24 range 0 .. 127;
   end record;
   for Source_Detail_V1'Size use 320;
   for Source_Detail_V1'Alignment use 8;
   type Source_Details_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Source_Details_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Source_Details_V1'Size use 128;
   for Source_Details_V1'Alignment use 8;
   type Input_View_V1 is record
      Original : Optional_Content_V1 := (others => <>);
      Position : Optional_Location_V1 := (others => <>);
      Images : Optional_Image_Views_V1 := (others => <>);
   end record with Convention => C;
   for Input_View_V1 use record
      Original at 0 range 0 .. 255;
      Position at 32 range 0 .. 511;
      Images at 96 range 0 .. 191;
   end record;
   for Input_View_V1'Size use 960;
   for Input_View_V1'Alignment use 8;
   type Input_Views_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Input_Views_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Input_Views_V1'Size use 128;
   for Input_Views_V1'Alignment use 8;
   type Details_V1 is record
      Question : Optional_Question_V1 := (others => <>);
      Threshold : Optional_Rule_V1 := (others => <>);
      Raw_Pick : Optional_String_V1 := (others => <>);
      Usage : Reported_Usage_V1 := (others => <>);
      Question_Sources : Source_Details_V1 := (others => <>);
      Observations : Observation_Identities_V1 := (others => <>);
      Inputs : Input_Views_V1 := (others => <>);
   end record with Convention => C;
   for Details_V1 use record
      Question at 0 range 0 .. 2815;
      Threshold at 352 range 0 .. 255;
      Raw_Pick at 384 range 0 .. 191;
      Usage at 408 range 0 .. 319;
      Question_Sources at 448 range 0 .. 127;
      Observations at 464 range 0 .. 127;
      Inputs at 480 range 0 .. 127;
   end record;
   for Details_V1'Size use 3968;
   for Details_V1'Alignment use 8;
   type Source_Entity_V1 is record
      Entity : Entity_V1 := (others => <>);
      Position : Optional_Location_V1 := (others => <>);
   end record with Convention => C;
   for Source_Entity_V1 use record
      Entity at 0 range 0 .. 511;
      Position at 64 range 0 .. 511;
   end record;
   for Source_Entity_V1'Size use 1024;
   for Source_Entity_V1'Alignment use 8;
   type Source_Entities_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Source_Entities_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Source_Entities_V1'Size use 128;
   for Source_Entities_V1'Alignment use 8;
   type Source_Entity_Edge_V1 is record
      Relation : Byte_String_V1 := (others => <>);
      Source : Source_Entity_V1 := (others => <>);
      Target : Source_Entity_V1 := (others => <>);
      Probability : Interfaces.C.double := 0.0;
      Either : Interfaces.C.int := 0;
   end record with Convention => C;
   for Source_Entity_Edge_V1 use record
      Relation at 0 range 0 .. 127;
      Source at 16 range 0 .. 1023;
      Target at 144 range 0 .. 1023;
      Probability at 272 range 0 .. 63;
      Either at 280 range 0 .. 31;
   end record;
   for Source_Entity_Edge_V1'Size use 2304;
   for Source_Entity_Edge_V1'Alignment use 8;
   type Source_Entity_Edges_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Source_Entity_Edges_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Source_Entity_Edges_V1'Size use 128;
   for Source_Entity_Edges_V1'Alignment use 8;
   type Optional_Source_Entity_Edges_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Source_Entity_Edges_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Source_Entity_Edges_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 127;
   end record;
   for Optional_Source_Entity_Edges_V1'Size use 192;
   for Optional_Source_Entity_Edges_V1'Alignment use 8;
   type Source_Recognition_V1 is record
      Present : Interfaces.C.int := 0;
      Entities : Source_Entities_V1 := (others => <>);
      Relations : Optional_Source_Entity_Edges_V1 := (others => <>);
   end record with Convention => C;
   for Source_Recognition_V1 use record
      Present at 0 range 0 .. 31;
      Entities at 8 range 0 .. 127;
      Relations at 24 range 0 .. 191;
   end record;
   for Source_Recognition_V1'Size use 384;
   for Source_Recognition_V1'Alignment use 8;
   type Source_Endpoint_V1 is record
      Ordinal : Interfaces.C.size_t := 0;
      Endpoint : Endpoint_V1 := (others => <>);
      Original_Record : Content_V1 := (others => <>);
      Position : Optional_Location_V1 := (others => <>);
   end record with Convention => C;
   for Source_Endpoint_V1 use record
      Ordinal at 0 range 0 .. 63;
      Endpoint at 8 range 0 .. 255;
      Original_Record at 40 range 0 .. 191;
      Position at 64 range 0 .. 511;
   end record;
   for Source_Endpoint_V1'Size use 1024;
   for Source_Endpoint_V1'Alignment use 8;
   type Source_Edge_V1 is record
      Relation : Byte_String_V1 := (others => <>);
      Source : Source_Endpoint_V1 := (others => <>);
      Target : Source_Endpoint_V1 := (others => <>);
      Probability : Interfaces.C.double := 0.0;
      Either : Interfaces.C.int := 0;
   end record with Convention => C;
   for Source_Edge_V1 use record
      Relation at 0 range 0 .. 127;
      Source at 16 range 0 .. 1023;
      Target at 144 range 0 .. 1023;
      Probability at 272 range 0 .. 63;
      Either at 280 range 0 .. 31;
   end record;
   for Source_Edge_V1'Size use 2304;
   for Source_Edge_V1'Alignment use 8;
   type Source_Edges_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Source_Edges_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Source_Edges_V1'Size use 128;
   for Source_Edges_V1'Alignment use 8;
   type Source_Relations_V1 is record
      Present : Interfaces.C.int := 0;
      Edges : Source_Edges_V1 := (others => <>);
   end record with Convention => C;
   for Source_Relations_V1 use record
      Present at 0 range 0 .. 31;
      Edges at 8 range 0 .. 127;
   end record;
   for Source_Relations_V1'Size use 192;
   for Source_Relations_V1'Alignment use 8;
end Thinkthen_C_Extensions;
