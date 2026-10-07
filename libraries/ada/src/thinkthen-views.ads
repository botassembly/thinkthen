with Interfaces.C;
with System;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
with Thinkthen_C_Answers; use Thinkthen_C_Answers;
with Thinkthen_C_Entities; use Thinkthen_C_Entities;
with Thinkthen_C_Metadata; use Thinkthen_C_Metadata;
with Thinkthen_C_Extensions; use Thinkthen_C_Extensions;
package Thinkthen.Views is
   -- Copies require a live owner; borrowed lists retain their native count.
   -- Index is zero-based. Invalid count/index/address arithmetic refuses.
   function Value (Item : Byte_String_V1) return String;
   function Question_At (Pointer : System.Address) return Question_View_V1;
   function Element (Items : Strings_V1; Index : Interfaces.C.size_t) return Byte_String_V1;
   function Element (Items : Choices_V1; Index : Interfaces.C.size_t) return Choice_V1;
   function Element (Items : Relations_V1; Index : Interfaces.C.size_t) return Relation_V1;
   function Element (Items : Member_Specs_V1; Index : Interfaces.C.size_t) return Member_Spec_V1;
   function Element (Items : Question_Members_V1; Index : Interfaces.C.size_t) return Question_Member_V1;
   function Element (Items : Images_V1; Index : Interfaces.C.size_t) return System.Address;
   function Element (Items : Image_Views_V1; Index : Interfaces.C.size_t) return Image_View_V1;
   function Element (Items : Probabilities_V1; Index : Interfaces.C.size_t) return Probability_V1;
   function Element (Items : Members_V1; Index : Interfaces.C.size_t) return Member_V1;
   function Element (Items : Entities_V1; Index : Interfaces.C.size_t) return Entity_V1;
   function Element (Items : Entity_Edges_V1; Index : Interfaces.C.size_t) return Entity_Edge_V1;
   function Element (Items : Pieces_V1; Index : Interfaces.C.size_t) return Piece_V1;
   function Element (Items : Names_V1; Index : Interfaces.C.size_t) return Name_V1;
   function Element (Items : Pairs_V1; Index : Interfaces.C.size_t) return Pair_V1;
   function Element (Items : Edges_V1; Index : Interfaces.C.size_t) return Edge_V1;
   function Element (Items : Relation_Answers_V1; Index : Interfaces.C.size_t) return Relation_Answer_V1;
   function Element (Items : Question_Sources_V1; Index : Interfaces.C.size_t) return Question_Source_V1;
   function Element (Items : Observation_Identities_V1; Index : Interfaces.C.size_t) return Observation_Identity_V1;
   function Element (Items : Attempts_V1; Index : Interfaces.C.size_t) return Attempt_V1;
   function Element (Items : Input_Properties_V1; Index : Interfaces.C.size_t) return Input_Property_V1;
   function Element (Items : Source_Details_V1; Index : Interfaces.C.size_t) return Source_Detail_V1;
   function Element (Items : Input_Views_V1; Index : Interfaces.C.size_t) return Input_View_V1;
   function Element (Items : Source_Entities_V1; Index : Interfaces.C.size_t) return Source_Entity_V1;
   function Element (Items : Source_Entity_Edges_V1; Index : Interfaces.C.size_t) return Source_Entity_Edge_V1;
   function Element (Items : Source_Edges_V1; Index : Interfaces.C.size_t) return Source_Edge_V1;
end Thinkthen.Views;
