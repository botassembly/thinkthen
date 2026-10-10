-- Generated named calls from canonical RequestCall variants.
with Thinkthen.Requests; use Thinkthen.Requests;
package Thinkthen.Sessions.Calls is
procedure Decide (Owner : in out Session; Request : T_RequestCall_decide);
procedure Choose (Owner : in out Session; Request : T_RequestCall_choose);
procedure Tag (Owner : in out Session; Request : T_RequestCall_tag);
procedure Score (Owner : in out Session; Request : T_RequestCall_score);
procedure Filter (Owner : in out Session; Request : T_RequestCall_filter);
procedure Rank (Owner : in out Session; Request : T_RequestCall_rank);
procedure Find (Owner : in out Session; Request : T_RequestCall_find);
procedure Annotate (Owner : in out Session; Request : T_RequestCall_annotate);
procedure Recognize (Owner : in out Session; Request : T_RequestCall_recognize);
procedure Relate (Owner : in out Session; Request : T_RequestCall_relate);
end Thinkthen.Sessions.Calls;
