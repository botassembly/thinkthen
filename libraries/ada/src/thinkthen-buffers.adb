with Ada.Unchecked_Deallocation;
with Interfaces.C;
with System;
package body Thinkthen.Buffers is
   procedure Release is new Ada.Unchecked_Deallocation (String, String_Owner);
   procedure Set (Item : in out Buffer; Bytes : String) is
      Copy : constant String_Owner := new String'(Bytes);
   begin
      Release (Item.Bytes);
      Item.Bytes := Copy;
   end Set;
   overriding procedure Finalize (Item : in out Buffer) is
   begin
      Release (Item.Bytes);
   end Finalize;
   function View (Item : Buffer) return Thinkthen_C_Inputs.Byte_String_V1 is
   begin
      if Item.Bytes = null or else Item.Bytes'Length = 0 then
         return (Data => System.Null_Address, Len => 0);
      end if;
      return (Data => Item.Bytes (Item.Bytes'First)'Address,
              Len => Interfaces.C.size_t (Item.Bytes'Length));
   end View;
   function Text (Item : Buffer) return Thinkthen_C_Inputs.Content_V1 is
     ((Kind => Thinkthen_C_Inputs.C_CONTENT_TEXT_V1, Data => View (Item)));
   function Authored_JSON (Item : Buffer) return Thinkthen_C_Inputs.Content_V1 is
     ((Kind => Thinkthen_C_Inputs.C_CONTENT_JSON_V1, Data => View (Item)));
end Thinkthen.Buffers;
