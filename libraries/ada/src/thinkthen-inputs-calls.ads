-- Generated named calls over friendly inputs.
with Thinkthen.Sessions;
package Thinkthen.Inputs.Calls is
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
end Thinkthen.Inputs.Calls;
