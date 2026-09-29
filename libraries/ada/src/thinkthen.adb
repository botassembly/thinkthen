with Ada.Finalization;
with Ada.Containers.Indefinite_Vectors;
with Ada.Strings.Fixed;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces.C; use Interfaces.C;
with Interfaces.C.Strings; use Interfaces.C.Strings;
with Thinkthen_C; use Thinkthen_C;
with System; use System;
with Interfaces; use Interfaces;
package body Thinkthen is
   type Owned_String is new Ada.Finalization.Limited_Controlled with record
      Pointer : aliased chars_ptr := Null_Ptr;
      Native : Boolean := False;
   end record;
   overriding procedure Finalize (Item : in out Owned_String) is
   begin
      if Item.Pointer /= Null_Ptr then
         if Item.Native then Free_String (Item.Pointer); else Free (Item.Pointer); end if;
      end if;
   end Finalize;
   procedure Set (Item : in out Owned_String; Text : String) is
   begin
      Item.Pointer := New_String (Text);
   end Set;
   procedure Require_C_String (Text : String) is
   begin
      for C of Text loop
         if C = Character'Val (0) then raise Constraint_Error with "NUL in C string"; end if;
      end loop;
   end Require_C_String;
   function Raw_Token (Token : access Cancel_Token) return Handle is
   begin
      return (if Token = null then Null_Handle else Token.Owner.Handle);
   end Raw_Token;
   function Message (Error : Failure) return String is (To_String (Error.Text));
   function Failure_Facts (Error : Failure) return String is (To_String (Error.Facts_JSON));
   function Named (Code : int) return Error_Kind is
   begin
      case Code is
         when 0 => return None;
         when 1 => return Usage;
         when 2 => return Backend;
         when 3 => return Deadline;
         when 4 => return Local;
         when 5 => return Cancelled;
         when 6 => return Defect;
         when others => raise Program_Error with "unknown native error kind";
      end case;
   end Named;
   procedure Capture (Client : Handle; Code : int; Error : out Failure) is
   begin
      if Code = 0 then
         Error := (Kind => None, Retryable => False, Text => Null_Unbounded_String,
                   Facts_JSON => Null_Unbounded_String);
      else
         declare
            Message_Ptr : constant chars_ptr := Error_Message (Client);
            Facts_Ptr : constant chars_ptr := Error_Facts_JSON (Client);
            Message_Copy : constant String := (if Message_Ptr = Null_Ptr then "" else Value (Message_Ptr));
            Facts_Copy : constant String := (if Facts_Ptr = Null_Ptr then "" else Value (Facts_Ptr));
         begin
            Error := (Kind => Named (Code), Retryable => Error_Retryable (Client) /= 0,
                      Text => To_Unbounded_String (Message_Copy),
                      Facts_JSON => To_Unbounded_String (Facts_Copy));
         end;
         if Error_Code (Client) /= Code then
            raise Program_Error with "native error trio mismatch";
         end if;
      end if;
   end Capture;
   overriding procedure Initialize (Client : in out Engine) is
   begin
      Client.Handle := Engine_New;
      if Client.Handle = Null_Handle then
         raise Program_Error with "engine construction failed: " & Value (Error_Message (Null_Handle));
      end if;
   end Initialize;
   procedure Configure (Client : in out Engine; Settings_JSON : String; Error : out Failure) is
      Settings : Owned_String;
      New_Handle : Handle;
   begin
      Require_C_String (Settings_JSON);
      Set (Settings, Settings_JSON);
      New_Handle := Engine_New_With (Settings.Pointer);
      if New_Handle = Null_Handle then
         Capture (Null_Handle, Error_Code (Null_Handle), Error);
      else
         Engine_Free (Client.Handle);
         Client.Handle := New_Handle;
         Capture (Client.Handle, 0, Error);
      end if;
   end Configure;
   overriding procedure Finalize (Client : in out Engine) is
   begin
      if Client.Handle /= Null_Handle then Engine_Free (Client.Handle); Client.Handle := Null_Handle; end if;
   end Finalize;
   overriding procedure Initialize (Token : in out Token_Owner) is
   begin
      Token.Handle := Token_New;
      if Token.Handle = Null_Handle then raise Program_Error with "token construction failed"; end if;
   end Initialize;
   overriding procedure Finalize (Token : in out Token_Owner) is
   begin
      if Token.Handle /= Null_Handle then Token_Free (Token.Handle); Token.Handle := Null_Handle; end if;
   end Finalize;
   procedure Cancel (Token : in out Cancel_Token) is
   begin
      Thinkthen_C.Cancel (Token.Owner.Handle);
   end Cancel;
   function Converted (Raw : Answer) return Decision is
   begin
      if Raw.Outcome < 0 or Raw.Outcome > 2 then raise Program_Error with "invalid outcome"; end if;
      return (Value => Outcome'Val (Integer (Raw.Outcome)), Probability => Long_Float (Raw.Probability));
   end Converted;
   procedure Decide (Client : in out Engine; Question, Evidence : String;
                     Result : out Decision; Error : out Failure;
                     Deadline_Ms : Interfaces.Integer_64 := -1;
                     Token : access Cancel_Token := null) is
      Q, T : Owned_String;
      Raw : aliased Answer := (Outcome => 123, Probability => -1.0);
      Code : int;
   begin
      Require_C_String (Question);
      Set (Q, Question); Set (T, Evidence);
      Code := Decide_Opts (Client.Handle, Q.Pointer, T.Pointer, size_t (Evidence'Length),
                           Deadline_Ms, Raw_Token (Token), Raw'Access);
      Capture (Client.Handle, Code, Error);
      Result := (if Code = 0 then Converted (Raw) else (Value => Not_Sure, Probability => 0.0));
   end Decide;
   procedure Decide_Many (Client : in out Engine; Question : String;
                          Evidence : Evidence_Array; Result : out Decision_Array;
                          Error : out Failure; Deadline_Ms : Interfaces.Integer_64 := -1;
                          Token : access Cancel_Token := null) is
      Q : Owned_String;
      type Owned_Array is array (Positive range <>) of Owned_String;
      Owned : Owned_Array (Evidence'Range);
      Texts : aliased Text_Array (0 .. size_t (Evidence'Length) - 1);
      Lengths : aliased Length_Array (Texts'Range);
      Answers : aliased Answer_Array (Texts'Range) := (others => (Outcome => 123, Probability => -1.0));
      Code : int;
   begin
      if Evidence'Length = 0 or Result'Length /= Evidence'Length then
         raise Constraint_Error with "bulk arrays must have equal nonzero length";
      end if;
      Require_C_String (Question); Set (Q, Question);
      for I in Evidence'Range loop
         Set (Owned (I), To_String (Evidence (I)));
         Texts (size_t (I - Evidence'First)) := Owned (I).Pointer;
         Lengths (size_t (I - Evidence'First)) := size_t (Length (Evidence (I)));
      end loop;
      Code := Decide_Many_Opts (Client.Handle, Q.Pointer, Texts'Address, Lengths'Address,
                                size_t (Evidence'Length), Deadline_Ms, Raw_Token (Token), Answers'Address);
      Capture (Client.Handle, Code, Error);
      if Code = 0 then
         for I in Result'Range loop Result (I) := Converted (Answers (size_t (I - Result'First))); end loop;
      else
         for I in Result'Range loop Result (I) := (Value => Not_Sure, Probability => 0.0); end loop;
      end if;
   end Decide_Many;
   function Bare_Labels (Names : String_List) return Label_Set is
      Result : Label_Set (Names'Range);
   begin
      for I in Names'Range loop Result (I) := (Names (I), Null_Unbounded_String); end loop;
      return Result;
   end Bare_Labels;
   function Quoted (Text : String) return String is
      Result : Unbounded_String := To_Unbounded_String ("""");
   begin
      for C of Text loop
         case C is
            when '"' | '\' => Append (Result, '\' & C);
            when Character'Val (10) => Append (Result, "\n");
            when Character'Val (13) => Append (Result, "\r");
            when Character'Val (9) => Append (Result, "\t");
            when others =>
               if Character'Pos (C) < 32 then raise Constraint_Error with "control in label"; end if;
               Append (Result, C);
         end case;
      end loop;
      return To_String (Result) & '"';
   end Quoted;
   -- A small strict JSON validator. It checks syntax, escapes, nesting and the
   -- full input, rather than mistaking a substring in an escaped string for a key.
   procedure Validate (Text : String) is
      Pos : Natural := Text'First;
      procedure Space is
      begin
         while Pos <= Text'Last and then Text (Pos) in ' ' | Character'Val (9) | Character'Val (10) | Character'Val (13) loop Pos := Pos + 1; end loop;
      end Space;
      procedure String_Value is
         Closed : Boolean := False;
      begin
         if Pos > Text'Last or else Text (Pos) /= '"' then raise Constraint_Error with "expected JSON string"; end if;
         Pos := Pos + 1;
         while Pos <= Text'Last loop
            if Text (Pos) = '"' then Closed := True; Pos := Pos + 1; exit;
            elsif Text (Pos) = '\' then
               Pos := Pos + 1;
               if Pos > Text'Last or else Text (Pos) not in '"' | '\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' | 'u' then
                  raise Constraint_Error with "invalid JSON escape";
               end if;
               if Text (Pos) = 'u' then
                  for N in 1 .. 4 loop
                     Pos := Pos + 1;
                     if Pos > Text'Last or else Text (Pos) not in '0' .. '9' | 'a' .. 'f' | 'A' .. 'F' then
                        raise Constraint_Error with "invalid Unicode escape";
                     end if;
                  end loop;
               end if;
            elsif Character'Pos (Text (Pos)) < 32 then raise Constraint_Error with "unescaped control";
            end if;
            Pos := Pos + 1;
         end loop;
         if not Closed then raise Constraint_Error with "unterminated string"; end if;
      end String_Value;
      procedure Value (Depth : Natural);
      procedure Value (Depth : Natural) is
         Closing : Character;
      begin
         if Depth > 64 then raise Constraint_Error with "JSON nesting too deep"; end if;
         Space;
         if Pos > Text'Last then raise Constraint_Error with "missing JSON value"; end if;
         if Text (Pos) = '"' then String_Value;
         elsif Text (Pos) in '{' | '[' then
            Closing := (if Text (Pos) = '{' then '}' else ']'); Pos := Pos + 1; Space;
            if Pos <= Text'Last and then Text (Pos) = Closing then Pos := Pos + 1; return; end if;
            loop
               if Closing = '}' then
                  String_Value; Space;
                  if Pos > Text'Last or else Text (Pos) /= ':' then raise Constraint_Error with "missing colon"; end if;
                  Pos := Pos + 1;
               end if;
               Value (Depth + 1); Space;
               if Pos > Text'Last then raise Constraint_Error with "unfinished JSON collection"; end if;
               exit when Text (Pos) = Closing;
               if Text (Pos) /= ',' then raise Constraint_Error with "missing comma"; end if;
               Pos := Pos + 1; Space;
            end loop;
            Pos := Pos + 1;
         elsif Text (Pos) in 't' | 'f' | 'n' then
            declare
               Literal : constant String := (if Text (Pos) = 't' then "true" elsif Text (Pos) = 'f' then "false" else "null");
            begin
               if Pos + Literal'Length - 1 > Text'Last or else Text (Pos .. Pos + Literal'Length - 1) /= Literal then
                  raise Constraint_Error with "invalid JSON literal";
               end if;
               Pos := Pos + Literal'Length;
            end;
         else
            declare
               Start : constant Natural := Pos;
            begin
               if Text (Pos) = '-' then Pos := Pos + 1; end if;
               if Pos > Text'Last or else Text (Pos) not in '0' .. '9' then raise Constraint_Error with "invalid JSON number"; end if;
               if Text (Pos) = '0' then Pos := Pos + 1;
               else while Pos <= Text'Last and then Text (Pos) in '0' .. '9' loop Pos := Pos + 1; end loop; end if;
               if Pos <= Text'Last and then Text (Pos) = '.' then
                  Pos := Pos + 1;
                  if Pos > Text'Last or else Text (Pos) not in '0' .. '9' then raise Constraint_Error with "invalid fraction"; end if;
                  while Pos <= Text'Last and then Text (Pos) in '0' .. '9' loop Pos := Pos + 1; end loop;
               end if;
               if Pos <= Text'Last and then Text (Pos) in 'e' | 'E' then
                  Pos := Pos + 1;
                  if Pos <= Text'Last and then Text (Pos) in '+' | '-' then Pos := Pos + 1; end if;
                  if Pos > Text'Last or else Text (Pos) not in '0' .. '9' then raise Constraint_Error with "invalid exponent"; end if;
                  while Pos <= Text'Last and then Text (Pos) in '0' .. '9' loop Pos := Pos + 1; end loop;
               end if;
               if Pos = Start then raise Constraint_Error with "invalid JSON value"; end if;
            end;
         end if;
      end Value;
   begin
      Value (0); Space;
      if Pos <= Text'Last then raise Constraint_Error with "trailing JSON bytes"; end if;
   end Validate;
   function Label_Descriptions (Labels : Label_Set) return String is
      Result : Unbounded_String;
      Described : Boolean;
   begin
      if Labels'Length = 0 then raise Constraint_Error with "empty label set"; end if;
      Described := Length (Labels (Labels'First).Description_JSON) /= 0;
      Result := To_Unbounded_String ((if Described then "{" else "["));
      for I in Labels'Range loop
         if (Length (Labels (I).Description_JSON) /= 0) /= Described then
            raise Constraint_Error with "mixed bare/described label: " & To_String (Labels (I).Name);
         end if;
         if I /= Labels'First then Append (Result, ','); end if;
         Append (Result, Quoted (To_String (Labels (I).Name)));
         if Described then
            Validate (To_String (Labels (I).Description_JSON));
            Append (Result, ':');
            Append (Result, Labels (I).Description_JSON);
         end if;
      end loop;
      return To_String (Result) & (if Described then "}" else "]");
   end Label_Descriptions;
   function Extract (Object_Text, Name : String) return String;
   function Unquote (Text : String) return Unbounded_String;
   function Decode_Field (Text : String) return Annotated_Field is
      Kind : Answer_Kind;
      Marker : Failure_Marker := (Kind => None, Cause => Null_Unbounded_String);
   begin
      Validate (Text);
      if Text = "null" then Kind := Null_Answer;
      elsif Text = "true" or Text = "false" then Kind := Boolean_Answer;
      elsif Text'Length > 0 and then Text (Text'First) = '"' then Kind := Label_Answer;
      elsif Text'Length > 0 and then Text (Text'First) = '[' then Kind := Label_List_Answer;
      elsif Text'Length > 0 and then Text (Text'First) = '{' then
         declare
            Failure_Text : constant String := Extract (Text, "failed");
            Name : constant String := To_String (Unquote (Extract (Failure_Text, "kind")));
         begin
            if Name = "usage" then Marker.Kind := Usage;
            elsif Name = "backend" then Marker.Kind := Backend;
            elsif Name = "deadline" then Marker.Kind := Deadline;
            elsif Name = "local" then Marker.Kind := Local;
            elsif Name = "cancelled" then Marker.Kind := Cancelled;
            elsif Name = "defect" then Marker.Kind := Defect;
            else raise Constraint_Error with "unknown failed field kind";
            end if;
            Marker.Cause := Unquote (Extract (Failure_Text, "cause"));
         end;
         Kind := Failed_Answer;
      elsif Text'Length > 0 and then Text (Text'First) in '-' | '0' .. '9' then Kind := Number_Answer;
      else raise Constraint_Error with "unexpected annotate answer";
      end if;
      return (Kind => Kind, JSON => To_Unbounded_String (Text), Marker => Marker);
   end Decode_Field;
   procedure Call (Client : in out Engine; Request : String; Result : out JSON_Result;
                   Error : out Failure; Deadline_Ms : Interfaces.Integer_64 := -1;
                   Token : access Cancel_Token := null) is
      Input, Output : Owned_String;
   begin
      Require_C_String (Request); Validate (Request); Set (Input, Request);
      Output.Pointer := Call_JSON_Opts (Client.Handle, Input.Pointer, Deadline_Ms, Raw_Token (Token));
      if Output.Pointer = Null_Ptr then
         Capture (Client.Handle, Error_Code (Client.Handle), Error);
         Result.JSON := Null_Unbounded_String;
      else
         Output.Native := True;
         Validate (Value (Output.Pointer));
         Result.JSON := To_Unbounded_String (Value (Output.Pointer));
         Capture (Client.Handle, 0, Error);
      end if;
   end Call;
   procedure Recognize (Client : in out Engine; Specification, Evidence : String;
                        Result : out JSON_Result; Error : out Failure;
                        Deadline_Ms : Interfaces.Integer_64 := -1;
                        Token : access Cancel_Token := null) is
      Spec, Text : Owned_String;
      Output : aliased Owned_String;
      Out_Len : aliased size_t := 0;
      Code : int;
   begin
      Require_C_String (Specification); Validate (Specification);
      Set (Spec, Specification); Set (Text, Evidence);
      Code := Recognize_Opts (Client.Handle, Spec.Pointer, Text.Pointer, size_t (Evidence'Length),
                              Deadline_Ms, Raw_Token (Token), Output.Pointer'Access, Out_Len'Access);
      Capture (Client.Handle, Code, Error);
      if Code = 0 then
         Output.Native := True;
         declare
            Data : constant String := Value (Output.Pointer, Out_Len);
         begin
            Validate (Data);
            declare
               Shape : constant String := Extract (Data, "entities");
            begin
               if Shape (Shape'First) /= '[' then raise Constraint_Error with "recognize entities must be an array"; end if;
            end;
            Result.JSON := To_Unbounded_String (Data);
         end;
      else Result.JSON := Null_Unbounded_String;
      end if;
   end Recognize;
   procedure Relate (Client : in out Engine; Specification : String;
                     Records : Evidence_Array; Result : out JSON_Result; Error : out Failure;
                     Deadline_Ms : Interfaces.Integer_64 := -1;
                     Token : access Cancel_Token := null) is
      Spec : Owned_String;
      Output : aliased Owned_String;
      type Owned_Array is array (Positive range <>) of Owned_String;
      Owned : Owned_Array (Records'Range);
      Texts : aliased Text_Array (0 .. size_t (Records'Length) - 1);
      Lengths : aliased Length_Array (Texts'Range);
      Out_Len : aliased size_t := 0;
      Code : int;
   begin
      if Records'Length = 0 then raise Constraint_Error with "no records"; end if;
      Require_C_String (Specification); Validate (Specification); Set (Spec, Specification);
      for I in Records'Range loop
         Set (Owned (I), To_String (Records (I)));
         Texts (size_t (I - Records'First)) := Owned (I).Pointer;
         Lengths (size_t (I - Records'First)) := size_t (Length (Records (I)));
      end loop;
      Code := Relate_Opts (Client.Handle, Spec.Pointer, Texts'Address, Lengths'Address,
                           size_t (Records'Length), Deadline_Ms, Raw_Token (Token),
                           Output.Pointer'Access, Out_Len'Access);
      Capture (Client.Handle, Code, Error);
      if Code = 0 then
         Output.Native := True;
         declare
            Data : constant String := Value (Output.Pointer, Out_Len);
         begin
            Validate (Data);
            declare
               Shape : constant String := Extract (Data, "edges");
            begin
               if Shape (Shape'First) /= '[' then raise Constraint_Error with "relate edges must be an array"; end if;
            end;
            Result.JSON := To_Unbounded_String (Data);
         end;
      else Result.JSON := Null_Unbounded_String;
      end if;
   end Relate;
   function Extract (Object_Text, Name : String) return String is
      P : Natural := Object_Text'First;
      Found : Unbounded_String;
      Present : Boolean := False;
      procedure Space is
      begin
         while P <= Object_Text'Last and then Object_Text (P) in ' ' | Character'Val (9) | Character'Val (10) | Character'Val (13) loop
            P := P + 1;
         end loop;
      end Space;
      function String_End (First : Natural) return Natural is
         At_Byte : Natural := First + 1;
         Escaped : Boolean := False;
      begin
         if Object_Text (First) /= '"' then raise Constraint_Error with "expected member name"; end if;
         while At_Byte <= Object_Text'Last loop
            if Escaped then Escaped := False;
            elsif Object_Text (At_Byte) = '\' then Escaped := True;
            elsif Object_Text (At_Byte) = '"' then return At_Byte;
            end if;
            At_Byte := At_Byte + 1;
         end loop;
         raise Constraint_Error with "unterminated member name";
      end String_End;
      function Value_End (First : Natural) return Natural is
         At_Byte : Natural := First;
         Depth : Natural := 0;
         In_String, Escaped : Boolean := False;
      begin
         while At_Byte <= Object_Text'Last loop
            declare
               C : constant Character := Object_Text (At_Byte);
            begin
               if Escaped then Escaped := False;
               elsif In_String and C = '\' then Escaped := True;
               elsif C = '"' then In_String := not In_String;
               elsif not In_String then
                  if C in '{' | '[' then Depth := Depth + 1;
                  elsif C in '}' | ']' then
                     if Depth = 0 then return At_Byte; end if;
                     Depth := Depth - 1;
                  elsif C = ',' and Depth = 0 then return At_Byte;
                  end if;
               end if;
            end;
            At_Byte := At_Byte + 1;
         end loop;
         raise Constraint_Error with "unterminated JSON object member";
      end Value_End;
   begin
      Validate (Object_Text);
      Space;
      if P > Object_Text'Last or else Object_Text (P) /= '{' then raise Constraint_Error with "expected JSON object"; end if;
      P := P + 1;
      loop
         Space;
         exit when P <= Object_Text'Last and then Object_Text (P) = '}';
         if P > Object_Text'Last or else Object_Text (P) /= '"' then raise Constraint_Error with "expected member name"; end if;
         declare
            End_Name : constant Natural := String_End (P);
            Key : constant String := To_String (Unquote (Object_Text (P .. End_Name)));
            Start_Value, End_Value : Natural;
         begin
            P := End_Name + 1; Space;
            if P > Object_Text'Last or else Object_Text (P) /= ':' then raise Constraint_Error with "expected member colon"; end if;
            P := P + 1; Space; Start_Value := P;
            End_Value := Value_End (P);
            if Key = Name then
               if Present then raise Constraint_Error with "duplicate result member: " & Name; end if;
               Found := To_Unbounded_String (Ada.Strings.Fixed.Trim (Object_Text (Start_Value .. End_Value - 1), Ada.Strings.Both));
               Present := True;
            end if;
            P := End_Value;
            if Object_Text (P) = ',' then P := P + 1;
            else exit when Object_Text (P) = '}'; end if;
         end;
      end loop;
      if not Present then raise Constraint_Error with "missing result field: " & Name; end if;
      return To_String (Found);
   end Extract;
   function Call_Facts (Result : JSON_Result) return String is
      Data : constant String := To_String (Result.JSON);
      Facts : constant String := Extract (Data, "facts");
      -- The schema requires these four; tokens/model are optional, but
      -- the installed fixture checks all seven when the backend sends them.
      Names : constant array (Positive range 1 .. 4) of String (1 .. 13) :=
        ("records      ", "requests_sent", "cache_answers", "seconds      ");
   begin
      for Name of Names loop
         declare
            Field : constant String := Ada.Strings.Fixed.Trim (Name, Ada.Strings.Both);
            Discard : constant String := Extract (Facts, Field);
         begin
            if Discard'Length = 0 then raise Constraint_Error with "empty facts member"; end if;
         end;
      end loop;
      return Facts;
   end Call_Facts;
   function Call_Value (Result : JSON_Result) return String is
      Data : constant String := To_String (Result.JSON);
      Facts : constant String := Call_Facts (Result);
   begin
      if Facts'Length < 2 then raise Constraint_Error with "invalid call facts"; end if;
      return Extract (Data, "value");
   end Call_Value;
   package Text_Vectors is new Ada.Containers.Indefinite_Vectors (Positive, String);
   function Items (Array_JSON : String) return Text_Vectors.Vector is
      Items_Out : Text_Vectors.Vector;
      P, Start, Depth : Natural;
      In_String, Escape : Boolean := False;
   begin
      Validate (Array_JSON);
      if Array_JSON'Length < 2 or else Array_JSON (Array_JSON'First) /= '[' then
         raise Constraint_Error with "result is not an array";
      end if;
      P := Array_JSON'First + 1; Start := P; Depth := 0;
      while P < Array_JSON'Last loop
         declare
            C : constant Character := Array_JSON (P);
         begin
            if Escape then Escape := False;
            elsif In_String and C = '\' then Escape := True;
            elsif C = '"' then In_String := not In_String;
            elsif not In_String then
               if C in '{' | '[' then Depth := Depth + 1;
               elsif C in '}' | ']' then Depth := Depth - 1;
               elsif C = ',' and Depth = 0 then
                  Items_Out.Append (Ada.Strings.Fixed.Trim (Array_JSON (Start .. P - 1), Ada.Strings.Both));
                  Start := P + 1;
               end if;
            end if;
         end;
         P := P + 1;
      end loop;
      if Ada.Strings.Fixed.Trim (Array_JSON (Start .. P - 1), Ada.Strings.Both)'Length /= 0 then
         Items_Out.Append (Ada.Strings.Fixed.Trim (Array_JSON (Start .. P - 1), Ada.Strings.Both));
      end if;
      return Items_Out;
   end Items;
   function Annotation (Result_JSON : String; Question : String; Row : Positive := 1) return Annotated_Field is
      Data : constant String := Ada.Strings.Fixed.Trim (Result_JSON, Ada.Strings.Both);
   begin
      Validate (Data);
      if Data'Length > 0 and then Data (Data'First) = '[' then
         declare
            Rows : constant Text_Vectors.Vector := Items (Data);
         begin
            if Row > Natural (Rows.Length) then raise Constraint_Error with "annotate row out of range"; end if;
            return Decode_Field (Extract (Rows.Element (Row), Question));
         end;
      elsif Row = 1 then
         return Decode_Field (Extract (Data, Question));
      else raise Constraint_Error with "annotate row out of range";
      end if;
   end Annotation;
   function Unquote (Text : String) return Unbounded_String is
      Result : Unbounded_String;
      P : Natural := Text'First + 1;
      function Hex (C : Character) return Natural is
      begin
         case C is
            when '0' .. '9' => return Character'Pos (C) - Character'Pos ('0');
            when 'a' .. 'f' => return Character'Pos (C) - Character'Pos ('a') + 10;
            when 'A' .. 'F' => return Character'Pos (C) - Character'Pos ('A') + 10;
            when others => raise Constraint_Error with "invalid Unicode escape digit";
         end case;
      end Hex;
      function Four_Digits return Natural is
         Code : Natural := 0;
      begin
         if P + 3 >= Text'Last then raise Constraint_Error with "short Unicode escape"; end if;
         for I in 1 .. 4 loop
            Code := Code * 16 + Hex (Text (P)); P := P + 1;
         end loop;
         return Code;
      end Four_Digits;
      procedure Append_UTF8 (Code : Natural) is
         function Byte (Value : Natural) return Character is (Character'Val (Value));
      begin
         if Code <= 16#7F# then Append (Result, Byte (Code));
         elsif Code <= 16#7FF# then
            Append (Result, Byte (16#C0# + Code / 64));
            Append (Result, Byte (16#80# + Code mod 64));
         elsif Code <= 16#FFFF# then
            Append (Result, Byte (16#E0# + Code / 4_096));
            Append (Result, Byte (16#80# + (Code / 64) mod 64));
            Append (Result, Byte (16#80# + Code mod 64));
         elsif Code <= 16#10FFFF# then
            Append (Result, Byte (16#F0# + Code / 262_144));
            Append (Result, Byte (16#80# + (Code / 4_096) mod 64));
            Append (Result, Byte (16#80# + (Code / 64) mod 64));
            Append (Result, Byte (16#80# + Code mod 64));
         else raise Constraint_Error with "Unicode scalar out of range";
         end if;
      end Append_UTF8;
      Code : Natural;
   begin
      if Text'Length < 2 or else Text (Text'First) /= '"' or else Text (Text'Last) /= '"' then
         raise Constraint_Error with "expected JSON string result";
      end if;
      while P < Text'Last loop
         if Text (P) /= '\' then
            if Character'Pos (Text (P)) < 32 then raise Constraint_Error with "unescaped control"; end if;
            Append (Result, Text (P)); P := P + 1;
         else
            P := P + 1;
            if P >= Text'Last then raise Constraint_Error with "unfinished JSON escape"; end if;
            case Text (P) is
               when '"' | '\' | '/' => Append (Result, Text (P)); P := P + 1;
               when 'b' => Append (Result, Character'Val (8)); P := P + 1;
               when 'f' => Append (Result, Character'Val (12)); P := P + 1;
               when 'n' => Append (Result, Character'Val (10)); P := P + 1;
               when 'r' => Append (Result, Character'Val (13)); P := P + 1;
               when 't' => Append (Result, Character'Val (9)); P := P + 1;
               when 'u' =>
                  P := P + 1; Code := Four_Digits;
                  if Code in 16#D800# .. 16#DBFF# then
                     if P + 5 >= Text'Last or else Text (P .. P + 1) /= "\u" then
                        raise Constraint_Error with "missing low surrogate";
                     end if;
                     P := P + 2;
                     declare
                        Low : constant Natural := Four_Digits;
                     begin
                        if Low not in 16#DC00# .. 16#DFFF# then raise Constraint_Error with "invalid low surrogate"; end if;
                        Code := 16#10000# + (Code - 16#D800#) * 1_024 + Low - 16#DC00#;
                     end;
                  elsif Code in 16#DC00# .. 16#DFFF# then
                     raise Constraint_Error with "lone low surrogate";
                  end if;
                  Append_UTF8 (Code);
               when others => raise Constraint_Error with "unknown JSON escape";
            end case;
         end if;
      end loop;
      return Result;
   end Unquote;
   function Entities (Result : JSON_Result) return Entity_Vectors.Vector is
      Answer : Entity_Vectors.Vector;
      Data : constant String := To_String (Result.JSON);
   begin
      Validate (Data);
      for Item of Items (Extract (Data, "entities")) loop
         declare
            Start_At : constant Natural := Natural'Value (Extract (Item, "start"));
            End_At : constant Natural := Natural'Value (Extract (Item, "end"));
            Span : constant Natural := Natural'Value (Extract (Item, "length"));
         begin
            if End_At < Start_At or else Span /= End_At - Start_At then
               raise Constraint_Error with "recognize span mismatch";
            end if;
            Answer.Append (Entity'(Text => Unquote (Extract (Item, "text")),
                            Kind => Unquote (Extract (Item, "kind")),
                            Start_Offset => Start_At, End_Offset => End_At, Length => Span,
                            Strength => Long_Float'Value (Extract (Item, "strength"))));
         end;
      end loop;
      return Answer;
   end Entities;
   function Edges (Result : JSON_Result) return Edge_Vectors.Vector is
      Answer : Edge_Vectors.Vector;
      Data : constant String := To_String (Result.JSON);
   begin
      Validate (Data);
      for Item of Items (Extract (Data, "edges")) loop
         declare
            Source : constant String := Extract (Item, "source");
            Target : constant String := Extract (Item, "target");
         begin
            Answer.Append (Relation_Edge'(
               Relation => Unquote (Extract (Item, "relation")),
               Source => (Name => Unquote (Extract (Source, "name")),
                          Kind => Unquote (Extract (Source, "kind"))),
               Target => (Name => Unquote (Extract (Target, "name")),
                          Kind => Unquote (Extract (Target, "kind"))),
               Probability => Long_Float'Value (Extract (Item, "probability"))));
         end;
      end loop;
      return Answer;
   end Edges;
end Thinkthen;
