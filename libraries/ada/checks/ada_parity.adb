package body Ada_Parity is
   function Result_Source_Recognition (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Source_Recognition_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Source_Recognition (P1, P2, P3);
   end Result_Source_Recognition;
   function Result_Source_Relations (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Source_Relations_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Source_Relations (P1, P2, P3);
   end Result_Source_Relations;
   function Result_Details (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Details_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Details (P1, P2, P3);
   end Result_Details;
   function Result_Observation_Details (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Details_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Observation_Details (P1, P2, P3);
   end Result_Observation_Details;
   function Decide_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Batches.Decide_Batch_Start (P1, P2, P3, P4, P5);
   end Decide_Batch_Start;
   function Choose_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Batches.Choose_Batch_Start (P1, P2, P3, P4, P5);
   end Choose_Batch_Start;
   function Tag_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Batches.Tag_Batch_Start (P1, P2, P3, P4, P5);
   end Tag_Batch_Start;
   function Score_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Batches.Score_Batch_Start (P1, P2, P3, P4, P5);
   end Score_Batch_Start;
   function Filter_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Batches.Filter_Batch_Start (P1, P2, P3, P4, P5);
   end Filter_Batch_Start;
   function Annotate_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Batches.Annotate_Batch_Start (P1, P2, P3, P4, P5);
   end Annotate_Batch_Start;
   function Batch_Next (P1 : System.Address; P2 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Batches.Batch_Next (P1, P2);
   end Batch_Next;
   function Batch_Facts (P1 : System.Address; P2 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Batches.Batch_Facts (P1, P2);
   end Batch_Facts;
   procedure Batch_Free (P1 : System.Address) is
   begin
      Thinkthen.Native.Batches.Batch_Free (P1);
   end Batch_Free;
   function Decide_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Decide (P1, P2, P3, P4, P5);
   end Decide_Complete;
   function Choose_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Choose (P1, P2, P3, P4, P5);
   end Choose_Complete;
   function Tag_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Tag (P1, P2, P3, P4, P5);
   end Tag_Complete;
   function Score_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Score (P1, P2, P3, P4, P5);
   end Score_Complete;
   function Filter_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Filter (P1, P2, P3, P4, P5);
   end Filter_Complete;
   function Rank_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Rank (P1, P2, P3, P4, P5);
   end Rank_Complete;
   function Find_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Find (P1, P2, P3, P4, P5);
   end Find_Complete;
   function Annotate_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Annotate (P1, P2, P3, P4, P5);
   end Annotate_Complete;
   function Recognize_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Recognize (P1, P2, P3, P4, P5);
   end Recognize_Complete;
   function Relate_Complete (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Relate (P1, P2, P3, P4, P5);
   end Relate_Complete;
   procedure Result_Free (P1 : System.Address) is
   begin
      Thinkthen.Native.Results.Result_Free (P1);
   end Result_Free;
   function Result_Summary (P1 : System.Address; P2 : access Summary_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Summary (P1, P2);
   end Result_Summary;
   function Result_Observation (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Observation_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Observation (P1, P2, P3);
   end Result_Observation;
   function Result_Decide (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Decide_View_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Decide (P1, P2, P3);
   end Result_Decide;
   function Result_Choose (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Choose_View_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Choose (P1, P2, P3);
   end Result_Choose;
   function Result_Tag (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Tag_View_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Tag (P1, P2, P3);
   end Result_Tag;
   function Result_Score (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Score_View_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Score (P1, P2, P3);
   end Result_Score;
   function Result_Filter (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Filter_View_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Filter (P1, P2, P3);
   end Result_Filter;
   function Result_Rank (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Rank_View_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Rank (P1, P2, P3);
   end Result_Rank;
   function Result_Rank_Member_Count (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Interfaces.C.size_t) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Rank_Member_Count (P1, P2, P3);
   end Result_Rank_Member_Count;
   function Result_Rank_Member (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : Interfaces.C.size_t; P4 : access Rank_View_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Rank_Member (P1, P2, P3, P4);
   end Result_Rank_Member;
   function Result_Find (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Find_View_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Find (P1, P2, P3);
   end Result_Find;
   function Result_Annotate (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Annotate_View_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Annotate (P1, P2, P3);
   end Result_Annotate;
   function Result_Recognize (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Recognize_View_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Recognize (P1, P2, P3);
   end Result_Recognize;
   function Result_Relate (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Relate_View_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Relate (P1, P2, P3);
   end Result_Relate;
   function Error_Complete (P1 : System.Address; P2 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Error_Complete (P1, P2);
   end Error_Complete;
   function Question_New (P1 : System.Address; P2 : access constant Question_Spec_V1; P3 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Inputs.Question_New (P1, P2, P3);
   end Question_New;
   function Question_Load (P1 : System.Address; P2 : Byte_String_V1; P3 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Inputs.Question_Load (P1, P2, P3);
   end Question_Load;
   function Question_New_Authored (P1 : System.Address; P2 : access constant Question_Spec_V1; P3 : access constant Question_Author_V1; P4 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Inputs.Question_New_Authored (P1, P2, P3, P4);
   end Question_New_Authored;
   function Question_Author (P1 : System.Address; P2 : access Question_Author_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Inputs.Question_Author (P1, P2);
   end Question_Author;
   function Question_Parse (P1 : System.Address; P2 : Interfaces.Unsigned_32; P3 : Byte_String_V1; P4 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Inputs.Question_Parse (P1, P2, P3, P4);
   end Question_Parse;
   function Question_Load_Named (P1 : System.Address; P2 : Interfaces.Unsigned_32; P3 : Byte_String_V1; P4 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Inputs.Question_Load_Named (P1, P2, P3, P4);
   end Question_Load_Named;
   function Question_Load_Reference (P1 : System.Address; P2 : Interfaces.Unsigned_32; P3 : Byte_String_V1; P4 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Inputs.Question_Load_Reference (P1, P2, P3, P4);
   end Question_Load_Reference;
   function Result_Question_Author (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Question_Author_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Question_Author (P1, P2, P3);
   end Result_Question_Author;
   function Result_Member_Author (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : Interfaces.C.size_t; P4 : access Question_Author_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Member_Author (P1, P2, P3, P4);
   end Result_Member_Author;
   function Result_Observation_Author (P1 : System.Address; P2 : Interfaces.C.size_t; P3 : access Question_Author_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Results.Result_Observation_Author (P1, P2, P3);
   end Result_Observation_Author;
   procedure Question_Free (P1 : System.Address) is
   begin
      Thinkthen.Native.Inputs.Question_Free (P1);
   end Question_Free;
   function Image_Clone (P1 : System.Address; P2 : System.Address; P3 : Interfaces.C.size_t; P4 : Interfaces.Unsigned_32; P5 : Optional_String_V1; P6 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Inputs.Image_Clone (P1, P2, P3, P4, P5, P6);
   end Image_Clone;
   function Image_View (P1 : System.Address; P2 : access Image_View_V1) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Inputs.Image_View (P1, P2);
   end Image_View;
   procedure Image_Free (P1 : System.Address) is
   begin
      Thinkthen.Native.Inputs.Image_Free (P1);
   end Image_Free;
   function Source_Records (P1 : System.Address; P2 : access constant Record_Data_V1; P3 : Interfaces.C.size_t; P4 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Inputs.Source_Records (P1, P2, P3, P4);
   end Source_Records;
   function Source_Files (P1 : System.Address; P2 : access constant Source_Spec_V1; P3 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Inputs.Source_Files (P1, P2, P3);
   end Source_Files;
   function Source_Image_Files (P1 : System.Address; P2 : access constant Source_Spec_V1; P3 : access System.Address) return Interfaces.C.int is
   begin
      return Thinkthen.Native.Inputs.Source_Image_Files (P1, P2, P3);
   end Source_Image_Files;
   procedure Source_Free (P1 : System.Address) is
   begin
      Thinkthen.Native.Inputs.Source_Free (P1);
   end Source_Free;
end Ada_Parity;
