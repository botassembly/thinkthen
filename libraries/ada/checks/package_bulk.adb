with Ada.Directories;
with Ada.Environment_Variables;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Interfaces;
with Thinkthen; use Thinkthen;
procedure Package_Bulk is
   Client : Engine;
   Barrier : constant String := Ada.Environment_Variables.Value ("TT_BARRIER_DIR");
   procedure Require (Good : Boolean; Note : String) is
   begin
      if not Good then raise Program_Error with Note; end if;
   end Require;
   procedure Arrive (Name : String) is
   begin
      for I in 1 .. 2_000 loop
         exit when Ada.Directories.Exists (Barrier & "/arrived-" & Name);
         delay 0.005;
      end loop;
      Require (Ada.Directories.Exists (Barrier & "/arrived-" & Name), "missing held arrival: " & Name);
   end Arrive;
   procedure Release (Name : String) is
      F : File_Type;
   begin
      Create (F, Out_File, Barrier & "/release-" & Name); Close (F);
   end Release;
   Basic : constant Evidence_Array := (To_Unbounded_String ("first"), To_Unbounded_String ("second"), To_Unbounded_String ("third"));
   Answers : Decision_Array (1 .. 3);
   Error : Failure;
begin
   Decide_Many (Client, "Is it?", Basic, Answers, Error);
   Require (Error.Kind = None and Answers (1).Value = Yes and Answers (2).Value = No and
            Answers (3).Value = Yes and Answers (1).Probability = 0.9 and
            Answers (2).Probability = 0.1 and Answers (3).Probability = 0.6,
            "typed bulk result order");
   declare
      Token : aliased Cancel_Token;
      Answer : Decision;
      Kind : Error_Kind := None;
      task Caller is entry Start; end Caller;
      task body Caller is
         Err : Failure;
      begin
         accept Start;
         Decide (Client, "Is it?", "hold-scalar", Answer, Err, Token => Token'Access);
         Kind := Err.Kind;
      end Caller;
   begin
      Caller.Start; Arrive ("hold-scalar"); Cancel (Token); Cancel (Token); Release ("hold-scalar");
      -- The task joins at this scope's end before its token is finalized.
      for I in 1 .. 2_000 loop exit when Caller'Terminated; delay 0.005; end loop;
      Require (Caller'Terminated and Kind = Cancelled, "typed held scalar");
   end;
   declare
      Token : aliased Cancel_Token;
      Evidence : constant Evidence_Array := (To_Unbounded_String ("hold-bulk-1"), To_Unbounded_String ("hold-bulk-2"));
      Results : Decision_Array (1 .. 2);
      Kind : Error_Kind := None;
      task Caller is entry Start; end Caller;
      task body Caller is
         Err : Failure;
      begin
         accept Start;
         Decide_Many (Client, "Is it?", Evidence, Results, Err, Token => Token'Access);
         Kind := Err.Kind;
      end Caller;
   begin
      Caller.Start; Arrive ("hold-bulk-1"); Cancel (Token); Cancel (Token);
      Release ("hold-bulk-1"); Release ("hold-bulk-2");
      for I in 1 .. 2_000 loop exit when Caller'Terminated; delay 0.005; end loop;
      Require (Caller'Terminated and Kind = Cancelled, "typed held bulk");
   end;
   declare
      Kind : Error_Kind := None;
      Answer : Decision;
      task Caller is entry Start; end Caller;
      task body Caller is
         Err : Failure;
      begin
         accept Start;
         Decide (Client, "Is it?", "hold-deadline", Answer, Err, Deadline_Ms => 80);
         Kind := Err.Kind;
      end Caller;
   begin
      Caller.Start; Arrive ("hold-deadline"); delay 0.11; Release ("hold-deadline");
      for I in 1 .. 2_000 loop exit when Caller'Terminated; delay 0.005; end loop;
      Require (Caller'Terminated and Kind = Deadline, "typed held deadline");
   end;
   Decide (Client, "Is it?", "recovery-package", Answers (1), Error);
   Require (Error.Kind = None and Answers (1).Value = Yes, "fresh-token recovery");
   Put_Line ("TYPED_ADA_MATRIX_PASS");
end Package_Bulk;
