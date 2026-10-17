-- Generated native value ownership. Do not edit.
with Ada.Finalization;
with Ada.Strings.Unbounded;
package Thinkthen.Values is
   type Value_Kind is (Null_Value, Boolean_Value, Number_Value, String_Value, Array_Value, Object_Value);
   type Value is new Ada.Finalization.Controlled with private;
   function "=" (Left, Right : Value) return Boolean;
   type Value_List is array (Positive range <>) of Value;
   type Field is record
      Name : Ada.Strings.Unbounded.Unbounded_String;
      Item : Value;
   end record;
   type Field_List is array (Positive range <>) of Field;
   function Null_Item return Value;
   function Boolean_Item (Item : Boolean) return Value;
   -- Keep the decimal spelling intact; native admission validates numbers.
   function Number_Item (Decimal : String) return Value;
   function String_Item (Item : String) return Value;
   function Array_Item (Items : Value_List) return Value;
   function Object_Item (Fields : Field_List) return Value;
   function Named (Name : String; Item : Value) return Field;
   function Kind (Item : Value) return Value_Kind;
   function As_Boolean (Item : Value) return Boolean;
   function As_Number (Item : Value) return String;
   function As_String (Item : Value) return String;
   function Length (Item : Value) return Natural;
   function Element (Item : Value; Index : Positive) return Value;
   function Field_Name (Item : Value; Index : Positive) return String;
   function Has_Field (Item : Value; Name : String) return Boolean;
   function Field_Value (Item : Value; Name : String) return Value;
private
   type Node;
   type Node_Access is access Node;
   type Value is new Ada.Finalization.Controlled with record
      Data : Node_Access := null;
   end record;
   type Value_List_Access is access Value_List;
   type Field_List_Access is access Field_List;
   type Node is record
      Kind : Value_Kind := Null_Value;
      Flag : Boolean := False;
      Text : Ada.Strings.Unbounded.Unbounded_String;
      Items : Value_List_Access := null;
      Fields : Field_List_Access := null;
   end record;
   overriding procedure Adjust (Item : in out Value);
   overriding procedure Finalize (Item : in out Value);
end Thinkthen.Values;
