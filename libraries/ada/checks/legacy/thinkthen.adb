with Ada.Strings.Fixed;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces.C; use Interfaces.C;
with Interfaces.C.Strings; use Interfaces.C.Strings;
with Thinkthen_C; use Thinkthen_C;
package body Thinkthen is
   procedure Capture (Engine : Handle; Code : Interfaces.C.int; Error : out Failure) is
   begin
      -- The calling Ada task must snapshot all three fields immediately after failure.
      Error.Code := Code;
      Error.Retryable := Error_Retryable (Engine) /= 0;
      Error.Message := To_Unbounded_String (Value (Error_Message (Engine)));
      if Error_Code (Engine) /= Code then
         raise Program_Error with "native error trio mismatch";
      end if;
   end Capture;
   procedure Check (Text : String) is
   begin
      for C of Text loop
         if C = Character'Val (0) then
            raise Constraint_Error with "interior NUL in C string";
         end if;
      end loop;
   end Check;
   procedure Decide (Engine, Token : Handle; Question, Evidence : String;
                     Deadline : Interfaces.Integer_64; Result : aliased in out Answer;
                     Error : out Failure) is
      Q, T : chars_ptr;
      Code : Interfaces.C.int;
   begin
      Check (Question);
      -- Evidence is byte-counted, so an embedded zero is valid.
      Q := New_String (Question);
      T := New_String (Evidence);
      Code := Decide_Opts (Engine, Q, T, Interfaces.C.size_t (Evidence'Length), Deadline, Token, Result'Access);
      if Code /= 0 then
         Capture (Engine, Code, Error);
      else
         Error := (Code => 0, Retryable => False, Message => Null_Unbounded_String);
      end if;
      Free (T);
      Free (Q);
   end Decide;
   function Call_Value (Envelope : Unbounded_String) return String is
      Data : constant String := To_String (Envelope);
      Marker : constant String := """value"":";
      Pos : Natural := Ada.Strings.Fixed.Index (Data, Marker);
      Start_At, Depth : Natural := 0;
      In_String, Escaped : Boolean := False;
   begin
      if Pos = 0 or else Ada.Strings.Fixed.Index (Data, """facts"":{") = 0 then
         raise Constraint_Error with "missing C JSON result envelope";
      end if;
      Start_At := Pos + Marker'Length;
      for I in Start_At .. Data'Last loop
         if Escaped then Escaped := False;
         elsif In_String and then Data (I) = '\' then Escaped := True;
         elsif Data (I) = '"' then In_String := not In_String;
         elsif not In_String then
            if Data (I) in '{' | '[' then Depth := Depth + 1;
            elsif Data (I) in '}' | ']' then
               if Depth = 0 then return Data (Start_At .. I - 1); end if;
               Depth := Depth - 1;
            elsif Data (I) = ',' and Depth = 0 then
               return Data (Start_At .. I - 1);
            end if;
         end if;
      end loop;
      raise Constraint_Error with "unterminated C JSON result envelope";
   end Call_Value;
   procedure JSON (Engine, Token : Handle; Request : String;
                   Deadline : Interfaces.Integer_64; Result : out Unbounded_String;
                   Error : out Failure) is
      Input, Output : chars_ptr;
   begin
      Check (Request);
      Input := New_String (Request);
      Output := Call_JSON_Opts (Engine, Input, Deadline, Token);
      if Output = Null_Ptr then
         Capture (Engine, Error_Code (Engine), Error);
         Result := Null_Unbounded_String;
      else
         Result := To_Unbounded_String (Value (Output));
         Free_String (Output); -- native owner, NEVER Interfaces.C.Strings.Free
         Error := (Code => 0, Retryable => False, Message => Null_Unbounded_String);
      end if;
      Free (Input);
   end JSON;
end Thinkthen;
