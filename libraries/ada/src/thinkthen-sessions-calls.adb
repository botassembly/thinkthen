-- Generated named calls from the canonical request schema; do not edit.
package body Thinkthen.Sessions.Calls is
procedure Decide (Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_decide;
Start (Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Decide;
procedure Decide (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_decide;
Start (Client, Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Decide;
procedure Choose (Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_choose;
Start (Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Choose;
procedure Choose (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_choose;
Start (Client, Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Choose;
procedure Tag (Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_tag;
Start (Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Tag;
procedure Tag (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_tag;
Start (Client, Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Tag;
procedure Score (Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_score;
Start (Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Score;
procedure Score (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_score;
Start (Client, Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Score;
procedure Filter (Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_filter;
Start (Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Filter;
procedure Filter (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_filter;
Start (Client, Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Filter;
procedure Rank (Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_rank;
Start (Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Rank;
procedure Rank (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_rank;
Start (Client, Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Rank;
procedure Find (Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_find;
Start (Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Find;
procedure Find (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_find;
Start (Client, Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Find;
procedure Annotate (Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_annotate;
Start (Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Annotate;
procedure Annotate (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_annotate;
Start (Client, Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Annotate;
procedure Recognize (Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_recognize;
Start (Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Recognize;
procedure Recognize (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_recognize;
Start (Client, Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Recognize;
procedure Relate (Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_relate;
Start (Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Relate;
procedure Relate (Client : Engine; Owner : in out Session; Request : T_RequestCall) is
Call : T_RequestCall := Request;
begin
Call.T_function := T_RequestFunction_relate;
Start (Client, Owner, (T_schema => T_RequestVersion_thinkthen_request_1, T_call => Call));
end Relate;
end Thinkthen.Sessions.Calls;
