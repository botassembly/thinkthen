package thinkthen;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.invoke.MethodHandle;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.Map;
import static java.lang.foreign.ValueLayout.*;

/** Java 21 preview FFM binding for the immutable ThinkThen C door. */
public final class Door implements AutoCloseable, CompleteEngine {
    public record Answer(int outcome, double probability) {}
    /** A call's value and its facts object; the result schema describes the facts. */
    public record TypedResult<T>(T value, Map<String, Object> facts) {}
    public enum Outcome { NO, YES, NOT_SURE }
    public enum FailureKind { USAGE, BACKEND, DEADLINE, LOCAL, CANCELLED, DEFECT }
    public record Failure(int code, FailureKind kind, boolean retryable, String message, String factsJson) {}
    /** One member of an annotate row's value or answers (ADR 0112 section 4). */
    public sealed interface AnnotatedField {
        /** JSON null: the question was not sure. */
        record Unresolved() implements AnnotatedField {}
        record Answered(Object value) implements AnnotatedField {}
        /** The one-member object {"failed": {...}}. */
        record Failed(String kind, String cause) implements AnnotatedField {}
    }
    /** Read one member as Json reads it. No answered value is an object, so an object that is not a failure is an error. */
    public static AnnotatedField field(Object member) {
        if (member == null) return new AnnotatedField.Unresolved();
        if (!(member instanceof Map<?, ?> object)) return new AnnotatedField.Answered(member);
        if (!(object.get("failed") instanceof Map<?, ?> failed))
            throw new IllegalArgumentException("annotate member is an object but not a failure");
        return new AnnotatedField.Failed(failed.get("kind") instanceof String kind ? kind : "",
            failed.get("cause") instanceof String cause ? cause : "");
    }
    public static Outcome outcome(Answer answer) {
        return switch (answer.outcome()) { case 0 -> Outcome.NO; case 1 -> Outcome.YES;
            case 2 -> Outcome.NOT_SURE; default -> throw new IllegalStateException("invalid native outcome"); };
    }
    public static final class NativeFailure extends RuntimeException {
        public final Failure failure;
        public final Complete.OptionalValue<Complete.CompleteError> complete;
        NativeFailure(Failure failure) { this(failure,Complete.OptionalValue.absent()); }
        NativeFailure(Failure failure,Complete.OptionalValue<Complete.CompleteError> complete) { super(failure.message()); this.failure = failure; this.complete=complete; }
    }
    private static final MemoryLayout ANSWER = MemoryLayout.structLayout(JAVA_INT.withName("outcome"), MemoryLayout.paddingLayout(4), JAVA_DOUBLE.withName("probability"));
    private static final Map<String, MethodHandle> CALLS = new HashMap<>();
    static {
        String path = System.getProperty("thinkthen.library");
        if (path == null || !Path.of(path).isAbsolute()) throw new IllegalArgumentException("-Dthinkthen.library requires an absolute path");
        SymbolLookup symbols = SymbolLookup.libraryLookup(Path.of(path), Arena.global());
        Linker linker = Linker.nativeLinker();
        register(linker, symbols, "thinkthen_engine_new", FunctionDescriptor.of(ADDRESS));
        register(linker, symbols, "thinkthen_engine_new_with", FunctionDescriptor.of(ADDRESS, ADDRESS));
        register(linker, symbols, "thinkthen_engine_free", FunctionDescriptor.ofVoid(ADDRESS));
        register(linker, symbols, "thinkthen_error_code", FunctionDescriptor.of(JAVA_INT, ADDRESS));
        register(linker, symbols, "thinkthen_error_retryable", FunctionDescriptor.of(JAVA_INT, ADDRESS));
        register(linker, symbols, "thinkthen_error_message", FunctionDescriptor.of(ADDRESS, ADDRESS));
        register(linker, symbols, "thinkthen_error_facts_json", FunctionDescriptor.of(ADDRESS, ADDRESS));
        register(linker, symbols, "thinkthen_cancel_token_new", FunctionDescriptor.of(ADDRESS));
        register(linker, symbols, "thinkthen_cancel_token_free", FunctionDescriptor.ofVoid(ADDRESS));
        register(linker, symbols, "thinkthen_cancel", FunctionDescriptor.ofVoid(ADDRESS));
        register(linker, symbols, "thinkthen_decide_opts", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, JAVA_LONG, JAVA_LONG, ADDRESS, ADDRESS));
        register(linker, symbols, "thinkthen_decide_with_facts_opts", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, JAVA_LONG, JAVA_LONG, ADDRESS, ADDRESS, ADDRESS, ADDRESS));
        register(linker, symbols, "thinkthen_decide_many_opts", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, ADDRESS, JAVA_LONG, JAVA_LONG, ADDRESS, ADDRESS));
        register(linker, symbols, "thinkthen_decide_many_with_facts_opts", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, ADDRESS, JAVA_LONG, JAVA_LONG, ADDRESS, ADDRESS, ADDRESS, ADDRESS));
        register(linker, symbols, "thinkthen_call_opts", FunctionDescriptor.of(ADDRESS, ADDRESS, ADDRESS, JAVA_LONG, ADDRESS));
        register(linker, symbols, "thinkthen_recognize_opts", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, JAVA_LONG, JAVA_LONG, ADDRESS, ADDRESS, ADDRESS));
        register(linker, symbols, "thinkthen_recognize_with_facts_opts", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, JAVA_LONG, JAVA_LONG, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS));
        register(linker, symbols, "thinkthen_relate_opts", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, ADDRESS, JAVA_LONG, JAVA_LONG, ADDRESS, ADDRESS, ADDRESS));
        register(linker, symbols, "thinkthen_relate_with_facts_opts", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, ADDRESS, JAVA_LONG, JAVA_LONG, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS));
        register(linker, symbols, "thinkthen_plan_json", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, ADDRESS));
        register(linker, symbols, "thinkthen_free_string", FunctionDescriptor.ofVoid(ADDRESS));
    }
    private static void register(Linker linker, SymbolLookup symbols, String name, FunctionDescriptor descriptor) {
        CALLS.put(name, linker.downcallHandle(symbols.find(name).orElseThrow(), descriptor));
    }
    static Object invoke(String name, Object... arguments) {
        try { return CALLS.get(name).invokeWithArguments(arguments); }
        catch (Throwable failure) { throw new IllegalStateException("native call " + name, failure); }
    }
    static boolean nullPointer(MemorySegment pointer) { return pointer.address() == 0; }
    static MemorySegment cstr(Arena arena, String value) {
        if (value.indexOf('\0') >= 0) throw new IllegalArgumentException("interior NUL in C string");
        return arena.allocateUtf8String(value);
    }
    static MemorySegment bytes(Arena arena, byte[] value) {
        MemorySegment result = arena.allocate(Math.max(1, value.length), 1);
        MemorySegment.copy(MemorySegment.ofArray(value), 0, result, 0, value.length);
        return result;
    }
    /** Uncounted C strings have a one-MiB scan limit; never return a truncated result. */
    static String copyCString(MemorySegment pointer) {
        try { return pointer.reinterpret(1 << 20).getUtf8String(0); }
        catch (IndexOutOfBoundsException tooLong) {
            throw new IllegalStateException("native C string has no terminator within one MiB", tooLong);
        }
    }
    static String copyCounted(MemorySegment pointer, long length) {
        if (nullPointer(pointer) || length < 0 || length > Integer.MAX_VALUE)
            throw new IllegalStateException("invalid native string length");
        return new String(pointer.reinterpret(length).toArray(JAVA_BYTE), StandardCharsets.UTF_8);
    }
    static void freeOutput(MemorySegment slot) {
        MemorySegment pointer = slot.get(ADDRESS, 0);
        if (!nullPointer(pointer)) invoke("thinkthen_free_string", pointer);
    }
    static Map<String, Object> copyObject(MemorySegment pointer, long length) {
        return Json.parseObject(copyCounted(pointer, length));
    }
    static Failure failure(MemorySegment engine) {
        int code = (int) invoke("thinkthen_error_code", engine);
        if (code < 1 || code > FailureKind.values().length) throw new IllegalStateException("invalid native failure kind");
        boolean retryable = (int) invoke("thinkthen_error_retryable", engine) != 0;
        String message = copyCString((MemorySegment) invoke("thinkthen_error_message", engine));
        MemorySegment facts = (MemorySegment) invoke("thinkthen_error_facts_json", engine);
        return new Failure(code, FailureKind.values()[code - 1], retryable, message,
            nullPointer(facts) ? null : copyCString(facts));
    }
    private MemorySegment engine;
    final String surface;
    public Door() { this(null); }
    public Door(String settingsJson) { this(settingsJson,"java"); }
    public static Door forKotlin(String settingsJson) {return new Door(settingsJson,"kotlin");}
    public static Door forScala(String settingsJson) {return new Door(settingsJson,"scala");}
    private Door(String settingsJson,String surface) {
        this.surface=surface;
        // Java 21 pins a virtual thread's carrier while this monitor is held.
        synchronized (Thread.currentThread()) {
            if (settingsJson == null) engine = (MemorySegment) invoke("thinkthen_engine_new");
            else try (Arena arena = Arena.ofConfined()) {
                engine = (MemorySegment) invoke("thinkthen_engine_new_with", cstr(arena, settingsJson));
            }
            if (nullPointer(engine)) throw new NativeFailure(failure(MemorySegment.NULL));
        }
    }
    synchronized MemorySegment live() {
        if (engine == null) throw new IllegalStateException("engine closed");
        return engine;
    }
    public final class Token implements AutoCloseable {
        private MemorySegment pointer;
        private Token() {
            pointer = (MemorySegment) invoke("thinkthen_cancel_token_new");
            if (nullPointer(pointer)) throw new IllegalStateException("native cancellation token allocation failed");
        }
        public synchronized void fire() { if (pointer != null) invoke("thinkthen_cancel", pointer); }
        private synchronized MemorySegment livePointer() { if (pointer == null) throw new IllegalStateException("token closed"); return pointer; }
        @Override public synchronized void close() {
            if (pointer != null) { invoke("thinkthen_cancel_token_free", pointer); pointer = null; }
        }
    }
    public Token token() { live(); return new Token(); }
    MemorySegment tokenPointer(Token token) { return token == null ? MemorySegment.NULL : token.livePointer(); }
    public TypedResult<Answer> decide(String question, byte[] evidence) { return decide(question, evidence, -1, null); }
    public TypedResult<Answer> decide(String question, byte[] evidence, long deadlineMs, Token token) {
        MemorySegment e = live();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment q = cstr(arena, question), text = bytes(arena, evidence), out = arena.allocate(ANSWER);
            MemorySegment facts = arena.allocate(ADDRESS), factsLength = arena.allocate(JAVA_LONG);
            if (token != null) { out.set(JAVA_INT, 0, 7); out.set(JAVA_DOUBLE, 8, 7.0); }
            synchronized (Thread.currentThread()) {
                int rc = (int) invoke("thinkthen_decide_with_facts_opts", e, q, text, (long) evidence.length, deadlineMs, tokenPointer(token), out, facts, factsLength);
                if (rc != 0) {
                    Failure error = failure(e);
                    if (rc == 5 && token != null && (out.get(JAVA_INT, 0) != 7 || out.get(JAVA_DOUBLE, 8) != 7.0))
                        throw new AssertionError("cancelled native scalar wrote output");
                    throw new NativeFailure(error);
                }
                try { return new TypedResult<>(new Answer(out.get(JAVA_INT, 0), out.get(JAVA_DOUBLE, 8)),
                    copyObject(facts.get(ADDRESS, 0), factsLength.get(JAVA_LONG, 0))); }
                finally { freeOutput(facts); }
            }
        }
    }
    public TypedResult<Answer[]> decideMany(String question, byte[][] evidence, long deadlineMs, Token token) {
        MemorySegment e = live();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment q = cstr(arena, question);
            int n = evidence.length;
            MemorySegment ptrs = arena.allocate(Math.max(1L, Math.multiplyExact((long)n, ADDRESS.byteSize())), ADDRESS.byteAlignment());
            MemorySegment lens = arena.allocate(Math.max(1L, Math.multiplyExact((long)n, JAVA_LONG.byteSize())), JAVA_LONG.byteAlignment());
            MemorySegment out = arena.allocate(Math.max(16L, Math.multiplyExact((long)n, ANSWER.byteSize())), ANSWER.byteAlignment());
            MemorySegment facts = arena.allocate(ADDRESS), factsLength = arena.allocate(JAVA_LONG);
            for (int i=0;i<n;i++) { ptrs.setAtIndex(ADDRESS,i,bytes(arena,evidence[i])); lens.setAtIndex(JAVA_LONG,i,evidence[i].length); }
            synchronized (Thread.currentThread()) {
                int rc = (int) invoke("thinkthen_decide_many_with_facts_opts",e,q,ptrs,lens,(long)n,deadlineMs,tokenPointer(token),out,facts,factsLength);
                if (rc != 0) throw new NativeFailure(failure(e));
                try {
                    Answer[] result = new Answer[n];
                    for (int i=0;i<n;i++) result[i]=new Answer(out.get(JAVA_INT,i*16L),out.get(JAVA_DOUBLE,i*16L+8));
                    return new TypedResult<>(result, copyObject(facts.get(ADDRESS,0),factsLength.get(JAVA_LONG,0)));
                } finally { freeOutput(facts); }
            }
        }
    }
    /** Explicit reader for all ten verbs; returns the native value/facts envelope. */
    public String files(String question, java.util.List<String> paths, String unit, Integer window,
                        long deadlineMs, Token token) {
        var members = Json.parseObject(question);
        if (members.containsKey("source")) throw new IllegalArgumentException("source is supplied by files");
        StringBuilder source = new StringBuilder("{\"paths\":[");
        for (int i = 0; i < paths.size(); i++) {
            if (i > 0) source.append(',');
            source.append(Json.quote(paths.get(i)));
        }
        source.append("],\"unit\":").append(Json.quote(unit));
        if (window != null) source.append(",\"window\":").append(window);
        source.append('}');
        String held = question.strip();
        return call(held.substring(0, held.length()-1) + (members.isEmpty() ? "" : ",") +
                    "\"source\":" + source + "}", deadlineMs, token);
    }
    public String files(String question, java.util.List<String> paths) {
        return files(question, paths, "line", null, -1, null);
    }
    public String call(String request) { return call(request,-1,null); }
    public String call(String request,long deadlineMs,Token token) {
        MemorySegment e = live();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment req = cstr(arena,request);
            synchronized (Thread.currentThread()) {
                MemorySegment result = (MemorySegment) invoke("thinkthen_call_opts",e,req,deadlineMs,tokenPointer(token));
                if (nullPointer(result)) throw new NativeFailure(failure(e));
                try { return copyCString(result); } finally { invoke("thinkthen_free_string",result); }
            }
        }
    }
    /**
     * Preview a judgment call through thinkthen_plan_json and return the result schema's plan
     * object. verb is decide, choose, score or tag. question is bare question text, or one
     * question object when it starts with "{", as decide reads it. settingsJson is null or a
     * thinkthen.settings/1 object. The preview needs no key, reads no cache and sends nothing.
     */
    public Map<String, Object> plan(String verb, String question, String[] input, String settingsJson) {
        StringBuilder json = new StringBuilder("{\"verb\":").append(Json.quote(verb)).append(",\"question\":")
            .append(question.strip().startsWith("{") ? question : Json.quote(question)).append(",\"input\":[");
        for (int i = 0; i < input.length; i++) json.append(i == 0 ? "" : ",").append(Json.quote(input[i]));
        json.append(']');
        if (settingsJson != null) json.append(",\"settings\":").append(settingsJson);
        MemorySegment e = live();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment request = cstr(arena, json.append('}').toString()), out = arena.allocate(ADDRESS), len = arena.allocate(JAVA_LONG);
            synchronized (Thread.currentThread()) {
                if ((int) invoke("thinkthen_plan_json", e, request, out, len) != 0) throw new NativeFailure(failure(e));
                try { return copyObject(out.get(ADDRESS, 0), len.get(JAVA_LONG, 0)); } finally { freeOutput(out); }
            }
        }
    }
    public TypedResult<Map<String, Object>> recognize(String spec, byte[] text) { return recognize(spec, text, -1, null); }
    public TypedResult<Map<String, Object>> recognize(String spec, byte[] text, long deadlineMs, Token token) {
        return structured("thinkthen_recognize_with_facts_opts", spec, new byte[][]{text}, deadlineMs, token);
    }
    public TypedResult<Map<String, Object>> relate(String spec, byte[][] records) { return relate(spec, records, -1, null); }
    public TypedResult<Map<String, Object>> relate(String spec, byte[][] records, long deadlineMs, Token token) {
        return structured("thinkthen_relate_with_facts_opts", spec, records, deadlineMs, token);
    }
    private TypedResult<Map<String, Object>> structured(String fn, String spec, byte[][] values, long deadlineMs, Token token) {
        MemorySegment e = live();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment q=cstr(arena,spec), out=arena.allocate(ADDRESS), len=arena.allocate(JAVA_LONG);
            MemorySegment facts=arena.allocate(ADDRESS), factsLen=arena.allocate(JAVA_LONG);
            MemorySegment text;
            Object[] args;
            if (fn.equals("thinkthen_recognize_with_facts_opts")) {
                text=bytes(arena,values[0]);
                args=new Object[]{e,q,text,(long)values[0].length,deadlineMs,tokenPointer(token),out,len,facts,factsLen};
            } else {
                int n=values.length;
                MemorySegment ptrs=arena.allocate(Math.max(1L,Math.multiplyExact((long)n,ADDRESS.byteSize())),ADDRESS.byteAlignment());
                MemorySegment lens=arena.allocate(Math.max(1L,Math.multiplyExact((long)n,JAVA_LONG.byteSize())),JAVA_LONG.byteAlignment());
                for(int i=0;i<n;i++){ptrs.setAtIndex(ADDRESS,i,bytes(arena,values[i]));lens.setAtIndex(JAVA_LONG,i,values[i].length);}
                args=new Object[]{e,q,ptrs,lens,(long)n,deadlineMs,tokenPointer(token),out,len,facts,factsLen};
            }
            synchronized (Thread.currentThread()) {
                int rc=(int)invoke(fn,args);
                if(rc!=0)throw new NativeFailure(failure(e));
                try {
                    return new TypedResult<>(copyObject(out.get(ADDRESS,0),len.get(JAVA_LONG,0)),copyObject(facts.get(ADDRESS,0),factsLen.get(JAVA_LONG,0)));
                } finally {try {freeOutput(out);} finally {freeOutput(facts);}}
            }
        }
    }
    @Override public synchronized void close() {
        if (engine!=null) { invoke("thinkthen_engine_free",engine); engine=null; }
    }
    public synchronized CompleteEngine.CompleteCall<Complete.DecideRow> decideComplete(Requests.QuestionInput q, Requests.InputSource source, Complete.CallControls controls, long deadlineMs, Token token) {
        synchronized(Thread.currentThread()) {return NativeExecution.execute(live(),q,source,controls,deadlineMs,tokenPointer(token),surface,"decide",NativeDetails::decide);}
    }

    public synchronized CompleteEngine.CompleteCall<Complete.ChooseRow> chooseComplete(Requests.QuestionInput q, Requests.InputSource source, Complete.CallControls controls, long deadlineMs, Token token) {
        synchronized(Thread.currentThread()) {return NativeExecution.execute(live(),q,source,controls,deadlineMs,tokenPointer(token),surface,"choose",NativeDetails::choose);}
    }

    public synchronized CompleteEngine.CompleteCall<Complete.TagRow> tagComplete(Requests.QuestionInput q, Requests.InputSource source, Complete.CallControls controls, long deadlineMs, Token token) {
        synchronized(Thread.currentThread()) {return NativeExecution.execute(live(),q,source,controls,deadlineMs,tokenPointer(token),surface,"tag",NativeDetails::tag);}
    }

    public synchronized CompleteEngine.CompleteCall<Complete.ScoreRow> scoreComplete(Requests.QuestionInput q, Requests.InputSource source, Complete.CallControls controls, long deadlineMs, Token token) {
        synchronized(Thread.currentThread()) {return NativeExecution.execute(live(),q,source,controls,deadlineMs,tokenPointer(token),surface,"score",NativeDetails::score);}
    }

    public synchronized CompleteEngine.CompleteCall<Complete.FilterRow> filterComplete(Requests.QuestionInput q, Requests.InputSource source, Complete.CallControls controls, long deadlineMs, Token token) {
        synchronized(Thread.currentThread()) {return NativeExecution.execute(live(),q,source,controls,deadlineMs,tokenPointer(token),surface,"filter",NativeDetails::filter);}
    }

    public synchronized CompleteEngine.CompleteCall<Complete.RankRow> rankComplete(Requests.QuestionInput q, Requests.InputSource source, Complete.CallControls controls, long deadlineMs, Token token) {
        synchronized(Thread.currentThread()) {return NativeExecution.execute(live(),q,source,controls,deadlineMs,tokenPointer(token),surface,"rank",NativeDetails::rank);}
    }

    public synchronized CompleteEngine.CompleteCall<Complete.FindRow> findComplete(Requests.QuestionInput q, Requests.InputSource source, Complete.CallControls controls, long deadlineMs, Token token) {
        synchronized(Thread.currentThread()) {return NativeExecution.execute(live(),q,source,controls,deadlineMs,tokenPointer(token),surface,"find",NativeDetails::find);}
    }

    public synchronized CompleteEngine.CompleteCall<Complete.AnnotateRow> annotateComplete(Requests.QuestionInput q, Requests.InputSource source, Complete.CallControls controls, long deadlineMs, Token token) {
        synchronized(Thread.currentThread()) {return NativeExecution.execute(live(),q,source,controls,deadlineMs,tokenPointer(token),surface,"annotate",NativeDetails::annotate);}
    }

    public synchronized CompleteEngine.CompleteCall<Complete.RecognizeRow> recognizeComplete(Requests.QuestionInput q, Requests.InputSource source, Complete.CallControls controls, long deadlineMs, Token token) {
        synchronized(Thread.currentThread()) {return NativeExecution.execute(live(),q,source,controls,deadlineMs,tokenPointer(token),surface,"recognize",NativeDetails::recognize);}
    }

    public synchronized CompleteEngine.CompleteCall<Complete.RelateRow> relateComplete(Requests.QuestionInput q, Requests.InputSource source, Complete.CallControls controls, long deadlineMs, Token token) {
        synchronized(Thread.currentThread()) {return NativeExecution.execute(live(),q,source,controls,deadlineMs,tokenPointer(token),surface,"relate",NativeDetails::relate);}
    }

    public NativeBatch<Complete.DecideRow> decideBatch(Requests.QuestionInput q,Requests.InputSource s,Complete.CallControls c,long deadline,Token token){return NativeBatch.start(this,q,s,c,deadline,token,"decide",NativeDetails::decide);}

    public NativeBatch<Complete.ChooseRow> chooseBatch(Requests.QuestionInput q,Requests.InputSource s,Complete.CallControls c,long deadline,Token token){return NativeBatch.start(this,q,s,c,deadline,token,"choose",NativeDetails::choose);}

    public NativeBatch<Complete.TagRow> tagBatch(Requests.QuestionInput q,Requests.InputSource s,Complete.CallControls c,long deadline,Token token){return NativeBatch.start(this,q,s,c,deadline,token,"tag",NativeDetails::tag);}

    public NativeBatch<Complete.ScoreRow> scoreBatch(Requests.QuestionInput q,Requests.InputSource s,Complete.CallControls c,long deadline,Token token){return NativeBatch.start(this,q,s,c,deadline,token,"score",NativeDetails::score);}

    public NativeBatch<Complete.FilterRow> filterBatch(Requests.QuestionInput q,Requests.InputSource s,Complete.CallControls c,long deadline,Token token){return NativeBatch.start(this,q,s,c,deadline,token,"filter",NativeDetails::filter);}

    public NativeBatch<Complete.AnnotateRow> annotateBatch(Requests.QuestionInput q,Requests.InputSource s,Complete.CallControls c,long deadline,Token token){return NativeBatch.start(this,q,s,c,deadline,token,"annotate",NativeDetails::annotate);}

}
