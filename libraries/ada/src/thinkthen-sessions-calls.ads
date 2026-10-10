-- Generated named calls from canonical RequestCall variants.
with Thinkthen.Requests; use Thinkthen.Requests;
package Thinkthen.Sessions.Calls is
procedure Decide (Owner : in out Session; Request : T_RequestCall_decide);
procedure Decide (Client : Engine; Owner : in out Session; Request : T_RequestCall_decide);
procedure Choose (Owner : in out Session; Request : T_RequestCall_choose);
procedure Choose (Client : Engine; Owner : in out Session; Request : T_RequestCall_choose);
procedure Tag (Owner : in out Session; Request : T_RequestCall_tag);
procedure Tag (Client : Engine; Owner : in out Session; Request : T_RequestCall_tag);
procedure Score (Owner : in out Session; Request : T_RequestCall_score);
procedure Score (Client : Engine; Owner : in out Session; Request : T_RequestCall_score);
procedure Filter (Owner : in out Session; Request : T_RequestCall_filter);
procedure Filter (Client : Engine; Owner : in out Session; Request : T_RequestCall_filter);
procedure Rank (Owner : in out Session; Request : T_RequestCall_rank);
procedure Rank (Client : Engine; Owner : in out Session; Request : T_RequestCall_rank);
procedure Find (Owner : in out Session; Request : T_RequestCall_find);
procedure Find (Client : Engine; Owner : in out Session; Request : T_RequestCall_find);
procedure Annotate (Owner : in out Session; Request : T_RequestCall_annotate);
procedure Annotate (Client : Engine; Owner : in out Session; Request : T_RequestCall_annotate);
procedure Recognize (Owner : in out Session; Request : T_RequestCall_recognize);
procedure Recognize (Client : Engine; Owner : in out Session; Request : T_RequestCall_recognize);
procedure Relate (Owner : in out Session; Request : T_RequestCall_relate);
procedure Relate (Client : Engine; Owner : in out Session; Request : T_RequestCall_relate);
end Thinkthen.Sessions.Calls;
