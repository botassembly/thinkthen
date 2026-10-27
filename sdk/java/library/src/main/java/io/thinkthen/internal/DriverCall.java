package io.thinkthen.internal;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.util.concurrent.CancellationException;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionStage;

/** One native operation may run at a time; cancellation alone may race it. */
public final class DriverCall implements AutoCloseable {
    public sealed interface Event permits NeedsInput, End, DriverRow {}
    public record NeedsInput(long version) implements Event {}
    public record End() implements Event {}
    public record DriverRow(byte[] fields, byte[] detail, long position) implements Event {}
    private final Object lifetime = new Object();
    private final java.util.concurrent.locks.ReentrantLock serial = new java.util.concurrent.locks.ReentrantLock();
    private final DriverClient client;
    private MemorySegment handle = MemorySegment.NULL;
    private DriverEvents events;
    private boolean cancelled;
    private boolean releasing;
    private CompletableFuture<?> active;
    private final CompletableFuture<Void> released = new CompletableFuture<>();
    DriverCall(DriverClient client, MemorySegment clientHandle, byte[] request, java.util.List<DriverRecord> initial) {
        this.client = client;
        synchronized (lifetime) {
            active = client.submit(() -> {
                synchronized (lifetime) { if (cancelled) return null; }
                try (var arena = Arena.ofConfined()) {
                    var out = arena.allocate(ValueLayout.ADDRESS);
                    var error = Native.error(arena);
                    if (initial == null) Native.check((int) Native.call("call_open_json", clientHandle, Native.bytes(arena,request), (long) request.length,1,out,error),error);
                    else {
                        var records = arena.allocate(NativeLayouts.RECORD_INPUT,Math.max(1,initial.size()));
                        for (int i = 0; i < initial.size(); i++) {
                            var record = records.asSlice(i * NativeLayouts.RECORD_INPUT.byteSize(),NativeLayouts.RECORD_INPUT.byteSize());
                            var value = initial.get(i);
                            record.set(ValueLayout.JAVA_INT,NativeLayouts.RECORD_INPUT.byteOffset(java.lang.foreign.MemoryLayout.PathElement.groupElement("struct_size")),(int) NativeLayouts.RECORD_INPUT.byteSize());
                            record.set(ValueLayout.JAVA_INT,NativeLayouts.RECORD_INPUT.byteOffset(java.lang.foreign.MemoryLayout.PathElement.groupElement("kind")),value.kind());
                            record.set(ValueLayout.ADDRESS,NativeLayouts.RECORD_INPUT.byteOffset(java.lang.foreign.MemoryLayout.PathElement.groupElement("data")),value.bytes().length == 0 ? MemorySegment.NULL : Native.bytes(arena,value.bytes()));
                            record.set(Native.SIZE,NativeLayouts.RECORD_INPUT.byteOffset(java.lang.foreign.MemoryLayout.PathElement.groupElement("len")),(long) value.bytes().length);
                        }
                        Native.check((int) Native.call("call_open_records_json",clientHandle,Native.bytes(arena,request),(long)request.length,records,(long)initial.size(),NativeLayouts.RECORD_INPUT.byteSize(),1,out,error),error);
                    }
                    synchronized (lifetime) {
                        handle = out.get(ValueLayout.ADDRESS,0);
                        events = new DriverEvents(client,handle);
                        if (cancelled) Native.call("call_cancel",handle);
                    }
                }
                return null;
            });
        }
    }
    private <T> T operation(java.util.concurrent.Callable<T> operation) {
        serial.lock();
        try { return perform(operation); }
        finally { serial.unlock(); }
    }
    private <T> T perform(java.util.concurrent.Callable<T> operation) {
        CompletableFuture<?> previous;
        synchronized (lifetime) { previous = active; }
        try { DriverClient.await(previous); }
        catch (RuntimeException error) { close(); throw error; }
        CompletableFuture<T> future;
        synchronized (lifetime) {
            if (cancelled) throw DriverClient.cancelled();
            future = client.submit(operation);
            active = future;
        }
        try { return DriverClient.await(future); }
        catch (RuntimeException error) { close(); throw error; }
    }
    public void awaitAdmission() {
        CompletableFuture<?> opening;
        synchronized (lifetime) { opening = active; }
        DriverClient.await(opening);
    }
    public Event next() {
        return operation(() -> {
            try (var arena = Arena.ofConfined()) {
                var row = arena.allocate(Native.rowLayout());
                row.set(ValueLayout.JAVA_INT,Native.offset("struct_size"),(int) Native.rowLayout().byteSize());
                var error = Native.error(arena);
                long version = events.version();
                int status = (int) Native.call("call_next",handle,row,error);
                Native.check(status,error);
                if (status == 1) return new End();
                if (status == 2) return new NeedsInput(version);
                try {
                    var fields = Json.MAPPER.createObjectNode();
                    long position = row.get(Native.SIZE,Native.offset("position"));
                    fields.put("position",position);
                    fields.put("function",Contract.functionName(row.get(ValueLayout.JAVA_INT,Native.offset("function"))));
                    fields.put("status",Contract.statusName(row.get(ValueLayout.JAVA_INT,Native.offset("status"))));
                    double probability = row.get(ValueLayout.JAVA_DOUBLE,Native.offset("probability"));
                    if (!Double.isNaN(probability)) fields.put("probability",probability);
                    for (String field : new String[]{"id","answer","source","error"}) {
                        byte[] bytes = Native.rowBytes(row,field);
                        if (bytes.length != 0) fields.set(field,Json.read(bytes));
                    }
                    return new DriverRow(Json.bytes(fields),Native.rowBytes(row,"detail"),position);
                } finally { Native.call("row_free",row); }
            }
        });
    }
    public DriverEvents.Wait waitForEvent(long version) {
        synchronized (lifetime) {
            if (cancelled) throw DriverClient.cancelled();
            return events.waitForChange(version);
        }
    }
    public void sendText(String text) {
        operation(() -> {
            try (var arena = Arena.ofConfined()) {
                var error = Native.error(arena);
                Native.check((int) Native.call("call_send_utf16",handle,arena.allocateFrom(ValueLayout.JAVA_CHAR,text.toCharArray()),(long) text.length(),error),error);
            }
            return null;
        });
    }
    public void sendJson(byte[] bytes) {
        operation(() -> {
            try (var arena = Arena.ofConfined()) {
                var error = Native.error(arena);
                Native.check((int) Native.call("call_send_json",handle,Native.bytes(arena,bytes),(long) bytes.length,error),error);
            }
            return null;
        });
    }
    public void sendMissing() { signal("call_send_missing"); }
    public void sendInvalid() { signal("call_send_invalid"); }
    private void signal(String name) {
        operation(() -> {
            try (var arena = Arena.ofConfined()) {
                var error = Native.error(arena);
                Native.check((int) Native.call(name,handle,error),error);
            }
            return null;
        });
    }
    public void endInput() { operation(() -> { Native.call("call_end_input",handle); return null; }); }
    public boolean isCancelled() { synchronized (lifetime) { return cancelled; } }
    public void cancel() {
        synchronized (lifetime) {
            cancelled = true;
            if (handle != MemorySegment.NULL) Native.call("call_cancel",handle);
            if (events != null) events.wake();
        }
    }
    public CompletionStage<Void> releaseCompletion() { return released.minimalCompletionStage(); }
    @Override public void close() {
        CompletableFuture<?> pending;
        synchronized (lifetime) {
            cancel();
            if (releasing) return;
            releasing = true;
            pending = active;
        }
        pending.whenComplete((result,error) -> {
            DriverEvents retained;
            synchronized (lifetime) {
                if (handle != MemorySegment.NULL) { Native.call("call_free",handle); handle = MemorySegment.NULL; }
                retained = events;
            }
            var done = retained == null ? CompletableFuture.<Void>completedFuture(null) : retained.close();
            done.whenComplete((ignored,failure) -> {
                client.released(this);
                released.complete(null);
            });
        });
    }
}
