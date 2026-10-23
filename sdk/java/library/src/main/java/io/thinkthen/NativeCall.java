package io.thinkthen;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.util.Iterator;
import java.util.concurrent.locks.ReentrantLock;

final class NativeCall implements AutoCloseable {
    record Packet(byte[] fields, byte[] detail, long position, boolean kept) {}
    private MemorySegment handle;
    private final Object lifetime = new Object();
    private final ReentrantLock driver = new ReentrantLock();
    private final Iterator<?> input;
    private final java.util.Map<Long, Object> originals;
    private long sent;
    private final Cancel cancel;
    NativeCall(MemorySegment handle, Iterator<?> input, Cancel cancel, java.util.Map<Long, Object> originals) {
        this.handle = handle;
        this.input = input;
        this.originals = originals;
        this.cancel = cancel;
        cancel.attach(this);
    }
    Packet next() {
        driver.lock();
        try (var arena = Arena.ofConfined()) {
            if (handle == MemorySegment.NULL) return null;
            var row = arena.allocate(Native.ROW);
            var error = arena.allocate(Native.BUFFER);
            while (true) {
                int status = (int) Native.call("next", handle, row, error);
                Native.check(status, error);
                if (status == 1) return null;
                if (status == 2) {
                    if (input.hasNext()) {
                        Object value = input.next();
                        sent++;
                        if (originals != null) originals.put(sent, value);
                        try (var sendArena = Arena.ofConfined()) {
                            if (value instanceof String text) {
                                var units = sendArena.allocateFrom(java.lang.foreign.ValueLayout.JAVA_CHAR, text.toCharArray());
                                Native.check((int) Native.call("send_utf16", handle, units, (long) text.length(), error), error);
                            } else {
                                byte[] bytes = Json.bytes(value);
                                Native.check((int) Native.call("send_json", handle, Native.bytes(sendArena, bytes), (long) bytes.length, error), error);
                            }
                        }
                    } else { Native.call("end_input", handle); }
                } else {
                    try { return new Packet(Native.copy(row.asSlice(0, Native.BUFFER.byteSize())), Native.copy(row.asSlice(Native.BUFFER.byteSize(), Native.BUFFER.byteSize())), row.get(Native.SIZE, 32), row.get(java.lang.foreign.ValueLayout.JAVA_BYTE, 40) != 0); }
                    finally { Native.call("row_free", row); }
                }
            }
        } finally { driver.unlock(); }
    }
    void cancel() {
        synchronized (lifetime) {
            if (handle != MemorySegment.NULL) Native.call("cancel", handle);
        }
    }
    @Override public void close() {
        cancel();
        driver.lock();
        try {
            synchronized (lifetime) {
                if (handle != MemorySegment.NULL) {
                    Native.call("free", handle);
                    handle = MemorySegment.NULL;
                }
            }
        } finally { driver.unlock(); cancel.detach(this); }
    }
}
