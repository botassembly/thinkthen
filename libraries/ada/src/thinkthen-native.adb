package body Thinkthen.Native is
   function Native_Decide (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_decide_complete";
   function Decide (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if Options /= null then Copy := Options.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Decide (Client, Question, Input, Copy'Access, Output);
   end Decide;
   function Native_Choose (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_choose_complete";
   function Choose (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if Options /= null then Copy := Options.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Choose (Client, Question, Input, Copy'Access, Output);
   end Choose;
   function Native_Tag (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_tag_complete";
   function Tag (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if Options /= null then Copy := Options.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Tag (Client, Question, Input, Copy'Access, Output);
   end Tag;
   function Native_Score (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_score_complete";
   function Score (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if Options /= null then Copy := Options.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Score (Client, Question, Input, Copy'Access, Output);
   end Score;
   function Native_Filter (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_filter_complete";
   function Filter (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if Options /= null then Copy := Options.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Filter (Client, Question, Input, Copy'Access, Output);
   end Filter;
   function Native_Rank (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_rank_complete";
   function Rank (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if Options /= null then Copy := Options.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Rank (Client, Question, Input, Copy'Access, Output);
   end Rank;
   function Native_Find (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_find_complete";
   function Find (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if Options /= null then Copy := Options.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Find (Client, Question, Input, Copy'Access, Output);
   end Find;
   function Native_Annotate (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_annotate_complete";
   function Annotate (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if Options /= null then Copy := Options.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Annotate (Client, Question, Input, Copy'Access, Output);
   end Annotate;
   function Native_Recognize (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_recognize_complete";
   function Recognize (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if Options /= null then Copy := Options.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Recognize (Client, Question, Input, Copy'Access, Output);
   end Recognize;
   function Native_Relate (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_relate_complete";
   function Relate (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if Options /= null then Copy := Options.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Relate (Client, Question, Input, Copy'Access, Output);
   end Relate;
end Thinkthen.Native;
