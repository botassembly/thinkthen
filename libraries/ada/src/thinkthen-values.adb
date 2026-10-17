-- Generated native value ownership. Do not edit.
with Ada.Unchecked_Deallocation;
package body Thinkthen.Values is
   use Ada.Strings.Unbounded;
   procedure Release_Items is new Ada.Unchecked_Deallocation (Value_List, Value_List_Access);
   procedure Release_Fields is new Ada.Unchecked_Deallocation (Field_List, Field_List_Access);
   procedure Release is new Ada.Unchecked_Deallocation (Node, Node_Access);
   function "=" (Left, Right : Value) return Boolean is
   begin
      if Kind (Left) /= Kind (Right) then return False; end if;
      case Kind (Left) is
         when Null_Value => return True;
         when Boolean_Value => return Left.Data.Flag = Right.Data.Flag;
         when Number_Value | String_Value => return Left.Data.Text = Right.Data.Text;
         when Array_Value => return Left.Data.Items.all = Right.Data.Items.all;
         when Object_Value => return Left.Data.Fields.all = Right.Data.Fields.all;
      end case;
   end "=";
   overriding procedure Adjust (Item : in out Value) is
      Original : constant Node_Access := Item.Data;
   begin
      Item.Data := null;
      if Original /= null then
         Item.Data := new Node'(Kind => Original.Kind, Flag => Original.Flag,
           Text => Original.Text, others => <>);
         if Original.Items /= null then Item.Data.Items := new Value_List'(Original.Items.all); end if;
         if Original.Fields /= null then Item.Data.Fields := new Field_List'(Original.Fields.all); end if;
      end if;
   exception
      when others => Finalize (Item); raise;
   end Adjust;
   overriding procedure Finalize (Item : in out Value) is
   begin
      if Item.Data /= null then
         Release_Items (Item.Data.Items);
         Release_Fields (Item.Data.Fields);
         Release (Item.Data);
      end if;
   end Finalize;
   function Null_Item return Value is
   begin
      return Item : Value do null; end return;
   end Null_Item;
   function Boolean_Item (Item : Boolean) return Value is
   begin
      return Result : Value do
         Result.Data := new Node'(Kind => Boolean_Value, Flag => Item, others => <>);
      end return;
   end Boolean_Item;
   function Number_Item (Decimal : String) return Value is
   begin
      return Result : Value do
         Result.Data := new Node'(Kind => Number_Value, Text => To_Unbounded_String (Decimal), others => <>);
      end return;
   end Number_Item;
   function String_Item (Item : String) return Value is
   begin
      return Result : Value do
         Result.Data := new Node'(Kind => String_Value, Text => To_Unbounded_String (Item), others => <>);
      end return;
   end String_Item;
   function Array_Item (Items : Value_List) return Value is
   begin
      return Result : Value do
         Result.Data := new Node'(Kind => Array_Value, Items => new Value_List (1 .. Items'Length), others => <>);
         Result.Data.Items.all := Items;
      end return;
   end Array_Item;
   function Object_Item (Fields : Field_List) return Value is
   begin
      return Result : Value do
         Result.Data := new Node'(Kind => Object_Value, Fields => new Field_List (1 .. Fields'Length), others => <>);
         Result.Data.Fields.all := Fields;
      end return;
   end Object_Item;
   function Named (Name : String; Item : Value) return Field is
     ((To_Unbounded_String (Name), Item));
   function Kind (Item : Value) return Value_Kind is
     (if Item.Data = null then Null_Value else Item.Data.Kind);
   procedure Require (Item : Value; Expected : Value_Kind) is
   begin
      if Kind (Item) /= Expected then raise Constraint_Error with "value has a different kind"; end if;
   end Require;
   function As_Boolean (Item : Value) return Boolean is
   begin
      Require (Item, Boolean_Value); return Item.Data.Flag;
   end As_Boolean;
   function As_Number (Item : Value) return String is
   begin
      Require (Item, Number_Value); return To_String (Item.Data.Text);
   end As_Number;
   function As_String (Item : Value) return String is
   begin
      Require (Item, String_Value); return To_String (Item.Data.Text);
   end As_String;
   function Length (Item : Value) return Natural is
   begin
      case Kind (Item) is
         when Array_Value => return Item.Data.Items'Length;
         when Object_Value => return Item.Data.Fields'Length;
         when others => raise Constraint_Error with "value is not a collection";
      end case;
   end Length;
   function Element (Item : Value; Index : Positive) return Value is
   begin
      case Kind (Item) is
         when Array_Value => return Item.Data.Items (Index);
         when Object_Value => return Item.Data.Fields (Index).Item;
         when others => raise Constraint_Error with "value is not a collection";
      end case;
   end Element;
   function Field_Name (Item : Value; Index : Positive) return String is
   begin
      Require (Item, Object_Value); return To_String (Item.Data.Fields (Index).Name);
   end Field_Name;
   function Has_Field (Item : Value; Name : String) return Boolean is
   begin
      Require (Item, Object_Value);
      for Field of Item.Data.Fields.all loop
         if To_String (Field.Name) = Name then return True; end if;
      end loop;
      return False;
   end Has_Field;
   function Field_Value (Item : Value; Name : String) return Value is
   begin
      Require (Item, Object_Value);
      for Field of Item.Data.Fields.all loop
         if To_String (Field.Name) = Name then return Field.Item; end if;
      end loop;
      raise Constraint_Error with "field is missing";
   end Field_Value;
end Thinkthen.Values;
