with Interfaces.C;
with System;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
package Thinkthen.Native is
   function Decide (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int;
   function Choose (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int;
   function Tag (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int;
   function Score (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int;
   function Filter (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int;
   function Rank (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int;
   function Find (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int;
   function Annotate (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int;
   function Recognize (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int;
   function Relate (Client : System.Address; Question : System.Address; Input : System.Address; Options : access constant Controls_V1; Output : access System.Address) return Interfaces.C.int;
end Thinkthen.Native;
