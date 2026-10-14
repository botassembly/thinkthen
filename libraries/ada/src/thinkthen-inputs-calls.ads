-- Generated named calls over friendly inputs.
with Thinkthen.Sessions;
package Thinkthen.Inputs.Calls is
type Plan_Estimate is private;
type Token_Band is record Lower, Upper : Interfaces.Unsigned_64; end record;
type Optional_Text (Present : Boolean := False) is record
case Present is when True => Value : Ada.Strings.Unbounded.Unbounded_String; when False => null; end case;
end record;
procedure Decide (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Decide (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Choose (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Choose (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Tag (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Tag (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Score (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Score (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Filter (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Filter (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Rank (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Rank (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Find (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Find (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Annotate (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Annotate (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Recognize (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Recognize (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Relate (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Relate (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options);
procedure Configure (Client : in out Engine; Settings : Engine_Settings; Error : out Failure);
procedure Push (Owner : in out Thinkthen.Sessions.Session; Item : Feed_Item; Status : out Thinkthen.Sessions.Push_Status);
procedure Finish (Owner : in out Thinkthen.Sessions.Session; Error : Reader_Error);
function Plan (Client : Engine; Function_Name : Function_Kind; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) return Plan_Estimate;
function Estimated_Bytes (Item : Plan_Estimate) return Interfaces.Unsigned_64;
function Estimated_Input_Tokens (Item : Plan_Estimate) return Token_Band;
function First_Body_Utf8 (Item : Plan_Estimate) return Optional_Text;
function Largest_Request_Bytes (Item : Plan_Estimate) return Interfaces.Unsigned_64;
function Largest_Request_Estimated_Input_Tokens (Item : Plan_Estimate) return Interfaces.Unsigned_64;
function Records (Item : Plan_Estimate) return Interfaces.Unsigned_64;
function Requests (Item : Plan_Estimate) return Interfaces.Unsigned_64;
function Token_Estimate_Method (Item : Plan_Estimate) return String;
function Upper_Bound (Item : Plan_Estimate) return Boolean;
private
type Plan_Estimate is record Data : Thinkthen.Values.Value; end record;
end Thinkthen.Inputs.Calls;
