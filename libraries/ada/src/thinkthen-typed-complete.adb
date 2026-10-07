with Thinkthen.Native;
with Thinkthen.Native.Results;
package body Thinkthen.Typed.Complete is
   use type Interfaces.C.int;
   use type System.Address;
   function Borrow (Item : Result) return System.Address is (Item.Handle);
   function Exists (Item : Result) return Boolean is (Item.Handle /= System.Null_Address);
   procedure Result_Free (Item : System.Address)
     with Import, Convention => C, External_Name => "thinkthen_result_free";
   overriding procedure Finalize (Item : in out Result) is
   begin
      if Item.Handle /= System.Null_Address then Result_Free (Item.Handle); end if;
      Item.Handle := System.Null_Address;
   end Finalize;
   function Native_Decide (E, Q, Input : System.Address;
                           Options : access constant Controls_V1;
                           Out_Handle : access System.Address) return Interfaces.C.int renames Thinkthen.Native.Decide;
   function View_Decide (Item : System.Address; Index : Interfaces.C.size_t;
                          View : access Decide_View_V1) return Interfaces.C.int renames Thinkthen.Native.Results.Result_Decide;
   procedure Decide (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int) is
      Copy : aliased Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native_Decide (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Decide;
   procedure Decide (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Decide_View_V1; Code : out Interfaces.C.int) is
      Copy : aliased Decide_View_V1 := View;
   begin
      Code := View_Decide (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Decide;
   function Native_Choose (E, Q, Input : System.Address;
                           Options : access constant Controls_V1;
                           Out_Handle : access System.Address) return Interfaces.C.int renames Thinkthen.Native.Choose;
   function View_Choose (Item : System.Address; Index : Interfaces.C.size_t;
                          View : access Choose_View_V1) return Interfaces.C.int renames Thinkthen.Native.Results.Result_Choose;
   procedure Choose (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int) is
      Copy : aliased Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native_Choose (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Choose;
   procedure Choose (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Choose_View_V1; Code : out Interfaces.C.int) is
      Copy : aliased Choose_View_V1 := View;
   begin
      Code := View_Choose (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Choose;
   function Native_Tag (E, Q, Input : System.Address;
                           Options : access constant Controls_V1;
                           Out_Handle : access System.Address) return Interfaces.C.int renames Thinkthen.Native.Tag;
   function View_Tag (Item : System.Address; Index : Interfaces.C.size_t;
                          View : access Tag_View_V1) return Interfaces.C.int renames Thinkthen.Native.Results.Result_Tag;
   procedure Tag (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int) is
      Copy : aliased Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native_Tag (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Tag;
   procedure Tag (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Tag_View_V1; Code : out Interfaces.C.int) is
      Copy : aliased Tag_View_V1 := View;
   begin
      Code := View_Tag (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Tag;
   function Native_Score (E, Q, Input : System.Address;
                           Options : access constant Controls_V1;
                           Out_Handle : access System.Address) return Interfaces.C.int renames Thinkthen.Native.Score;
   function View_Score (Item : System.Address; Index : Interfaces.C.size_t;
                          View : access Score_View_V1) return Interfaces.C.int renames Thinkthen.Native.Results.Result_Score;
   procedure Score (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int) is
      Copy : aliased Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native_Score (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Score;
   procedure Score (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Score_View_V1; Code : out Interfaces.C.int) is
      Copy : aliased Score_View_V1 := View;
   begin
      Code := View_Score (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Score;
   function Native_Filter (E, Q, Input : System.Address;
                           Options : access constant Controls_V1;
                           Out_Handle : access System.Address) return Interfaces.C.int renames Thinkthen.Native.Filter;
   function View_Filter (Item : System.Address; Index : Interfaces.C.size_t;
                          View : access Filter_View_V1) return Interfaces.C.int renames Thinkthen.Native.Results.Result_Filter;
   procedure Filter (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int) is
      Copy : aliased Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native_Filter (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Filter;
   procedure Filter (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Filter_View_V1; Code : out Interfaces.C.int) is
      Copy : aliased Filter_View_V1 := View;
   begin
      Code := View_Filter (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Filter;
   function Native_Rank (E, Q, Input : System.Address;
                           Options : access constant Controls_V1;
                           Out_Handle : access System.Address) return Interfaces.C.int renames Thinkthen.Native.Rank;
   function View_Rank (Item : System.Address; Index : Interfaces.C.size_t;
                          View : access Rank_View_V1) return Interfaces.C.int renames Thinkthen.Native.Results.Result_Rank;
   procedure Rank (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int) is
      Copy : aliased Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native_Rank (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Rank;
   procedure Rank (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Rank_View_V1; Code : out Interfaces.C.int) is
      Copy : aliased Rank_View_V1 := View;
   begin
      Code := View_Rank (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Rank;
   function Native_Find (E, Q, Input : System.Address;
                           Options : access constant Controls_V1;
                           Out_Handle : access System.Address) return Interfaces.C.int renames Thinkthen.Native.Find;
   function View_Find (Item : System.Address; Index : Interfaces.C.size_t;
                          View : access Find_View_V1) return Interfaces.C.int renames Thinkthen.Native.Results.Result_Find;
   procedure Find (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int) is
      Copy : aliased Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native_Find (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Find;
   procedure Find (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Find_View_V1; Code : out Interfaces.C.int) is
      Copy : aliased Find_View_V1 := View;
   begin
      Code := View_Find (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Find;
   function Native_Annotate (E, Q, Input : System.Address;
                           Options : access constant Controls_V1;
                           Out_Handle : access System.Address) return Interfaces.C.int renames Thinkthen.Native.Annotate;
   function View_Annotate (Item : System.Address; Index : Interfaces.C.size_t;
                          View : access Annotate_View_V1) return Interfaces.C.int renames Thinkthen.Native.Results.Result_Annotate;
   procedure Annotate (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int) is
      Copy : aliased Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native_Annotate (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Annotate;
   procedure Annotate (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Annotate_View_V1; Code : out Interfaces.C.int) is
      Copy : aliased Annotate_View_V1 := View;
   begin
      Code := View_Annotate (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Annotate;
   function Native_Recognize (E, Q, Input : System.Address;
                           Options : access constant Controls_V1;
                           Out_Handle : access System.Address) return Interfaces.C.int renames Thinkthen.Native.Recognize;
   function View_Recognize (Item : System.Address; Index : Interfaces.C.size_t;
                          View : access Recognize_View_V1) return Interfaces.C.int renames Thinkthen.Native.Results.Result_Recognize;
   procedure Recognize (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int) is
      Copy : aliased Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native_Recognize (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Recognize;
   procedure Recognize (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Recognize_View_V1; Code : out Interfaces.C.int) is
      Copy : aliased Recognize_View_V1 := View;
   begin
      Code := View_Recognize (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Recognize;
   function Native_Relate (E, Q, Input : System.Address;
                           Options : access constant Controls_V1;
                           Out_Handle : access System.Address) return Interfaces.C.int renames Thinkthen.Native.Relate;
   function View_Relate (Item : System.Address; Index : Interfaces.C.size_t;
                          View : access Relate_View_V1) return Interfaces.C.int renames Thinkthen.Native.Results.Result_Relate;
   procedure Relate (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int) is
      Copy : aliased Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native_Relate (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Relate;
   procedure Relate (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Relate_View_V1; Code : out Interfaces.C.int) is
      Copy : aliased Relate_View_V1 := View;
   begin
      Code := View_Relate (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Relate;
   function Native_Summary (Item : System.Address; View : access Summary_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_summary";
   procedure Summary (Item : Result; View : in out Summary_V1; Code : out Interfaces.C.int) is
      Copy : aliased Summary_V1 := View;
   begin
      Code := Native_Summary (Item.Handle, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Summary;
   function Native_Observation (Item : System.Address; Index : Interfaces.C.size_t; View : access Observation_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_result_observation";
   procedure Observation (Item : Result; Index : Interfaces.C.size_t; View : in out Observation_V1; Code : out Interfaces.C.int) is
      Copy : aliased Observation_V1 := View;
   begin
      Code := Native_Observation (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Observation;
   procedure Row (Item : Result; Index : Interfaces.C.size_t; View : in out Row_Observation_V1; Code : out Interfaces.C.int) is
      Copy : aliased Row_Observation_V1 := View;
   begin
      Code := Native.Results.Result_Row (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Row;
   function Native_Error (E : System.Address; Out_Handle : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_error_complete";
   procedure Error_Snapshot (Client : Engine; Item : in out Result; Code : out Interfaces.C.int) is
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native_Error (Client.Handle, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Error_Snapshot;
   procedure Error_Snapshot (Item : in out Result; Code : out Interfaces.C.int) is
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native_Error (System.Null_Address, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Error_Snapshot;
   procedure Details (Item : Result; Index : Interfaces.C.size_t; View : in out Details_V1; Code : out Interfaces.C.int) is
      Copy : aliased Details_V1 := View;
   begin
      Code := Native.Results.Result_Details
        (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Details;
   procedure Observation_Details (Item : Result; Index : Interfaces.C.size_t; View : in out Details_V1; Code : out Interfaces.C.int) is
      Copy : aliased Details_V1 := View;
   begin
      Code := Native.Results.Result_Observation_Details
        (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Observation_Details;
   procedure Question_Author (Item : Result; Index : Interfaces.C.size_t; View : in out Question_Author_V1; Code : out Interfaces.C.int) is
      Copy : aliased Question_Author_V1 := View;
   begin
      Code := Native.Results.Result_Question_Author
        (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Question_Author;
   procedure Member_Author (Item : Result; Index : Interfaces.C.size_t; Member : Interfaces.C.size_t; View : in out Question_Author_V1; Code : out Interfaces.C.int) is
      Copy : aliased Question_Author_V1 := View;
   begin
      Code := Native.Results.Result_Member_Author
        (Item.Handle, Index, Member, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Member_Author;
   procedure Observation_Author (Item : Result; Index : Interfaces.C.size_t; View : in out Question_Author_V1; Code : out Interfaces.C.int) is
      Copy : aliased Question_Author_V1 := View;
   begin
      Code := Native.Results.Result_Observation_Author
        (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Observation_Author;
   procedure Source_Recognition (Item : Result; Index : Interfaces.C.size_t; View : in out Source_Recognition_V1; Code : out Interfaces.C.int) is
      Copy : aliased Source_Recognition_V1 := View;
   begin
      Code := Native.Results.Result_Source_Recognition
        (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Source_Recognition;
   procedure Source_Relations (Item : Result; Index : Interfaces.C.size_t; View : in out Source_Relations_V1; Code : out Interfaces.C.int) is
      Copy : aliased Source_Relations_V1 := View;
   begin
      Code := Native.Results.Result_Source_Relations
        (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Source_Relations;
   procedure Rank_Member_Details (Item : Result; Index : Interfaces.C.size_t; Member : Interfaces.C.size_t; View : in out Details_V1; Code : out Interfaces.C.int) is
      Copy : aliased Details_V1 := View;
   begin
      Code := Native.Results.Result_Rank_Member_Details
        (Item.Handle, Index, Member, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Rank_Member_Details;
   procedure Rank_Member (Item : Result; Index : Interfaces.C.size_t; Member : Interfaces.C.size_t; View : in out Rank_View_V1; Code : out Interfaces.C.int) is
      Copy : aliased Rank_View_V1 := View;
   begin
      Code := Native.Results.Result_Rank_Member
        (Item.Handle, Index, Member, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Rank_Member;
   procedure Rank_Member_Count (Item : Result; Index : Interfaces.C.size_t; View : in out Interfaces.C.size_t; Code : out Interfaces.C.int) is
      Copy : aliased Interfaces.C.size_t := View;
   begin
      Code := Native.Results.Result_Rank_Member_Count
        (Item.Handle, Index, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Rank_Member_Count;
end Thinkthen.Typed.Complete;
