-- Generated native admission; Rust owns all question semantics.
with Ada.Unchecked_Conversion;
with Interfaces.C.Strings;
with Thinkthen.Session_C;
with Thinkthen.Requests.Native;
with System;
with Thinkthen.Inputs.Native;
with Thinkthen.Native_Sessions;
with Thinkthen.Requests; use Thinkthen.Requests;
package body Thinkthen.Inputs.Calls is
use type Thinkthen.Values.Value_Kind;
procedure Decide (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "decide");
end Decide;
procedure Decide (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "decide");
end Decide;
procedure Choose (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "choose");
end Choose;
procedure Choose (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "choose");
end Choose;
procedure Tag (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "tag");
end Tag;
procedure Tag (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "tag");
end Tag;
procedure Score (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "score");
end Score;
procedure Score (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "score");
end Score;
procedure Filter (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "filter");
end Filter;
procedure Filter (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "filter");
end Filter;
procedure Rank (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "rank");
end Rank;
procedure Rank (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "rank");
end Rank;
procedure Find (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "find");
end Find;
procedure Find (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "find");
end Find;
procedure Annotate (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "annotate");
end Annotate;
procedure Annotate (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "annotate");
end Annotate;
procedure Recognize (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "recognize");
end Recognize;
procedure Recognize (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "recognize");
end Recognize;
procedure Relate (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "relate");
end Relate;
procedure Relate (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Native_Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "relate");
end Relate;
procedure Push (Owner : in out Thinkthen.Sessions.Session; Item : Feed_Item; Status : out Thinkthen.Sessions.Push_Status) is
begin
Thinkthen.Native_Sessions.Push (Owner, Native.Unwrap (Item), Status);
end Push;
procedure Finish (Owner : in out Thinkthen.Sessions.Session; Error : Reader_Error) is
begin
Thinkthen.Native_Sessions.Finish (Owner, Native.Unwrap (Error));
end Finish;
function Estimated_Bytes (Item : Plan_Estimate) return Interfaces.Unsigned_64 is (Interfaces.Unsigned_64'Value (Thinkthen.Values.As_Number (Thinkthen.Values.Field_Value (Item.Data, "estimated_bytes"))));
function Estimated_Input_Tokens (Item : Plan_Estimate) return Token_Band is ((Lower => Interfaces.Unsigned_64'Value (Thinkthen.Values.As_Number (Thinkthen.Values.Field_Value (Thinkthen.Values.Field_Value (Item.Data, "estimated_input_tokens"), "lower"))), Upper => Interfaces.Unsigned_64'Value (Thinkthen.Values.As_Number (Thinkthen.Values.Field_Value (Thinkthen.Values.Field_Value (Item.Data, "estimated_input_tokens"), "upper")))));
function First_Body_Utf8 (Item : Plan_Estimate) return Optional_Text is ((if Thinkthen.Values.Kind (Thinkthen.Values.Field_Value (Item.Data, "first_body_utf8")) = Thinkthen.Values.Null_Value then (Present => False) else (Present => True, Value => Ada.Strings.Unbounded.To_Unbounded_String (Thinkthen.Values.As_String (Thinkthen.Values.Field_Value (Item.Data, "first_body_utf8"))))));
function Largest_Request_Bytes (Item : Plan_Estimate) return Interfaces.Unsigned_64 is (Interfaces.Unsigned_64'Value (Thinkthen.Values.As_Number (Thinkthen.Values.Field_Value (Item.Data, "largest_request_bytes"))));
function Largest_Request_Estimated_Input_Tokens (Item : Plan_Estimate) return Interfaces.Unsigned_64 is (Interfaces.Unsigned_64'Value (Thinkthen.Values.As_Number (Thinkthen.Values.Field_Value (Item.Data, "largest_request_estimated_input_tokens"))));
function Records (Item : Plan_Estimate) return Interfaces.Unsigned_64 is (Interfaces.Unsigned_64'Value (Thinkthen.Values.As_Number (Thinkthen.Values.Field_Value (Item.Data, "records"))));
function Requests (Item : Plan_Estimate) return Interfaces.Unsigned_64 is (Interfaces.Unsigned_64'Value (Thinkthen.Values.As_Number (Thinkthen.Values.Field_Value (Item.Data, "requests"))));
function Token_Estimate_Method (Item : Plan_Estimate) return String is (Thinkthen.Values.As_String (Thinkthen.Values.Field_Value (Item.Data, "token_estimate_method")));
function Upper_Bound (Item : Plan_Estimate) return Boolean is (Thinkthen.Values.As_Boolean (Thinkthen.Values.Field_Value (Item.Data, "upper_bound")));
function Plan (Client : Engine; Function_Name : Function_Kind; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) return Plan_Estimate is
begin
return (Data => Thinkthen.Native_Sessions.Plan (Client, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), (case Function_Name is when Function_Kind_Decide => "decide", when Function_Kind_Choose => "choose", when Function_Kind_Tag => "tag", when Function_Kind_Score => "score", when Function_Kind_Filter => "filter", when Function_Kind_Rank => "rank", when Function_Kind_Find => "find", when Function_Kind_Annotate => "annotate", when Function_Kind_Recognize => "recognize", when Function_Kind_Relate => "relate")));
end Plan;
procedure Configure (Client : in out Engine; Settings : Engine_Settings; Error : out Failure) is
use Thinkthen.Session_C;
use Interfaces.C;
Storage : Thinkthen.Requests.Native.Arena;
type Settings_Access is access constant thinkthen_request_engine_settings_v1;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Settings_Access);
Tree : constant Settings_Access := Pointer (Thinkthen.Requests.Native.Convert (Native.Unwrap (Settings), Storage));
Replacement : constant Engine_Access := Engine_Access (thinkthen_engine_new_typed (Tree));
Code : int;
begin
if Replacement = null then
Code := thinkthen_error_code (null);
if Code not in 1 .. 6 then raise Program_Error with "unknown native error kind"; end if;
Error := (Kind => Error_Kind'Val (Code), Retryable => thinkthen_error_retryable (null) /= 0,
Text => Ada.Strings.Unbounded.To_Unbounded_String (Interfaces.C.Strings.Value (thinkthen_error_message (null))));
else
thinkthen_engine_free (Engine_Pointer (Client.Handle));
Client.Handle := Engine_Address (Replacement);
Error := (others => <>);
end if;
end Configure;
end Thinkthen.Inputs.Calls;
