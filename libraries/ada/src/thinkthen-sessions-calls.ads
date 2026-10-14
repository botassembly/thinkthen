-- Generated named calls from canonical RequestCall variants.
with Thinkthen.Inputs;
package Thinkthen.Sessions.Calls is
procedure Decide (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Decide (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Choose (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Choose (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Tag (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Tag (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Score (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Score (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Filter (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Filter (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Rank (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Rank (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Find (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Find (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Annotate (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Annotate (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Recognize (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Recognize (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Relate (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
procedure Relate (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options);
end Thinkthen.Sessions.Calls;
