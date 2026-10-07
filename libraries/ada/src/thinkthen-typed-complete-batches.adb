package body Thinkthen.Typed.Complete.Batches is
   use type Interfaces.C.int;
   use type System.Address;
   procedure Decide (Client : Engine; Q : Question; Input : Source;
                     Options : Controls_V1; Item : in out Batch;
                     Code : out Interfaces.C.int) is
      Copy : aliased constant Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native.Batches.Decide_Batch_Start
        (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Decide;
   procedure Choose (Client : Engine; Q : Question; Input : Source;
                     Options : Controls_V1; Item : in out Batch;
                     Code : out Interfaces.C.int) is
      Copy : aliased constant Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native.Batches.Choose_Batch_Start
        (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Choose;
   procedure Tag (Client : Engine; Q : Question; Input : Source;
                     Options : Controls_V1; Item : in out Batch;
                     Code : out Interfaces.C.int) is
      Copy : aliased constant Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native.Batches.Tag_Batch_Start
        (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Tag;
   procedure Score (Client : Engine; Q : Question; Input : Source;
                     Options : Controls_V1; Item : in out Batch;
                     Code : out Interfaces.C.int) is
      Copy : aliased constant Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native.Batches.Score_Batch_Start
        (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Score;
   procedure Filter (Client : Engine; Q : Question; Input : Source;
                     Options : Controls_V1; Item : in out Batch;
                     Code : out Interfaces.C.int) is
      Copy : aliased constant Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native.Batches.Filter_Batch_Start
        (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Filter;
   procedure Annotate (Client : Engine; Q : Question; Input : Source;
                     Options : Controls_V1; Item : in out Batch;
                     Code : out Interfaces.C.int) is
      Copy : aliased constant Controls_V1 := Options;
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native.Batches.Annotate_Batch_Start
        (Client.Handle, Borrow (Q), Borrow (Input), Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Annotate;
   procedure Next (Item : in out Batch; Output : in out Result;
                   Code : out Interfaces.C.int) is
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native.Batches.Batch_Next (Item.Handle, New_Handle'Access);
      if Code = 0 then Finalize (Output); Output.Handle := New_Handle; end if;
   end Next;
   procedure Facts (Item : Batch; Output : in out Result;
                   Code : out Interfaces.C.int) is
      New_Handle : aliased System.Address := System.Null_Address;
   begin
      Code := Native.Batches.Batch_Facts (Item.Handle, New_Handle'Access);
      if Code = 0 then Finalize (Output); Output.Handle := New_Handle; end if;
   end Facts;
   overriding procedure Finalize (Item : in out Batch) is
   begin
      if Item.Handle /= System.Null_Address then
         Native.Batches.Batch_Free (Item.Handle);
      end if;
      Item.Handle := System.Null_Address;
   end Finalize;
end Thinkthen.Typed.Complete.Batches;
