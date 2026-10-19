"""Copy shared canonical native values into independent Ada values."""


def outputs():
    spec = """-- Generated canonical value copy adapter. Do not edit.
with Thinkthen.Values;
with Thinkthen.Session_C;
private package Thinkthen.Native_Values is
   function Read (P : access constant Thinkthen.Session_C.thinkthen_complete_json_v1) return Thinkthen.Values.Value;
end Thinkthen.Native_Values;
"""
    body = """-- Generated canonical value copy adapter. Do not edit.
with Ada.Unchecked_Conversion;
with Interfaces;
with Thinkthen.Native_Sessions; use Thinkthen.Native_Sessions;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with System;
package body Thinkthen.Native_Values is
   use Thinkthen.Values;
   use Thinkthen.Session_C;
   use type Interfaces.Unsigned_32;
""" + JSON_READER.replace('Read_Json', 'Read').split('\n', 1)[1] + '\nend Thinkthen.Native_Values;\n'
    return {'thinkthen-native_values.ads': spec, 'thinkthen-native_values.adb': body}


JSON_READER = '''function Read_Json (P : access constant thinkthen_complete_json_v1) return Value;
function Read_Json (P : access constant thinkthen_complete_json_v1) return Value is
begin
   if P = null then raise Constraint_Error with "missing native value"; end if;
   case P.kind is
      when K_THINKTHEN_COMPLETE_JSON_NULL_V1 => return Null_Item;
      when K_THINKTHEN_COMPLETE_JSON_BOOLEAN_V1 => return Boolean_Item (P.data.boolean /= 0);
      when K_THINKTHEN_COMPLETE_JSON_NUMBER_V1 => return Number_Item (Text (P.data.number));
      when K_THINKTHEN_COMPLETE_JSON_STRING_V1 => return String_Item (Text (P.data.string));
      when K_THINKTHEN_COMPLETE_JSON_ARRAY_V1 =>
         declare
            Count : constant Natural := Index (P.data.c_array.len);
            type Item_Pointer is access constant thinkthen_complete_json_v1;
            type Items_Type is array (1 .. Count) of Item_Pointer with Convention => C;
            type Items_Access is access constant Items_Type;
            function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
            Items : constant Items_Access := Pointer (P.data.c_array.data);
            Copy : Value_List (1 .. Count);
         begin
            for I in 1 .. Count loop Copy (I) := Read_Json (Items (I)); end loop;
            return Array_Item (Copy);
         end;
      when K_THINKTHEN_COMPLETE_JSON_OBJECT_V1 =>
         declare
            Count : constant Natural := Index (P.data.object.len);
            type Items_Type is array (1 .. Count) of thinkthen_complete_json_entry_v1 with Convention => C;
            type Items_Access is access constant Items_Type;
            function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
            Items : Items_Access;
            Copy : Field_List (1 .. Count);
         begin
            if Count > 0 then Items := Pointer (P.data.object.data.all'Address); end if;
            for I in 1 .. Count loop Copy (I) := Named (Text (Items (I).name), Read_Json (Items (I).value)); end loop;
            return Object_Item (Copy);
         end;
      when others => raise Constraint_Error with "invalid native value kind";
   end case;
end Read_Json;
'''
