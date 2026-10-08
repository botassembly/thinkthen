using System.Runtime.InteropServices;
namespace ThinkThen;
internal static partial class NativeComplete {
 [DllImport(Library)]internal static extern int thinkthen_question_new_recognition_v1(IntPtr e,ref QuestionSpecV1 spec,ref QuestionAuthorV1 author,ref RecognitionTaskV1 task,out IntPtr q);
 [DllImport(Library)]internal static extern int thinkthen_question_recognition_task_v1(IntPtr q,out RecognitionTaskV1 task);
 [DllImport(Library)]internal static extern int thinkthen_result_recognition_task_v1(IntPtr r,nuint i,out RecognitionTaskV1 task);
 const string Library="libthinkthen.so.0";
 [DllImport(Library)]internal static extern int thinkthen_question_new(IntPtr e,ref QuestionSpecV1 spec,out IntPtr q);
 [DllImport(Library)]internal static extern int thinkthen_question_load(IntPtr e,StringV1 path,out IntPtr q);
 [DllImport(Library)]internal static extern int thinkthen_question_parse(IntPtr e,uint role,StringV1 value,out IntPtr q);
 [DllImport(Library)]internal static extern int thinkthen_question_load_named(IntPtr e,uint role,StringV1 value,out IntPtr q);
 [DllImport(Library)]internal static extern int thinkthen_question_load_reference(IntPtr e,uint role,StringV1 value,out IntPtr q);
 [DllImport(Library)]internal static extern void thinkthen_question_free(IntPtr q);
 [DllImport(Library)]internal static extern int thinkthen_image_clone(IntPtr e,IntPtr data,nuint len,uint media,OptionalStringV1 filename,out IntPtr image);
 [DllImport(Library)]internal static extern void thinkthen_image_free(IntPtr image);
 [DllImport(Library)]internal static extern int thinkthen_source_records(IntPtr e,IntPtr data,nuint len,out IntPtr source);
 [DllImport(Library)]internal static extern int thinkthen_source_files(IntPtr e,ref SourceSpecV1 spec,out IntPtr source);
[DllImport(Library)]internal static extern int thinkthen_source_image_files(IntPtr e,ref SourceSpecV1 spec,out IntPtr source);
 [DllImport(Library)]internal static extern void thinkthen_source_free(IntPtr source);
 [DllImport(Library)]internal static extern int thinkthen_result_summary(IntPtr r,out SummaryV1 summary);
 [DllImport(Library)]internal static extern int thinkthen_result_observation(IntPtr r,nuint i,out ObservationV1 observation);
 [DllImport(Library)]internal static extern void thinkthen_result_free(IntPtr r);
 [DllImport(Library)]internal static extern int thinkthen_error_complete(IntPtr e,out IntPtr r);
 internal static Failure Failure(IntPtr e,int rc) {
  string message=Marshal.PtrToStringUTF8(Native.thinkthen_error_message(e))??"missing native failure";bool retry=Native.thinkthen_error_retryable(e)!=0;string? json=Marshal.PtrToStringUTF8(Native.thinkthen_error_facts_json(e));var failure=new Failure(rc,retry,message,json);
  if(thinkthen_error_complete(e,out var result)==0&&result!=IntPtr.Zero){try{if(thinkthen_result_summary(result,out var summary)==0&&summary.error.present!=0){var error=summary.error.value;failure.Complete=Optional.Some(new CompleteError((ulong)error.code,String(error.message),error.retryable!=0,Opt(error.stopped.present,()=>ReadStopped(error.stopped.value)),Opt(summary.facts.present,()=>ReadCallFacts(summary.facts.value)),Opt(summary.attempts.present,()=>Array<AttemptV1,Attempt>(summary.attempts.value.data,summary.attempts.value.len,ReadAttempt))));}}finally{thinkthen_result_free(result);}}
  return failure;
 }
 internal static void Check(IntPtr e,int rc){if(rc!=0)throw Failure(e,rc);}
 internal static CompleteCall<T> Copy<T>(IntPtr result,Func<IntPtr,nuint,T> read){if(thinkthen_result_summary(result,out var s)!=0)throw new InvalidOperationException("native summary failed");T[] rows=new T[checked((int)s.count)];for(int i=0;i<rows.Length;i++)rows[i]=read(result,(nuint)i);ObservationEvent[] events=new ObservationEvent[checked((int)s.observation_count)];for(int i=0;i<events.Length;i++){if(thinkthen_result_observation(result,(nuint)i,out var v)!=0)throw new InvalidOperationException("native observation failed");events[i]=Event(result,(nuint)i,ReadObservationEvent(v));}return new(String(s.schema),Opt(s.function.present,()=>(Function)(s.function.value-1)),Opt(s.answer_id.present,()=>new AnswerId(String(s.answer_id.value))),Opt(s.meta.present,()=>ReadMeta(s.meta.value)),Opt(s.facts.present,()=>ReadCallFacts(s.facts.value)),Opt(s.attempts.present,()=>Array<AttemptV1,Attempt>(s.attempts.value.data,s.attempts.value.len,ReadAttempt)),rows,events){Error=Opt(s.error.present,()=>new CompleteError((ulong)s.error.value.code,String(s.error.value.message),s.error.value.retryable!=0,Opt(s.error.value.stopped.present,()=>ReadStopped(s.error.value.stopped.value)),Opt(s.facts.present,()=>ReadCallFacts(s.facts.value)),Opt(s.attempts.present,()=>Array<AttemptV1,Attempt>(s.attempts.value.data,s.attempts.value.len,ReadAttempt))))};}
 internal delegate int Call(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr r);
}
public sealed partial class Engine:ICompleteEngine {
 CompleteCall<T> Complete<T>(QuestionInput q,InputSource source,CallControls controls,TimeSpan? budget,CancellationToken cancellation,NativeComplete.Call call,Func<IntPtr,nuint,T> read)=>Invoke(budget,cancellation,(e,deadline,token)=>{using var inputs=new NativeInputs();var question=inputs.Asked(e,q);var s=inputs.Input(e,source);var c=inputs.Controls(controls,deadline,token);int rc=call(e,question,s,ref c,out var result);NativeComplete.Check(e,rc);try{return NativeComplete.Copy(result,read);}finally{NativeComplete.thinkthen_result_free(result);}});
}
