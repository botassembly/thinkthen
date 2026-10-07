with Thinkthen.Native.Inputs;
with Thinkthen_C; use Thinkthen_C;
package body Thinkthen.Typed is
   use type Interfaces.C.int;
   use type System.Address;
   Surface : aliased constant String := "ada";
   function Native_Question (E : Handle; Spec : access constant Question_Spec_V1;
                             Out_Handle : access Handle) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_question_new";
   function Native_Load (E : Handle; Path : Byte_String_V1;
                         Out_Handle : access Handle) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_question_load";
   function Native_Image (E : Handle; Bytes : System.Address; Count : Interfaces.C.size_t;
                          Media : Interfaces.Unsigned_32; Filename : Optional_String_V1;
                          Out_Handle : access Handle) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_image_clone";
   function Native_Image_View (Item : Handle; View : access Image_View_V1) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_image_view";
   function Native_Records (E : Handle; Data : System.Address; Count : Interfaces.C.size_t;
                            Out_Handle : access Handle) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_source_records";
   function Native_Files (E : Handle; Spec : access constant Source_Spec_V1;
                          Out_Handle : access Handle) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_source_files";
   procedure Free_Question (Item : Handle)
     with Import, Convention => C, External_Name => "thinkthen_question_free";
   overriding procedure Finalize (Item : in out Question) is
   begin
      if Item.Handle /= Null_Handle then Free_Question (Item.Handle); end if;
      Item.Handle := Null_Handle;
   end Finalize;
   function Borrow (Item : Question) return System.Address is (Item.Handle);
   procedure Free_Source (Item : Handle)
     with Import, Convention => C, External_Name => "thinkthen_source_free";
   overriding procedure Finalize (Item : in out Source) is
   begin
      if Item.Handle /= Null_Handle then Free_Source (Item.Handle); end if;
      Item.Handle := Null_Handle;
   end Finalize;
   function Borrow (Item : Source) return System.Address is (Item.Handle);
   procedure Free_Image (Item : Handle)
     with Import, Convention => C, External_Name => "thinkthen_image_free";
   overriding procedure Finalize (Item : in out Image) is
   begin
      if Item.Handle /= Null_Handle then Free_Image (Item.Handle); end if;
      Item.Handle := Null_Handle;
   end Finalize;
   function Borrow (Item : Image) return System.Address is (Item.Handle);
   procedure New_Question (Client : Engine; Specification : Question_Spec_V1;
                           Item : in out Question; Code : out Interfaces.C.int) is
      Copy : aliased constant Question_Spec_V1 := Specification;
      New_Handle : aliased Handle := Null_Handle;
   begin
      Code := Native_Question (Client.Handle, Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end New_Question;
   procedure Load_Question (Client : Engine; Path : Byte_String_V1;
                            Item : in out Question; Code : out Interfaces.C.int) is
      New_Handle : aliased Handle := Null_Handle;
   begin
      Code := Native_Load (Client.Handle, Path, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Load_Question;
   procedure Clone_Image (Client : Engine; Bytes : System.Address;
                          Count : Interfaces.C.size_t; Media : Interfaces.Unsigned_32;
                          Filename : Optional_String_V1; Item : in out Image;
                          Code : out Interfaces.C.int) is
      New_Handle : aliased Handle := Null_Handle;
   begin
      Code := Native_Image (Client.Handle, Bytes, Count, Media, Filename, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Clone_Image;
   procedure Image_View (Item : Image; View : in out Image_View_V1;
                         Code : out Interfaces.C.int) is
      Copy : aliased Image_View_V1 := View;
   begin
      Code := Native_Image_View (Item.Handle, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Image_View;
   procedure Records (Client : Engine; Values : Record_Array;
                      Item : in out Source; Code : out Interfaces.C.int) is
      New_Handle : aliased Handle := Null_Handle;
   begin
      Code := Native_Records (Client.Handle, Values'Address,
                             Interfaces.C.size_t (Values'Length), New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Records;
   procedure Files (Client : Engine; Specification : Source_Spec_V1;
                    Item : in out Source; Code : out Interfaces.C.int) is
      Copy : aliased constant Source_Spec_V1 := Specification;
      New_Handle : aliased Handle := Null_Handle;
   begin
      Code := Native_Files (Client.Handle, Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Files;
   function Controls (Deadline_Ms : Interfaces.Integer_64 := -1;
                      Token : access Cancel_Token := null) return Controls_V1 is
   begin
      return (Deadline_Ms => Deadline_Ms,
              Cancel => (if Token = null then Null_Handle else Token.Owner.Handle),
              Surface => (Surface (Surface'First)'Address, 3), others => <>);
   end Controls;
   procedure New_Question (Client : Engine; Specification : Question_Spec_V1;
                          Author : Question_Author_V1; Item : in out Question;
                          Code : out Interfaces.C.int) is
      Spec : aliased constant Question_Spec_V1 := Specification;
      Metadata : aliased constant Question_Author_V1 := Author;
      New_Handle : aliased Handle := Null_Handle;
   begin
      Code := Native.Inputs.Question_New_Authored
        (Client.Handle, Spec'Access, Metadata'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end New_Question;
   procedure Parse_Question (Client : Engine; Role : Interfaces.Unsigned_32;
                            JSON : Byte_String_V1; Item : in out Question;
                            Code : out Interfaces.C.int) is
      New_Handle : aliased Handle := Null_Handle;
   begin
      Code := Native.Inputs.Question_Parse
        (Client.Handle, Role, JSON, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Parse_Question;
   procedure Load_Named (Client : Engine; Role : Interfaces.Unsigned_32;
                            Name : Byte_String_V1; Item : in out Question;
                            Code : out Interfaces.C.int) is
      New_Handle : aliased Handle := Null_Handle;
   begin
      Code := Native.Inputs.Question_Load_Named
        (Client.Handle, Role, Name, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Load_Named;
   procedure Load_Reference (Client : Engine; Role : Interfaces.Unsigned_32;
                            Reference : Byte_String_V1; Item : in out Question;
                            Code : out Interfaces.C.int) is
      New_Handle : aliased Handle := Null_Handle;
   begin
      Code := Native.Inputs.Question_Load_Reference
        (Client.Handle, Role, Reference, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Load_Reference;
   procedure Author (Item : Question; View : in out Question_Author_V1;
                     Code : out Interfaces.C.int) is
      Copy : aliased Question_Author_V1 := View;
   begin
      Code := Native.Inputs.Question_Author (Item.Handle, Copy'Access);
      if Code = 0 then View := Copy; end if;
   end Author;
   procedure Image_Files (Client : Engine; Specification : Source_Spec_V1;
                          Item : in out Source; Code : out Interfaces.C.int) is
      Copy : aliased constant Source_Spec_V1 := Specification;
      New_Handle : aliased Handle := Null_Handle;
   begin
      Code := Native.Inputs.Source_Image_Files
        (Client.Handle, Copy'Access, New_Handle'Access);
      if Code = 0 then Finalize (Item); Item.Handle := New_Handle; end if;
   end Image_Files;
end Thinkthen.Typed;
