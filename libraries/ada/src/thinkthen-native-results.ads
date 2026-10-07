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
package Thinkthen.Native.Results is
   function Result_Source_Recognition (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Source_Recognition_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_source_recognition";
   function Result_Source_Relations (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Source_Relations_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_source_relations";
   function Result_Details (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Details_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_details";
   function Result_Observation_Details (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Details_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_observation_details";
   procedure Result_Free (P1 : System.Address)
     with Import, Convention => C, External_Name => "thinkthen_result_free";
   function Result_Summary (P1 : System.Address; P2 : access Summary_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_summary";
   function Result_Observation (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Observation_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_observation";
   function Result_Decide (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Decide_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_decide";
   function Result_Choose (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Choose_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_choose";
   function Result_Tag (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Tag_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_tag";
   function Result_Score (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Score_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_score";
   function Result_Filter (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Filter_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_filter";
   function Result_Rank (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Rank_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_rank";
   function Result_Rank_Member_Count (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Interfaces.C.size_t) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_rank_member_count";
   function Result_Rank_Member_Details (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : Interfaces.C.size_t; P4 : access Details_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_rank_member_details";
   function Result_Rank_Member (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : Interfaces.C.size_t; P4 : access Rank_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_rank_member";
   function Result_Find (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Find_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_find";
   function Result_Annotate (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Annotate_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_annotate";
   function Result_Recognize (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Recognize_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_recognize";
   function Result_Relate (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Relate_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_relate";
   function Error_Complete (P1 : System.Address; P2 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_error_complete";
   function Result_Question_Author (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Question_Author_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_question_author";
   function Result_Member_Author (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : Interfaces.C.size_t; P4 : access Question_Author_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_member_author";
   function Result_Observation_Author (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Question_Author_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_observation_author";
end Thinkthen.Native.Results;
