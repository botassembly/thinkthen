-- Generated named calls from the canonical request schema; do not edit.
with Ada.Unchecked_Conversion;
with System;
with Thinkthen.Requests.Native;
with Interfaces.C.Strings;
package body Thinkthen.Sessions.Calls is
procedure Decide (Owner : in out Session; Request : T_RequestCall) is
begin
Start (Owner, Request, "decide");
end Decide;
procedure Decide (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
begin
Start (Client, Owner, Request, "decide");
end Decide;
procedure Choose (Owner : in out Session; Request : T_RequestCall) is
begin
Start (Owner, Request, "choose");
end Choose;
procedure Choose (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
begin
Start (Client, Owner, Request, "choose");
end Choose;
procedure Tag (Owner : in out Session; Request : T_RequestCall) is
begin
Start (Owner, Request, "tag");
end Tag;
procedure Tag (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
begin
Start (Client, Owner, Request, "tag");
end Tag;
procedure Score (Owner : in out Session; Request : T_RequestCall) is
begin
Start (Owner, Request, "score");
end Score;
procedure Score (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
begin
Start (Client, Owner, Request, "score");
end Score;
procedure Filter (Owner : in out Session; Request : T_RequestCall) is
begin
Start (Owner, Request, "filter");
end Filter;
procedure Filter (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
begin
Start (Client, Owner, Request, "filter");
end Filter;
procedure Rank (Owner : in out Session; Request : T_RequestCall) is
begin
Start (Owner, Request, "rank");
end Rank;
procedure Rank (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
begin
Start (Client, Owner, Request, "rank");
end Rank;
procedure Find (Owner : in out Session; Request : T_RequestCall) is
begin
Start (Owner, Request, "find");
end Find;
procedure Find (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
begin
Start (Client, Owner, Request, "find");
end Find;
procedure Annotate (Owner : in out Session; Request : T_RequestCall) is
begin
Start (Owner, Request, "annotate");
end Annotate;
procedure Annotate (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
begin
Start (Client, Owner, Request, "annotate");
end Annotate;
procedure Recognize (Owner : in out Session; Request : T_RequestCall) is
begin
Start (Owner, Request, "recognize");
end Recognize;
procedure Recognize (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
begin
Start (Client, Owner, Request, "recognize");
end Recognize;
procedure Relate (Owner : in out Session; Request : T_RequestCall) is
begin
Start (Owner, Request, "relate");
end Relate;
procedure Relate (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
begin
Start (Client, Owner, Request, "relate");
end Relate;
function Plan (Client : Engine; Request : T_RequestCall) return String is
use Thinkthen_Session_C; use Interfaces.C; use Interfaces.C.Strings;
Storage : Thinkthen.Requests.Native.Arena;
type Arguments_Access is access constant thinkthen_request_arguments_v1;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Arguments_Access);
Arguments : constant Arguments_Access := Pointer (Thinkthen.Requests.Native.Convert (Request, Storage));
Function_Name : constant String := (case Request.T_function is when T_RequestFunction_decide => "decide", when T_RequestFunction_choose => "choose", when T_RequestFunction_tag => "tag", when T_RequestFunction_score => "score", when T_RequestFunction_filter => "filter", when T_RequestFunction_rank => "rank", when T_RequestFunction_find => "find", when T_RequestFunction_annotate => "annotate", when T_RequestFunction_recognize => "recognize", when T_RequestFunction_relate => "relate");
Function_Bytes : chars_ptr := Null_Ptr; Surface : chars_ptr := Null_Ptr;
Output : aliased chars_ptr := Null_Ptr; Length : aliased size_t := 0; Code : int;
begin
Function_Bytes := New_String (Function_Name); Surface := New_String ("ada");
Code := thinkthen_request_plan_typed_json (Client.Handle, (data => Function_Bytes, len => Function_Name'Length), Arguments, (data => Surface, len => 3), Output'Address, Length'Access);
Free (Function_Bytes); Free (Surface);
if Code /= 0 then declare Message : constant String := Value (thinkthen_session_error_message); begin case Code is
when K_THINKTHEN_EBACKEND => raise Backend_Error with Message;
when K_THINKTHEN_ECANCELLED => raise Cancelled_Error with Message;
when K_THINKTHEN_EDEADLINE => raise Deadline_Error with Message;
when K_THINKTHEN_EDEFECT => raise Defect_Error with Message;
when K_THINKTHEN_ELOCAL => raise Local_Error with Message;
when K_THINKTHEN_EUSAGE => raise Usage_Error with Message;
when others => raise Native_Error with Message; end case; end; end if;
declare Result : constant String := Text ((data => Output, len => Length)); begin thinkthen_free_string (Output); Output := Null_Ptr; return Result; end;
exception when others => Free (Function_Bytes); Free (Surface); thinkthen_free_string (Output); raise;
end Plan;
end Thinkthen.Sessions.Calls;
