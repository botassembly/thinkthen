-- Generated named calls from the canonical request schema; do not edit.
with Ada.Unchecked_Conversion;
with System;
with Thinkthen_Kinds;
with Thinkthen.Requests.Native;
with Thinkthen.Inputs.Calls;
with Thinkthen.Native_Sessions;
with Thinkthen.Native_Values;
with Thinkthen.Session_C;
with Interfaces.C.Strings;
package body Thinkthen.Sessions.Calls is
procedure Decide (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Decide (Owner, Question, Input, Options);
end Decide;
procedure Decide (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Decide (Client, Owner, Question, Input, Options);
end Decide;
procedure Choose (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Choose (Owner, Question, Input, Options);
end Choose;
procedure Choose (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Choose (Client, Owner, Question, Input, Options);
end Choose;
procedure Tag (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Tag (Owner, Question, Input, Options);
end Tag;
procedure Tag (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Tag (Client, Owner, Question, Input, Options);
end Tag;
procedure Score (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Score (Owner, Question, Input, Options);
end Score;
procedure Score (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Score (Client, Owner, Question, Input, Options);
end Score;
procedure Filter (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Filter (Owner, Question, Input, Options);
end Filter;
procedure Filter (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Filter (Client, Owner, Question, Input, Options);
end Filter;
procedure Rank (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Rank (Owner, Question, Input, Options);
end Rank;
procedure Rank (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Rank (Client, Owner, Question, Input, Options);
end Rank;
procedure Find (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Find (Owner, Question, Input, Options);
end Find;
procedure Find (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Find (Client, Owner, Question, Input, Options);
end Find;
procedure Annotate (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Annotate (Owner, Question, Input, Options);
end Annotate;
procedure Annotate (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Annotate (Client, Owner, Question, Input, Options);
end Annotate;
procedure Recognize (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Recognize (Owner, Question, Input, Options);
end Recognize;
procedure Recognize (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Recognize (Client, Owner, Question, Input, Options);
end Recognize;
procedure Relate (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Relate (Owner, Question, Input, Options);
end Relate;
procedure Relate (Client : Engine; Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options) is
begin
Thinkthen.Inputs.Calls.Relate (Client, Owner, Question, Input, Options);
end Relate;
end Thinkthen.Sessions.Calls;
