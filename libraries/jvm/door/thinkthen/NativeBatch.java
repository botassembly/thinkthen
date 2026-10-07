package thinkthen;
import java.lang.foreign.*;
import java.util.concurrent.*;
import thinkthen.Complete.*;
import thinkthen.Requests.*;
import static java.lang.foreign.ValueLayout.*;
import static thinkthen.NativeCalls.*;
/** Native lazy batch. Close before its engine; copied rows outlive either owner. */
public final class NativeBatch<T> implements AutoCloseable {
 private record Command<T>(String operation,CompletableFuture<CompleteEngine.CompleteCall<T>> reply) {}
 private final BlockingQueue<Command<T>> commands=new LinkedBlockingQueue<>();private final Thread worker;private boolean closed;
 private NativeBatch(Door engine,QuestionInput q,InputSource source,CallControls controls,long deadline,Door.Token token,String verb,NativeExecution.RowReader<T> read){
  CompletableFuture<Void> ready=new CompletableFuture<>();worker=Thread.ofPlatform().daemon().start(()->{
   synchronized(engine){synchronized(Thread.currentThread()){
    try(NativeInputs inputs=new NativeInputs()){
     MemorySegment e=engine.live(),question=inputs.asked(e,q),input=inputs.input(e,source),c=inputs.controls(controls,deadline,engine.tokenPointer(token),engine.surface),out=inputs.arena.allocate(ADDRESS);
     check(e,(int)call("thinkthen_"+verb+"_batch_start",e,question,input,c,out));MemorySegment batch=out.get(ADDRESS,0);
     try{ready.complete(null);while(true){Command<T> command=commands.take();if(command.operation().equals("close")){call("thinkthen_batch_free",batch);batch=MemorySegment.NULL;command.reply().complete(null);break;}try{MemorySegment resultSlot=out;out.set(ADDRESS,0,MemorySegment.NULL);check(e,(int)call(command.operation().equals("next")?"thinkthen_batch_next":"thinkthen_batch_facts",batch,resultSlot));MemorySegment result=resultSlot.get(ADDRESS,0);if(result.address()==0)command.reply().complete(null);else try{command.reply().complete(NativeExecution.copy(result,verb,read));}finally{call("thinkthen_result_free",result);}}catch(Throwable failure){command.reply().completeExceptionally(failure);}}}finally{call("thinkthen_batch_free",batch);}
    }catch(Throwable failure){ready.completeExceptionally(failure);}
   }}
  });await(ready);
 }
 private static <R> R await(CompletableFuture<R> future){try{return future.join();}catch(CompletionException e){if(e.getCause() instanceof RuntimeException cause)throw cause;throw e;}}
 static <T> NativeBatch<T> start(Door e,QuestionInput q,InputSource s,CallControls c,long deadline,Door.Token token,String verb,NativeExecution.RowReader<T> read){return new NativeBatch<>(e,q,s,c,deadline,token,verb,read);}
 private synchronized CompleteEngine.CompleteCall<T> send(String operation){if(closed)throw new IllegalStateException("batch closed");CompletableFuture<CompleteEngine.CompleteCall<T>> reply=new CompletableFuture<>();commands.add(new Command<>(operation,reply));return await(reply);}
 /** Returns null at the end; a failure keeps the prefix already returned. */
 public CompleteEngine.CompleteCall<T> next(){return send("next");}
 public CompleteEngine.CompleteCall<T> facts(){return send("facts");}
 @Override public synchronized void close(){if(closed)return;try{send("close");}finally{closed=true;boolean interrupted=false;while(worker.isAlive())try{worker.join();}catch(InterruptedException e){interrupted=true;}if(interrupted)Thread.currentThread().interrupt();}}
}
