with Thinkthen.Native.Batches;
package Thinkthen.Typed.Complete.Batches is
   type Batch is new Ada.Finalization.Limited_Controlled with private;
   procedure Decide (Client : Engine; Q : Question; Input : Source;
                     Options : Controls_V1; Item : in out Batch;
                     Code : out Interfaces.C.int);
   procedure Choose (Client : Engine; Q : Question; Input : Source;
                     Options : Controls_V1; Item : in out Batch;
                     Code : out Interfaces.C.int);
   procedure Tag (Client : Engine; Q : Question; Input : Source;
                     Options : Controls_V1; Item : in out Batch;
                     Code : out Interfaces.C.int);
   procedure Score (Client : Engine; Q : Question; Input : Source;
                     Options : Controls_V1; Item : in out Batch;
                     Code : out Interfaces.C.int);
   procedure Filter (Client : Engine; Q : Question; Input : Source;
                     Options : Controls_V1; Item : in out Batch;
                     Code : out Interfaces.C.int);
   procedure Annotate (Client : Engine; Q : Question; Input : Source;
                     Options : Controls_V1; Item : in out Batch;
                     Code : out Interfaces.C.int);
   procedure Next (Item : in out Batch; Output : in out Result;
                   Code : out Interfaces.C.int);
   procedure Facts (Item : Batch; Output : in out Result;
                    Code : out Interfaces.C.int);
   -- The engine stays live. Use and finalize the batch on its creating task.
   -- Children and input buffers may finalize after start. Each row is owned.
private
   type Batch is new Ada.Finalization.Limited_Controlled with record
      Handle : aliased System.Address := System.Null_Address;
   end record;
   overriding procedure Finalize (Item : in out Batch);
end Thinkthen.Typed.Complete.Batches;
