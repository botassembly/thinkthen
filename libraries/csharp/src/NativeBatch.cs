using System.Collections.Concurrent;
using System.Runtime.InteropServices;
namespace ThinkThen;
/// <summary>Owns native workers and retains its engine until Dispose. Returned rows own all views.</summary>
public sealed class CompleteBatch<T>:IDisposable {
 internal sealed record Command(string Operation,TaskCompletionSource<CompleteCall<T>?> Reply);
 readonly BlockingCollection<Command> commands=new();readonly Thread worker;readonly object gate=new();bool closed;
 internal CompleteBatch(Action<BlockingCollection<Command>,TaskCompletionSource<Exception?>> run){var ready=new TaskCompletionSource<Exception?>();worker=new Thread(()=>run(commands,ready)){IsBackground=true};worker.Start();var failure=ready.Task.GetAwaiter().GetResult();if(failure!=null){worker.Join();commands.Dispose();throw failure;}}
 CompleteCall<T>? Send(string operation){lock(gate){if(closed)throw new ObjectDisposedException(nameof(CompleteBatch<T>));var reply=new TaskCompletionSource<CompleteCall<T>?>();commands.Add(new(operation,reply));return reply.Task.GetAwaiter().GetResult();}}
 public CompleteCall<T>? Next()=>Send("next");
 public CompleteCall<T>? Facts()=>Send("facts");
 public void Dispose(){lock(gate){if(closed)return;try{Send("close");}finally{closed=true;worker.Join();commands.Dispose();}}}
}
internal static partial class NativeComplete {
 internal delegate int Start(IntPtr e,IntPtr q,IntPtr s,ref ControlsV1 c,out IntPtr b);
 [DllImport(Library)]internal static extern int thinkthen_batch_next(IntPtr b,out IntPtr r);
 [DllImport(Library)]internal static extern int thinkthen_batch_facts(IntPtr b,out IntPtr r);
 [DllImport(Library)]internal static extern void thinkthen_batch_free(IntPtr b);
}
public sealed partial class Engine {
 CompleteBatch<T> Batch<T>(QuestionInput q,InputSource source,CallControls controls,TimeSpan? budget,CancellationToken cancellation,NativeComplete.Start start,Func<IntPtr,nuint,T> read)=>new((commands,ready)=>{
  try {Invoke(budget,cancellation,(e,deadline,token)=>{using var inputs=new NativeInputs();var question=inputs.Asked(e,q);var s=inputs.Input(e,source);var c=inputs.Controls(controls,deadline,token);NativeComplete.Check(e,start(e,question,s,ref c,out var batch));try{ready.SetResult(null);foreach(var command in commands.GetConsumingEnumerable()){if(command.Operation=="close"){NativeComplete.thinkthen_batch_free(batch);batch=IntPtr.Zero;command.Reply.SetResult(null);break;}try{IntPtr result;int rc=command.Operation=="next"?NativeComplete.thinkthen_batch_next(batch,out result):NativeComplete.thinkthen_batch_facts(batch,out result);NativeComplete.Check(e,rc);try{command.Reply.SetResult(result==IntPtr.Zero?null:NativeComplete.Copy(result,read));}finally{NativeComplete.thinkthen_result_free(result);}}catch(Exception failure){command.Reply.SetException(failure);}}}finally{NativeComplete.thinkthen_batch_free(batch);}return 0;});}
  catch(Exception failure){ready.TrySetResult(failure);}
 });
}
