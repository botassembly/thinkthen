using System.Runtime.InteropServices;
namespace ThinkThen;
internal static partial class NativeComplete {
[DllImport(Library)]internal static extern int thinkthen_decide_batch_start(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr b);
[DllImport(Library)]internal static extern int thinkthen_choose_batch_start(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr b);
[DllImport(Library)]internal static extern int thinkthen_tag_batch_start(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr b);
[DllImport(Library)]internal static extern int thinkthen_score_batch_start(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr b);
[DllImport(Library)]internal static extern int thinkthen_filter_batch_start(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr b);
[DllImport(Library)]internal static extern int thinkthen_annotate_batch_start(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr b);
}
public sealed partial class Engine {
public CompleteBatch<DecideRow> DecideBatch(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Batch(q,s,c,budget,cancellation,NativeComplete.thinkthen_decide_batch_start,NativeComplete.RowDecide);
public CompleteBatch<ChooseRow> ChooseBatch(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Batch(q,s,c,budget,cancellation,NativeComplete.thinkthen_choose_batch_start,NativeComplete.RowChoose);
public CompleteBatch<TagRow> TagBatch(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Batch(q,s,c,budget,cancellation,NativeComplete.thinkthen_tag_batch_start,NativeComplete.RowTag);
public CompleteBatch<ScoreRow> ScoreBatch(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Batch(q,s,c,budget,cancellation,NativeComplete.thinkthen_score_batch_start,NativeComplete.RowScore);
public CompleteBatch<FilterRow> FilterBatch(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Batch(q,s,c,budget,cancellation,NativeComplete.thinkthen_filter_batch_start,NativeComplete.RowFilter);
public CompleteBatch<AnnotateRow> AnnotateBatch(QuestionInput q,InputSource s,CallControls c,TimeSpan? budget=null,CancellationToken cancellation=default)=>Batch(q,s,c,budget,cancellation,NativeComplete.thinkthen_annotate_batch_start,NativeComplete.RowAnnotate);
}
