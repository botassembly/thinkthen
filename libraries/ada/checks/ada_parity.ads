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
with Thinkthen.Native;
with Thinkthen.Native.Inputs;
with Thinkthen.Native.Results;
with Thinkthen.Native.Batches;
package Ada_Parity is
   function Result_Source_Recognition (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Source_Recognition_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_source_recognition";
   function Result_Source_Relations (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Source_Relations_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_source_relations";
   function Result_Details (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Details_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_details";
   function Result_Observation_Details (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Details_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_observation_details";
   function Decide_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_decide_batch_start";
   function Choose_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_choose_batch_start";
   function Tag_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_tag_batch_start";
   function Score_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_score_batch_start";
   function Filter_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_filter_batch_start";
   function Annotate_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_annotate_batch_start";
   function Batch_Next (P1 : System.Address; P2 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_batch_next";
   function Batch_Facts (P1 : System.Address; P2 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_batch_facts";
   procedure Batch_Free (P1 : System.Address)
     with Export, Convention => C, External_Name => "ada_thinkthen_batch_free";
   function Decide_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_decide_complete";
   function Choose_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_choose_complete";
   function Tag_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_tag_complete";
   function Score_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_score_complete";
   function Filter_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_filter_complete";
   function Rank_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_rank_complete";
   function Find_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_find_complete";
   function Annotate_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_annotate_complete";
   function Recognize_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_recognize_complete";
   function Relate_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_relate_complete";
   procedure Result_Free (P1 : System.Address)
     with Export, Convention => C, External_Name => "ada_thinkthen_result_free";
   function Result_Summary (P1 : System.Address; P2 : access Summary_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_summary";
   function Result_Observation (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Observation_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_observation";
   function Result_Decide (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Decide_View_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_decide";
   function Result_Choose (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Choose_View_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_choose";
   function Result_Tag (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Tag_View_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_tag";
   function Result_Score (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Score_View_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_score";
   function Result_Filter (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Filter_View_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_filter";
   function Result_Rank (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Rank_View_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_rank";
   function Result_Rank_Member_Count (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Interfaces.C.size_t) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_rank_member_count";
   function Result_Rank_Member (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : Interfaces.C.size_t; P4 : access Rank_View_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_rank_member";
   function Result_Find (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Find_View_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_find";
   function Result_Annotate (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Annotate_View_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_annotate";
   function Result_Recognize (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Recognize_View_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_recognize";
   function Result_Relate (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Relate_View_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_relate";
   function Error_Complete (P1 : System.Address; P2 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_error_complete";
   function Question_New (P1 : System.Address; P2 : access constant Question_Spec_V1; P3 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_question_new";
   function Question_Load (P1 : System.Address; P2 : Byte_String_V1; P3 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_question_load";
   function Question_New_Authored (P1 : System.Address; P2 : access constant Question_Spec_V1; P3 : access constant Question_Author_V1; P4 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_question_new_authored";
   function Question_Author (P1 : System.Address; P2 : access Question_Author_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_question_author";
   function Question_Parse (P1 : System.Address; P2 : Interfaces.Unsigned_32; P3 : Byte_String_V1; P4 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_question_parse";
   function Question_Load_Named (P1 : System.Address; P2 : Interfaces.Unsigned_32; P3 : Byte_String_V1; P4 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_question_load_named";
   function Question_Load_Reference (P1 : System.Address; P2 : Interfaces.Unsigned_32; P3 : Byte_String_V1; P4 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_question_load_reference";
   function Result_Question_Author (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Question_Author_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_question_author";
   function Result_Member_Author (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : Interfaces.C.size_t; P4 : access Question_Author_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_member_author";
   function Result_Observation_Author (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Question_Author_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_result_observation_author";
   procedure Question_Free (P1 : System.Address)
     with Export, Convention => C, External_Name => "ada_thinkthen_question_free";
   function Image_Clone (P1 : System.Address; P2 : System.Address; P3 : Interfaces.C.size_t; P4 : Interfaces.Unsigned_32; P5 : Optional_String_V1; P6 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_image_clone";
   function Image_View (P1 : System.Address; P2 : access Image_View_V1) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_image_view";
   procedure Image_Free (P1 : System.Address)
     with Export, Convention => C, External_Name => "ada_thinkthen_image_free";
   function Source_Records (P1 : System.Address; P2 : access constant Record_Data_V1; P3 : Interfaces.C.size_t; P4 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_source_records";
   function Source_Files (P1 : System.Address; P2 : access constant Source_Spec_V1; P3 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_source_files";
   function Source_Image_Files (P1 : System.Address; P2 : access constant Source_Spec_V1; P3 : access System.Address) return Interfaces.C.int
     with Export, Convention => C, External_Name => "ada_thinkthen_source_image_files";
   procedure Source_Free (P1 : System.Address)
     with Export, Convention => C, External_Name => "ada_thinkthen_source_free";
end Ada_Parity;
