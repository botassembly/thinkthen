-- Generated native admission; Rust owns all question semantics.
with Thinkthen.Inputs.Native;
with Thinkthen.Requests; use Thinkthen.Requests;
package body Thinkthen.Inputs.Calls is
procedure Decide (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "decide");
end Decide;
procedure Decide (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "decide");
end Decide;
procedure Choose (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "choose");
end Choose;
procedure Choose (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "choose");
end Choose;
procedure Tag (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "tag");
end Tag;
procedure Tag (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "tag");
end Tag;
procedure Score (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "score");
end Score;
procedure Score (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "score");
end Score;
procedure Filter (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "filter");
end Filter;
procedure Filter (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "filter");
end Filter;
procedure Rank (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "rank");
end Rank;
procedure Rank (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "rank");
end Rank;
procedure Find (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "find");
end Find;
procedure Find (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "find");
end Find;
procedure Annotate (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "annotate");
end Annotate;
procedure Annotate (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "annotate");
end Annotate;
procedure Recognize (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "recognize");
end Recognize;
procedure Recognize (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "recognize");
end Recognize;
procedure Relate (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "relate");
end Relate;
procedure Relate (Client : Engine; Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) is
begin
Thinkthen.Sessions.Start (Client, Owner, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "relate");
end Relate;
end Thinkthen.Inputs.Calls;
