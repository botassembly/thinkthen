with Ada.Finalization;
with Interfaces.C;
with System;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
package Thinkthen.Typed is
   -- Unpublished constructor slice. Native owners clone inputs; no send occurs.
   -- Code uses the existing six C error kinds. Failure snapshots are pending
   -- the complete C API; parent compatibility error accessors remain available.
   type Question is new Ada.Finalization.Limited_Controlled with private;
   type Source is new Ada.Finalization.Limited_Controlled with private;
   type Image is new Ada.Finalization.Limited_Controlled with private;
   type Record_Array is array (Positive range <>) of aliased Record_Data_V1
     with Convention => C;
   procedure New_Question (Client : Engine; Specification : Question_Spec_V1;
                           Item : in out Question; Code : out Interfaces.C.int);
   procedure Load_Question (Client : Engine; Path : Byte_String_V1;
                            Item : in out Question; Code : out Interfaces.C.int);
   procedure Clone_Image (Client : Engine; Bytes : System.Address;
                          Count : Interfaces.C.size_t; Media : Interfaces.Unsigned_32;
                          Filename : Optional_String_V1; Item : in out Image;
                          Code : out Interfaces.C.int);
   procedure Image_View (Item : Image; View : in out Image_View_V1;
                         Code : out Interfaces.C.int);
   procedure Records (Client : Engine; Values : Record_Array;
                      Item : in out Source; Code : out Interfaces.C.int);
   procedure Files (Client : Engine; Specification : Source_Spec_V1;
                    Item : in out Source; Code : out Interfaces.C.int);
   function Borrow (Item : Question) return System.Address;
   function Borrow (Item : Source) return System.Address;
   function Borrow (Item : Image) return System.Address;
   function Controls (Deadline_Ms : Interfaces.Integer_64 := -1;
                      Token : access Cancel_Token := null) return Controls_V1;
   -- Join all callers before owners finalize. Borrowed views are invalid after
   -- their image owner finalizes; source/question constructors deep clone them.
private
   type Question is new Ada.Finalization.Limited_Controlled with record
      Handle : aliased System.Address := System.Null_Address;
   end record;
   overriding procedure Finalize (Item : in out Question);
   type Source is new Ada.Finalization.Limited_Controlled with record
      Handle : aliased System.Address := System.Null_Address;
   end record;
   overriding procedure Finalize (Item : in out Source);
   type Image is new Ada.Finalization.Limited_Controlled with record
      Handle : aliased System.Address := System.Null_Address;
   end record;
   overriding procedure Finalize (Item : in out Image);
end Thinkthen.Typed;
