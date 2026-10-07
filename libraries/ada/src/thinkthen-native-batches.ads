with Interfaces.C;
with System;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
package Thinkthen.Native.Batches is
   function Decide_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int;
   function Choose_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int;
   function Tag_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int;
   function Score_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int;
   function Filter_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int;
   function Annotate_Batch_Start (P1 : System.Address; P2 : System.Address; P3 : System.Address; P4 : access constant Controls_V1; P5 : access System.Address) return Interfaces.C.int;
   function Batch_Next (P1 : System.Address; P2 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_batch_next";
   function Batch_Facts (P1 : System.Address; P2 : access System.Address) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_batch_facts";
   procedure Batch_Free (P1 : System.Address)
     with Import, Convention => C, External_Name => "thinkthen_batch_free";
end Thinkthen.Native.Batches;
