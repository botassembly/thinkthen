package io.thinkthen.internal;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.util.HashSet;
import java.util.Set;
import java.util.concurrent.Callable;
import java.util.concurrent.CancellationException;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

/** The shared JVM driver owns all foreign memory and blocking native operations. */
public final class DriverClient implements AutoCloseable {
    private final ExecutorService workers = Executors.newThreadPerTaskExecutor( Thread.ofPlatform().daemon().name("thinkthen-native-",0).factory());
    private final Set<DriverCall> calls = new HashSet<>();
    private MemorySegment handle;
    private boolean closed;
    public DriverClient(byte[] settings) {
        var opening = submit(() -> {
                try (var arena = Arena.ofConfined()) {
                    var out = arena.allocate(ValueLayout.ADDRESS);
                    var error = Native.error(arena);
                    Native.check((int) Native.call("client_open_json",Native.bytes(arena,settings),(long) settings.length,out,error),error);
                    return out.get(ValueLayout.ADDRESS,0);
                }
            });
        try { handle = await(opening); }
        catch (RuntimeException | Error error) {
            opening.whenComplete((opened,failure) -> { if (opened != null) Native.call("client_free",opened); });
            workers.shutdown();
            throw translate(error);
        }
    }
    <T> CompletableFuture<T> submit(Callable<T> operation) {
        var result = new CompletableFuture<T>();
        workers.execute(() -> {
            try { result.complete(operation.call()); }
            catch (Throwable error) { result.completeExceptionally(error); }
        });
        return result;
    }
    static RuntimeException translate(Throwable error) {
        if (error instanceof ThinkThenException coded) return new DriverException(coded);
        if (error instanceof RuntimeException runtime) return runtime;
        if (error instanceof Error fatal) throw fatal;
        return new IllegalStateException(error);
    }
    static <T> T await(CompletableFuture<T> future) {
        try { return future.get(); }
        catch (InterruptedException error) {
            Thread.currentThread().interrupt();
            throw cancelled();
        } catch (ExecutionException error) { throw translate(error.getCause()); }
    }
    public static DriverError cancelledError() {
        try (var arena = Arena.ofConfined()) {
            var error = Native.error(arena);
            Native.check((int) Native.call("cancelled_error",error),error);
            throw new AssertionError("Native cancellation did not fail");
        } catch (ThinkThenException error) { return new DriverException(error).error(); }
    }
    static CancellationException cancelled() {
        var shared = cancelledError();
        var error = new CancellationException(shared.message());
        error.initCause(new DriverException(new ThinkThenException(shared.name(),shared.message(),shared.httpStatus())));
        return error;
    }
    public synchronized DriverCall open(byte[] request) {
        if (closed) throw new IllegalStateException("Client is closed");
        var call = new DriverCall(this,handle,request,null);
        calls.add(call);
        return call;
    }
    public synchronized DriverCall openRecords(byte[] request, java.util.List<DriverRecord> records) {
        if (closed) throw new IllegalStateException("Client is closed");
        var call = new DriverCall(this,handle,request,java.util.List.copyOf(records));
        calls.add(call);
        return call;
    }
    synchronized void released(DriverCall call) {
        calls.remove(call);
        releaseClient();
    }
    private void releaseClient() {
        if (closed && calls.isEmpty() && handle != MemorySegment.NULL) {
            Native.call("client_free",handle);
            handle = MemorySegment.NULL;
            workers.shutdown();
        }
    }
    @Override public synchronized void close() {
        if (closed) return;
        closed = true;
        for (var call : Set.copyOf(calls)) call.close();
        releaseClient();
    }
    /** Admit a host question through the engine without opening a call. */
    public static void validateQuestion(byte[] json) {
        try (var arena = Arena.ofConfined()) {
            var out = arena.allocate(ValueLayout.ADDRESS);
            var error = Native.error(arena);
            try {
                Native.check((int) Native.call("question_from_json",Native.bytes(arena,json),(long) json.length,out,error),error);
            } finally {
                var question = out.get(ValueLayout.ADDRESS,0);
                if (!question.equals(MemorySegment.NULL)) Native.call("question_free",question);
            }
        } catch (RuntimeException error) { throw translate(error); }
    }
    /** Translate malformed native data at either JVM decoding boundary. */
    public static DriverException invalidRow(Throwable cause) {
        var error = new ThinkThenException(Contract.ABI_CODE,Contract.ABI_MESSAGE);
        error.initCause(cause);
        return new DriverException(error);
    }
    /** Return the engine-owned family of a stable native code. */
    @SuppressWarnings("restricted")
    public static String codeFamily(int value) {
        var pointer = (MemorySegment) Native.call("code_family",value);
        if (pointer.equals(MemorySegment.NULL)) throw invalidRow(new IllegalArgumentException("Missing native error family"));
        return pointer.reinterpret(Long.MAX_VALUE).getString(0);
    }
    // The ABI returns static NUL-terminated code names that remain valid for the process.
    @SuppressWarnings("restricted")
    public static java.util.List<DriverCode> codes() {
        long count = (long) Native.call("code_count");
        var result = new java.util.ArrayList<DriverCode>();
        for (long index = 0; index < count; index++) {
            int value = (int) Native.call("code_at", index);
            var pointer = (MemorySegment) Native.call("code_name", value);
            String name = pointer.reinterpret(Long.MAX_VALUE).getString(0);
            result.add(new DriverCode(value, name, codeFamily(value)));
        }
        return java.util.List.copyOf(result);
    }
}
