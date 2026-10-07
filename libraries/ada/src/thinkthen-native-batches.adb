package body Thinkthen.Native.Batches is
   function Native_Decide_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_decide_batch_start";
   function Decide_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if P4 /= null then Copy := P4.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Decide_Batch_Start (P1, P2, P3, Copy'Access, P5);
   end Decide_Batch_Start;
   function Native_Choose_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_choose_batch_start";
   function Choose_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if P4 /= null then Copy := P4.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Choose_Batch_Start (P1, P2, P3, Copy'Access, P5);
   end Choose_Batch_Start;
   function Native_Tag_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_tag_batch_start";
   function Tag_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if P4 /= null then Copy := P4.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Tag_Batch_Start (P1, P2, P3, Copy'Access, P5);
   end Tag_Batch_Start;
   function Native_Score_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_score_batch_start";
   function Score_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if P4 /= null then Copy := P4.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Score_Batch_Start (P1, P2, P3, Copy'Access, P5);
   end Score_Batch_Start;
   function Native_Filter_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_filter_batch_start";
   function Filter_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if P4 /= null then Copy := P4.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Filter_Batch_Start (P1, P2, P3, Copy'Access, P5);
   end Filter_Batch_Start;
   function Native_Annotate_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_annotate_batch_start";
   function Annotate_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int is
      Copy : aliased Controls_V1 := (Deadline_Ms => -1, others => <>);
      Surface : aliased constant String := "ada";
   begin
      if P4 /= null then Copy := P4.all; end if;
      Copy.Surface := (Surface'Address, 3);
      return Native_Annotate_Batch_Start (P1, P2, P3, Copy'Access, P5);
   end Annotate_Batch_Start;
end Thinkthen.Native.Batches;
