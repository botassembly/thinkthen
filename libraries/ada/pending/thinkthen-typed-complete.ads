with Thinkthen_C_Rows; use Thinkthen_C_Rows;
with Thinkthen_C_Events; use Thinkthen_C_Events;
package Thinkthen.Typed.Complete is
   -- Private integration slice: native result/2 execution not yet available.
   type Result is new Ada.Finalization.Limited_Controlled with private;
   function Exists (Item : Result) return Boolean;
   procedure Decide (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int);
   procedure Decide (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Decide_View_V1; Code : out Interfaces.C.int);
   procedure Choose (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int);
   procedure Choose (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Choose_View_V1; Code : out Interfaces.C.int);
   procedure Tag (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int);
   procedure Tag (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Tag_View_V1; Code : out Interfaces.C.int);
   procedure Score (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int);
   procedure Score (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Score_View_V1; Code : out Interfaces.C.int);
   procedure Filter (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int);
   procedure Filter (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Filter_View_V1; Code : out Interfaces.C.int);
   procedure Rank (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int);
   procedure Rank (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Rank_View_V1; Code : out Interfaces.C.int);
   procedure Find (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int);
   procedure Find (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Find_View_V1; Code : out Interfaces.C.int);
   procedure Annotate (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int);
   procedure Annotate (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Annotate_View_V1; Code : out Interfaces.C.int);
   procedure Recognize (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int);
   procedure Recognize (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Recognize_View_V1; Code : out Interfaces.C.int);
   procedure Relate (Client : Engine; Q : Question; Input : Source;
                    Options : Controls_V1; Item : in out Result; Code : out Interfaces.C.int);
   procedure Relate (Item : Result; Index : Interfaces.C.size_t;
                    View : in out Relate_View_V1; Code : out Interfaces.C.int);
   procedure Summary (Item : Result; View : in out Summary_V1; Code : out Interfaces.C.int);
   procedure Observation (Item : Result; Index : Interfaces.C.size_t; View : in out Observation_V1; Code : out Interfaces.C.int);
   procedure Error_Snapshot (Client : Engine; Item : in out Result; Code : out Interfaces.C.int);
   -- Null-engine overload reads the calling thread's failed-build slot.
   procedure Error_Snapshot (Item : in out Result; Code : out Interfaces.C.int);
private
   type Result is new Ada.Finalization.Limited_Controlled with record
      Handle : aliased System.Address := System.Null_Address;
   end record;
   overriding procedure Finalize (Item : in out Result);
end Thinkthen.Typed.Complete;
