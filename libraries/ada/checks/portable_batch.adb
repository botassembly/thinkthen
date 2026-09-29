with Ada.Command_Line; use Ada.Command_Line;
with Ada.Environment_Variables;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Interfaces; use Interfaces;
with Thinkthen; use Thinkthen;
procedure Portable_Batch is
   Client : Engine;
   Error : Thinkthen.Failure;
   Texts : Evidence_Array (1 .. 5);
   Answers : Decision_Array (1 .. 5);
   Facts : Run_Facts;
begin
   if Argument_Count /= 5 then raise Program_Error with "five shared texts required"; end if;
   for I in Texts'Range loop Texts (I) := To_Unbounded_String (Argument (I)); end loop;
   Configure (Client, Ada.Environment_Variables.Value ("TT_PORTABLE_SETTINGS"), Error);
   if Error.Kind /= None then raise Program_Error with "settings rejected"; end if;
   Decide_Many (Client, "Is it relevant?", Texts, Answers, Facts, Error);
   if Error.Kind /= None or Facts.Records /= 5 or Facts.Requests_Sent /= 3 then
      raise Program_Error with "bulk facts changed";
   end if;
   for I in Answers'Range loop
      if Answers (I).Value /= Yes or Answers (I).Probability /= 0.9 then
         raise Program_Error with "bulk answer changed at" & I'Image;
      end if;
   end loop;
   Put_Line ("ADA_PORTABLE_BATCH_PASS five typed rows");
end Portable_Batch;
