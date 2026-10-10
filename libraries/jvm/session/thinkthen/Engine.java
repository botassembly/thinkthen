package thinkthen;

import java.lang.foreign.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicReference;

/** Ten named asynchronous calls over one shared owned native session boundary. */
public final class Engine implements AutoCloseable {
    /** Outer JVM facade identity; native code validates the corresponding token. */
    public enum Surface { JAVA, KOTLIN, SCALA }
    private final Surface surface;
    private MemorySegment pointer;
    private static final ScheduledExecutorService POLLER = Executors.newScheduledThreadPool(2, runnable -> {
        Thread thread = new Thread(runnable, "thinkthen-session"); thread.setDaemon(true); return thread;
    });
    public Engine(Inputs.EngineSettings settings) { this(settings, Surface.JAVA); }
    public Engine(Inputs.EngineSettings settings, Surface surface) { this(Values.object(settings.json()), surface); }
    public OwnedSession startSession(Inputs.Request request) { return startSession(Values.object(request.json())); }
    public Results.Plan plan(Inputs.Request request) { return plan(Values.object(request.json())); }
    public CompletableFuture<OwnedCall> execute(Inputs.Request request) { return execute(Values.object(request.json())); }
    public Engine(Map<String,?> settings) { this(settings, Surface.JAVA); }
    /** Shared transport construction for the Kotlin and Scala facade packages. */
    public Engine(Map<String,?> settings, Surface surface) {
        this.surface = Objects.requireNonNull(surface);
        try (Arena arguments = Arena.ofConfined()) {
            byte[] bytes = NativeSession.utf8(Json.write(settings));
            var text = arguments.allocate(bytes.length + 1L);
            MemorySegment.copy(bytes, 0, text, ValueLayout.JAVA_BYTE, 0, bytes.length);
            pointer = (MemorySegment)NativeSession.call("thinkthen_engine_new_with", ValueLayout.ADDRESS,
                new MemoryLayout[]{ValueLayout.ADDRESS}, text);
            if (pointer.equals(MemorySegment.NULL)) throw engineFailure();
        }
    }
    public Engine() { this(new Inputs.EngineSettings()); }
    private NativeFailure engineFailure() {
        var args = new MemoryLayout[]{ValueLayout.ADDRESS};
        int code = (int)NativeSession.call("thinkthen_error_code", ValueLayout.JAVA_INT, args, pointer);
        boolean retryable = (int)NativeSession.call("thinkthen_error_retryable", ValueLayout.JAVA_INT, args, pointer) != 0;
        var text = (MemorySegment)NativeSession.call("thinkthen_error_message", ValueLayout.ADDRESS, args, pointer);
        var facts = (MemorySegment)NativeSession.call("thinkthen_error_facts_json", ValueLayout.ADDRESS, args, pointer);
        return new NativeFailure(code, retryable, text.reinterpret(Long.MAX_VALUE).getString(0),
            facts.equals(MemorySegment.NULL) ? null : Json.parse(facts.reinterpret(Long.MAX_VALUE).getString(0)));
    }
    private void live() { if (pointer.equals(MemorySegment.NULL)) throw new IllegalStateException("Engine is closed"); }
    public synchronized OwnedSession startSession(Map<String,?> request) {
        live();
        try (Arena arguments = Arena.ofConfined()) {
            var bytes = NativeSession.bytes(arguments, request);
            var output = arguments.allocate(ValueLayout.ADDRESS);
            var token = arguments.allocateFrom(ValueLayout.JAVA_BYTE, NativeSession.utf8(surface.name().toLowerCase(Locale.ROOT)));
            NativeSession.check((int)NativeSession.call("thinkthen_session_new_with_surface", ValueLayout.JAVA_INT,
                new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS, NativeSession.SIZE, ValueLayout.ADDRESS, NativeSession.SIZE, ValueLayout.ADDRESS}, pointer, bytes, bytes.byteSize(), token, token.byteSize(), output));
            return new OwnedSession(output.get(ValueLayout.ADDRESS, 0));
        }
    }
    public synchronized Results.Plan plan(Map<String,?> request) {
        live();
        try (Arena arguments = Arena.ofConfined()) {
            var bytes = NativeSession.bytes(arguments, request);
            var output = arguments.allocate(ValueLayout.ADDRESS);
            var length = arguments.allocate(NativeSession.SIZE);
            NativeSession.check((int)NativeSession.call("thinkthen_request_plan_json", ValueLayout.JAVA_INT,
                new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS, NativeSession.SIZE, ValueLayout.ADDRESS, ValueLayout.ADDRESS}, pointer, bytes, bytes.byteSize(), output, length));
            var json = output.get(ValueLayout.ADDRESS, 0);
            try { return new Results.Plan(NativeSession.copied(json, length.get(NativeSession.SIZE, 0))); }
            finally { NativeSession.free("thinkthen_free_string", json); }
        }
    }
    public record OwnedCall(List<Results.SessionPacket> packets, Results.SessionPacketTerminal terminal) {
        public OwnedCall { packets = List.copyOf(packets); Objects.requireNonNull(terminal); }
    }
    public static final class SessionFailure extends RuntimeException {
        private final OwnedCall call;
        SessionFailure(OwnedCall call) { super(call.terminal().failure().value().error().message()); this.call = call; }
        public OwnedCall call() { return call; }
        public Results.CallError failure() { return call.terminal().failure().value(); }
    }
    /** Future cancellation stops the native session and releases its owner promptly. */
    public CompletableFuture<OwnedCall> execute(Map<String,?> request) {
        var result = new CompletableFuture<OwnedCall>();
        final OwnedSession session;
        try {
            session = startSession(request);
            try { session.finish(); } catch (RuntimeException error) { session.close(); throw error; }
        }
        catch (RuntimeException error) { result.completeExceptionally(error); return result; }
        var scheduled = new AtomicReference<ScheduledFuture<?>>();
        result.whenComplete((value,error) -> {
            var pending = scheduled.get(); if (pending != null) pending.cancel(false);
            session.close();
        });
        var packets = new ArrayList<Results.SessionPacket>();
        var terminal = new AtomicReference<Results.SessionPacketTerminal>();
        Runnable read = () -> {
            if (result.isDone()) return;
            try {
                // One bounded native read per turn lets other sessions progress.
                var next = session.tryRead();
                if (next.packet() != null) {
                    packets.add(next.packet());
                    if (next.packet() instanceof Results.SessionPacketTerminal done) terminal.set(done);
                }
                if (next.ended()) {
                    var call = new OwnedCall(packets, terminal.get());
                    if (call.terminal().failure().state() == Presence.State.VALUE) result.completeExceptionally(new SessionFailure(call));
                    else result.complete(call);
                }
            } catch (RuntimeException error) { result.completeExceptionally(error); }
        };
        scheduled.set(POLLER.scheduleWithFixedDelay(read, 0, 10, TimeUnit.MILLISECONDS));
        if (result.isDone()) scheduled.get().cancel(false);
        return result;
    }
    public CompletableFuture<OwnedCall> decide(Inputs.RequestQuestion question, Inputs.RequestInput input) { return decide(question, input, null); }
    public CompletableFuture<OwnedCall> decide(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call("decide", Values.object(question.json()), Values.object(input.json()), options == null ? null : Values.object(options.json())); }
    public CompletableFuture<OwnedCall> choose(Inputs.RequestQuestion question, Inputs.RequestInput input) { return choose(question, input, null); }
    public CompletableFuture<OwnedCall> choose(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call("choose", Values.object(question.json()), Values.object(input.json()), options == null ? null : Values.object(options.json())); }
    public CompletableFuture<OwnedCall> tag(Inputs.RequestQuestion question, Inputs.RequestInput input) { return tag(question, input, null); }
    public CompletableFuture<OwnedCall> tag(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call("tag", Values.object(question.json()), Values.object(input.json()), options == null ? null : Values.object(options.json())); }
    public CompletableFuture<OwnedCall> score(Inputs.RequestQuestion question, Inputs.RequestInput input) { return score(question, input, null); }
    public CompletableFuture<OwnedCall> score(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call("score", Values.object(question.json()), Values.object(input.json()), options == null ? null : Values.object(options.json())); }
    public CompletableFuture<OwnedCall> filter(Inputs.RequestQuestion question, Inputs.RequestInput input) { return filter(question, input, null); }
    public CompletableFuture<OwnedCall> filter(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call("filter", Values.object(question.json()), Values.object(input.json()), options == null ? null : Values.object(options.json())); }
    public CompletableFuture<OwnedCall> rank(Inputs.RequestQuestion question, Inputs.RequestInput input) { return rank(question, input, null); }
    public CompletableFuture<OwnedCall> rank(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call("rank", Values.object(question.json()), Values.object(input.json()), options == null ? null : Values.object(options.json())); }
    public CompletableFuture<OwnedCall> find(Inputs.RequestQuestion question, Inputs.RequestInput input) { return find(question, input, null); }
    public CompletableFuture<OwnedCall> find(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call("find", Values.object(question.json()), Values.object(input.json()), options == null ? null : Values.object(options.json())); }
    public CompletableFuture<OwnedCall> annotate(Inputs.RequestQuestion question, Inputs.RequestInput input) { return annotate(question, input, null); }
    public CompletableFuture<OwnedCall> annotate(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call("annotate", Values.object(question.json()), Values.object(input.json()), options == null ? null : Values.object(options.json())); }
    public CompletableFuture<OwnedCall> recognize(Inputs.RequestQuestion question, Inputs.RequestInput input) { return recognize(question, input, null); }
    public CompletableFuture<OwnedCall> recognize(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call("recognize", Values.object(question.json()), Values.object(input.json()), options == null ? null : Values.object(options.json())); }
    public CompletableFuture<OwnedCall> relate(Inputs.RequestQuestion question, Inputs.RequestInput input) { return relate(question, input, null); }
    public CompletableFuture<OwnedCall> relate(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call("relate", Values.object(question.json()), Values.object(input.json()), options == null ? null : Values.object(options.json())); }
    private CompletableFuture<OwnedCall> call(String function, Map<String,?> question, Map<String,?> input, Map<String,?> options) {
        var call = new LinkedHashMap<String,Object>();
        call.put("function", function); call.put("question", question); call.put("input", input);
        if (options != null) call.put("options", options);
        return execute(Map.of("schema", RequestVersion.VALUE, "call", call));
    }
    public CompletableFuture<OwnedCall> decide(Map<String,?> question, Map<String,?> input, Map<String,?> options) { return call("decide", question, input, options); }
    public CompletableFuture<OwnedCall> choose(Map<String,?> question, Map<String,?> input, Map<String,?> options) { return call("choose", question, input, options); }
    public CompletableFuture<OwnedCall> tag(Map<String,?> question, Map<String,?> input, Map<String,?> options) { return call("tag", question, input, options); }
    public CompletableFuture<OwnedCall> score(Map<String,?> question, Map<String,?> input, Map<String,?> options) { return call("score", question, input, options); }
    public CompletableFuture<OwnedCall> filter(Map<String,?> question, Map<String,?> input, Map<String,?> options) { return call("filter", question, input, options); }
    public CompletableFuture<OwnedCall> rank(Map<String,?> question, Map<String,?> input, Map<String,?> options) { return call("rank", question, input, options); }
    public CompletableFuture<OwnedCall> find(Map<String,?> question, Map<String,?> input, Map<String,?> options) { return call("find", question, input, options); }
    public CompletableFuture<OwnedCall> annotate(Map<String,?> question, Map<String,?> input, Map<String,?> options) { return call("annotate", question, input, options); }
    public CompletableFuture<OwnedCall> recognize(Map<String,?> question, Map<String,?> input, Map<String,?> options) { return call("recognize", question, input, options); }
    public CompletableFuture<OwnedCall> relate(Map<String,?> question, Map<String,?> input, Map<String,?> options) { return call("relate", question, input, options); }
    /** Observe persistence without waiting for the usage writer. */
    public synchronized UsagePersistence usagePersistence() {
        live(); return NativeSession.usage(pointer, "thinkthen_engine_usage_persistence_v1");
    }
    /** Drain current deltas; only usage-lock acquisition has a deadline. */
    public synchronized UsagePersistence finishUsageStatus() {
        live(); return NativeSession.usage(pointer, "thinkthen_engine_finish_usage_status_v1");
    }
    @Override public synchronized void close() {
        if (!pointer.equals(MemorySegment.NULL)) { NativeSession.free("thinkthen_engine_free", pointer); pointer = MemorySegment.NULL; }
    }
}
