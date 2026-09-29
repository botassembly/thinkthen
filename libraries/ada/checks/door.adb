with Ada.Characters.Handling; use Ada.Characters.Handling;
with Ada.Command_Line; use Ada.Command_Line;
with Ada.Environment_Variables;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Thinkthen; use Thinkthen;
procedure Door is
   Client : Engine;
   Result : JSON_Result;
   Error : Thinkthen.Failure;
begin
   if Argument_Count /= 1 then raise Program_Error with "one request required"; end if;
   if Ada.Environment_Variables.Exists ("TT_SETTINGS_JSON") then
      Configure (Client, Ada.Environment_Variables.Value ("TT_SETTINGS_JSON"), Error);
      if Error.Kind /= None then
         Put_Line ("{""error"":""" & To_Lower (Error_Kind'Image (Error.Kind)) & """}");
         return;
      end if;
   end if;
   Call (Client, Argument (1), Result, Error);
   if Error.Kind = None then
      Put_Line (To_String (Result.JSON));
   else
      Put_Line ("{""error"":""" & To_Lower (Error_Kind'Image (Error.Kind)) & """}");
   end if;
end Door;
