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
    public Engine(Inputs.EngineSettings settings) { this(settings, Surface.JAVA); }
    public Engine(Inputs.EngineSettings settings, Surface surface) { this(Values.object(settings.json()), surface); }
    public OwnedSession startSession(Inputs.Request request) { return startSession(Values.object(request.json())); }
    public Results.Plan plan(Inputs.Request request) { return plan(Values.object(request.json())); }
    public CompletableFuture<OwnedCall> execute(Inputs.Request request) { return execute(Values.object(request.json())); }
    /** Shared transport construction for the Kotlin and Scala facade packages. */
    private Engine(Map<String,?> settings, Surface surface) {
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
    private synchronized OwnedSession startSession(Map<String,?> request) {
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
    private synchronized Results.Plan plan(Map<String,?> request) {
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
    private CompletableFuture<OwnedCall> execute(Map<String,?> request) {
        var result = new CompletableFuture<OwnedCall>();
        final OwnedSession session;
        try {
            session = startSession(request);
            try { session.finish(); } catch (RuntimeException error) { session.close(); throw error; }
        }
        catch (RuntimeException error) { result.completeExceptionally(error); return result; }
        result.whenComplete((value,error) -> session.close());
        Thread reader = new Thread(() -> {
            var packets = new ArrayList<Results.SessionPacket>();
            Results.SessionPacketTerminal terminal = null;
            try {
                while (!result.isDone()) {
                    var next = session.tryRead();
                    if (next.packet() != null) {
                        packets.add(next.packet());
                        if (next.packet() instanceof Results.SessionPacketTerminal done) terminal = done;
                    }
                    if (next.ended()) {
                        var call = new OwnedCall(packets, terminal);
                        if (call.terminal().failure().state() == Presence.State.VALUE) result.completeExceptionally(new SessionFailure(call));
                        else result.complete(call);
                        return;
                    }
                    if (next.packet() == null) session.waitNative(false);
                }
            } catch (RuntimeException error) { result.completeExceptionally(error); }
        }, "thinkthen-session");
        reader.setDaemon(true); reader.start();
        return result;
    }
    public CompletableFuture<OwnedCall> decide(Inputs.RequestQuestion question, Inputs.RequestInput input) { return decide(question, input, null); }
    public CompletableFuture<OwnedCall> decide(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call(new Inputs.RequestCallDecide(), question, input, options); }
    public CompletableFuture<OwnedCall> choose(Inputs.RequestQuestion question, Inputs.RequestInput input) { return choose(question, input, null); }
    public CompletableFuture<OwnedCall> choose(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call(new Inputs.RequestCallChoose(), question, input, options); }
    public CompletableFuture<OwnedCall> tag(Inputs.RequestQuestion question, Inputs.RequestInput input) { return tag(question, input, null); }
    public CompletableFuture<OwnedCall> tag(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call(new Inputs.RequestCallTag(), question, input, options); }
    public CompletableFuture<OwnedCall> score(Inputs.RequestQuestion question, Inputs.RequestInput input) { return score(question, input, null); }
    public CompletableFuture<OwnedCall> score(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call(new Inputs.RequestCallScore(), question, input, options); }
    public CompletableFuture<OwnedCall> filter(Inputs.RequestQuestion question, Inputs.RequestInput input) { return filter(question, input, null); }
    public CompletableFuture<OwnedCall> filter(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call(new Inputs.RequestCallFilter(), question, input, options); }
    public CompletableFuture<OwnedCall> rank(Inputs.RequestQuestion question, Inputs.RequestInput input) { return rank(question, input, null); }
    public CompletableFuture<OwnedCall> rank(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call(new Inputs.RequestCallRank(), question, input, options); }
    public CompletableFuture<OwnedCall> find(Inputs.RequestQuestion question, Inputs.RequestInput input) { return find(question, input, null); }
    public CompletableFuture<OwnedCall> find(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call(new Inputs.RequestCallFind(), question, input, options); }
    public CompletableFuture<OwnedCall> annotate(Inputs.RequestQuestion question, Inputs.RequestInput input) { return annotate(question, input, null); }
    public CompletableFuture<OwnedCall> annotate(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call(new Inputs.RequestCallAnnotate(), question, input, options); }
    public CompletableFuture<OwnedCall> recognize(Inputs.RequestQuestion question, Inputs.RequestInput input) { return recognize(question, input, null); }
    public CompletableFuture<OwnedCall> recognize(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call(new Inputs.RequestCallRecognize(), question, input, options); }
    public CompletableFuture<OwnedCall> relate(Inputs.RequestQuestion question, Inputs.RequestInput input) { return relate(question, input, null); }
    public CompletableFuture<OwnedCall> relate(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return call(new Inputs.RequestCallRelate(), question, input, options); }
    private CompletableFuture<OwnedCall> call(Inputs.RequestCall call, Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) {
        call.question(question).input(input);
        if (options != null) call.options(options);
        return execute(new Inputs.Request().call(call));
    }
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
