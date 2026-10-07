with Ada.Finalization;
with Thinkthen_C_Inputs;
package Thinkthen.Buffers is
   -- Own UTF-8 bytes for counted descriptors. Set copies without interpreting
   -- JSON, paths, Unicode or NUL. The native constructor validates semantics.
   type Buffer is new Ada.Finalization.Limited_Controlled with private;
   procedure Set (Item : in out Buffer; Bytes : String);
   function View (Item : Buffer) return Thinkthen_C_Inputs.Byte_String_V1;
   function Text (Item : Buffer) return Thinkthen_C_Inputs.Content_V1;
   function Authored_JSON (Item : Buffer) return Thinkthen_C_Inputs.Content_V1;
   -- Views borrow Item until Set/finalization. Constructors clone synchronously.
private
   type String_Owner is access String;
   type Buffer is new Ada.Finalization.Limited_Controlled with record
      Bytes : String_Owner := null;
   end record;
   overriding procedure Finalize (Item : in out Buffer);
end Thinkthen.Buffers;
