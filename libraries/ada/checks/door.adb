with Ada.Characters.Handling; use Ada.Characters.Handling;
with Ada.Command_Line; use Ada.Command_Line;
with Ada.Environment_Variables;
with Ada.Strings.Fixed;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Thinkthen; use Thinkthen;
-- "door REQUEST" prints one JSON-door reply. "door fields REQUEST NAME..." reads
-- each annotate row's named members through Annotation. "door plan VERB
-- QUESTION SETTINGS TEXT..." prints the plan object. "door limits" checks
-- ticket 0291's zero budgets and zero cap; "door helper" checks Decode_Field's
-- edge table.
procedure Door is
   Client : Engine;
   Result : Unbounded_String;
   Error : Thinkthen.Failure;
   procedure Require (Good : Boolean; Note : String) is
   begin
      if not Good then raise Program_Error with Note; end if;
   end Require;
   function Image (Kind : Error_Kind) return String is (To_Lower (Error_Kind'Image (Kind)));
   procedure Print_Failure is
   begin
      Put_Line ("{""failed"":{""kind"":""" & Image (Error.Kind) & """,""code"":" &
                Ada.Strings.Fixed.Trim (Error_Kind'Pos (Error.Kind)'Image, Ada.Strings.Both) & "}}");
   end Print_Failure;
   function State (Field : Annotated_Field) return String is
     (case Field.Kind is
         when Null_Answer => "unresolved",
         when Failed_Answer => "failed " & Image (Field.Marker.Kind) & " " & To_String (Field.Marker.Cause),
         when others => "answered");
   -- ADR 0112 section 4: null is unresolved, {"failed": ...} is a failure whose
   -- unknown extra member reads without error, and any other value is answered.
   procedure Helper is
      type Row is record
         Text, Expected : Unbounded_String;
      end record;
      function "+" (Text : String) return Unbounded_String renames To_Unbounded_String;
      Cases : constant array (1 .. 6) of Row :=
        [(+"null", +"unresolved"), (+"true", +"answered"), (+"""billing""", +"answered"),
         (+"[""billing"",""urgent""]", +"answered"), (+"1.2", +"answered"),
         (+"{""failed"":{""kind"":""backend"",""cause"":""missing_probability"",""later"":1}}",
          +"failed backend missing_probability")];
      Refused : constant array (1 .. 3) of Unbounded_String :=
        [+"{""failed"":null}", +"{""team"":""billing""}", +"{""failed"":{""kind"":""later"",""cause"":""x""}}"];
   begin
      for C of Cases loop
         Require (State (Decode_Field (To_String (C.Text))) = To_String (C.Expected), To_String (C.Text));
      end loop;
      for Text of Refused loop
         declare
            Caught : Boolean := False;
         begin
            begin
               Require (Decode_Field (To_String (Text)).Kind = Null_Answer, "unreachable");
            exception
               when Constraint_Error => Caught := True;
            end;
            Require (Caught, To_String (Text));
         end;
      end loop;
      Put_Line ("{""helper"":""pass""}");
   end Helper;
   -- Ticket 0291: a zero cap and a zero budget each refuse before sending. Relate
   -- gets two entities, since one entity has no pair to ask.
   procedure Limits is
      Capped : Engine;
      Answer : Decision;
      Facts : Unbounded_String;
      function "+" (Text : String) return Unbounded_String renames To_Unbounded_String;
      Pair : constant Evidence_Array :=
        [+"{""name"":""A"",""kind"":""alert""}", +"{""name"":""B"",""kind"":""alert""}"];
   begin
      Configure (Capped, "{""max_requests_total"":0,""cache"":false}", Error);
      Require (Error.Kind = None, "capped engine");
      Decide (Capped, "Is it?", "capped", Answer, Facts, Error);
      Require (Error.Kind = Usage and Error_Kind'Pos (Error.Kind) = 1 and
               Ada.Strings.Fixed.Index (Message (Error), "process send budget") > 0 and Facts = "",
               "zero cap refuses as usage");
      Call (Client, "{""decide"":""Is it?"",""evidence"":""zero-call""}", Result, Error, Deadline_Ms => 0);
      Require (Error.Kind = Deadline and Result = "", "zero budget call");
      Recognize (Client, "{""version"":1,""recognize"":{""kinds"":{""person"":""A person's name.""}}}",
                 "zero-recognize", Result, Facts, Error, Deadline_Ms => 0);
      Require (Error.Kind = Deadline and Error_Kind'Pos (Error.Kind) = 3 and Result = "", "zero budget recognize");
      Relate (Client, "{""version"":1,""relate"":{""relations"":[{""name"":""caused_by"",""source"":""alert"",""target"":""alert""}]}}",
              Pair, Result, Facts, Error, Deadline_Ms => 0);
      Require (Error.Kind = Deadline and Result = "", "zero budget relate");
      Put_Line ("{""limits"":""pass""}");
   end Limits;
begin
   if Argument_Count < 1 then raise Program_Error with "one request required"; end if;
   if Ada.Environment_Variables.Exists ("TT_SETTINGS_JSON") then
      Configure (Client, Ada.Environment_Variables.Value ("TT_SETTINGS_JSON"), Error);
      if Error.Kind /= None then Print_Failure; return; end if;
   end if;
   if Argument_Count = 1 and then Argument (1) = "helper" then Helper;
   elsif Argument_Count = 1 and then Argument (1) = "limits" then Limits;
   elsif Argument_Count >= 4 and then Argument (1) = "plan" then
      declare
         Texts : Evidence_Array (1 .. Argument_Count - 4);
      begin
         for I in Texts'Range loop Texts (I) := To_Unbounded_String (Argument (I + 4)); end loop;
         Plan (Client, Argument (2), Argument (3), Texts, Result, Error, Settings_JSON => Argument (4));
         if Error.Kind = None then Put_Line (To_String (Result)); else Print_Failure; end if;
      end;
   elsif Argument_Count >= 3 and then Argument (1) = "fields" then
      Call (Client, Argument (2), Result, Error);
      Require (Error.Kind = None, "annotate call");
      declare
         Rows : constant String := Member (To_String (Result), "value");
         Row : Positive := 1;
      begin
         Put ("[");
         while Element (Rows, Row) /= "" loop
            Put ((if Row > 1 then ",{" else "{"));
            for I in 3 .. Argument_Count loop
               Put ((if I > 3 then "," else "") & """" & Argument (I) & """:""" &
                    State (Annotation (Rows, Argument (I), Row)) & """");
            end loop;
            Put ("}");
            Row := Row + 1;
         end loop;
         Put_Line ("]");
      end;
   elsif Argument_Count = 1 then
      Call (Client, Argument (1), Result, Error);
      if Error.Kind = None then Put_Line (To_String (Result)); else Print_Failure; end if;
   else
      raise Program_Error with "unknown door mode";
   end if;
end Door;
