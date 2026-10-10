package body Thinkthen.Sessions.Calls is
procedure Decide (Owner : in out Session; Request : T_RequestCall_decide) is
begin
Start (Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_decide, V_0 => Request)));
end Decide;
procedure Decide (Client : Engine; Owner : in out Session; Request : T_RequestCall_decide) is
begin
Start (Client, Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_decide, V_0 => Request)));
end Decide;
procedure Choose (Owner : in out Session; Request : T_RequestCall_choose) is
begin
Start (Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_choose, V_1 => Request)));
end Choose;
procedure Choose (Client : Engine; Owner : in out Session; Request : T_RequestCall_choose) is
begin
Start (Client, Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_choose, V_1 => Request)));
end Choose;
procedure Tag (Owner : in out Session; Request : T_RequestCall_tag) is
begin
Start (Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_tag, V_2 => Request)));
end Tag;
procedure Tag (Client : Engine; Owner : in out Session; Request : T_RequestCall_tag) is
begin
Start (Client, Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_tag, V_2 => Request)));
end Tag;
procedure Score (Owner : in out Session; Request : T_RequestCall_score) is
begin
Start (Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_score, V_3 => Request)));
end Score;
procedure Score (Client : Engine; Owner : in out Session; Request : T_RequestCall_score) is
begin
Start (Client, Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_score, V_3 => Request)));
end Score;
procedure Filter (Owner : in out Session; Request : T_RequestCall_filter) is
begin
Start (Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_filter, V_4 => Request)));
end Filter;
procedure Filter (Client : Engine; Owner : in out Session; Request : T_RequestCall_filter) is
begin
Start (Client, Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_filter, V_4 => Request)));
end Filter;
procedure Rank (Owner : in out Session; Request : T_RequestCall_rank) is
begin
Start (Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_rank, V_5 => Request)));
end Rank;
procedure Rank (Client : Engine; Owner : in out Session; Request : T_RequestCall_rank) is
begin
Start (Client, Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_rank, V_5 => Request)));
end Rank;
procedure Find (Owner : in out Session; Request : T_RequestCall_find) is
begin
Start (Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_find, V_6 => Request)));
end Find;
procedure Find (Client : Engine; Owner : in out Session; Request : T_RequestCall_find) is
begin
Start (Client, Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_find, V_6 => Request)));
end Find;
procedure Annotate (Owner : in out Session; Request : T_RequestCall_annotate) is
begin
Start (Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_annotate, V_7 => Request)));
end Annotate;
procedure Annotate (Client : Engine; Owner : in out Session; Request : T_RequestCall_annotate) is
begin
Start (Client, Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_annotate, V_7 => Request)));
end Annotate;
procedure Recognize (Owner : in out Session; Request : T_RequestCall_recognize) is
begin
Start (Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_recognize, V_8 => Request)));
end Recognize;
procedure Recognize (Client : Engine; Owner : in out Session; Request : T_RequestCall_recognize) is
begin
Start (Client, Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_recognize, V_8 => Request)));
end Recognize;
procedure Relate (Owner : in out Session; Request : T_RequestCall_relate) is
begin
Start (Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_relate, V_9 => Request)));
end Relate;
procedure Relate (Client : Engine; Owner : in out Session; Request : T_RequestCall_relate) is
begin
Start (Client, Owner, (T_schema => (null record), T_call => (Kind => T_RequestCall_arm_RequestCall_relate, V_9 => Request)));
end Relate;
end Thinkthen.Sessions.Calls;
