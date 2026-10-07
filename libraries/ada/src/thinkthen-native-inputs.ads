with Interfaces;
with Interfaces.C;
with System;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
with Thinkthen_C_Answers; use Thinkthen_C_Answers;
with Thinkthen_C_Entities; use Thinkthen_C_Entities;
with Thinkthen_C_Metadata; use Thinkthen_C_Metadata;
with Thinkthen_C_Rows; use Thinkthen_C_Rows;
with Thinkthen_C_Events; use Thinkthen_C_Events;
with Thinkthen_C_Extensions; use Thinkthen_C_Extensions;
package Thinkthen.Native.Inputs is
   function Question_New (P1 : System.Address; P2 : access constant Question_Spec_V1; P3 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_question_new";
   function Question_Load (P1 : System.Address; P2 : Byte_String_V1; P3 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_question_load";
   function Question_New_Authored (P1 : System.Address; P2 : access constant Question_Spec_V1; P3 : access constant Question_Author_V1; P4 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_question_new_authored";
   function Question_Author (P1 : System.Address; P2 : access Question_Author_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_question_author";
   function Question_Parse (P1 : System.Address; P2 : Interfaces.Unsigned_32; P3 : Byte_String_V1; P4 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_question_parse";
   function Question_Load_Named (P1 : System.Address; P2 : Interfaces.Unsigned_32; P3 : Byte_String_V1; P4 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_question_load_named";
   function Question_Load_Reference (P1 : System.Address; P2 : Interfaces.Unsigned_32; P3 : Byte_String_V1; P4 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_question_load_reference";
   procedure Question_Free (P1 : System.Address)
     with Import, Convention => C, External_Name => "thinkthen_question_free";
   function Image_Clone (P1 : System.Address; P2 : System.Address; P3 : Interfaces.C.size_t; P4 : Interfaces.Unsigned_32; P5 : Optional_String_V1; P6 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_image_clone";
   function Image_View (P1 : System.Address; P2 : access Image_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_image_view";
   procedure Image_Free (P1 : System.Address)
     with Import, Convention => C, External_Name => "thinkthen_image_free";
   function Source_Records (P1 : System.Address; P2 : access constant Record_Data_V1; P3 : Interfaces.C.size_t; P4 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_source_records";
   function Source_Files (P1 : System.Address; P2 : access constant Source_Spec_V1; P3 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_source_files";
   function Source_Image_Files (P1 : System.Address; P2 : access constant Source_Spec_V1; P3 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_source_image_files";
   procedure Source_Free (P1 : System.Address)
     with Import, Convention => C, External_Name => "thinkthen_source_free";
end Thinkthen.Native.Inputs;
