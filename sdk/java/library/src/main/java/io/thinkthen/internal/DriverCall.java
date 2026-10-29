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
    private volatile Thread driverThread;
    private final java.util.concurrent.ExecutorService worker = java.util.concurrent.Executors.newSingleThreadExecutor(task -> {
        var thread = Thread.ofPlatform().daemon().name("thinkthen-stream-",0).unstarted(task);
        driverThread = thread;
        return thread;
    });
    private final DriverClient client;
    private final String expectedFunction;
    private final MemorySegment clientHandle;
    private MemorySegment handle = MemorySegment.NULL;
    private DriverEvents events;
    private boolean cancelled;
    private boolean releasing;
    private final CompletableFuture<Void> admission;
    private final CompletableFuture<Void> released = new CompletableFuture<>();
    DriverCall(DriverClient client, MemorySegment clientHandle, byte[] request, java.util.List<DriverRecord> initial) {
        this.client = client;
        expectedFunction = Json.read(request).path("question").path("function").asText();
        this.clientHandle = clientHandle;
        synchronized (lifetime) {
            admission = submit(() -> {
                synchronized (lifetime) { if (cancelled) return null; }
                openNative(request,initial);
                return null;
            });
        }
    }
    private void openNative(byte[] request, java.util.List<DriverRecord> initial) {
        try (var arena = Arena.ofConfined()) {
            var out = arena.allocate(ValueLayout.ADDRESS);
            var error = Native.error(arena);
            if (initial == null) Native.check(NativeStream.call_open_json(clientHandle, Native.bytes(arena,request), (long) request.length,1,out,error),error);
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
                Native.check(NativeStream.call_open_records_json(clientHandle,Native.bytes(arena,request),(long)request.length,records,(long)initial.size(),NativeLayouts.RECORD_INPUT.byteSize(),1,out,error),error);
            }
            synchronized (lifetime) {
                handle = out.get(ValueLayout.ADDRESS,0);
                events = new DriverEvents(this,handle);
                if (cancelled) NativeStream.call_cancel(handle);
            }
        }
    }
    /** Preserve admission-before-encoding while sharing the stream's driver. */
    public void replaceRecords(byte[] request, java.util.List<DriverRecord> records) {
        var snapshot = java.util.List.copyOf(records);
        operation(() -> {
            synchronized (lifetime) {
                NativeStream.call_free(handle);
                handle = MemorySegment.NULL;
                events.close().toCompletableFuture().join();
                events = null;
            }
            openNative(request,snapshot);
            return null;
        });
    }
    /** Schedule work on this call's sole driver, including retained event waits. */
    public <T> CompletableFuture<T> submit(java.util.concurrent.Callable<T> operation) {
        synchronized (lifetime) {
            if (releasing) throw DriverClient.cancelled();
            var result = new CompletableFuture<T>();
            worker.execute(() -> {
                try { result.complete(operation.call()); }
                catch (Throwable error) { result.completeExceptionally(error); }
            });
            return result;
        }
    }
    private <T> T operation(java.util.concurrent.Callable<T> operation) {
        try {
            DriverClient.await(admission);
            synchronized (lifetime) { if (cancelled) throw DriverClient.cancelled(); }
            if (Thread.currentThread() == driverThread) return operation.call();
            return DriverClient.await(submit(operation));
        } catch (Throwable error) {
            close();
            throw DriverClient.translate(error);
        }
    }
    public void awaitAdmission() {
        DriverClient.await(admission);
    }
    public Event next() {
        return operation(() -> {
            try (var arena = Arena.ofConfined()) {
                var row = arena.allocate(Native.rowLayout());
                row.set(ValueLayout.JAVA_INT,Native.offset("struct_size"),(int) Native.rowLayout().byteSize());
                var error = Native.error(arena);
                long version = events.version();
                int status = NativeStream.call_next(handle,row,error);
                Native.check(status,error);
                if (status == 1) return new End();
                if (status == 2) return new NeedsInput(version);
                try {
                    var fields = Json.MAPPER.createObjectNode();
                    long position = row.get(Native.SIZE,Native.offset("position"));
                    fields.put("position",position);
                    fields.put("function",Contract.functionName(row.get(ValueLayout.JAVA_INT,Native.offset("function"))));
                    String nativeStatus = Contract.statusName(row.get(ValueLayout.JAVA_INT,Native.offset("status")));
                    fields.put("status",nativeStatus);
                    double probability = row.get(ValueLayout.JAVA_DOUBLE,Native.offset("probability"));
                    if (!Double.isNaN(probability)) fields.put("probability",probability);
                    for (String field : new String[]{"id","answer","source","error"}) {
                        byte[] bytes = Native.rowBytes(row,field);
                        if (field.equals("answer") && bytes.length != 0 && !"answered".equals(nativeStatus))
                            throw new ThinkThenException(Contract.ABI_CODE,Contract.ABI_MESSAGE);
                        if (bytes.length != 0) fields.set(field,Json.read(bytes));
                    }
                    byte[] detail = Native.rowBytes(row,"detail");
                    if (!expectedFunction.equals(fields.path("function").asText())) throw new ThinkThenException(Contract.ABI_CODE,Contract.ABI_MESSAGE);
                    validateOutcome(fields,detail.length != 0);
                    var source = fields.path("source");
                    if (!source.isObject() || !source.path("question_identity").isTextual() || !source.path("backend").isTextual() || !source.path("model").isTextual()
                        || (source.hasNonNull("name") && !source.get("name").isTextual())
                        || source.properties().stream().anyMatch(entry -> !java.util.Set.of("question_identity","backend","model","name").contains(entry.getKey())))
                        throw new ThinkThenException(Contract.ABI_CODE,Contract.ABI_MESSAGE);
                    if (expectedFunction.equals("annotate") && fields.path("answer").isObject())
                        fields.get("answer").forEach(entry -> validateOutcome(entry,entry.has("detail")));
                    return new DriverRow(Json.bytes(fields),detail,position);
                } catch (java.io.UncheckedIOException cause) {
                    throw DriverClient.invalidRow(cause);
                } finally { NativeStream.row_free(row); }
            }
        });
    }
    private static void validateOutcome(com.fasterxml.jackson.databind.JsonNode fields, boolean hasDetail) {
        boolean valid = switch (fields.path("status").asText()) {
            case "answered" -> fields.has("answer") && !fields.has("error");
            case "unsure" -> !fields.has("answer") && !fields.has("error") && fields.path("probability").isNumber();
            case "failed" -> !fields.has("answer") && !fields.has("probability") && !hasDetail && fields.has("error");
            default -> false;
        };
        if (!valid) throw new ThinkThenException(Contract.ABI_CODE,Contract.ABI_MESSAGE);
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
                Native.check(NativeStream.call_send_utf16(handle,arena.allocateFrom(ValueLayout.JAVA_CHAR,text.toCharArray()),(long) text.length(),error),error);
            }
            return null;
        });
    }
    public void sendJson(byte[] bytes) {
        operation(() -> {
            try (var arena = Arena.ofConfined()) {
                var error = Native.error(arena);
                Native.check(NativeStream.call_send_json(handle,Native.bytes(arena,bytes),(long) bytes.length,error),error);
            }
            return null;
        });
    }
    public void sendMissing() { signal(true); }
    public void sendInvalid() { signal(false); }
    private void signal(boolean missing) {
        operation(() -> {
            try (var arena = Arena.ofConfined()) {
                var error = Native.error(arena);
                Native.check((missing ? NativeStream.call_send_missing(handle,error) : NativeStream.call_send_invalid(handle,error)),error);
            }
            return null;
        });
    }
    public void endInput() { operation(() -> { NativeStream.call_end_input(handle); return null; }); }
    public boolean isCancelled() { synchronized (lifetime) { return cancelled; } }
    public void cancel() {
        synchronized (lifetime) {
            cancelled = true;
            if (handle != MemorySegment.NULL) NativeStream.call_cancel(handle);
            if (events != null) events.wake();
        }
    }
    public CompletionStage<Void> releaseCompletion() { return released.minimalCompletionStage(); }
    @Override public void close() {
        synchronized (lifetime) {
            cancel();
            if (releasing) return;
            releasing = true;
        }
        worker.execute(() -> {
            DriverEvents retained;
            synchronized (lifetime) {
                if (handle != MemorySegment.NULL) { NativeStream.call_free(handle); handle = MemorySegment.NULL; }
                retained = events;
            }
            var done = retained == null ? CompletableFuture.<Void>completedFuture(null) : retained.close();
            done.whenComplete((ignored,failure) -> {
                client.released(this);
                worker.shutdown();
                released.complete(null);
            });
        });
    }
}
