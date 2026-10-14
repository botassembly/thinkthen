package thinkthen;

import java.lang.foreign.*;
import java.util.*;
import java.util.concurrent.*;

/** Ten named asynchronous calls over one shared owned native session boundary. */
public final class Engine extends NamedCalls implements AutoCloseable {
    /** Outer JVM facade identity; native code validates the corresponding token. */
    public enum Surface { JAVA, KOTLIN, SCALA }
    private final Surface surface;
    private MemorySegment pointer;
    private final SessionWorkers workers = new SessionWorkers();
    public Engine(Inputs.EngineSettings settings) { this(settings, Surface.JAVA); }
    public Engine(Inputs.EngineSettings settings, Surface surface) { this(Values.object(settings.json()), surface); }
    public OwnedSession startSession(Inputs.Request request) { return startSession(Values.object(request.json())); }
    public Results.Plan plan(Inputs.Request request) { return plan(Values.object(request.json())); }
    public CompletableFuture<OwnedCall> execute(Inputs.Request request) { return execute(request, null); }
    public CompletableFuture<OwnedCall> execute(Inputs.Request request, Iterable<Inputs.RequestSessionDescriptor> feed) { return execute(Values.object(request.json()), feed); }
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
    @SuppressWarnings("restricted") // Native-owned pointers satisfy the C ABI lifetime contract.
    private NativeFailure engineFailure() {
        var view = (MemorySegment)NativeSession.call("thinkthen_error_view", ValueLayout.ADDRESS, new MemoryLayout[]{ValueLayout.ADDRESS}, pointer);
        return NativeResults.failure(view);
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
            return new OwnedSession(output.get(ValueLayout.ADDRESS, 0), workers);
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
        public Boolean decide() { return Results.CallAnswers.decide(this); }
        public String choose() { return Results.CallAnswers.choose(this); }
        public List<String> tag() { return Results.CallAnswers.tag(this); }
        public java.math.BigDecimal score() { return Results.CallAnswers.score(this); }
        public List<Object> filter() { return Results.CallAnswers.filter(this); }
        public List<java.math.BigInteger> rank() { return Results.CallAnswers.rank(this); }
        public Results.Find find() { return Results.CallAnswers.find(this); }
        public List<Results.Annotation> annotate() { return Results.CallAnswers.annotate(this); }
        public List<Results.Recognition> recognize() { return Results.CallAnswers.recognize(this); }
        public Results.Relation relate() { return Results.CallAnswers.relate(this); }

    }
    public static final class SessionFailure extends RuntimeException {
        private static final long serialVersionUID = 1L;
        private final transient OwnedCall call;
        SessionFailure(OwnedCall call) { super(call.terminal().failure().value().error().message()); this.call = call; }
        public OwnedCall call() { return call; }
        public Results.CallError failure() { return call.terminal().failure().value(); }
    }
    /** Future cancellation stops the native session and releases its owner promptly. */
    private CompletableFuture<OwnedCall> execute(Map<String,?> request, Iterable<Inputs.RequestSessionDescriptor> feed) {
        try { return execute(startSession(request), feed); }
        catch (RuntimeException error) { return CompletableFuture.failedFuture(error); }
    }
    private CompletableFuture<OwnedCall> execute(OwnedSession session, Iterable<Inputs.RequestSessionDescriptor> feed) {
        var result = new CompletableFuture<OwnedCall>();
        if (feed == null) {
            try { session.finish(); } catch (RuntimeException error) { session.close(); return CompletableFuture.failedFuture(error); }
        }
        result.whenComplete((value,error) -> session.close());
        if (feed != null) {
            workers.executor.execute(() -> {
                try {
                    for (var descriptor : feed) if (session.push(descriptor).join() == 2) break;
                    session.finish();
                } catch (RuntimeException error) { result.completeExceptionally(error); }
            });
        }
        workers.executor.execute(() -> {
            var packets = new ArrayList<Results.SessionPacket>();
            Results.SessionPacketTerminal terminal = null;
            try {
                while (!result.isDone()) {
                    var next = session.nextPacket();
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
                }
            } catch (RuntimeException error) { result.completeExceptionally(error); }
        });
        return result;
    }
    @Override protected CompletableFuture<OwnedCall> named(Inputs.RequestFunction function, Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) {
        try {
            final OwnedSession session;
            synchronized (this) { live(); session = new OwnedSession(NativeNamed.create(pointer, surface, function, question, input, options), workers); }
            return execute(session, null);
        } catch (RuntimeException error) { return CompletableFuture.failedFuture(error); }
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
        if (!pointer.equals(MemorySegment.NULL)) { NativeSession.free("thinkthen_engine_free", pointer); pointer = MemorySegment.NULL; workers.close(); }
    }
}
