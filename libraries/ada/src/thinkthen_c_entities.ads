-- Unpublished 0430 typed C carriers. Counted data never uses a JSON buffer.
with Interfaces;
with Interfaces.C;
with System;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
with Thinkthen_C_Answers; use Thinkthen_C_Answers;
package Thinkthen_C_Entities is
   C_RELATION_YES_NO_V1 : constant Interfaces.Unsigned_32 := 1;
   C_RELATION_CHOICE_V1 : constant Interfaces.Unsigned_32 := 2;
   C_DIRECTION_SOURCE_TO_TARGET_V1 : constant Interfaces.Unsigned_32 := 1;
   C_DIRECTION_EITHER_V1 : constant Interfaces.Unsigned_32 := 2;
   type Entity_V1 is record
      Text : Byte_String_V1 := (others => <>);
      Start : Interfaces.C.size_t := 0;
      End_Index : Interfaces.C.size_t := 0;
      Length : Interfaces.C.size_t := 0;
      Kind : Byte_String_V1 := (others => <>);
      Strength : Interfaces.C.double := 0.0;
   end record with Convention => C;
   for Entity_V1 use record
      Text at 0 range 0 .. 127;
      Start at 16 range 0 .. 63;
      End_Index at 24 range 0 .. 63;
      Length at 32 range 0 .. 63;
      Kind at 40 range 0 .. 127;
      Strength at 56 range 0 .. 63;
   end record;
   for Entity_V1'Size use 512;
   for Entity_V1'Alignment use 8;
   type Entities_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Entities_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Entities_V1'Size use 128;
   for Entities_V1'Alignment use 8;
   type Entity_Edge_V1 is record
      Relation : Byte_String_V1 := (others => <>);
      Source : Entity_V1 := (others => <>);
      Target : Entity_V1 := (others => <>);
      Probability : Interfaces.C.double := 0.0;
      Either : Interfaces.C.int := 0;
   end record with Convention => C;
   for Entity_Edge_V1 use record
      Relation at 0 range 0 .. 127;
      Source at 16 range 0 .. 511;
      Target at 80 range 0 .. 511;
      Probability at 144 range 0 .. 63;
      Either at 152 range 0 .. 31;
   end record;
   for Entity_Edge_V1'Size use 1280;
   for Entity_Edge_V1'Alignment use 8;
   type Entity_Edges_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Entity_Edges_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Entity_Edges_V1'Size use 128;
   for Entity_Edges_V1'Alignment use 8;
   type Optional_Entity_Edges_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Entity_Edges_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Entity_Edges_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 127;
   end record;
   for Optional_Entity_Edges_V1'Size use 192;
   for Optional_Entity_Edges_V1'Alignment use 8;
   type Place_V1 is record
      Start : Interfaces.C.size_t := 0;
      End_Index : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Place_V1 use record
      Start at 0 range 0 .. 63;
      End_Index at 8 range 0 .. 63;
   end record;
   for Place_V1'Size use 128;
   for Place_V1'Alignment use 8;
   type Piece_V1 is record
      Start : Interfaces.C.size_t := 0;
      End_Index : Interfaces.C.size_t := 0;
      Tags : Probabilities_V1 := (others => <>);
   end record with Convention => C;
   for Piece_V1 use record
      Start at 0 range 0 .. 63;
      End_Index at 8 range 0 .. 63;
      Tags at 16 range 0 .. 127;
   end record;
   for Piece_V1'Size use 256;
   for Piece_V1'Alignment use 8;
   type Pieces_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Pieces_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Pieces_V1'Size use 128;
   for Pieces_V1'Alignment use 8;
   type Name_V1 is record
      Start : Interfaces.C.size_t := 0;
      End_Index : Interfaces.C.size_t := 0;
      Kinds : Optional_Probabilities_V1 := (others => <>);
      Edges : Optional_Probabilities_V1 := (others => <>);
   end record with Convention => C;
   for Name_V1 use record
      Start at 0 range 0 .. 63;
      End_Index at 8 range 0 .. 63;
      Kinds at 16 range 0 .. 191;
      Edges at 40 range 0 .. 191;
   end record;
   for Name_V1'Size use 512;
   for Name_V1'Alignment use 8;
   type Names_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Names_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Names_V1'Size use 128;
   for Names_V1'Alignment use 8;
   type Pair_V1 is record
      Relation : Byte_String_V1 := (others => <>);
      Source : Place_V1 := (others => <>);
      Target : Place_V1 := (others => <>);
      Probability : Interfaces.C.double := 0.0;
   end record with Convention => C;
   for Pair_V1 use record
      Relation at 0 range 0 .. 127;
      Source at 16 range 0 .. 127;
      Target at 32 range 0 .. 127;
      Probability at 48 range 0 .. 63;
   end record;
   for Pair_V1'Size use 448;
   for Pair_V1'Alignment use 8;
   type Pairs_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Pairs_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Pairs_V1'Size use 128;
   for Pairs_V1'Alignment use 8;
   type Recognize_Value_V1 is record
      Entities : Entities_V1 := (others => <>);
      Relations : Optional_Entity_Edges_V1 := (others => <>);
   end record with Convention => C;
   for Recognize_Value_V1 use record
      Entities at 0 range 0 .. 127;
      Relations at 16 range 0 .. 191;
   end record;
   for Recognize_Value_V1'Size use 320;
   for Recognize_Value_V1'Alignment use 8;
   type Recognize_Answer_V1 is record
      Pieces : Pieces_V1 := (others => <>);
      Names : Names_V1 := (others => <>);
      Pairs : Pairs_V1 := (others => <>);
   end record with Convention => C;
   for Recognize_Answer_V1 use record
      Pieces at 0 range 0 .. 127;
      Names at 16 range 0 .. 127;
      Pairs at 32 range 0 .. 127;
   end record;
   for Recognize_Answer_V1'Size use 384;
   for Recognize_Answer_V1'Alignment use 8;
   type Endpoint_V1 is record
      Name : Byte_String_V1 := (others => <>);
      Kind : Byte_String_V1 := (others => <>);
   end record with Convention => C;
   for Endpoint_V1 use record
      Name at 0 range 0 .. 127;
      Kind at 16 range 0 .. 127;
   end record;
   for Endpoint_V1'Size use 256;
   for Endpoint_V1'Alignment use 8;
   type Optional_Endpoint_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Endpoint_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Endpoint_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 255;
   end record;
   for Optional_Endpoint_V1'Size use 320;
   for Optional_Endpoint_V1'Alignment use 8;
   type Edge_V1 is record
      Relation : Byte_String_V1 := (others => <>);
      Source : Endpoint_V1 := (others => <>);
      Target : Endpoint_V1 := (others => <>);
      Probability : Interfaces.C.double := 0.0;
      Either : Interfaces.C.int := 0;
   end record with Convention => C;
   for Edge_V1 use record
      Relation at 0 range 0 .. 127;
      Source at 16 range 0 .. 255;
      Target at 48 range 0 .. 255;
      Probability at 80 range 0 .. 63;
      Either at 88 range 0 .. 31;
   end record;
   for Edge_V1'Size use 768;
   for Edge_V1'Alignment use 8;
   type Edges_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Edges_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Edges_V1'Size use 128;
   for Edges_V1'Alignment use 8;
   type Relation_Success_V1 is record
      Answer_Id : Byte_String_V1 := (others => <>);
      Probability : Interfaces.C.double := 0.0;
      Accepted : Interfaces.C.int := 0;
   end record with Convention => C;
   for Relation_Success_V1 use record
      Answer_Id at 0 range 0 .. 127;
      Probability at 16 range 0 .. 63;
      Accepted at 24 range 0 .. 31;
   end record;
   for Relation_Success_V1'Size use 256;
   for Relation_Success_V1'Alignment use 8;
   type Relation_Answer_V1_Data (Arm : Positive := 1) is record
      case Arm is
         when 1 => Success : Relation_Success_V1 := (others => <>);
         when 2 => Failure : Member_Failure_V1 := (others => <>);
         when others => null;
      end case;
   end record with Convention => C, Unchecked_Union;
   for Relation_Answer_V1_Data use record
      Success at 0 range 0 .. 255;
      Failure at 0 range 0 .. 191;
   end record;
   for Relation_Answer_V1_Data'Size use 256;
   for Relation_Answer_V1_Data'Alignment use 8;
   type Relation_Answer_V1 is record
      Relation : Byte_String_V1 := (others => <>);
      Reads : Byte_String_V1 := (others => <>);
      Method : Interfaces.Unsigned_32 := 0;
      Direction : Interfaces.Unsigned_32 := 0;
      Source : Endpoint_V1 := (others => <>);
      Target : Optional_Endpoint_V1 := (others => <>);
      Request : Byte_String_V1 := (others => <>);
      State : Interfaces.Unsigned_32 := 0;
      Data : Relation_Answer_V1_Data := (others => <>);
   end record with Convention => C;
   for Relation_Answer_V1 use record
      Relation at 0 range 0 .. 127;
      Reads at 16 range 0 .. 127;
      Method at 32 range 0 .. 31;
      Direction at 36 range 0 .. 31;
      Source at 40 range 0 .. 255;
      Target at 72 range 0 .. 319;
      Request at 112 range 0 .. 127;
      State at 128 range 0 .. 31;
      Data at 136 range 0 .. 255;
   end record;
   for Relation_Answer_V1'Size use 1344;
   for Relation_Answer_V1'Alignment use 8;
   type Relation_Answers_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Relation_Answers_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Relation_Answers_V1'Size use 128;
   for Relation_Answers_V1'Alignment use 8;
end Thinkthen_C_Entities;
