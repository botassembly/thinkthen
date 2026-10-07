package thinkthen;
import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.nio.file.Path;
import java.util.*;
import static java.lang.foreign.ValueLayout.*;
import static thinkthen.NativeLayouts.*;
final class NativeCalls {
 private static final Map<String,MethodHandle> CALLS=new HashMap<>();
 static {
  SymbolLookup symbols=SymbolLookup.libraryLookup(Path.of(System.getProperty("thinkthen.library")),Arena.global());Linker linker=Linker.nativeLinker();
  add(linker,symbols,"thinkthen_question_new",FunctionDescriptor.of(JAVA_INT,ADDRESS,ADDRESS,ADDRESS));
  add(linker,symbols,"thinkthen_question_new_authored",FunctionDescriptor.of(JAVA_INT,ADDRESS,ADDRESS,ADDRESS,ADDRESS));
  add(linker,symbols,"thinkthen_result_member_author",FunctionDescriptor.of(JAVA_INT,ADDRESS,JAVA_LONG,JAVA_LONG,ADDRESS));
  add(linker,symbols,"thinkthen_result_rank_member_count",FunctionDescriptor.of(JAVA_INT,ADDRESS,JAVA_LONG,ADDRESS));
  add(linker,symbols,"thinkthen_result_rank_member_details",FunctionDescriptor.of(JAVA_INT,ADDRESS,JAVA_LONG,JAVA_LONG,ADDRESS));
  add(linker,symbols,"thinkthen_result_rank_member",FunctionDescriptor.of(JAVA_INT,ADDRESS,JAVA_LONG,JAVA_LONG,ADDRESS));
  for(String n:List.of("thinkthen_question_load"))add(linker,symbols,n,FunctionDescriptor.of(JAVA_INT,ADDRESS,layout("thinkthen_string_v1"),ADDRESS));
  for(String n:List.of("thinkthen_question_parse","thinkthen_question_load_named","thinkthen_question_load_reference"))add(linker,symbols,n,FunctionDescriptor.of(JAVA_INT,ADDRESS,JAVA_INT,layout("thinkthen_string_v1"),ADDRESS));
  add(linker,symbols,"thinkthen_image_clone",FunctionDescriptor.of(JAVA_INT,ADDRESS,ADDRESS,JAVA_LONG,JAVA_INT,layout("thinkthen_optional_string_v1"),ADDRESS));
  add(linker,symbols,"thinkthen_source_records",FunctionDescriptor.of(JAVA_INT,ADDRESS,ADDRESS,JAVA_LONG,ADDRESS));
  add(linker,symbols,"thinkthen_source_files",FunctionDescriptor.of(JAVA_INT,ADDRESS,ADDRESS,ADDRESS));
  add(linker,symbols,"thinkthen_source_image_files",FunctionDescriptor.of(JAVA_INT,ADDRESS,ADDRESS,ADDRESS));
  for(String n:List.of("question","image","source","result","batch"))add(linker,symbols,"thinkthen_"+n+"_free",FunctionDescriptor.ofVoid(ADDRESS));
  add(linker,symbols,"thinkthen_result_summary",FunctionDescriptor.of(JAVA_INT,ADDRESS,ADDRESS));
  add(linker,symbols,"thinkthen_error_complete",FunctionDescriptor.of(JAVA_INT,ADDRESS,ADDRESS));
  for(String n:List.of("decide","choose","tag","score","filter","annotate"))add(linker,symbols,"thinkthen_"+n+"_batch_start",FunctionDescriptor.of(JAVA_INT,ADDRESS,ADDRESS,ADDRESS,ADDRESS,ADDRESS));
  for(String n:List.of("thinkthen_batch_next","thinkthen_batch_facts"))add(linker,symbols,n,FunctionDescriptor.of(JAVA_INT,ADDRESS,ADDRESS));
  for(String n:List.of("decide","choose","tag","score","filter","rank","find","annotate","recognize","relate")){
   add(linker,symbols,"thinkthen_"+n+"_complete",FunctionDescriptor.of(JAVA_INT,ADDRESS,ADDRESS,ADDRESS,ADDRESS,ADDRESS));
   add(linker,symbols,"thinkthen_result_"+n,FunctionDescriptor.of(JAVA_INT,ADDRESS,JAVA_LONG,ADDRESS));
  }
  for(String n:List.of("observation","details","observation_details","question_author","observation_author","source_recognition","source_relations"))add(linker,symbols,"thinkthen_result_"+n,FunctionDescriptor.of(JAVA_INT,ADDRESS,JAVA_LONG,ADDRESS));
 }
 private static void add(Linker linker,SymbolLookup symbols,String n,FunctionDescriptor d){CALLS.put(n,linker.downcallHandle(symbols.find(n).orElseThrow(),d));}
 static Object call(String n,Object... args){try{return CALLS.get(n).invokeWithArguments(args);}catch(Throwable e){throw new IllegalStateException("native call "+n,e);}}
 static void check(MemorySegment e,int code){if(code!=0)throw NativeExecution.failure(e);}
}
