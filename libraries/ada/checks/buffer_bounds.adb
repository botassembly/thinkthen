with Interfaces.C; use Interfaces.C;
with System;
with Thinkthen.Buffers;
with Thinkthen.Views; use Thinkthen.Views;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
procedure Buffer_Bounds is
   Item : Thinkthen.Buffers.Buffer;
   Empty : constant Byte_String_V1 := Thinkthen.Buffers.View (Item);
   procedure Require (Condition : Boolean) is
   begin
      if not Condition then raise Program_Error with "counted buffer boundary"; end if;
   end Require;
   procedure Refuses (Data : System.Address; Count : size_t) is
   begin
      declare
         Unused : constant String := Value ((Data, Count));
      begin
         raise Program_Error with "invalid string accepted " & Unused;
      end;
   exception when Constraint_Error => null; end;
begin
   Require (Empty.Len = 0 and Value (Empty) = "");
   Thinkthen.Buffers.Set (Item, (1 .. 9000 => 'x'));
   Require (Value (Thinkthen.Buffers.View (Item)) = (1 .. 9000 => 'x'));
   Thinkthen.Buffers.Set (Item, "é" & ASCII.CR & ASCII.LF & ASCII.NUL & "z");
   Require (Value (Thinkthen.Buffers.View (Item)) = "é" & ASCII.CR & ASCII.LF & ASCII.NUL & "z");
   Refuses (System.Null_Address, 1);
   Refuses (System.Null_Address, size_t (Natural'Last) + 1);
   Thinkthen.Buffers.Set (Item, "");
   Require (Thinkthen.Buffers.View (Item).Len = 0);
end Buffer_Bounds;
