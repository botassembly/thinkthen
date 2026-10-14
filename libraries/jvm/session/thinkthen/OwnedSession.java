package thinkthen;

import java.lang.foreign.*;

/** An owned native session with one producer and one reader. Cancel permits a drain. */
public final class OwnedSession implements AutoCloseable {
    private MemorySegment pointer;
    private int waiters;
    private boolean closed;
    private java.util.concurrent.CompletableFuture<Void> inputWait, resultWait;
    private final SessionWorkers workers;
    private final Arena arena = Arena.ofShared();
    private final MemorySegment status = arena.allocate(ValueLayout.JAVA_INT);
    private final MemorySegment output = arena.allocate(ValueLayout.ADDRESS);
    OwnedSession(MemorySegment pointer, SessionWorkers workers) { this.pointer = pointer; this.workers = workers.retain(); }
    private void live() { if (closed || pointer.equals(MemorySegment.NULL)) throw new IllegalStateException("Session is closed"); }
    public synchronized void cancel() {
        if (!pointer.equals(MemorySegment.NULL)) NativeSession.free("thinkthen_session_cancel", pointer);
    }
    /** 0 admits, 1 requests retry of the same descriptor, 2 stops intake. */
    public synchronized int tryPush(Inputs.RequestSessionDescriptor descriptor) {
        live();
        try (Arena arguments = Arena.ofConfined()) {
            var bytes = NativeSession.bytes(arguments, descriptor.json());
            NativeSession.check((int)NativeSession.call("thinkthen_session_try_push", ValueLayout.JAVA_INT,
                new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS, NativeSession.SIZE, ValueLayout.ADDRESS}, pointer, bytes, bytes.byteSize(), status));
            return status.get(ValueLayout.JAVA_INT, 0);
        }
    }
    public synchronized void finish(Inputs.RequestReaderFailure readerFailure) {
        live();
        try (Arena arguments = Arena.ofConfined()) {
            var bytes = readerFailure == null ? MemorySegment.NULL : NativeSession.bytes(arguments, Values.json(readerFailure));
            NativeSession.check((int)NativeSession.call("thinkthen_session_finish", ValueLayout.JAVA_INT,
                new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS, NativeSession.SIZE}, pointer, bytes, bytes.byteSize()));
        }
    }
    public void finish() { finish(null); }
    /** Empty while pending; End is explicit so pending facts cannot look settled. */
    public record Read(Results.SessionPacket packet, boolean ended) {}
    public synchronized Read tryRead() {
        live();
        NativeSession.check((int)NativeSession.call("thinkthen_session_try_read", ValueLayout.JAVA_INT,
            new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS}, pointer, status, output));
        return readPacket(status, output);
    }
    private Read readPacket(MemorySegment stateSlot, MemorySegment packetSlot) {
        int state = stateSlot.get(ValueLayout.JAVA_INT, 0);
        if (state == NativeWait.READ_PENDING) return new Read(null, false);
        if (state == NativeWait.READ_END) return new Read(null, true);
        if (state != NativeWait.READ_RESULT) throw new IllegalStateException("Unknown native read status");
        var packet = packetSlot.get(ValueLayout.ADDRESS, 0);
        try {
            return new Read(Results.SessionPacket.read(NativeResults.packet(packet)), false);
        } finally { NativeSession.free("thinkthen_session_result_free", packet); }
    }
    /** Block for a packet; settlement drains the terminal facts after cancellation. */
    public Read nextPacket(boolean settlement) {
        final MemorySegment retained;
        synchronized (this) {
            live();
            retained = pointer; waiters++;
        }
        try (Arena outputs = Arena.ofConfined()) {
            var state = outputs.allocate(ValueLayout.JAVA_INT);
            var packet = outputs.allocate(ValueLayout.ADDRESS);
            NativeSession.check(NativeWait.nextPacket(retained, settlement ? NativeWait.SETTLEMENT : NativeWait.RESULT, state, packet));
            return readPacket(state, packet);
        } finally { completedWait(); }
    }
    public Read nextPacket() { return nextPacket(false); }
    public java.util.concurrent.CompletableFuture<Read> nextPacketAsync(boolean settlement) {
        var pending = java.util.concurrent.CompletableFuture.supplyAsync(() -> nextPacket(settlement), workers.executor);
        pending.whenComplete((value, error) -> { if (pending.isCancelled()) cancel(); });
        return pending;
    }
    /** Transfer one descriptor when native input room is available. */
    public java.util.concurrent.CompletableFuture<Integer> push(Inputs.RequestSessionDescriptor descriptor) {
        var payload = descriptor.json();
        var pending = java.util.concurrent.CompletableFuture.supplyAsync(() -> {
            final MemorySegment retained;
            synchronized (this) { live(); retained = pointer; waiters++; }
            try (Arena arguments = Arena.ofConfined()) {
                var bytes = NativeSession.bytes(arguments, payload);
                var state = arguments.allocate(ValueLayout.JAVA_INT);
                NativeSession.check(NativeWait.push(retained, bytes, bytes.byteSize(), state));
                return state.get(ValueLayout.JAVA_INT, 0);
            } finally { completedWait(); }
        }, workers.executor);
        pending.whenComplete((value, error) -> { if (pending.isCancelled()) cancel(); });
        return pending;
    }
    private synchronized void completedWait() {
        waiters--; if (closed && waiters == 0) release();
    }
    /** Wait consumes nothing and runs on a dedicated platform thread. Close defers free until it returns. */
    public synchronized java.util.concurrent.CompletableFuture<Void> waitReady(boolean input) {
        live();
        var pending = input ? inputWait : resultWait;
        if (pending != null) return pending;
        var result = new java.util.concurrent.CompletableFuture<Void>();
        if (input) inputWait = result; else resultWait = result;
        result.whenComplete((value, error) -> { if (result.isCancelled()) cancel(); });
        workers.executor.execute(() -> {
            RuntimeException failure = null;
            try { waitNative(input); }
            catch (RuntimeException error) { failure = error; }
            finally {
                synchronized (this) {
                    if (input && inputWait == result) inputWait = null;
                    if (!input && resultWait == result) resultWait = null;
                }
                if (failure == null) result.complete(null); else result.completeExceptionally(failure);
            }
        });
        return result;
    }
    void waitNative(boolean input) {
        final MemorySegment retained;
        synchronized (this) { live(); retained = pointer; waiters++; }
        try (Arena outputs = Arena.ofConfined()) {
            var ready = outputs.allocate(ValueLayout.JAVA_INT);
            if (!input) NativeSession.check(NativeWait.waitReady(retained, NativeWait.SETTLEMENT_ARM, 0, ready));
            while (true) {
                synchronized (this) {
                    var pending = input ? inputWait : resultWait;
                    if (closed || pending == null || pending.isCancelled()) return;
                }
                NativeSession.check(NativeWait.waitReady(retained,
                    input ? NativeWait.INPUT : NativeWait.INTERRUPTIBLE_SETTLEMENT, 50, ready));
                if (ready.get(ValueLayout.JAVA_INT, 0) != NativeWait.READY_TIMEOUT) return;
            }
        } finally { completedWait(); }
    }
    private void release() {
        NativeSession.free("thinkthen_session_free", pointer);
        pointer = MemorySegment.NULL;
        arena.close();
        workers.close();
    }
    @Override public synchronized void close() {
        if (closed) return;
        cancel(); closed = true;
        if (waiters == 0) release();
    }
}
