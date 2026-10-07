package thinkthen;
import java.lang.foreign.*;
import java.util.*;
import java.util.function.Function;
import thinkthen.Complete.*;
import thinkthen.Ids.*;
import thinkthen.Requests.*;
import static java.lang.foreign.ValueLayout.*;
import static thinkthen.NativeLayouts.*;
import static thinkthen.NativeRead.*;
import static thinkthen.NativeReaders0.*;
import static thinkthen.NativeReaders1.*;
import static thinkthen.NativeReaders2.*;
import static thinkthen.NativeCalls.*;
final class NativeExecution {
 interface RowReader<T>{T read(MemorySegment result,long i,MemorySegment v);}
 static <T> CompleteEngine.CompleteCall<T> execute(MemorySegment engine,QuestionInput q,InputSource source,CallControls controls,long deadline,MemorySegment token,String surface,String verb,RowReader<T> read){
  try(NativeInputs inputs=new NativeInputs()){
   MemorySegment question=inputs.asked(engine,q),input=inputs.input(engine,source),c=inputs.controls(controls,deadline,token,surface),out=inputs.arena.allocate(ADDRESS);
   check(engine,(int)call("thinkthen_"+verb+"_complete",engine,question,input,c,out));MemorySegment result=out.get(ADDRESS,0);
   try{return copy(result,verb,read);}finally{call("thinkthen_result_free",result);}
  }
 }
 static <T> CompleteEngine.CompleteCall<T> copy(MemorySegment result,String verb,RowReader<T> read){try(Arena arena=Arena.ofConfined()){
  MemorySegment s=arena.allocate(layout("thinkthen_summary_v1"));if((int)call("thinkthen_result_summary",result,s)!=0)throw new IllegalStateException("native summary failed");List<T> rows=new ArrayList<>();long count=l(f(s,"summary","count"));
  for(long i=0;i<count;i++){MemorySegment row=arena.allocate(layout("thinkthen_"+verb+"_view_v1"));if((int)call("thinkthen_result_"+verb,result,i,row)!=0)throw new IllegalStateException("native row failed");rows.add(read.read(result,i,row));}
  List<ObservationEvent> events=new ArrayList<>();long n=l(f(s,"summary","observation_count"));for(long i=0;i<n;i++){MemorySegment event=arena.allocate(layout("thinkthen_observation_v1"));if((int)call("thinkthen_result_observation",result,i,event)!=0)throw new IllegalStateException("native observation failed");events.add(NativeDetails.event(result,i,readObservationEvent(event)));}
  return new CompleteEngine.CompleteCall<>(string(f(s,"summary","schema")),optional(f(s,"summary","function"),"thinkthen_optional_discriminator_v1",v->enumeration(Complete.Function.values(),i(v),1)),optional(f(s,"summary","answer_id"),"thinkthen_optional_string_v1",v->new AnswerId(string(v))),optional(f(s,"summary","meta"),"thinkthen_optional_meta_v1",NativeReaders1::readMeta),optional(f(s,"summary","facts"),"thinkthen_optional_facts_v1",NativeReaders1::readCallFacts),optional(f(s,"summary","attempts"),"thinkthen_optional_attempts_v1",v->array(v,"thinkthen_attempts_v1","thinkthen_attempt_v1",NativeReaders1::readAttempt)),List.copyOf(rows),List.copyOf(events),error(s));
 }}
 static OptionalValue<CompleteError> error(MemorySegment s){return optional(f(s,"summary","error"),"thinkthen_optional_error_v1",v->new CompleteError((long)i(f(v,"error","code")),string(f(v,"error","message")),i(f(v,"error","retryable"))!=0,optional(f(v,"error","stopped"),"thinkthen_optional_stopped_v1",NativeReaders1::readStopped),optional(f(s,"summary","facts"),"thinkthen_optional_facts_v1",NativeReaders1::readCallFacts),optional(f(s,"summary","attempts"),"thinkthen_optional_attempts_v1",a->array(a,"thinkthen_attempts_v1","thinkthen_attempt_v1",NativeReaders1::readAttempt))));}
 static Door.NativeFailure failure(MemorySegment e){Door.Failure old=Door.failure(e);OptionalValue<CompleteError> complete=OptionalValue.absent();try(Arena a=Arena.ofConfined()){MemorySegment out=a.allocate(ADDRESS);if((int)call("thinkthen_error_complete",e,out)==0){MemorySegment result=out.get(ADDRESS,0);if(result.address()!=0){try{MemorySegment summary=a.allocate(layout("thinkthen_summary_v1"));if((int)call("thinkthen_result_summary",result,summary)==0)complete=error(summary);}finally{call("thinkthen_result_free",result);}}}}return new Door.NativeFailure(old,complete);}

}
