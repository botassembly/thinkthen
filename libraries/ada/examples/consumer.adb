with Ada.Environment_Variables;
with Ada.Exceptions; use Ada.Exceptions;
with Ada.Strings.Fixed;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Thinkthen; use Thinkthen;
procedure Consumer is
   Client : Thinkthen.Engine;
   Answer : Thinkthen.Decision;
   Facts : Unbounded_String;
   Error : Thinkthen.Failure;
   JSON : Unbounded_String;
   Data : constant String := Ada.Environment_Variables.Value ("TT_CONSUMER_EVIDENCE", "consumer-ada");
begin
   Thinkthen.Decide (Client, "Is it?", Data, Answer, Facts, Error);
   if Error.Kind /= Thinkthen.None or Answer.Value /= Thinkthen.Yes or
      Thinkthen.Member (To_String (Facts), "records") /= "1" or
      Thinkthen.Member (To_String (Facts), "requests_sent") /= "1" or
      Thinkthen.Member (To_String (Facts), "model") = "" then
      raise Program_Error with "scalar consumer failure: " & Thinkthen.Message (Error);
   end if;
   Thinkthen.Call (Client, "{""choose"":""Which?"",""options"":{ ""first"":{ ""what"":""A first choice"", ""not_for"":""other choices"", ""examples"":[""sample""] }, ""second"":""Second choice"" }, ""evidence"":""consumer-ada-json""}", JSON, Error);
   if Error.Kind /= Thinkthen.None or else Thinkthen.Member (To_String (JSON), "value") /= """first""" or else
      Thinkthen.Member (Thinkthen.Member (To_String (JSON), "facts"), "records") /= "1" then
      raise Program_Error with "JSON consumer failure: " & Thinkthen.Message (Error);
   end if;
   Put_Line ("ADA_CALL_ENVELOPE=" & To_String (JSON));
   declare
      Null_Field : constant Annotated_Field := Decode_Field ("null");
      Failed_Field : constant Annotated_Field := Decode_Field ("{""failed"":{""kind"":""backend"",""cause"":""fixture""}}");
      Labels : constant Label_Set := (1 => (To_Unbounded_String ("first"),
         To_Unbounded_String ("{""what"":""A first choice"",""not_for"":""other choices"",""examples"":[""sample""]}")));
   begin
      if Annotation ("[{""check"":null},{""check"":{""failed"":{""kind"":""backend"",""cause"":""fixture""}}}]", "check", 1).Kind /= Null_Answer or
         Annotation ("[{""check"":null},{""check"":{""failed"":{""kind"":""backend"",""cause"":""fixture""}}}]", "check", 2).Kind /= Failed_Answer or
         Null_Field.Kind /= Null_Answer or Failed_Field.Kind /= Failed_Answer or
         Failed_Field.Marker.Kind /= Backend or To_String (Failed_Field.Marker.Cause) /= "fixture" then
         raise Program_Error with "null/failure conflated";
      end if;
      if Label_Descriptions (Labels) /=
        "{""first"":{""what"":""A first choice"",""not_for"":""other choices"",""examples"":[""sample""]}}" or else
         Label_Descriptions (Bare_Labels ((To_Unbounded_String ("first"), To_Unbounded_String ("second")))) /=
         "[""first"",""second""]" then
         raise Program_Error with "label forms changed";
      end if;
      declare
         Mixed : constant Label_Set := (Labels (1), (To_Unbounded_String ("second"), Null_Unbounded_String));
         Caught : Boolean := False;
      begin
         begin
            declare
               Discard : constant String := Label_Descriptions (Mixed);
            begin
               Put_Line (Discard);
            end;
         exception
            when E : Constraint_Error =>
               Caught := Ada.Strings.Fixed.Index (Exception_Message (E), "second") > 0;
         end;
         if not Caught then raise Program_Error with "mixed label set accepted"; end if;
      end;
   end;
   Put_Line ("INSTALLED_ADA_CONSUMER_PASS");
end Consumer;
