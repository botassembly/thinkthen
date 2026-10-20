"""Lower owned native Ada value trees into the canonical native value layout."""

WRITE = '''function Convert_Json_Value (Value : Thinkthen.Values.Value; Storage : in out Arena) return System.Address;
function Convert_Json_Value (Value : Thinkthen.Values.Value; Storage : in out Arena) return System.Address is
   Result : thinkthen_complete_json_v1;
begin
   case Thinkthen.Values.Kind (Value) is
      when Thinkthen.Values.Null_Value =>
         Result.kind := K_THINKTHEN_COMPLETE_JSON_NULL_V1;
         Result.data := (discr => 0, boolean => 0);
      when Thinkthen.Values.Boolean_Value =>
         Result.kind := K_THINKTHEN_COMPLETE_JSON_BOOLEAN_V1;
         Result.data := (discr => 0, boolean => (if Thinkthen.Values.As_Boolean (Value) then 1 else 0));
      when Thinkthen.Values.Number_Value =>
         Result.kind := K_THINKTHEN_COMPLETE_JSON_NUMBER_V1;
         Result.data := (discr => 1, number => UTF8 (Thinkthen.Values.As_Number (Value), Storage));
      when Thinkthen.Values.String_Value =>
         Result.kind := K_THINKTHEN_COMPLETE_JSON_STRING_V1;
         Result.data := (discr => 2, string => UTF8 (Thinkthen.Values.As_String (Value), Storage));
      when Thinkthen.Values.Array_Value =>
         declare
            Length : constant size_t := size_t (Thinkthen.Values.Length (Value));
            Address : constant System.Address := Allocate (Storage, Length * (System.Address'Size / 8));
            type Items_Type is array (size_t range <>) of System.Address with Convention => C;
            Items : Items_Type (1 .. Length) with Import, Address => Address;
         begin
            for I in 1 .. Length loop
               Items (I) := Convert_Json_Value (Thinkthen.Values.Element (Value, Positive (I)), Storage);
            end loop;
            Result.kind := K_THINKTHEN_COMPLETE_JSON_ARRAY_V1;
            Result.data := (discr => 3, c_array => (data => Address, len => Length));
         end;
      when Thinkthen.Values.Object_Value =>
         declare
            Length : constant size_t := size_t (Thinkthen.Values.Length (Value));
            Address : constant System.Address := Allocate (Storage, Length * (thinkthen_complete_json_entry_v1'Size / 8));
            type Items_Type is array (size_t range <>) of thinkthen_complete_json_entry_v1 with Convention => C;
            Items : Items_Type (1 .. Length) with Import, Address => Address;
         begin
            for I in 1 .. Length loop
               Items (I) := (name => UTF8 (Thinkthen.Values.Field_Name (Value, Positive (I)), Storage),
                 value => P_thinkthen_complete_json_v1 (Convert_Json_Value (Thinkthen.Values.Element (Value, Positive (I)), Storage)));
            end loop;
            Result.kind := K_THINKTHEN_COMPLETE_JSON_OBJECT_V1;
            Result.data := (discr => 4, object => (data => P_thinkthen_complete_json_entry_v1 (Address), len => Length));
         end;
   end case;
   return Store_thinkthen_complete_json_v1 (Result, Storage);
end Convert_Json_Value;
'''
