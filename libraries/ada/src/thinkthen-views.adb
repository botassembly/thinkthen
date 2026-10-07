with System.Address_To_Access_Conversions;
with System.Storage_Elements; use System.Storage_Elements;
package body Thinkthen.Views is
   use type Interfaces.C.size_t;
   use type System.Address;
   function Address_At (Data : System.Address; Count, Index : Interfaces.C.size_t;
                        Bytes, Alignment : Integer_Address) return System.Address is
      Offset : Integer_Address;
      Base : constant Integer_Address := To_Integer (Data);
   begin
      if Data = System.Null_Address or else Index >= Count
        or else Integer_Address (Count) > Integer_Address'Last / Bytes
      then
         raise Constraint_Error with "invalid native list";
      end if;
      Offset := Integer_Address (Index) * Bytes;
      if Base mod Alignment /= 0
        or else Base > Integer_Address'Last - Integer_Address (Count) * Bytes
      then
         raise Constraint_Error with "invalid native address range";
      end if;
      return To_Address (Base + Offset);
   end Address_At;
   function Value (Item : Byte_String_V1) return String is
      package Chars is new System.Address_To_Access_Conversions (Character);
   begin
      if Item.Len > Interfaces.C.size_t (Natural'Last) then
         raise Constraint_Error with "native string exceeds Ada String bound";
      end if;
      if Item.Len = 0 then return ""; end if;
      declare
         Base : constant Integer_Address := To_Integer
           (Address_At (Item.Data, Item.Len, 0, 1, 1));
         Copy : String (1 .. Natural (Item.Len));
      begin
         for I in Copy'Range loop
            Copy (I) := Chars.To_Pointer
              (To_Address (Base + Integer_Address (I - 1))).all;
         end loop;
         return Copy;
      end;
   end Value;
   function Question_At (Pointer : System.Address) return Question_View_V1 is
      package Questions is new System.Address_To_Access_Conversions (Question_View_V1);
   begin
      return Questions.To_Pointer
        (Address_At (Pointer, 1, 0, Question_View_V1'Size / 8, Question_View_V1'Alignment)).all;
   end Question_At;
   function Element (Items : Strings_V1; Index : Interfaces.C.size_t) return Byte_String_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Byte_String_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Byte_String_V1'Size / 8, Byte_String_V1'Alignment)).all;
   end Element;
   function Element (Items : Choices_V1; Index : Interfaces.C.size_t) return Choice_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Choice_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Choice_V1'Size / 8, Choice_V1'Alignment)).all;
   end Element;
   function Element (Items : Relations_V1; Index : Interfaces.C.size_t) return Relation_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Relation_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Relation_V1'Size / 8, Relation_V1'Alignment)).all;
   end Element;
   function Element (Items : Member_Specs_V1; Index : Interfaces.C.size_t) return Member_Spec_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Member_Spec_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Member_Spec_V1'Size / 8, Member_Spec_V1'Alignment)).all;
   end Element;
   function Element (Items : Question_Members_V1; Index : Interfaces.C.size_t) return Question_Member_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Question_Member_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Question_Member_V1'Size / 8, Question_Member_V1'Alignment)).all;
   end Element;
   function Element (Items : Images_V1; Index : Interfaces.C.size_t) return System.Address is
      package Pointers is new System.Address_To_Access_Conversions (System.Address);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, System.Address'Size / 8, System.Address'Alignment)).all;
   end Element;
   function Element (Items : Image_Views_V1; Index : Interfaces.C.size_t) return Image_View_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Image_View_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Image_View_V1'Size / 8, Image_View_V1'Alignment)).all;
   end Element;
   function Element (Items : Probabilities_V1; Index : Interfaces.C.size_t) return Probability_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Probability_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Probability_V1'Size / 8, Probability_V1'Alignment)).all;
   end Element;
   function Element (Items : Members_V1; Index : Interfaces.C.size_t) return Member_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Member_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Member_V1'Size / 8, Member_V1'Alignment)).all;
   end Element;
   function Element (Items : Entities_V1; Index : Interfaces.C.size_t) return Entity_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Entity_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Entity_V1'Size / 8, Entity_V1'Alignment)).all;
   end Element;
   function Element (Items : Entity_Edges_V1; Index : Interfaces.C.size_t) return Entity_Edge_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Entity_Edge_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Entity_Edge_V1'Size / 8, Entity_Edge_V1'Alignment)).all;
   end Element;
   function Element (Items : Pieces_V1; Index : Interfaces.C.size_t) return Piece_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Piece_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Piece_V1'Size / 8, Piece_V1'Alignment)).all;
   end Element;
   function Element (Items : Names_V1; Index : Interfaces.C.size_t) return Name_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Name_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Name_V1'Size / 8, Name_V1'Alignment)).all;
   end Element;
   function Element (Items : Pairs_V1; Index : Interfaces.C.size_t) return Pair_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Pair_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Pair_V1'Size / 8, Pair_V1'Alignment)).all;
   end Element;
   function Element (Items : Edges_V1; Index : Interfaces.C.size_t) return Edge_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Edge_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Edge_V1'Size / 8, Edge_V1'Alignment)).all;
   end Element;
   function Element (Items : Relation_Answers_V1; Index : Interfaces.C.size_t) return Relation_Answer_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Relation_Answer_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Relation_Answer_V1'Size / 8, Relation_Answer_V1'Alignment)).all;
   end Element;
   function Element (Items : Question_Sources_V1; Index : Interfaces.C.size_t) return Question_Source_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Question_Source_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Question_Source_V1'Size / 8, Question_Source_V1'Alignment)).all;
   end Element;
   function Element (Items : Observation_Identities_V1; Index : Interfaces.C.size_t) return Observation_Identity_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Observation_Identity_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Observation_Identity_V1'Size / 8, Observation_Identity_V1'Alignment)).all;
   end Element;
   function Element (Items : Attempts_V1; Index : Interfaces.C.size_t) return Attempt_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Attempt_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Attempt_V1'Size / 8, Attempt_V1'Alignment)).all;
   end Element;
   function Element (Items : Input_Properties_V1; Index : Interfaces.C.size_t) return Input_Property_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Input_Property_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Input_Property_V1'Size / 8, Input_Property_V1'Alignment)).all;
   end Element;
   function Element (Items : Source_Details_V1; Index : Interfaces.C.size_t) return Source_Detail_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Source_Detail_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Source_Detail_V1'Size / 8, Source_Detail_V1'Alignment)).all;
   end Element;
   function Element (Items : Input_Views_V1; Index : Interfaces.C.size_t) return Input_View_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Input_View_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Input_View_V1'Size / 8, Input_View_V1'Alignment)).all;
   end Element;
   function Element (Items : Source_Entities_V1; Index : Interfaces.C.size_t) return Source_Entity_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Source_Entity_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Source_Entity_V1'Size / 8, Source_Entity_V1'Alignment)).all;
   end Element;
   function Element (Items : Source_Entity_Edges_V1; Index : Interfaces.C.size_t) return Source_Entity_Edge_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Source_Entity_Edge_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Source_Entity_Edge_V1'Size / 8, Source_Entity_Edge_V1'Alignment)).all;
   end Element;
   function Element (Items : Source_Edges_V1; Index : Interfaces.C.size_t) return Source_Edge_V1 is
      package Pointers is new System.Address_To_Access_Conversions (Source_Edge_V1);
   begin
      return Pointers.To_Pointer
        (Address_At (Items.Data, Items.Len, Index, Source_Edge_V1'Size / 8, Source_Edge_V1'Alignment)).all;
   end Element;
end Thinkthen.Views;
