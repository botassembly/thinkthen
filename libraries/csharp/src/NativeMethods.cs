using System.Runtime.InteropServices;
namespace ThinkThen;
internal static partial class NativeComplete {
[DllImport(Library)]internal static extern int thinkthen_decide_complete(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr r);
[DllImport(Library)]internal static extern int thinkthen_result_decide(IntPtr r,nuint i,out DecideViewV1 v);
internal static DecideRow RowDecide(IntPtr r,nuint i) {if(thinkthen_result_decide(r,i,out var v)!=0)throw new InvalidOperationException("native decide row failed");var row=ReadDecideRow(v);row=row with {Common=Common(r,i,row.Common)};return row;}
[DllImport(Library)]internal static extern int thinkthen_choose_complete(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr r);
[DllImport(Library)]internal static extern int thinkthen_result_choose(IntPtr r,nuint i,out ChooseViewV1 v);
internal static ChooseRow RowChoose(IntPtr r,nuint i) {if(thinkthen_result_choose(r,i,out var v)!=0)throw new InvalidOperationException("native choose row failed");var row=ReadChooseRow(v);row=row with {Common=Common(r,i,row.Common)};return row;}
[DllImport(Library)]internal static extern int thinkthen_tag_complete(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr r);
[DllImport(Library)]internal static extern int thinkthen_result_tag(IntPtr r,nuint i,out TagViewV1 v);
internal static TagRow RowTag(IntPtr r,nuint i) {if(thinkthen_result_tag(r,i,out var v)!=0)throw new InvalidOperationException("native tag row failed");var row=ReadTagRow(v);row=row with {Common=Common(r,i,row.Common)};return row;}
[DllImport(Library)]internal static extern int thinkthen_score_complete(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr r);
[DllImport(Library)]internal static extern int thinkthen_result_score(IntPtr r,nuint i,out ScoreViewV1 v);
internal static ScoreRow RowScore(IntPtr r,nuint i) {if(thinkthen_result_score(r,i,out var v)!=0)throw new InvalidOperationException("native score row failed");var row=ReadScoreRow(v);row=row with {Common=Common(r,i,row.Common)};return row;}
[DllImport(Library)]internal static extern int thinkthen_filter_complete(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr r);
[DllImport(Library)]internal static extern int thinkthen_result_filter(IntPtr r,nuint i,out FilterViewV1 v);
internal static FilterRow RowFilter(IntPtr r,nuint i) {if(thinkthen_result_filter(r,i,out var v)!=0)throw new InvalidOperationException("native filter row failed");var row=ReadFilterRow(v);row=row with {Common=Common(r,i,row.Common)};return row;}
[DllImport(Library)]internal static extern int thinkthen_rank_complete(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr r);
[DllImport(Library)]internal static extern int thinkthen_result_rank(IntPtr r,nuint i,out RankViewV1 v);
internal static RankRow RowRank(IntPtr r,nuint i) {if(thinkthen_result_rank(r,i,out var v)!=0)throw new InvalidOperationException("native rank row failed");var row=ReadRankRow(v);row=row with {Common=Common(r,i,row.Common)};if(thinkthen_result_rank_member_count(r,i,out var count)!=0)throw new InvalidOperationException("native rank member count failed");RankRow[] members=new RankRow[checked((int)count)];for(int j=0;j<members.Length;j++){if(thinkthen_result_rank_member(r,i,(nuint)j,out var member)!=0)throw new InvalidOperationException("native rank member failed");if(thinkthen_result_member_author(r,i,(nuint)j,out var author)!=0||thinkthen_result_rank_member_details(r,i,(nuint)j,out var details)!=0)throw new InvalidOperationException("native rank member detail failed");var m=ReadRankRow(member);members[j]=m with {Common=m.Common with {Author=ReadQuestionAuthor(author),Details=ReadDetails(details)}};}row=row with {Members=members};return row;}
[DllImport(Library)]internal static extern int thinkthen_find_complete(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr r);
[DllImport(Library)]internal static extern int thinkthen_result_find(IntPtr r,nuint i,out FindViewV1 v);
internal static FindRow RowFind(IntPtr r,nuint i) {if(thinkthen_result_find(r,i,out var v)!=0)throw new InvalidOperationException("native find row failed");var row=ReadFindRow(v);row=row with {Common=Common(r,i,row.Common)};return row;}
[DllImport(Library)]internal static extern int thinkthen_annotate_complete(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr r);
[DllImport(Library)]internal static extern int thinkthen_result_annotate(IntPtr r,nuint i,out AnnotateViewV1 v);
internal static AnnotateRow RowAnnotate(IntPtr r,nuint i) {if(thinkthen_result_annotate(r,i,out var v)!=0)throw new InvalidOperationException("native annotate row failed");var row=ReadAnnotateRow(v);row=row with {Common=Common(r,i,row.Common)};var members=row.Answers.ToArray();for(int j=0;j<members.Length;j++){if(thinkthen_result_member_author(r,i,(nuint)j,out var a)!=0)throw new InvalidOperationException("native member author failed");members[j]=members[j] with {Author=ReadQuestionAuthor(a)};}row=row with {Answers=members};return row;}
[DllImport(Library)]internal static extern int thinkthen_recognize_complete(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr r);
[DllImport(Library)]internal static extern int thinkthen_result_recognize(IntPtr r,nuint i,out RecognizeViewV1 v);
internal static RecognizeRow RowRecognize(IntPtr r,nuint i) {if(thinkthen_result_recognize(r,i,out var v)!=0)throw new InvalidOperationException("native recognize row failed");var row=ReadRecognizeRow(v);row=row with {Common=Common(r,i,row.Common)};if(thinkthen_result_source_recognition(r,i,out var located)!=0)throw new InvalidOperationException("native source recognition failed");row=row with {Located=ReadSourceRecognition(located)};return row;}
[DllImport(Library)]internal static extern int thinkthen_relate_complete(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr r);
[DllImport(Library)]internal static extern int thinkthen_result_relate(IntPtr r,nuint i,out RelateViewV1 v);
internal static RelateRow RowRelate(IntPtr r,nuint i) {if(thinkthen_result_relate(r,i,out var v)!=0)throw new InvalidOperationException("native relate row failed");var row=ReadRelateRow(v);row=row with {Common=Common(r,i,row.Common)};if(thinkthen_result_source_relations(r,i,out var located)!=0)throw new InvalidOperationException("native source relations failed");row=row with {Located=ReadSourceRelations(located)};return row;}
}
public sealed partial class Engine {
public CompleteCall<DecideRow> DecideComplete(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Complete(q,s,c,budget,cancellation,NativeComplete.thinkthen_decide_complete,NativeComplete.RowDecide);
public CompleteCall<ChooseRow> ChooseComplete(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Complete(q,s,c,budget,cancellation,NativeComplete.thinkthen_choose_complete,NativeComplete.RowChoose);
public CompleteCall<TagRow> TagComplete(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Complete(q,s,c,budget,cancellation,NativeComplete.thinkthen_tag_complete,NativeComplete.RowTag);
public CompleteCall<ScoreRow> ScoreComplete(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Complete(q,s,c,budget,cancellation,NativeComplete.thinkthen_score_complete,NativeComplete.RowScore);
public CompleteCall<FilterRow> FilterComplete(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Complete(q,s,c,budget,cancellation,NativeComplete.thinkthen_filter_complete,NativeComplete.RowFilter);
public CompleteCall<RankRow> RankComplete(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Complete(q,s,c,budget,cancellation,NativeComplete.thinkthen_rank_complete,NativeComplete.RowRank);
public CompleteCall<FindRow> FindComplete(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Complete(q,s,c,budget,cancellation,NativeComplete.thinkthen_find_complete,NativeComplete.RowFind);
public CompleteCall<AnnotateRow> AnnotateComplete(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Complete(q,s,c,budget,cancellation,NativeComplete.thinkthen_annotate_complete,NativeComplete.RowAnnotate);
public CompleteCall<RecognizeRow> RecognizeComplete(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Complete(q,s,c,budget,cancellation,NativeComplete.thinkthen_recognize_complete,NativeComplete.RowRecognize);
public CompleteCall<RelateRow> RelateComplete(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Complete(q,s,c,budget,cancellation,NativeComplete.thinkthen_relate_complete,NativeComplete.RowRelate);
}
