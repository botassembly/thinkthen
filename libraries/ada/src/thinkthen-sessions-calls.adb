-- Generated named calls from the canonical request schema; do not edit.
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
end Thinkthen.Sessions.Calls;
