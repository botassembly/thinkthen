package thinkthen;

import java.lang.foreign.*;

/** An owned native session with one producer and one reader. Cancel permits a drain. */
public final class OwnedSession implements AutoCloseable {
    private MemorySegment pointer;
    private final Arena arena = Arena.ofShared();
    private final MemorySegment status = arena.allocate(ValueLayout.JAVA_INT);
    private final MemorySegment output = arena.allocate(ValueLayout.ADDRESS);
    OwnedSession(MemorySegment pointer) { this.pointer = pointer; }
    private void live() { if (pointer.equals(MemorySegment.NULL)) throw new IllegalStateException("Session is closed"); }
    public synchronized void cancel() {
        if (!pointer.equals(MemorySegment.NULL)) NativeSession.free("thinkthen_session_cancel", pointer);
    }
    /** 0 admits, 1 requests retry of the same descriptor, 2 stops intake. */
    public synchronized int tryPush(Object descriptor) {
        live();
        try (Arena arguments = Arena.ofConfined()) {
            var bytes = NativeSession.bytes(arguments, descriptor);
            NativeSession.check((int)NativeSession.call("thinkthen_session_try_push", ValueLayout.JAVA_INT,
                new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS, NativeSession.SIZE, ValueLayout.ADDRESS}, pointer, bytes, bytes.byteSize(), status));
            return status.get(ValueLayout.JAVA_INT, 0);
        }
    }
    public synchronized void finish(Object readerFailure) {
        live();
        try (Arena arguments = Arena.ofConfined()) {
            var bytes = readerFailure == null ? MemorySegment.NULL : NativeSession.bytes(arguments, readerFailure);
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
        int state = status.get(ValueLayout.JAVA_INT, 0);
        if (state == 1) return new Read(null, false);
        if (state == 2) return new Read(null, true);
        if (state != 0) throw new IllegalStateException("Unknown native read status");
        var packet = output.get(ValueLayout.ADDRESS, 0);
        try (Arena arguments = Arena.ofConfined()) {
            var json = arguments.allocate(ValueLayout.ADDRESS);
            var length = arguments.allocate(NativeSession.SIZE);
            NativeSession.check((int)NativeSession.call("thinkthen_session_result_json", ValueLayout.JAVA_INT,
                new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS}, packet, json, length));
            return new Read(Results.SessionPacket.read(NativeSession.copied(json.get(ValueLayout.ADDRESS, 0), length.get(NativeSession.SIZE, 0))), false);
        } finally { NativeSession.free("thinkthen_session_result_free", packet); }
    }
    @Override public synchronized void close() {
        if (pointer.equals(MemorySegment.NULL)) return;
        cancel();
        NativeSession.free("thinkthen_session_free", pointer);
        pointer = MemorySegment.NULL;
        arena.close();
    }
}
